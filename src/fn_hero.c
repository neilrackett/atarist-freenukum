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

/* --------------------------------------------------------------- */

int fn_hero_act(
    fn_hero_t * hero,
    FnLevelSolids * solids)
{
  int heromoved = 0;

  FnHeroHealth * health = fn_hero_data_get_health(hero->data);
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  FnHeroImmunity * immunity = fn_hero_data_get_immunity(hero->data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(position);

  fn_hero_immunity_count_down(immunity);
  if (
          !fn_hero_immunity_hero_is_protected(immunity) &&
          fn_hero_data_get_gets_hurt(hero->data))
  {
    fn_hero_immunity_enable(immunity);
    fn_hero_health_decrease(health, 1);
  }

  if (solids == NULL) {
    return fn_hero_health_get(health);
  }

  if (fn_hero_data_get_motion(hero->data) == Motion_Walking) {
    /* our hero is moving */
    if (fn_hero_data_get_just_turned_around(hero->data)) {
      fn_hero_data_reset_just_turned_around(hero->data);
    } else {
      switch(fn_hero_data_get_direction(hero->data)) {
        case HorizontalDirection_Left:
            {
                FnGeometry new_position = hero_geometry;
                new_position.x -= FN_HALFTILE_WIDTH;
                if (!fn_hero_data_would_collide(hero->data, solids,
                            new_position.x,
                            new_position.y
                            ))
                {
                    fn_hero_position_move_to(
                            position, new_position.x, new_position.y);
                    heromoved = 1;
                }
            }
            break;
        case HorizontalDirection_Right:
            {
                FnGeometry new_position = hero_geometry;
                new_position.x += FN_HALFTILE_WIDTH;
                if (!fn_hero_data_would_collide(hero->data, solids,
                            new_position.x,
                            new_position.y
                            ))
                {
                    fn_hero_position_move_to(
                            position, new_position.x, new_position.y);
                    heromoved = 1;
                }
            }
            break;
        default:
          /* do nothing else */
          break;
      }
    }
  }

  if (!fn_hero_data_get_is_in_the_air(hero->data)) {
    /* our hero is standing or walking */
    fn_hero_data_set_vertical_speed(hero->data, 0);
  } else {
    /* our hero is jumping or falling */
    if (fn_hero_data_get_counter(hero->data) > 0) {
      /* jumping */
      size_t counter = fn_hero_data_counter_subtract(hero->data, 1);
      switch(counter) {
        case 3:
        case 2:
    fn_hero_data_set_vertical_speed(hero->data, 1);
          break;
        case 1:
        case 0:
    fn_hero_data_set_vertical_speed(hero->data, 0);
          break;
        default:
    fn_hero_data_set_vertical_speed(hero->data, 2);
          break;

      }
      int i = 0;
      for (i = 0; i < fn_hero_data_get_vertical_speed(hero->data); i++) {
        FnGeometry geometry = fn_hero_position_get_geometry(position);
        if (!fn_hero_data_would_collide(hero->data, solids,
              geometry.x,
              geometry.y - FN_HALFTILE_HEIGHT
              )) {
          fn_hero_position_move_y_to(
                  position, geometry.y - FN_HALFTILE_HEIGHT);
          heromoved = 1;
        } else {
          /* we bumped against the ceiling */
          fn_hero_data_set_counter(hero->data, 0);
        }
      }
    } else {
      /* falling */
      fn_hero_data_increase_vertical_speed(hero->data, 1);

      int i = 0;
      for (
              i = 0;
              i < fn_hero_data_get_vertical_speed(hero->data) / 2;
              i++)
      {
        FnGeometry geometry = fn_hero_position_get_geometry(position);
        if (!fn_hero_data_would_collide(hero->data, solids,
              geometry.x,
              geometry.y + FN_HALFTILE_HEIGHT
              )) {
          fn_hero_position_move_y_to(
                  position, geometry.y + FN_HALFTILE_HEIGHT);
          heromoved = 1;
        }
      }
    }
  }

  FnGeometry geometry = fn_hero_position_get_geometry(position);
  if (fn_hero_data_would_collide(hero->data, solids,
        geometry.x,
        geometry.y + FN_HALFTILE_HEIGHT
        )) {
    if (fn_hero_data_get_is_in_the_air(hero->data)) {
      SDL_Event event;
      event.type = SDL_USEREVENT;
      event.user.code = UserEvent_HeroLanded;
      event.user.data1 = hero;
      event.user.data2 = 0;
      SDL_PushEvent(&event);
    }
    /* we are standing on solid ground */
    fn_hero_data_land(hero->data);
    fn_hero_data_set_counter(hero->data, 0);
  } else {
    /* we fall down */
    if (fn_hero_data_get_counter(hero->data) == 0) {
      fn_hero_data_fall(hero->data);
      fn_hero_data_set_counter(hero->data, 0);
    }
  }

  if (heromoved) {
    SDL_Event event;
    event.type = SDL_USEREVENT;
    event.user.code = UserEvent_HeroMoved;
    event.user.data1 = hero;
    event.user.data2 = 0;
    SDL_PushEvent(&event);
  }

  return fn_hero_health_get(health);
}

/* --------------------------------------------------------------- */
