/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Shot functions
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

#ifndef FN_SHOT_H
#define FN_SHOT_H

/* --------------------------------------------------------------- */

typedef struct fn_shot_t fn_shot_t;

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

#include "fn_level.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

/**
 * The shot struct.
 */
struct fn_shot_t {
  /**
   * The position of the shot.
   */
  FnGeometry position;

  /**
   * Flag that indicates if the shot is (still) alive.
   */
  int is_alive;

  /**
   * The direction to which the shot was fired.
   */
  FnHorizontalDirection direction;

  /**
   * The counter for the animation.
   */
  Uint8 counter;

  /**
   * The down until the shot is to be removed.
   */
  Uint8 countdown;
};

/* --------------------------------------------------------------- */

/**
 * Create a shot.
 *
 * @param  level      The level inside which to create the shot.
 * @param  x          The initial x position (in pixels).
 * @param  y          The initial y position (in pixels).
 * @param  direction  The direction to which the shot goes.
 *
 * @return  The newly created shot.
 */
fn_shot_t * fn_shot_create(
    Uint16 x, Uint16 y, FnHorizontalDirection direction);

/* --------------------------------------------------------------- */

/**
 * Delete a shot.
 *
 * @param  shot      The shot to delete.
 */
void fn_shot_free(fn_shot_t * shot);

/* --------------------------------------------------------------- */

/**
 * Make the shot act one game cycle.
 *
 * @param  shot      The shot.
 *
 * @return 0 if shot is obsolete, 1 if it is still alive.
 */
Uint8 fn_shot_act(
        fn_shot_t * shot,
        FnHeroData * hero,
        fn_level_t * level,
        FnLevelData * level_data,
        FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * Blit a shot.
 *
 * @param  shot       The shot to delete.
 */
void fn_shot_blit(
        fn_shot_t * shot,
        SDL_Surface * target,
        const FnTileCache * tilecache,
        bool draw_collision_bounds);

/* --------------------------------------------------------------- */

/**
 * Call this function when a shot gets out of sight.
 * It tells the shot that it is no longer alive.
 *
 * @param  shot      The shot.
 */
void fn_shot_gets_out_of_sight(fn_shot_t * shot);

/* --------------------------------------------------------------- */

/**
 * Check if a shot touches an actor.
 *
 * @param  shot   The shot.
 * @param  actor  The actor.
 *
 * @return 1 if they touch, otherwise 0.
 */
Uint8 fn_shot_touches_actor(fn_shot_t * shot, FnLevelActor * actor);

/* --------------------------------------------------------------- */

/**
 * Check if the shot hits a solid tile
 *
 * @param  shot   The shot.
 *
 * @return 1 if the shot hits solid tile, otherwise 0.
 */
Uint8 fn_shot_hits_solid(
    fn_shot_t * shot,
    const FnLevelSolids * solids);

/* --------------------------------------------------------------- */

/**
 * Push a shot to a side. This function also does all the calculation
 * which is necessary for the shot to collide.
 *
 * @param  shot    The shot.
 * @param  offset  The offset to which the shot gets pushed.
 */
void fn_shot_push(
        fn_shot_t * shot,
        FnHeroData * hero,
        fn_level_t * level,
        FnLevelData * level_data,
        Sint16 offset,
        FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * Get the information if the shot is still alive.
 *
 * @param  shot   The shot.
 *
 * @return 1 if the shot is still alive, otherwise 0.
 */
Uint8 fn_shot_is_alive(fn_shot_t * shot);

/* --------------------------------------------------------------- */

/**
 * Return the current position of the shot.
 *
 * @param  shot   The shot.
 *
 * @return  The current position of the shot.
 */
FnGeometry fn_shot_get_position(fn_shot_t * shot);

/* --------------------------------------------------------------- */

#endif /* FN_SHOT_H */
