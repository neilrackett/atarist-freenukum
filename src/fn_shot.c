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

#include <stdlib.h>

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_shot.h"
#include "fn_object.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

fn_shot_t * fn_shot_create(
    Uint16 x, Uint16 y, FnHorizontalDirection direction)
{
  fn_shot_t * shot = malloc(sizeof(fn_shot_t));
  shot->position.w = 4;
  shot->position.h = FN_TILE_HEIGHT - 4;

  shot->position.x = x + FN_HALFTILE_WIDTH - shot->position.w / 2;
  shot->position.y = y + FN_TILE_HEIGHT - shot->position.h;
  shot->is_alive = 1;
  shot->direction = direction;
  shot->counter = 0;
  shot->countdown = 2;

  return shot;
}

/* --------------------------------------------------------------- */

void fn_shot_free(fn_shot_t * shot)
{
  free(shot);
}

/* --------------------------------------------------------------- */

Uint8 fn_shot_act(
        fn_shot_t * shot,
        FnHeroData * hero,
        fn_level_t * level,
        FnLevelData * level_data,
        FnLevelActorQueue * actor_queue)
{
  shot->counter++;
  shot->counter %= 4;

  if (shot->countdown == 1) {
    shot->countdown--;
    shot->is_alive = 0;
  }

  if (shot->countdown == 2) {
    int distance = 0;
    if (shot->direction == HorizontalDirection_Right) {
        distance = FN_HALFTILE_WIDTH;
    } else {
        distance = -FN_HALFTILE_WIDTH;
    }

    /* we push twice so that also the intermediate position gets
     * covered, not just the end position. */

    fn_shot_push(shot, hero, level, level_data, distance, actor_queue);
    fn_shot_push(shot, hero, level, level_data, distance, actor_queue);
  }
  return shot->is_alive;
}

/* --------------------------------------------------------------- */

void fn_shot_blit(
        fn_shot_t * shot,
        SDL_Surface * target,
        const FnTileCache * tilecache,
        bool draw_collision_bounds)
{
  if (shot->is_alive) {
    FnGeometry destrect;
    const FnTexture * tile = fn_tilecache_get_tile(tilecache,
        OBJ_SHOT+shot->counter);
    destrect.x =
      (shot->position.x + shot->position.w / 2 - FN_HALFTILE_WIDTH);
    destrect.y = shot->position.y;
    destrect.w = FN_TILE_WIDTH;
    destrect.h = shot->position.h;
    fn_texture_blit_to_sdl_surface(tile, NULL, target, &destrect);

    if (draw_collision_bounds) {
        Uint32 collision_color = FN_COLLISION_DEBUG_COLOR(target->format);
        fn_geometry_draw_outline(target, shot->position, collision_color);
    }
  }
}

/* --------------------------------------------------------------- */

void fn_shot_gets_out_of_sight(fn_shot_t * shot)
{
  shot->is_alive = 0;
}

/* --------------------------------------------------------------- */

Uint8 fn_shot_touches_actor(fn_shot_t * shot, FnLevelActor * actor)
{
  FnGeometry actorpos = fn_level_actor_get_position(actor);
  return fn_geometry_touches(actorpos, shot->position);
}

/* --------------------------------------------------------------- */

Uint8 fn_shot_hits_solid(
    fn_shot_t * shot, const FnLevelSolids * solids)
{
  return fn_level_solids_collides(solids, shot->position);
}

/* --------------------------------------------------------------- */

void fn_shot_push(
        fn_shot_t * shot,
        FnHeroData * hero,
        fn_level_t * level,
        FnLevelData * level_data,
        Sint16 offset,
        FnLevelActorQueue * actor_queue)
{
  if (shot->countdown == 2) {
    shot->position.x += offset;
    FnLevelActorsList * actors = fn_level_data_get_actors_list(level_data);
    for (size_t i = 0; i < fn_level_actors_list_count(actors); i++) {
      FnLevelActor * actor = fn_level_actors_list_get(actors, i);

      if (fn_level_actor_can_get_shot(actor) &&
          fn_shot_touches_actor(shot, actor) &&
          fn_level_actor_shot(actor, level_data, hero, actor_queue)) {
        shot->countdown = 1;
      }
    }
  }
  if (shot->countdown == 2) {
    if (fn_shot_hits_solid(shot, fn_level_data_get_solids(level_data))) {
      shot->countdown = 1;

      fn_level_actor_queue_push_back(
          actor_queue,
          ActorType_Explosion,
          shot->position.x + shot->position.w / 2 - FN_HALFTILE_WIDTH,
          shot->position.y);
    }
  }
}

/* --------------------------------------------------------------- */

Uint8 fn_shot_is_alive(fn_shot_t * shot)
{
  return shot->countdown == 2;
}

/* --------------------------------------------------------------- */

FnGeometry fn_shot_get_position(fn_shot_t * shot)
{
  return shot->position;
}

/* --------------------------------------------------------------- */

