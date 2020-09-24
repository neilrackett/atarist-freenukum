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

#include "fn_tile.h"
#include "fn_drop.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

FnTexture * fn_drop_load(FnFile * file, fn_environment_t * env)
{
    FnTexture * drop;
    FnGeometry geometry;
    size_t num_read = 0;

    FnTexture * tile;
    FnTileHeader h;

    drop = fn_texture_new_with_params(
        FN_DROP_WIDTH * FN_TILE_WIDTH,
        FN_DROP_HEIGHT * FN_TILE_HEIGHT,
        fn_environment_build_texture_creation_params(env));

    size_t num_loads = FN_DROP_WIDTH *  FN_DROP_HEIGHT;

    int  x      = 0;
    int  y      = 0;
    unsigned int width  = FN_TILE_WIDTH;
    unsigned int height = FN_TILE_HEIGHT;

    h.width = 2;
    h.height = 16;

    geometry = fn_geometry_create(x, y, width, height);

    while(num_read != num_loads)
    {
        tile = fn_tile_load(file,
            env,
            h,
            false);
        fn_texture_clone_to_texture(tile, NULL, drop, &geometry);
        fn_texture_free(tile);
        x += 16;
        if (x == 16 * FN_DROP_WIDTH)
        {
            x = 0;
            y += 16;
        }
        geometry.x = x;
        geometry.y = y;
        num_read++;
    }

    return drop;
}

/* --------------------------------------------------------------- */

