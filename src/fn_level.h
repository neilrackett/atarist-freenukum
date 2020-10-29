/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Level functions
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

#ifndef FN_LEVEL_H
#define FN_LEVEL_H

/* --------------------------------------------------------------- */

typedef struct fn_level_t fn_level_t;

/* --------------------------------------------------------------- */

/*
 * includes below use fn_level_t this is why they are not
 * at the beginning of this file because fn_level_t is
 * not yet defined there.
 */
#include "rusted.h"

/* --------------------------------------------------------------- */

/**
 * A struct representing a level.
 */
struct fn_level_t {
  /**
   * Level data
   */
  FnLevelData * data;
};

/* --------------------------------------------------------------- */

/**
 * Load a level from a file.
 *
 * @param  file  An already opened file descriptor to the level file.
 * @param  env   The environment of the game.
 *
 * @return  The fully loaded level. If it was not possible to load
 *          the level, NULL is returned. Examine errno in order
 *          to find out what error occured.
 */
fn_level_t * fn_level_load(
        FnFile * file,
        FnHeroData * hero,
        const FnTileCache * tilecache,
        FnTextureCreationParams texture_creation_params,
        FnLevelRaw * out_param_raw_to_fill);

/* --------------------------------------------------------------- */

/**
 * Destroy a level.
 *
 * @param  level  The level to destroy.
 */
void fn_level_free(fn_level_t * lv);

/* --------------------------------------------------------------- */

/**
 * Blit the current state of the level to an SDL Surface.
 *
 * @param  lv         The level to blit.
 * @param  target     The target SDL Surface.
 * @param  targetrect The target area to which to blit.
 * @param  sourcerect The source rectangle.
 * @param  backdrop1  The first backdrop type.
 * @param  backdrop2  The second backdrop type.
 */
void fn_level_blit_to_surface(
        fn_level_t * lv,
        const FnTileCache * tilecache,
        FnHeroData * hero,
        bool draw_collision_bounds,
        SDL_Surface * target,
        FnGeometry * targetrect,
        FnGeometry * sourcerect,
        FnTexture * backdrop1,
        FnTexture * backdrop2);

/* --------------------------------------------------------------- */

/**
 * Indicate if we are still playing this level.
 * 
 * @param  lv    The level we want to play.
 * 
 * @return  non-zero if we are still playing, otherwise zero.
 */
int fn_level_keep_on_playing(fn_level_t * lv);

/* --------------------------------------------------------------- */

/**
 * Call this function make the game act one step further.
 *
 * @param  lv  The level which should step.
 *
 * @return zero if the level is finished, otherwise non-zero.
 */
int fn_level_act(
        fn_level_t * lv,
        FnHeroData * hero,
        FnLevelActorQueue * actor_queue,
        FnLevelActorMessageQueue * actor_message_queue);

/* --------------------------------------------------------------- */

#endif /* FN_LEVEL_H */
