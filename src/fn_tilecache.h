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

#ifndef FN_TILECACHE_H
#define FN_TILECACHE_H

/* --------------------------------------------------------------- */

#include <SDL.h>
#include <unistd.h>

/* --------------------------------------------------------------- */

typedef struct fn_tilecache_t fn_tilecache_t;

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_tile.h"
#include "fn_environment.h"

/* --------------------------------------------------------------- */

/*
 * The tilecache holds ~1260 tiles. Giving each one its own
 * SDL_Surface used to cost three heap blocks - header, pixels,
 * mask - for a 160 byte payload: about 430K on a machine whose
 * whole heap is 600K, which left the 224K level stripe nowhere to
 * live and the playfield stayed black on a 1MB ST.
 *
 * Now the pixels and masks of every tile sit end to end in one
 * arena, and the per-tile bookkeeping is three small arrays: where
 * the tile starts, which class it belongs to and whether it is
 * opaque. A class is one geometry-and-mask combination (16x16
 * masked, 16x16 opaque, 8x8 masked) and holds everything else a
 * surface header carries.
 *
 * fn_tilecache_get_tile() still hands out an SDL_Surface *, so the
 * couple of hundred call sites keep working unchanged. It comes
 * from a small ring of headers per class, refilled per call, and
 * is therefore valid until FN_TILECACHE_RING further tiles of the
 * same class have been fetched. Every caller in the game blits the
 * tile it asked for straight away and none holds more than four at
 * once, so the ring is a wide margin - but do not stash one of
 * these pointers in a struct or across a frame.
 */

#define FN_TILECACHE_RING    16
#define FN_TILECACHE_CLASSES  4

typedef struct fn_tileclass_t {
    /* pre-built headers; only pixels, mask and the opacity flag
     * are written per fetch */
    SDL_Surface ring[FN_TILECACHE_RING];
    Uint16 next;
    Uint16 maskoff;    /* mask offset inside the tile, 0 if opaque */
    Uint32 tilesize;   /* pixels plus mask, in bytes               */
} fn_tileclass_t;

struct fn_tilecache_t {
    Uint8 ** tiledata;    /* per tile: the start of its arena slice */
    Uint8 *  tileclass;   /* per tile: index into classes           */
    Uint8 *  tileopaque;  /* per tile: STDL opaque_state, 1 or 2    */
    Uint8 *  arena;       /* every tile's pixels and mask           */
    SDL_PixelFormat * format;  /* shared by every tile              */
    fn_tileclass_t classes[FN_TILECACHE_CLASSES];
    Uint8 nclasses;
    Uint8 pixelsize;
    ssize_t size;
};

/* --------------------------------------------------------------- */

fn_tilecache_t * fn_tilecache_create();

/* --------------------------------------------------------------- */

int fn_tilecache_loadtiles(
        fn_tilecache_t * tc,
        fn_environment_t * env
        );

/* --------------------------------------------------------------- */

void fn_tilecache_destroy(
        fn_tilecache_t * tc);

/* --------------------------------------------------------------- */

SDL_Surface * fn_tilecache_get_tile(fn_tilecache_t * tc, size_t pos);

/* --------------------------------------------------------------- */

#endif /* FN_TILECACHE_H */

