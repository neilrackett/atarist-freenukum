/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Hero behavior functions
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

#ifndef FN_HERO_H
#define FN_HERO_H

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

typedef struct fn_hero_t fn_hero_t;

/* --------------------------------------------------------------- */

#include "fn_environment.h"
#include "fn_list.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

#define FN_HERO_MOTION_NONE        0
#define FN_HERO_MOTION_WALKING     1

#define FN_HERO_FLYING_FALSE       0
#define FN_HERO_FLYING_TRUE        1

#define FN_HERO_SHOOTING_FALSE     0
#define FN_HERO_SHOOTING_TRUE      1

/* --------------------------------------------------------------- */

/**
 * Our hero.
 */
struct fn_hero_t {
  /**
   * The motion state (none or walking).
   */
  Uint8 motion;

  /**
   * Is the hero going up (flying) or resting (falling/standing)?
   */
  Uint8 flying;

  /**
   * Is the hero currently shooting?
   */
  Uint8 shooting;

  /**
   * The counter for actions that take longer than one animation frame.
   */
  Uint8 counter;

  /**
   * The tile number that currently represents the hero.
   */
  int tilenr;

  /**
   * The current representation of the hero within the tile cache.
   */
  Uint8 animationframe;
  /**
   * The number of frames inside the current animation.
   */
  Uint8 num_animationframes;

  /**
   * The hero data.
   */
  FnHeroData * data;

  /**
   * The speed with which our hero falls or jumps.
   * Either 0, 1 or 2.
   */
  Uint8 verticalspeed;

  /**
   * A contdown how long the hero is immune when he was hurt.
   */
  Uint8 immunitycountdown;

  /**
   * The duration how long our hero is immune after being hurt.
   */
  Uint8 immunityduration;

  /**
   * True as long as any actors are hurting the hero.
   */
  bool gets_hurt;

  /**
   * Indicates if the hero is currently moving horizontally.
   */
  Uint8 is_moving_horizontally;
};

/* --------------------------------------------------------------- */

typedef void (*fn_hero_changed_f)(fn_hero_t *);

/* --------------------------------------------------------------- */

/**
 * Blit the hero.
 *
 * @param  hero        The hero.
 * @param  target      The target surface.
 * @param  level       The level to which the hero is blit.
 *                     Can be NULL in order to blit without a level.
 */
void fn_hero_blit(
    fn_hero_t * hero,
    SDL_Surface * target,
    const FnTileCache * tilecache,
    FnLevelSolids * solids,
    bool draw_collision_bounds);

/* --------------------------------------------------------------- */

/**
 * Let the hero act execute his next timed action.
 *
 * @param  hero  The hero that has to act.
 * @param  data  A pointer to the level inside which the hero
 *               is placed. Maybe I will find a better way to
 *               use a direct fn_level_t pointer instead of a
 *               void* one day, but currently this is the
 *               simplest solution because we have a chicken-egg
 *               problem if we include fn_hero.h inside fn_level.h
 *               and other way round too.
 * 
 * @return Zero if the hero has died, otherwise non-zero.
 */
int fn_hero_act(fn_hero_t * hero,
    FnLevelSolids * solids);

/* --------------------------------------------------------------- */

void fn_hero_next_animationframe(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

void fn_hero_update_animation(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Create a hero.
 *
 * @param  env  The environment for the hero.
 *
 * @return The newly created hero.
 */
fn_hero_t * fn_hero_create();

/* --------------------------------------------------------------- */

/**
 * Delete a hero.
 *
 * @param  hero  The hero to delete.
 */
void fn_hero_delete(fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Reset all values of a hero to default.
 *
 * @param  hero  The hero to reset.
 */
void fn_hero_reset(fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Update a hero for a new level.
 *
 * @param  hero  The hero that is to be updated.
 * @param  x     The initial x position of our hero (in pixels)
 * @param  y     The initial x position of our hero (in pixels)
 */
void fn_hero_enterlevel(
    fn_hero_t * hero,
    Uint32 x,
    Uint32 y);

/* --------------------------------------------------------------- */

void fn_hero_set_motion(
    fn_hero_t * hero,
    Uint8 motion);

/* --------------------------------------------------------------- */

Uint8 fn_hero_is_moving_horizontally(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

void fn_hero_set_flying(
    fn_hero_t * hero,
    Uint8 flying);

/* --------------------------------------------------------------- */

void fn_hero_set_shooting(
    fn_hero_t * hero,
    Uint8 shooting);

/* --------------------------------------------------------------- */

void fn_hero_set_counter(
    fn_hero_t * hero,
    Uint8 counter);

/* --------------------------------------------------------------- */

void fn_hero_jump(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Set the x position of the hero (in pixels).
 *
 * @param  hero  The hero.
 * @param  x     The desired x position.
 */
void fn_hero_set_x(
    fn_hero_t * hero, Uint32 x);

/* --------------------------------------------------------------- */

/**
 * Get the x position of the hero.
 *
 * @param  hero  The hero.
 *
 * @return  The x position.
 */
Uint32 fn_hero_get_x(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Set the y position of the hero (in pixels).
 *
 * @param  hero  The hero.
 * @param  y     The desired y position.
 */
void fn_hero_set_y(
    fn_hero_t * hero, Uint32 y);

/* --------------------------------------------------------------- */

/**
 * Get the y position of the hero.
 *
 * @param  hero  The hero.
 *
 * @return  The y position.
 */
Uint32 fn_hero_get_y(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Get the width of our hero.
 *
 * @param  hero  The hero.
 *
 * @return  The width.
 */
Uint16 fn_hero_get_w(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Get the height of our hero.
 *
 * @param  hero  The hero.
 *
 * @return  The height.
 */
Uint16 fn_hero_get_h(
    fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Calculate if our hero would  collide with a solid tile in the level.
 *
 * @param  hero        The hero.
 * @param  level       The level.
 * @param  x           The x coordinate.
 * @param  y           The y coordinate.
 *
 * @return 1 if hero collides, 0 if not.
 */
int fn_hero_would_collide(
        fn_hero_t * hero,
        FnLevelSolids * solids,
        Uint32 x,
        Uint32 y);

/* --------------------------------------------------------------- */

/**
 * Fire a shot.
 *
 * @param  hero  The hero.
 */
void fn_hero_fire_start(fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Stop firing a shot.
 *
 * @param  hero  The hero.
 */
void fn_hero_fire_stop(fn_hero_t * hero);

/* --------------------------------------------------------------- */

/**
 * Get the current position of the hero.
 *
 * @param  hero  The hero.
 *
 * @return The current position.
 */
FnGeometry fn_hero_get_position(fn_hero_t * hero);

/* --------------------------------------------------------------- */

#endif /* FN_HERO_H */
