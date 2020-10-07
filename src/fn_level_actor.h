/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Actor functions
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

#ifndef FN_LEVEL_ACTOR_H
#define FN_LEVEL_ACTOR_H

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_t fn_level_actor_t;

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

#include "fn_level.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

/**
 * The actor struct.
 */
struct fn_level_actor_t {
  /**
   * The general data of the actor.
   */
  FnLevelActorData * general;

  /**
   * Type-specific data.
   */
  void * specific;

  /**
   * Flag that indicates if the actor is being touched by the hero
   */
  int touches_hero;

  /**
   * Is the actor visible in the currently shown area?
   */
  Uint8 is_visible;

  /**
   * Does the actor act even if outside the visible area?
   */
  Uint8 acts_while_invisible;
};

/* --------------------------------------------------------------- */

/**
 * Create a new actor inside a level.
 *
 * @param  level  The level inside which the actor is located.
 * @param  type   The type of the actor.
 * @param  x      The x coordinate of the top left corner.
 * @param  y      The y coordinate of the top left corner.
 *
 * @return  The newly created actor.
 */
fn_level_actor_t * fn_level_actor_create(fn_level_t * level,
    FnLevelActorType type,
    Uint16 x,
    Uint16 y);

/* --------------------------------------------------------------- */

/**
 * Delete an actor and free its memory.
 *
 * @param  actor  The actor to delete.
 */
void fn_level_actor_free(fn_level_actor_t * actor, fn_level_t * level);

/* --------------------------------------------------------------- */

/**
 * Check if the hero does touch the actor.
 *
 * @param  actor  The actor.
 *
 * @return Non-zero if the hero touches, otherwise zero.
 */
int fn_level_actor_touches_hero(fn_level_actor_t * actor, fn_hero_t * hero);


/* --------------------------------------------------------------- */

/**
 * Check if the hero does touch the actor and accordingly
 * set the flags.
 *
 * @param  actor  The actor.
 */
void fn_level_actor_check_hero_touch(fn_level_actor_t * actor, fn_level_t * level, FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * The hero starts touching the actor.
 *
 * @param  actor  The actor which got touched.
 */
void fn_level_actor_hero_touch_start(fn_level_actor_t * actor, fn_level_t * level, FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * The hero stops touching the actor.
 *
 * @param  actor  The actor which got touched.
 */
void fn_level_actor_hero_touch_end(fn_level_actor_t * actor, fn_level_t * level);

/* --------------------------------------------------------------- */

/**
 * Tells if the hero can interact with an actor.
 *
 * @param  actor  The actor.
 *
 * @return 1 if the hero can interact with the actor, otherwise 0.
 */
Uint8 fn_level_actor_hero_can_interact(fn_level_actor_t * actor, fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * The hero starts to interact with the actor
 * (player pressed up arrow)
 *
 * @param  actor  The actor with whom the player interacts.
 */
void fn_level_actor_hero_interact_start(fn_level_actor_t * actor, fn_level_t * level);

/* --------------------------------------------------------------- */

/**
 * The hero stops to interact with the actor
 * (player released up arrow)
 *
 * @param  actor  The actor with whom the player interacts.
 */
void fn_level_actor_hero_interact_stop(fn_level_actor_t * actor, fn_level_t * level);

/* --------------------------------------------------------------- */


/**
 * The actor acts.
 *
 * @param  actor  The actor.
 *
 * @return  Zero if the actor died, otherwise a non-zero value.
 */
int fn_level_actor_act(fn_level_actor_t * actor, fn_level_t * level, FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * Blit the actor to the level.
 *
 * @param  actor  The actor.
 */
void fn_level_actor_blit(fn_level_actor_t * actor, fn_level_t * level);

/* --------------------------------------------------------------- */

/**
 * The actor gets hit by a shot.
 *
 * @param  actor  The actor.
 *
 * @return 1 if the actor absorbs the shot, otherwise 0.
 */
Uint8 fn_level_actor_shot(fn_level_actor_t * actor, fn_level_t * level, FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * Get the x coordinate (in pixels) of the actor.
 *
 * @param  actor  The actor.
 *
 * @return  The x coordinate.
 */
Uint16 fn_level_actor_get_x(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Get the y coordinate (in pixels) of the actor.
 *
 * @param  actor  The actor.
 *
 * @return  The y coordinate.
 */
Uint16 fn_level_actor_get_y(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Get the width (in pixels) of the actor.
 *
 * @param  actor  The actor.
 *
 * @return  The width.
 */
Uint16 fn_level_actor_get_w(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Get the height (in pixels) of the actor.
 *
 * @param  actor  The actor.
 *
 * @return  The height.
 */
Uint16 fn_level_actor_get_h(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Tells if an actor can get shot.
 *
 * @param  actor  The actor.
 *
 * @return 1 if the actor can get shot, otherwise 0.
 */
Uint8 fn_level_actor_can_get_shot(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Tells if an actor is in the foreground.
 *
 * @param  actor  The actor.
 *
 * @return 1 if the actor is in foreground, otherwise 0.
 */
Uint8 fn_level_actor_in_foreground(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Get the current position of the actor.
 *
 * @param  actor  The actor.
 *
 * @return A rectangle containing the current position of the actor.
 */
FnGeometry fn_level_actor_get_position(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

/**
 * Set the visibility of the actor.
 *
 * @param  actor       The actor.
 * @param  visibility  If 1 set the actor visible,
 *                     if 0 set it invisible.
 */
void fn_level_actor_set_visible(fn_level_actor_t * actor, Uint8 visibility);

/* --------------------------------------------------------------- */

/**
 * Get the visibility of the actor.
 *
 * @param  actor  The actor.
 *
 * @return 1 if the actor is visible, 0 if invisible.
 */
Uint8 fn_level_actor_is_visible(fn_level_actor_t * actor);

/* --------------------------------------------------------------- */

#endif /* FN_LEVEL_ACTOR_H */
