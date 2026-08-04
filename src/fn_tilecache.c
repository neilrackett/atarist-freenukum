/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Tile Cache
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

#include <SDL.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <string.h>
#include <stdlib.h>

/* --------------------------------------------------------------- */

#include "fn_tilecache.h"

/* --------------------------------------------------------------- */

/*
 * Slack at each end of the arena. STDL's unaligned blit path may
 * read (never write) one 8-byte group before or after the
 * rectangle it copies; between two tiles that lands on the
 * neighbour, so only the two ends of the arena need padding.
 */
#define FN_TILE_GUARD 8

/*
 * The graphics files, in the order that defines the global tile
 * numbers used throughout the game (fn_object.h). `count` is how
 * many tiles the game takes from each file - the on-disk header
 * disagrees for some of them and the game's number wins, as it
 * always has. `transparent` marks the files whose tiles have
 * see-through pixels; the rest need no mask at all, which saves
 * their masks and puts their blits on STDL's opaque fast path.
 */
static const struct {
    const char * name;
    Uint8        count;
    Uint8        transparent;
} fn_tilefiles[] = {
    { "BACK0.DN1",   48, 1 },
    { "BACK1.DN1",   48, 0 },
    { "BACK2.DN1",   48, 0 },
    { "BACK3.DN1",   48, 0 },
    { "SOLID0.DN1",  48, 1 },
    { "SOLID1.DN1",  48, 0 },
    { "SOLID2.DN1",  48, 0 },
    { "SOLID3.DN1",  48, 0 },
    { "ANIM0.DN1",   48, 1 },
    { "ANIM1.DN1",   48, 1 },
    { "ANIM2.DN1",   48, 1 },
    { "ANIM3.DN1",   48, 1 },
    { "ANIM4.DN1",   48, 1 },
    { "ANIM5.DN1",   48, 1 },
    { "OBJECT0.DN1", 50, 1 },
    { "OBJECT1.DN1", 50, 1 },
    { "OBJECT2.DN1", 50, 1 },
    { "MAN0.DN1",    48, 1 },
    { "MAN1.DN1",    48, 1 },
    { "MAN2.DN1",    48, 1 },
    { "MAN3.DN1",    48, 1 },
    { "MAN4.DN1",    48, 1 },
    { "FONT1.DN1",   50, 1 },
    { "FONT2.DN1",   50, 1 },
    { "BORDER.DN1",  48, 1 },
    { "NUMBERS.DN1", 48, 1 }
};

#define FN_TILEFILES (sizeof(fn_tilefiles) / sizeof(fn_tilefiles[0]))

/* --------------------------------------------------------------- */

fn_tilecache_t * fn_tilecache_create()
{
  fn_tilecache_t * tc = malloc(sizeof(fn_tilecache_t));
  if (tc != NULL) {
    memset(tc, 0, sizeof(fn_tilecache_t));
  }
  return tc;
}

/* --------------------------------------------------------------- */

/* the pixel geometry one tile of the given header ends up with */
static void fn_tilecache_geometry(const fn_tileheader_t * h,
    Uint8 pixelsize, int * w, int * height, int * groups)
{
  *w = h->width * 8 * pixelsize;
  *height = h->height * pixelsize;
  *groups = (*w + 15) >> 4;
}

/* --------------------------------------------------------------- */

/*
 * Find, or create, the class for a geometry. There are only a
 * handful of them in the whole game (16x16 with a mask, 16x16
 * without, 8x8 fonts) so a linear scan over the classes is the
 * whole lookup.
 */
static int fn_tilecache_class(fn_tilecache_t * tc,
    fn_environment_t * env, int w, int h, int groups, int masked)
{
  int i;
  Uint32 pixbytes = (Uint32)(groups * 8) * h;
  Uint16 maskoff = masked ? (Uint16)pixbytes : 0;

  for (i = 0; i != tc->nclasses; i++) {
    SDL_Surface * s = &tc->classes[i].ring[0];
    if (s->w == w && s->h == h
        && tc->classes[i].maskoff == maskoff) {
      return i;
    }
  }
  if (tc->nclasses == FN_TILECACHE_CLASSES) {
    return -1;
  }

  i = tc->nclasses++;
  tc->classes[i].next = 0;
  tc->classes[i].maskoff = maskoff;
  tc->classes[i].tilesize = pixbytes
    + (masked ? (Uint32)(groups * 2) * h : 0);

  {
    int slot;
    for (slot = 0; slot != FN_TILECACHE_RING; slot++) {
      SDL_Surface * s = &tc->classes[i].ring[slot];
      memset(s, 0, sizeof(*s));
      s->w = (Sint16)w;
      s->h = (Sint16)h;
      s->pitch = (Uint16)(groups * 8);
      s->planes = 4;
      s->colourkey = (Uint8)fn_environment_get_transparent(env);
      s->flags = masked ? SDL_SRCCOLORKEY : 0;
      s->clip.w = (Uint16)w;
      s->clip.h = (Uint16)h;
      s->maskstride = masked ? (Uint16)(groups * 2) : 0;
      s->format = tc->format;
    }
  }
  return i;
}

/* --------------------------------------------------------------- */

int fn_tilecache_loadtiles(fn_tilecache_t * tc,
    fn_environment_t * env)
{
    char * directory = fn_environment_get_datapath(env);
    Uint8 pixelsize = fn_environment_get_pixelsize(env);
    SDL_Surface * screen = fn_environment_get_screen(env);
    fn_tileheader_t headers[FN_TILEFILES];
    Uint8 classof[FN_TILEFILES];
    Uint32 arenabytes = 0;
    size_t ntiles = 0;
    size_t pathlen;
    size_t i;
    char * path;
    Uint8 * slice;
    int fd;

    if (tc == NULL) {
        return -1;
    }

    pathlen = strlen(directory) + 15;
    path = malloc(pathlen);
    if (path == NULL) {
        return -1;
    }

    /* the shared pixel format has to exist before the classes are
     * built, because every header points at it */
    tc->format = calloc(1, sizeof(SDL_PixelFormat)
        + sizeof(SDL_Palette) + 16 * sizeof(SDL_Color));
    if (tc->format == NULL) {
        free(path);
        return -1;
    }
    tc->format->palette = (SDL_Palette *)(tc->format + 1);
    tc->format->palette->colors = (SDL_Color *)(tc->format->palette + 1);
    tc->format->palette->ncolors = 16;
    tc->format->BitsPerPixel = 4;
    tc->format->BytesPerPixel = 1;
    /* on paletted displays blits copy raw indices, so the tiles
     * must share the screen's palette for colours to stay right */
    if (screen != NULL && screen->format != NULL
        && screen->format->palette != NULL) {
        memcpy(tc->format->palette->colors,
            screen->format->palette->colors,
            16 * sizeof(SDL_Color));
    }

    /*
     * Pass one reads the headers only: the arena cannot be sized
     * until every file's tile geometry is known. A file that will
     * not open keeps its slots (at the usual 16x16) so the global
     * tile numbers of the files behind it stay correct.
     */
    for (i = 0; i != FN_TILEFILES; i++) {
        int w, h, groups, cls;

        headers[i].tiles = fn_tilefiles[i].count;
        headers[i].width = 2;
        headers[i].height = 16;

        snprintf(path, pathlen, "%s/%s", directory,
            fn_tilefiles[i].name);
        fd = open(path, O_RDONLY);
        if (fd == -1) {
            printf("Failed to open file %s\n", path);
        } else {
            fn_tile_loadheader(fd, &headers[i]);
            close(fd);
        }

        fn_tilecache_geometry(&headers[i], pixelsize, &w, &h,
            &groups);
        cls = fn_tilecache_class(tc, env, w, h, groups,
            fn_tilefiles[i].transparent);
        if (cls < 0) {
            printf("Too many tile geometries\n");
            free(path);
            return -1;
        }
        classof[i] = (Uint8)cls;
        arenabytes += tc->classes[cls].tilesize
            * fn_tilefiles[i].count;
        ntiles += fn_tilefiles[i].count;
    }

    tc->tiledata = calloc(ntiles, sizeof(Uint8 *));
    tc->tileclass = calloc(ntiles, 1);
    tc->tileopaque = calloc(ntiles, 1);
    tc->arena = calloc(1, arenabytes + 2 * FN_TILE_GUARD);

    if (tc->tiledata == NULL || tc->tileclass == NULL
        || tc->tileopaque == NULL || tc->arena == NULL) {
        printf("Not enough memory for the tile cache\n");
        free(path);
        return -1;
    }

    /* pass two: hand each tile its slice of the arena and decode
     * into it through a throwaway header */
    slice = tc->arena + FN_TILE_GUARD;
    tc->pixelsize = pixelsize;
    tc->size = 0;

    for (i = 0; i != FN_TILEFILES; i++) {
        fn_tileclass_t * cls = &tc->classes[classof[i]];
        SDL_Surface tile = cls->ring[0];
        size_t n;

        snprintf(path, pathlen, "%s/%s", directory,
            fn_tilefiles[i].name);
        fd = open(path, O_RDONLY);
        if (fd != -1) {
            fn_tileheader_t skip;
            fn_tile_loadheader(fd, &skip);
        }

        for (n = 0; n != fn_tilefiles[i].count; n++) {
            size_t pos = (size_t)tc->size;

            tc->tiledata[pos] = slice;
            tc->tileclass[pos] = classof[i];
            tc->tileopaque[pos] = 1;
            tile.pixels = slice;
            tile.mask = (cls->maskoff != 0)
                ? slice + cls->maskoff : NULL;
            tile.opaque_state = 0;
            slice += cls->tilesize;
            tc->size++;

            if (fd != -1) {
                fn_tile_read(fd, env, &headers[i],
                    fn_tilefiles[i].transparent, &tile, 0, 0);
            }

            /* the opacity of a tile never changes, so scan its
             * mask once here instead of on every compose */
            if (tile.mask != NULL) {
                tc->tileopaque[pos] =
                    STDL_SurfaceIsOpaque(&tile) ? 1 : 2;
            }
        }

        if (fd != -1) {
            close(fd);
        }
    }

    free(path);
    return 0;
}

/* --------------------------------------------------------------- */

void fn_tilecache_destroy(fn_tilecache_t * tc)
{
    if (tc == NULL) {
        return;
    }
    free(tc->tiledata);
    free(tc->tileclass);
    free(tc->tileopaque);
    free(tc->arena);
    free(tc->format);
    free(tc);
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_tilecache_get_tile(fn_tilecache_t * tc, size_t pos)
{
    fn_tileclass_t * cls;
    SDL_Surface * s;
    Uint8 * data;

    if (tc == NULL || tc->tiledata == NULL || tc->size <= 0
        || pos >= (size_t)tc->size) {
        return NULL;
    }

    cls = &tc->classes[tc->tileclass[pos]];
    s = &cls->ring[cls->next];
    cls->next = (cls->next + 1) & (FN_TILECACHE_RING - 1);

    data = tc->tiledata[pos];
    s->pixels = data;
    if (cls->maskoff != 0) {
        s->mask = data + cls->maskoff;
    }
    s->opaque_state = tc->tileopaque[pos];
    return s;
}

/* --------------------------------------------------------------- */

