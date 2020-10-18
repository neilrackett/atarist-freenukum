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

#include "fn_list.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

#define FN_HERO_SHOOTING_FALSE     0
#define FN_HERO_SHOOTING_TRUE      1

/* --------------------------------------------------------------- */

/**
 * Our hero.
 */
struct fn_hero_t {
  /**
   * The hero data.
   */
  FnHeroData * data;
};

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

/**
 * Create a hero.
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
 * Get the current position of the hero.
 *
 * @param  hero  The hero.
 *
 * @return The current position.
 */
FnGeometry fn_hero_get_position(fn_hero_t * hero);

/* --------------------------------------------------------------- */

#endif /* FN_HERO_H */
