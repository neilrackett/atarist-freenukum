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

#ifndef FN_TILE_H
#define FN_TILE_H

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

typedef struct fn_tile_t fn_tile_t;

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_environment.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

FnTexture * fn_tile_load(
    FnFile * file,
    fn_environment_t * env,
    FnTileHeader h,
    bool has_transparency);

/* --------------------------------------------------------------- */

SDL_Surface * fn_tile_load_to_sdl(
        int fd,
        fn_environment_t * env,
        FnTileHeader h,
        Uint8 transparent
        );

/* --------------------------------------------------------------- */

int fn_tile_is_solid(
    Uint16 tile);

/* --------------------------------------------------------------- */

#endif /* FN_TILE_H */
