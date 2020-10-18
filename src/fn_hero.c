/*******************************************************************Item
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

#include <SDL/SDL.h>
#include <assert.h>

/* --------------------------------------------------------------- */

#include "fn_hero.h"
#include "fn_object.h"
#include "fn.h"
#include "fn_level.h"

/* --------------------------------------------------------------- */

fn_hero_t * fn_hero_create()
{
  fn_hero_t * hero = malloc(sizeof(fn_hero_t));

  hero->data = fn_hero_data_create();

  return hero;
}

/* --------------------------------------------------------------- */

void fn_hero_delete(fn_hero_t * hero)
{
  fn_hero_data_free(hero->data); hero->data = NULL;
  free(hero); hero = NULL;
}
