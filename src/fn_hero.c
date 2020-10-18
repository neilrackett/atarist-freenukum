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

fn_hero_t * fn_hero_create(fn_environment_t * env)
{
  fn_hero_t * hero = malloc(sizeof(fn_hero_t));

  hero->data = fn_hero_data_create();

  assert(env != NULL);
  fn_hero_reset(hero);

  return hero;
}

/* --------------------------------------------------------------- */

void fn_hero_delete(fn_hero_t * hero)
{
  fn_hero_data_free(hero->data); hero->data = NULL;
  free(hero); hero = NULL;
}

/* --------------------------------------------------------------- */

void fn_hero_reset(fn_hero_t * hero)
{
  fn_hero_set_x(hero, 0);
  fn_hero_set_y(hero, 0);

  fn_hero_data_reset(hero->data);

  hero->direction = HorizontalDirection_Right;
  hero->motion = FN_HERO_MOTION_NONE;
  hero->flying = FN_HERO_FLYING_FALSE;
  hero->shooting = FN_HERO_SHOOTING_FALSE;
  hero->counter = 0;
  hero->tilenr = HERO_STANDING_RIGHT;
  hero->verticalspeed = 0;

  hero->animationframe = 0;
  hero->num_animationframes = 1;

  hero->immunitycountdown = 0;
  hero->immunityduration = 16;
  hero->gets_hurt = false;

  hero->turned_around = 0;

  hero->is_moving_horizontally = 0;
}

/* --------------------------------------------------------------- */

void fn_hero_enterlevel(
    fn_hero_t * hero,
    Uint32 x,
    Uint32 y)
{
  fn_hero_set_x(hero, x);
  fn_hero_set_y(hero, y);
  hero->direction = HorizontalDirection_Right;
  hero->motion = FN_HERO_MOTION_NONE;
  hero->flying = FN_HERO_FLYING_FALSE;
  hero->shooting = FN_HERO_SHOOTING_FALSE;
  hero->verticalspeed = 0;

  hero->counter = 0;
  hero->tilenr = HERO_STANDING_RIGHT;

  hero->animationframe = 0;
  hero->num_animationframes = 1;

  FnHeroInventory * inventory = fn_hero_data_get_inventory(hero->data);
  fn_hero_inventory_unset(inventory, InventoryItem_KeyRed);
  fn_hero_inventory_unset(inventory, InventoryItem_KeyGreen);
  fn_hero_inventory_unset(inventory, InventoryItem_KeyBlue);
  fn_hero_inventory_unset(inventory, InventoryItem_KeyPink);
  fn_hero_data_set_hidden(hero->data, false);

  FnHeroFetchedLetterState * fetched_letter_state =
      fn_hero_data_get_fetched_letter_state(hero->data);
  fn_hero_fetched_letter_state_reset(fetched_letter_state);
}

/* --------------------------------------------------------------- */

void fn_hero_blit(
    fn_hero_t * hero,
    SDL_Surface * target,
    const FnTileCache * tilecache,
    FnLevelSolids * solids,
    bool draw_collision_bounds)
{
  int tilenr;
  const FnTexture * tile;

  if (fn_hero_data_get_hidden(hero->data)) {
    return;
  }
  if (hero->immunitycountdown % 2 != 0) {
    return;
  }

  FnHeroPosition * hero_position = fn_hero_data_get_position(hero->data);

  FnGeometry dstrect = fn_hero_position_get_geometry(hero_position);
  dstrect.x -= FN_HALFTILE_WIDTH;

  tilenr = hero->tilenr;
  if (hero->immunitycountdown > hero->immunityduration - 1) {
    if (hero->direction == HorizontalDirection_Left) {
      tilenr = HERO_SKELETON_LEFT;
    } else {
      tilenr = HERO_SKELETON_RIGHT;
    }
  }

  tile = fn_tilecache_get_tile(tilecache, tilenr);
  fn_texture_blit_to_sdl_surface(tile, NULL, target, &dstrect);

  dstrect.x += dstrect.w;
  tile = fn_tilecache_get_tile(tilecache, tilenr+1);
  fn_texture_blit_to_sdl_surface(tile, NULL, target, &dstrect);

  dstrect.x -= dstrect.w;
  dstrect.y += dstrect.h / 2;
  tile = fn_tilecache_get_tile(tilecache, tilenr+2);
  fn_texture_blit_to_sdl_surface(tile, NULL, target, &dstrect);

  dstrect.x += dstrect.w;
  tile = fn_tilecache_get_tile(tilecache, tilenr+3);
  fn_texture_blit_to_sdl_surface(tile, NULL, target, &dstrect);

  Uint32 collision_color = FN_COLLISION_DEBUG_COLOR(target->format);
  if (draw_collision_bounds) {
    FnHeroPosition * hero_position = fn_hero_data_get_position(hero->data);
    FnGeometry geometry = fn_hero_position_get_geometry(hero_position);
    fn_geometry_draw_outline(target, geometry, collision_color);

    Uint16 i = 0;
    Uint16 j = 0;

    for (i = geometry.x - FN_TILE_WIDTH;
        i < geometry.x + FN_TILE_WIDTH * 2;
        i += FN_TILE_WIDTH) {
      for (j = geometry.y - FN_TILE_HEIGHT;
          j < geometry.y + FN_TILE_HEIGHT * 3;
          j += FN_TILE_HEIGHT) {
        Uint16 tile_x = i / FN_TILE_WIDTH;
        Uint16 tile_y = j / FN_TILE_HEIGHT;
        if (solids != NULL) {
          if (fn_level_solids_get(solids, tile_x, tile_y))
          {
            FnGeometry obstacle;
            obstacle.x = tile_x * FN_TILE_WIDTH;
            obstacle.y = tile_y * FN_TILE_HEIGHT;
            obstacle.w = FN_TILE_WIDTH;
            obstacle.h = FN_TILE_HEIGHT;

            fn_geometry_draw_outline(target, obstacle, collision_color);
          }
        }
      }
    }
  }
}

/* --------------------------------------------------------------- */

int fn_hero_act(
    fn_hero_t * hero,
    FnLevelSolids * solids)
{
  int heromoved = 0;

  FnHeroHealth * health = fn_hero_data_get_health(hero->data);
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(position);

  if (hero->immunitycountdown > 0) {
    hero->immunitycountdown--;
  }
  if (hero->immunitycountdown == 0 && hero->gets_hurt) {
    hero->immunitycountdown = hero->immunityduration;
    fn_hero_health_decrease(health, 1);
  }

  if (solids == NULL) {
    return fn_hero_health_get(health);
  }

  if (hero->motion == FN_HERO_MOTION_WALKING) {
    /* our hero is moving */
    if (hero->turned_around) {
      hero->turned_around = 0;
    } else {
      switch(hero->direction) {
        case HorizontalDirection_Left:
            {
                FnGeometry new_position = hero_geometry;
                new_position.x -= FN_HALFTILE_WIDTH;
                if (!fn_hero_would_collide(hero, solids,
                            new_position.x,
                            new_position.y
                            ))
                {
                    fn_hero_position_move_to(
                            position, new_position.x, new_position.y);
                    heromoved = 1;
                    hero->is_moving_horizontally = 1;
                } else {
                    hero->is_moving_horizontally = 0;
                }
            }
            break;
        case HorizontalDirection_Right:
            {
                FnGeometry new_position = hero_geometry;
                new_position.x += FN_HALFTILE_WIDTH;
                if (!fn_hero_would_collide(hero, solids,
                            new_position.x,
                            new_position.y
                            ))
                {
                    fn_hero_position_move_to(
                            position, new_position.x, new_position.y);
                    heromoved = 1;
                    hero->is_moving_horizontally = 1;
                } else {
                    hero->is_moving_horizontally = 0;
                }
            }
            break;
        default:
          /* do nothing else */
          break;
      }
    }
  }

  if (hero->flying == FN_HERO_FLYING_FALSE) {
    /* our hero is standing or walking */
    hero->verticalspeed = 0;
  } else {
    /* our hero is jumping or falling */
    if (hero->counter > 0) {
      /* jumping */
      hero->counter--;
      switch(hero->counter) {
        case 3:
        case 2:
          hero->verticalspeed = 1;
          break;
        case 1:
        case 0:
          hero->verticalspeed = 0;
          break;
        default:
          hero->verticalspeed = 2;
          break;

      }
      int i = 0;
      for (i = 0; i < hero->verticalspeed; i++) {
        if (!fn_hero_would_collide(hero, solids,
              fn_hero_get_x(hero),
              fn_hero_get_y(hero) - FN_HALFTILE_HEIGHT
              )) {
          fn_hero_set_y(hero, fn_hero_get_y(hero) - FN_HALFTILE_HEIGHT);
          heromoved = 1;
        } else {
          /* we bumped against the ceiling */
          hero->counter = 0;
        }
      }
    } else {
      /* falling */
      if (hero->verticalspeed != 6) {
        hero->verticalspeed++;
      }

      int i = 0;
      for (i = 0; i < hero->verticalspeed/2; i++) {
        if (!fn_hero_would_collide(hero, solids,
              fn_hero_get_x(hero),
              fn_hero_get_y(hero) + FN_HALFTILE_HEIGHT
              )) {
          fn_hero_set_y(hero, fn_hero_get_y(hero) + FN_HALFTILE_HEIGHT);
          heromoved = 1;
        }
      }
    }
  }

  if (fn_hero_would_collide(hero, solids,
        fn_hero_get_x(hero),
        fn_hero_get_y(hero) + FN_HALFTILE_HEIGHT
        )) {
    if (hero->flying == FN_HERO_FLYING_TRUE) {
      SDL_Event event;
      event.type = SDL_USEREVENT;
      event.user.code = UserEvent_HeroLanded;
      event.user.data1 = hero;
      event.user.data2 = 0;
      SDL_PushEvent(&event);
    }
    /* we are standing on solid ground */
    fn_hero_set_flying(hero, FN_HERO_FLYING_FALSE);
    hero->counter = 0;
  } else {
    /* we fall down */
    if (hero->counter == 0) {
      fn_hero_set_flying(hero, FN_HERO_FLYING_TRUE);
      hero->counter = 0;
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

void fn_hero_next_animationframe(
    fn_hero_t * hero)
{
  hero->animationframe++;
  hero->animationframe %= hero->num_animationframes;
  /*
  if (hero->counter > 0) {
    hero->counter--;
  }
  */
}

/* --------------------------------------------------------------- */

void fn_hero_update_animation(
    fn_hero_t * hero)
{
  if (hero->flying == FN_HERO_FLYING_FALSE) {

    /* hero is standing or walking on ground */
    if (hero->motion == FN_HERO_MOTION_NONE) {

      /* hero is standing */
      if (hero->direction == HorizontalDirection_Left) {
        if (hero->shooting == FN_HERO_SHOOTING_TRUE) {
          hero->tilenr = HERO_WALKING_LEFT + 12;
        } else {
          hero->tilenr = HERO_STANDING_LEFT;
        }
      } else {
        if (hero->shooting == FN_HERO_SHOOTING_TRUE) {
          hero->tilenr = HERO_WALKING_RIGHT + 12;
        } else {
          hero->tilenr = HERO_STANDING_RIGHT;
        }
      }
      hero->num_animationframes = HERO_NUM_ANIM_STANDING;

    } else {

      /* hero is walking */
      if (hero->direction == HorizontalDirection_Left) {
        hero->tilenr = HERO_WALKING_LEFT + 4 * hero->animationframe;
      } else {
        hero->tilenr = HERO_WALKING_RIGHT + 4 * hero->animationframe;
      }
      hero->num_animationframes = HERO_NUM_ANIM_WALKING;
    }

  } else {

    /* hero is jumping or falling */
    if (hero->counter > 0) {

      /* hero is jumping */
      if (hero->direction == HorizontalDirection_Left) {
        hero->tilenr = HERO_JUMPING_LEFT;
      } else {
        hero->tilenr = HERO_JUMPING_RIGHT;
      }
      hero->num_animationframes = HERO_NUM_ANIM_JUMPING;

    } else {

      /* hero is falling */
      if (hero->direction == HorizontalDirection_Left) {
        hero->tilenr = HERO_FALLING_LEFT;
      } else {
        hero->tilenr = HERO_FALLING_RIGHT;
      }
      hero->num_animationframes = HERO_NUM_ANIM_FALLING;

    }
  }

}

/* --------------------------------------------------------------- */

void fn_hero_set_direction(
    fn_hero_t * hero,
    FnHorizontalDirection direction)
{
  if (hero->direction != direction) {
    hero->turned_around = 1;
    hero->direction = direction;
  }
}

/* --------------------------------------------------------------- */

void fn_hero_set_motion(
    fn_hero_t * hero,
    Uint8 motion)
{
  hero->motion = motion;
}

/* --------------------------------------------------------------- */

Uint8 fn_hero_is_moving_horizontally(
    fn_hero_t * hero)
{
  return hero->is_moving_horizontally;
}

/* --------------------------------------------------------------- */

void fn_hero_set_flying(
    fn_hero_t * hero,
    Uint8 flying)
{
  FnHeroInventory * inventory = fn_hero_data_get_inventory(hero->data);
  if (flying == FN_HERO_FLYING_TRUE) {
    if (hero->flying != flying) {
      if (fn_hero_inventory_is_set(inventory, InventoryItem_Boot)) {
        hero->counter = 7;
        hero->verticalspeed = 2;
      } else {
        hero->counter = 6;
        hero->verticalspeed = 2;
      }
    }
  }
  hero->flying = flying;
}

/* --------------------------------------------------------------- */

void fn_hero_set_shooting(
    fn_hero_t * hero,
    Uint8 shooting)
{
  hero->shooting = shooting;
}

/* --------------------------------------------------------------- */

void fn_hero_set_counter(
    fn_hero_t * hero,
    Uint8 counter)
{
  hero->counter = counter;
}

/* --------------------------------------------------------------- */

void fn_hero_jump(
    fn_hero_t * hero)
{
  fn_hero_set_counter(hero, 6);
  fn_hero_set_flying(hero, FN_HERO_FLYING_TRUE);
}

/* --------------------------------------------------------------- */

void fn_hero_set_x(
    fn_hero_t * hero, Uint32 x)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  fn_hero_position_move_x_to(position, x);
}

/* --------------------------------------------------------------- */

Uint32 fn_hero_get_x(
    fn_hero_t * hero)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  return fn_hero_position_get_geometry(position).x;
}

/* --------------------------------------------------------------- */

void fn_hero_set_y(
    fn_hero_t * hero, Uint32 y)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  fn_hero_position_move_y_to(position, y);
}

/* --------------------------------------------------------------- */

Uint32 fn_hero_get_y(
    fn_hero_t * hero)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  return fn_hero_position_get_geometry(position).y;
}

/* --------------------------------------------------------------- */

Uint16 fn_hero_get_w(
    fn_hero_t * hero)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  return fn_hero_position_get_geometry(position).w;
}

/* --------------------------------------------------------------- */

Uint16 fn_hero_get_h(
    fn_hero_t * hero)
{
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);
  return fn_hero_position_get_geometry(position).h;
}

/* --------------------------------------------------------------- */

int fn_hero_would_collide(
        fn_hero_t * hero,
        FnLevelSolids * solids,
        Uint32 x,
        Uint32 y)
{
  if (solids == NULL) {
    return 1;
  }

  FnHeroPosition * hero_position = fn_hero_data_get_position(hero->data);
  FnGeometry hero_rect = fn_hero_position_get_geometry(hero_position);
  hero_rect.x = x;
  hero_rect.y = y;

  return fn_level_solids_collides(solids, hero_rect);
}

/* --------------------------------------------------------------- */

void fn_hero_fire_start(fn_hero_t * hero)
{
  fn_hero_set_shooting(hero, FN_HERO_SHOOTING_TRUE);
}

/* --------------------------------------------------------------- */

void fn_hero_fire_stop(fn_hero_t * hero)
{
  fn_hero_set_shooting(hero, FN_HERO_SHOOTING_FALSE);
}

/* --------------------------------------------------------------- */

FnGeometry fn_hero_get_position(fn_hero_t * hero)
{
  FnHeroPosition * hero_position = fn_hero_data_get_position(hero->data);
  return fn_hero_position_get_geometry(hero_position);
}

/* --------------------------------------------------------------- */
