/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Tile loader
 *
 * *****************************************************************
 *
 * Copyright 2007-2008 Wolfgang Silbermayr
 *
 * *****************************************************************
 *
 * This file is part of Freenukum.
 * 
 * Freenukum is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Freenukum is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 *******************************************************************/

#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>

/* --------------------------------------------------------------- */

#include "fn_object.h"
#include "fn_tile.h"
#include "fn_draw.h"
#include "fn.h"

/* --------------------------------------------------------------- */

int fn_tile_loadheader(int fd, fn_tileheader_t * h)
{
    /* read the three header bytes explicitly; reading into the
     * struct reads sizeof(*h) bytes, which is 4 on platforms that
     * pad the struct (m68k), shifting the whole tile data stream
     * by one byte. */
    unsigned char buf[3];

    if (read(fd, buf, 3) != 3) {
        return 0;
    }
    h->tiles = buf[0];
    h->width = buf[1];
    h->height = buf[2];
    return 1;
}

/* --------------------------------------------------------------- */

int fn_tile_read(
        int fd,
        fn_environment_t * env,
        fn_tileheader_t * h,
        Uint8 transparent,
        SDL_Surface * tile,
        int x,
        int y)
{
    SDL_Rect r;
    size_t num_read = 0;
    fn_byterow_t br = {0, 0, 0, 0, 0};
    char readbuf[5];

    Uint8 pixelsize = fn_environment_get_pixelsize(env);

    size_t num_loads = h->width * h->height;

    r.x = x * pixelsize;
    r.y = y * pixelsize;
    r.w = 8 * pixelsize;
    r.h = 1 * pixelsize;

    Uint32 transparent_color = (
        transparent ?
        fn_environment_get_transparent(env) :
        0);


    /* bulk-read the whole tile: one system call instead of one
     * per 8-pixel group (tile loading was dominated by the
     * ~50000 GEMDOS traps this used to make). The buffer is
     * reused across calls - the ~1500 tiles of a tilecache load
     * otherwise spend real time in malloc/free. */
    static Uint8 * data = NULL;
    static size_t data_size = 0;
    if (num_loads * 5 > data_size) {
        free(data);
        data_size = num_loads * 5;
        data = malloc(data_size);
        if (data == NULL) {
            data_size = 0;
        }
    }
    if (data != NULL) {
        size_t total = num_loads * 5;
        size_t got = 0;
        while (got < total) {
            ssize_t n = read(fd, data + got, total - got);
            if (n <= 0) {
                break;
            }
            got += (size_t)n;
        }
    }

    if (data != NULL) {
        /* the tile's bytes are consumed either way, so a missing
         * destination does not desynchronise the file */
        if (tile == NULL) {
            return 0;
        }
        /* decode the whole image in one call */
        fn_draw_byterow_run(tile, x, y, data, num_loads,
            h->width, transparent_color, pixelsize);
        return 1;
    }

    while (num_read < num_loads && tile != NULL)
    {
        read(fd, readbuf, 5);
        br.trans  = readbuf[0];
        br.blue   = readbuf[1];
        br.green  = readbuf[2];
        br.red    = readbuf[3];
        br.brighten = readbuf[4];
        fn_draw_byterow(
                tile,
                r,
                &br,
                transparent_color,
                pixelsize);
        /* wrap without the 32-bit modulo: that division was a
         * library call per 8-pixel group on the 68000 */
        r.x += 8 * pixelsize;
        if (r.x >= (x + 8 * h->width) * pixelsize) {
            r.x = x * pixelsize;
            r.y += pixelsize;
        }
        num_read++;
    }

    return 1;
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_tile_load(
        int fd,
        fn_environment_t * env,
        fn_tileheader_t * h,
        Uint8 transparent)
{
    SDL_Surface * tile = fn_environment_create_surface(
        env,
        h->width * 8,
        h->height);

    if (tile == NULL) {
        return NULL;
    }
    fn_tile_read(fd, env, h, transparent, tile, 0, 0);
    return tile;
}

/* --------------------------------------------------------------- */

int fn_tile_is_solid(
    Uint16 tile)
{
  return tile >= SOLID_START && tile < ANIM_START;
}

/* --------------------------------------------------------------- */

