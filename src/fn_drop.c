/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Drop (Level Background) loader
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

#include <stdlib.h>
#include <unistd.h>

/* --------------------------------------------------------------- */

#include "fn_draw.h"
#include "fn_tile.h"
#include "fn_drop.h"

/* --------------------------------------------------------------- */

SDL_Surface * fn_drop_load(int fd, fn_environment_t * env)
{
    SDL_Surface * drop;
    size_t num_read = 0;
    fn_tileheader_t h;
    int x = 0;
    int y = 0;

    /* backdrops are fully opaque, so they need no colour key and
     * no transparency mask */
    drop = fn_environment_create_opaque_surface(env,
            FN_DROP_WIDTH * FN_TILE_WIDTH,
            FN_DROP_HEIGHT * FN_TILE_WIDTH);

    if (drop == NULL) {
        return NULL;
    }

    size_t num_loads = FN_DROP_WIDTH *  FN_DROP_HEIGHT;

    h.tiles = 0;
    h.width = 2;
    h.height = 16;

    /* decode each tile straight into the backdrop: the surface per
     * tile that this used to allocate and free 130 times over cost
     * more heap than the backdrop itself on a 1MB machine */
    while(num_read != num_loads)
    {
        fn_tile_read(fd, env, &h, 0, drop, x, y);
        x += FN_TILE_WIDTH;
        if (x == FN_TILE_WIDTH * FN_DROP_WIDTH)
        {
            x = 0;
            y += FN_TILE_HEIGHT;
        }
        num_read++;
    }

    return drop;
}

/* --------------------------------------------------------------- */

