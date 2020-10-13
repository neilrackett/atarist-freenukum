/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Level actor functions
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

#include "fn_level_actor.h"
#include "fn_object.h"
#include "fn_error_cmdline.h"

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef void (* fn_level_actor_create_function_t)(
        FnLevelActorCreateParams p);

typedef void (* fn_level_actor_free_function_t)(
        FnLevelActorFreeParams p);

typedef void (* fn_level_actor_hero_touch_start_function_t)(
        FnLevelActorHeroTouchStartParams p);

typedef void (* fn_level_actor_hero_touch_end_function_t)(
        FnLevelActorHeroTouchEndParams p);

typedef void (* fn_level_actor_interact_start_function_t)(
        FnLevelActorHeroInteractStartParams p);

typedef void (* fn_level_actor_interact_end_function_t)(
        FnLevelActorHeroInteractEndParams p);

typedef void (* fn_level_actor_act_function_t)(
        FnLevelActorActParams p);

typedef void (* fn_level_actor_blit_function_t)(
        FnLevelActorBlitParams p);

typedef void (* fn_level_actor_shot_function_t)(
        FnLevelActorShotParams p);

typedef void (* fn_level_actor_receive_message_function_t)(
        FnLevelActorReceiveMessageParams p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_functions_t {
    fn_level_actor_create_function_t create;
    fn_level_actor_free_function_t free;
    fn_level_actor_hero_touch_start_function_t hero_touch_start;
    fn_level_actor_hero_touch_end_function_t hero_touch_end;
    fn_level_actor_interact_start_function_t hero_interact_start;
    fn_level_actor_interact_end_function_t hero_interact_end;
    fn_level_actor_act_function_t act;
    fn_level_actor_blit_function_t blit;
    fn_level_actor_shot_function_t shot;
    fn_level_actor_receive_message_function_t receive_message;
} fn_level_actor_functions_t;

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef enum fn_level_actor_rocket_state_e {
  fn_level_actor_rocket_state_idle,
  fn_level_actor_rocket_state_flying
} fn_level_actor_rocket_state_e;

/* --------------------------------------------------------------- */

/**
 * The rocket.
 */
typedef struct fn_level_actor_rocket_data_t {
  /**
   * The state.
   */
  fn_level_actor_rocket_state_e state;
} fn_level_actor_rocket_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a rocket.
 *
 * @param  actor  The rocket actor.
 */
void fn_level_actor_function_rocket_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_rocket_data_t * data = malloc(
      sizeof(fn_level_actor_rocket_data_t));
  *(p.specific) = data;
  data->state = fn_level_actor_rocket_state_idle;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_rocket_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_rocket_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_rocket_act(
        FnLevelActorActParams p)
{
  fn_level_actor_rocket_data_t * data = p.specific;
  if (data->state == fn_level_actor_rocket_state_idle) {
    /* idle, so do nothing */
    return;
  }
  if (data->state == fn_level_actor_rocket_state_flying) {
    p.general->position.y -= FN_HALFTILE_HEIGHT;
    if (fn_level_solids_collides(&(p.level_data->solids), p.general->position)) {
      Uint16 tile_x = p.general->position.x / FN_TILE_WIDTH;
      Uint16 tile_y = p.general->position.y / FN_TILE_HEIGHT;
      fn_level_solids_set(&(p.level_data->solids), tile_x, tile_y, 0);
      fn_level_tiles_copy_from_to(
              &(p.level_data->tiles), tile_x, tile_y - 1, tile_x, tile_y);
      p.general->is_alive = 0;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_rocket_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_rocket_data_t * data = p.specific;
  FnGeometry destrect;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET);
  destrect.x = p.general->position.x;
  destrect.y = (p.general->position.y - FN_TILE_HEIGHT * 3);
  destrect.w = FN_TILE_WIDTH;
  destrect.h = FN_TILE_HEIGHT;

  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET + 1);
  destrect.y += FN_TILE_HEIGHT;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  destrect.y += FN_TILE_HEIGHT;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  destrect.y += FN_TILE_HEIGHT;

  tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET + 2);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET + 3);
  destrect.x -= FN_TILE_WIDTH;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  destrect.x += FN_TILE_WIDTH * 2;
  tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET + 4);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  if (data->state == fn_level_actor_rocket_state_flying) {
    destrect.x -= FN_TILE_WIDTH;
    destrect.y += FN_TILE_HEIGHT;
    tile = fn_tilecache_get_tile(p.tilecache, OBJ_ROCKET + 6);
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_rocket_shot(
        FnLevelActorShotParams p)
{
  fn_level_actor_rocket_data_t * data = p.specific;
  data->state = fn_level_actor_rocket_state_flying;
  /* TODO create animation */
  Uint16 tile_x = p.general->position.x / FN_TILE_WIDTH;
  Uint16 tile_y = (p.general->position.y + p.general->position.h) /
    FN_TILE_HEIGHT;
  fn_level_solids_set(&(p.level_data->solids), tile_x, tile_y, 0);
  fn_level_tiles_copy_from_to(
          &(p.level_data->tiles), tile_x, tile_y + 1, tile_x, tile_y);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef struct fn_level_actor_bomb_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames for the animation.
   */
  Uint8 num_frames;
  /**
   * The counter for the number of cycles.
   */
  Uint8 counter;
  /**
   * Flag showing if the bomb explodes to the left.
   */
  Uint8 explode_left;
  /**
   * Flag showing if the bomb explodes to the right.
   */
  Uint8 explode_right;
  /**
   * The threshold when the bomb should explode.
   */
  Uint8 explode_threshold;
  /**
   * The number of flames that should be produced by the bomb
   * to each direction.
   */
  Uint8 num_flames;
} fn_level_actor_bomb_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_bomb_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_bomb_data_t * data = malloc(
      sizeof(fn_level_actor_bomb_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;

  data->tile = ANIM_BOMB;
  data->current_frame = 0;
  data->num_frames = 2;
  data->counter = 0;
  data->explode_left = 1;
  data->explode_right = 1;
  data->explode_threshold = 12;
  data->num_flames = 4;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bomb_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_bomb_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bomb_act(
        FnLevelActorActParams p)
{
  fn_level_actor_bomb_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;

  data->counter++;

  if (data->counter < data->explode_threshold) {
    /* do nothing here, the counter counts anyway. */
  }
  else if (data->counter < data->explode_threshold + data->num_flames)
  {
    Uint8 distance = data->counter - data->explode_threshold;
    if (data->explode_left) {
      /* explode to the left if possible */
      if (
          /* check if the place for the flame is free */
          !fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x -
             distance * FN_TILE_WIDTH) / FN_TILE_WIDTH,
            p.general->position.y / FN_TILE_HEIGHT) &&
          /* check if there is solid place below */
          fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x -
             distance * FN_TILE_WIDTH) / FN_TILE_WIDTH,
            (p.general->position.y / FN_TILE_HEIGHT) + 1))
      {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_BombFire,
            p.general->position.x - distance * FN_TILE_WIDTH,
            p.general->position.y);
      } else {
        data->explode_left = 0;
      }
    }

    if (data->explode_right) {
      /* explode to the right if possible */
      if (
          /* check if the place for the flame is free */
          !fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x + distance * FN_TILE_WIDTH) /
            FN_TILE_WIDTH,
            p.general->position.y / FN_TILE_HEIGHT) &&
          /* check if there is solid place below */
          fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x + distance * FN_TILE_WIDTH) /
            FN_TILE_WIDTH,
            (p.general->position.y / FN_TILE_HEIGHT) + 1))
      {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_BombFire,
            p.general->position.x + distance * FN_TILE_WIDTH,
            p.general->position.y);
      } else {
        data->explode_right = 0;
      }
    }

  } else {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_bomb_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_bomb_data_t * data = p.specific;
  if (data->counter < data->explode_threshold) {
    const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
        data->tile + data->current_frame);
    FnGeometry destrect = p.general->position;
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef struct fn_level_actor_bombfire_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames total.
   */
  Uint8 num_frames;
  /**
   * Flag indicating if the flame is touching the hero.
   */
  Uint8 touching_hero;
} fn_level_actor_bombfire_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_bombfire_data_t * data = malloc(
      sizeof(fn_level_actor_bombfire_data_t));
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  *(p.specific) = data;
  data->tile = ANIM_BOMBFIRE;
  data->current_frame = 0;
  data->num_frames = 6;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_bombfire_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_hero_touch_start(
        FnLevelActorHeroTouchStartParams p)
{
  fn_level_actor_bombfire_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_hero_touch_end(
        FnLevelActorHeroTouchEndParams p)
{
  p.general->hurts_hero = false;
  fn_level_actor_bombfire_data_t * data = p.specific;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_act(
        FnLevelActorActParams p)
{
  fn_level_actor_bomb_data_t * data = p.specific;
  data->current_frame++;
  if (data->current_frame == data->num_frames) {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_bombfire_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Explosion data struct.
 */
typedef struct fn_level_actor_explosion_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames for the animation.
   */
  Uint8 num_frames;
} fn_level_actor_explosion_data_t;

/* --------------------------------------------------------------- */

/**
 * Create an explosion.
 *
 * @param  actor  The explosion actor.
 */
void fn_level_actor_function_explosion_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_explosion_data_t * data = malloc(
      sizeof(fn_level_actor_explosion_data_t));

  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 0;

  data->tile = ANIM_EXPLODE;
  data->current_frame = 0;
  data->num_frames = 6;
}

/* --------------------------------------------------------------- */

/**
 * Delete an explosion.
 *
 * @param  actor  The explosion actor.
 */
void fn_level_actor_function_explosion_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_explosion_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Act an explosion.
 *
 * @param  actor  The explosion actor.
 */
void fn_level_actor_function_explosion_act(
        FnLevelActorActParams p)
{
  fn_level_actor_explosion_data_t * data = p.specific;

  data->current_frame++;
  if (data->current_frame == data->num_frames) {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit an explosion.
 *
 * @param  actor  The explosion actor.
 */
void fn_level_actor_function_explosion_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_explosion_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Create a camera.
 *
 * @param  actor  The camera actor.
 */
void fn_level_actor_function_camera_create(
        FnLevelActorCreateParams p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 0;
}

/**
 * Blit the camera.
 *
 * @param  actor  The camera actor.
 */
void fn_level_actor_function_camera_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile;

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);
  size_t x = hero_geometry.x;
  if (x-1 > p.general->position.x) {
    tile = fn_tilecache_get_tile(p.tilecache,
        ANIM_CAMERA_RIGHT);
  } else if (x+1 < p.general->position.x) {
    tile = fn_tilecache_get_tile(p.tilecache,
        ANIM_CAMERA_LEFT);
  } else {
    tile = fn_tilecache_get_tile(p.tilecache,
        ANIM_CAMERA_CENTER);
  }
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

/**
 * A camera gets shot.
 *
 * @param  actor  The camera actor.
 */
void fn_level_actor_function_camera_shot(
        FnLevelActorShotParams p)
{
  p.general->is_alive = 0;
  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  fn_hero_score_add(score, 100);
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Score100,
      p.general->position.x,
      p.general->position.y);
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Explosion,
      p.general->position.x,
      p.general->position.y);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The score struct.
 * Scores are elements which get shown when the hero has fetched
 * an item for which he gets points. The score element slowly
 * flys up until it disappears by itself. Scores are animated by
 * two frames which appear alternating.
 */
typedef struct fn_level_actor_score_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The counter which defines for how many steps the score
   * will fly up until it disappears. Gets reduced every
   * time the act function is called.
   */
  Uint8 countdown;
} fn_level_actor_score_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a score.
 *
 * @param  actor  The score actor.
 */
void fn_level_actor_function_score_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_score_data_t * data = malloc(
      sizeof(fn_level_actor_score_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 1;
  data->countdown = 40;

  switch(p.general->actor_type) {
    case ActorType_Score100:
      data->tile = NUMB_100;
      break;
    case ActorType_Score200:
      data->tile = NUMB_200;
      break;
    case ActorType_Score500:
      data->tile = NUMB_500;
      break;
    case ActorType_Score1000:
      data->tile = NUMB_1000;
      break;
    case ActorType_Score2000:
      data->tile = NUMB_2000;
      break;
    case ActorType_Score5000:
      data->tile = NUMB_5000;
      break;
    case ActorType_Score10000:
      data->tile = NUMB_10000;
      break;
    case ActorType_ScoreBonus1Left:
      data->tile = NUMB_BONUS_1_LEFT;
      break;
    case ActorType_ScoreBonus1Right:
      data->tile = NUMB_BONUS_1_RIGHT;
      break;
    case ActorType_ScoreBonus2Left:
      data->tile = NUMB_BONUS_2_LEFT;
      break;
    case ActorType_ScoreBonus2Right:
      data->tile = NUMB_BONUS_2_RIGHT;
      break;
    case ActorType_ScoreBonus3Left:
      data->tile = NUMB_BONUS_3_LEFT;
      break;
    case ActorType_ScoreBonus3Right:
      data->tile = NUMB_BONUS_3_RIGHT;
      break;
    case ActorType_ScoreBonus4Left:
      data->tile = NUMB_BONUS_4_LEFT;
      break;
    case ActorType_ScoreBonus4Right:
      data->tile = NUMB_BONUS_4_RIGHT;
      break;
    case ActorType_ScoreBonus5Left:
      data->tile = NUMB_BONUS_5_LEFT;
      break;
    case ActorType_ScoreBonus5Right:
      data->tile = NUMB_BONUS_5_RIGHT;
      break;
    case ActorType_ScoreBonus6Left:
      data->tile = NUMB_BONUS_6_LEFT;
      break;
    case ActorType_ScoreBonus6Right:
      data->tile = NUMB_BONUS_6_RIGHT;
      break;
    case ActorType_ScoreBonus7Left:
      data->tile = NUMB_BONUS_7_LEFT;
      break;
    case ActorType_ScoreBonus7Right:
      data->tile = NUMB_BONUS_7_RIGHT;
      break;
    default:
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Delete a score.
 *
 * @param  actor  The score actor.
 */
void fn_level_actor_function_score_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_score_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Action for score.
 * 
 * @param  actor  The score actor.
 */
void fn_level_actor_function_score_act(
        FnLevelActorActParams p)
{
  fn_level_actor_score_data_t * data = p.specific;
  data->countdown--;
  p.general->position.y--;
  if (data->countdown == 0 || p.general->position.y == 0) {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit the score.
 *
 * @param  actor  The score actor.
 */
void fn_level_actor_function_score_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_score_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->countdown % 2));
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef struct fn_level_actor_unstablefloor_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The flag showing if the hero has already been standing upon the
   * floor
   */
  Uint8 touched;
  /**
   * The flag showing if the hero is currently touching the floor.
   */
  Uint8 touching;
} fn_level_actor_unstablefloor_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_unstablefloor_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_unstablefloor_data_t * data = malloc(
      sizeof(fn_level_actor_unstablefloor_data_t));
  *(p.specific) = data;
  data->tile = SOLID_START + 77;
  data->touched = 0;
  data->touching = 0;
  p.general->position.w = 0;
  p.general->position.h = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_unstablefloor_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_unstablefloor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_unstablefloor_act(
        FnLevelActorActParams p)
{
  fn_level_actor_unstablefloor_data_t * data = p.specific;

  Uint8 floorlength = 0;

  /*
   * postponed initialization because when floor is created,
   * the necessary information is not yet loaded from the level.
   */
  if (p.general->position.w == 0) {
    while (!fn_level_solids_get(&(p.level_data->solids),
          p.general->position.x / FN_TILE_WIDTH + floorlength,
          p.general->position.y / FN_TILE_HEIGHT))
    {
      fn_level_solids_set(&(p.level_data->solids),
          p.general->position.x / FN_TILE_WIDTH + floorlength,
          p.general->position.y / FN_TILE_HEIGHT,
          1);
      floorlength++;
    }
    p.general->position.w = floorlength * FN_TILE_WIDTH;
    p.general->position.h = FN_TILE_HEIGHT;
  }

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);

  if (fn_geometry_touches(hero_geometry, p.general->position))
  {
    if (data->touched) {
      floorlength = 0;
      while (floorlength < p.general->position.w / FN_TILE_WIDTH) {
        fn_level_solids_set(&(p.level_data->solids),
            p.general->position.x / FN_TILE_WIDTH + floorlength,
            p.general->position.y / FN_TILE_HEIGHT,
            0);
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_Explosion,
            p.general->position.x + floorlength * FN_TILE_WIDTH,
            p.general->position.y);
        fn_level_actor_queue_push_particle_firework(
            p.actor_queue,
            p.general->position.x + floorlength * FN_TILE_WIDTH,
            p.general->position.y, 4);
        floorlength++;
      }
      p.general->is_alive = 0;
    } else {
      data->touching = 1;
    }
  } else {
    if (data->touching) {
      data->touching = 0;
      data->touched = 1;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_unstablefloor_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_unstablefloor_data_t * data = p.specific;
  const FnTexture * tile = NULL;
  FnGeometry destrect = p.general->position;

  int i = 0;
  for (i = 0; i < (p.general->position.w / FN_TILE_WIDTH); i++) {
    tile = fn_tilecache_get_tile(p.tilecache, data->tile + i % 2);
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
    destrect.x += FN_TILE_WIDTH;
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The expanding floor struct.
 */
typedef struct fn_level_actor_expandingfloor_data_t {
  /**
   * A flag indicating if the actor is currently expanding.
   */
  Uint8 expanding;

  /**
   * A flag indicating if the actor is already expanded.
   */
  Uint8 finished;
} fn_level_actor_expandingfloor_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_expandingfloor_data_t * data =
    malloc(sizeof(fn_level_actor_expandingfloor_data_t));
  *(p.specific) = data;
  data->expanding = false;
  data->finished = false;

  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_expandingfloor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_act(
        FnLevelActorActParams p)
{
    fn_level_actor_expandingfloor_data_t * data = p.specific;

    if (data->expanding) {
        bool action = false;
        if (!fn_level_solids_get(
                    &(p.level_data->solids),
                    (p.general->position.x + p.general->position.w) /
                    FN_TILE_WIDTH,
                    (p.general->position.y) / FN_TILE_HEIGHT))
        {
            fn_level_solids_set(&(p.level_data->solids),
                    (p.general->position.x + p.general->position.w) /
                    FN_TILE_WIDTH,
                    (p.general->position.y) / FN_TILE_HEIGHT, 1);
            action = true;
            p.general->position.w += FN_TILE_WIDTH;
        }
        if (!action) {
            data->expanding = false;
            data->finished = true;
        }
    }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache, SOLID_EXPANDINGFLOOR);

  FnGeometry destrect = p.general->position;

  int i = 0;
  for (i = 0; i < (p.general->position.w / FN_TILE_WIDTH); i++) {
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
    destrect.x += FN_TILE_WIDTH;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_receive_message(
        FnLevelActorReceiveMessageParams p)
{
    if (p.message != ActorMessageType_Expand) {
        return;
    }
    fn_level_actor_expandingfloor_data_t * data = p.specific;
    if (data->expanding == false && data->finished == false) {
        data->expanding = true;
    }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The conveyor belt.
 */
typedef struct fn_level_actor_conveyor_data_t {
  /**
   * The animation counter.
   */
  Uint8 current_frame;
  /**
   * The number of animation frames.
   */
  Uint8 num_frames;
  /**
   * The direction to which the conveyor runs.
   */
  FnHorizontalDirection direction;
} fn_level_actor_conveyor_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_conveyor_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_conveyor_data_t * data = malloc(
      sizeof(fn_level_actor_conveyor_data_t));
  *(p.specific) = data;
  p.general->is_in_foreground = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;

  data->current_frame = 0;

  data->num_frames = 4;

  switch(p.general->actor_type) {
    case ActorType_ConveyorLeftMovingRightEnd:
      data->direction = HorizontalDirection_Left;
      break;
    case ActorType_ConveyorRightMovingRightEnd:
      data->direction = HorizontalDirection_Right;
      break;
    default: /* error */
      printf(__FILE__ ":%d: warning: conveyor #%d"
          " added which is no conveyor\n",
          __LINE__, p.general->actor_type);
      break;
  }

  /* find the beginning of the conveyor belt */
  Uint8 found_begin = 0;
  Uint16 tile = 0;
  while(!found_begin) {
    p.general->position.x -= FN_TILE_WIDTH;
    p.general->position.w += FN_TILE_WIDTH;
    tile = fn_level_tiles_get(&(p.level_data->tiles),
        p.general->position.x / FN_TILE_WIDTH,
        p.general->position.y / FN_TILE_HEIGHT);
    if (tile == SOLID_CONVEYORBELT_LEFTEND ||
        p.general->position.x == 0 ||
        tile == 0) {
      found_begin = 1;
      fn_level_tiles_set(&(p.level_data->tiles),
          p.general->position.x / FN_TILE_WIDTH,
          p.general->position.y / FN_TILE_HEIGHT,
          SOLID_BLACK);
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_conveyor_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_conveyor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_conveyor_act(
        FnLevelActorActParams p)
{
  fn_level_actor_conveyor_data_t * data = p.specific;
  if (data->direction == HorizontalDirection_Left) {
    if (data->current_frame == 0) {
      data->current_frame = data->num_frames;
    }
    data->current_frame--;
  } else {
    data->current_frame++;
    data->current_frame %= data->num_frames;
  }

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);

  int direction = (
      data->direction == HorizontalDirection_Right ?  1 : -1);
  if (
      hero_geometry.x + hero_geometry.w > p.general->position.x &&
      hero_geometry.x <
              p.general->position.x +
              p.general->position.w &&
      hero_geometry.y + hero_geometry.h == p.general->position.y) {
      fn_hero_position_push_horizontally(
              hero_position,
              &(p.level_data->solids),
              direction * FN_HALFTILE_WIDTH);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_conveyor_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_conveyor_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;

  Uint16 i = 0;

  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      SOLID_CONVEYORBELT_LEFTEND + data->current_frame);

  for (
          i = p.general->position.x;
          i < p.general->position.x + p.general->position.w;
          i+= FN_TILE_WIDTH) {
    destrect.x = i;
    if (i + FN_TILE_WIDTH == p.general->position.x + p.general->position.w)
    {
      /* last element */
      tile = fn_tilecache_get_tile(p.tilecache,
          SOLID_CONVEYORBELT_RIGHTEND + data->current_frame);
    }

    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

    tile = fn_tilecache_get_tile(p.tilecache,
        SOLID_CONVEYORBELT_CENTER + data->current_frame % 2);
  }

}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_create(
        FnLevelActorCreateParams p)
{
  p.general->position.w = FN_TILE_WIDTH * 2;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_free(
        FnLevelActorFreeParams p)
{
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_interact_start(
        FnLevelActorHeroInteractStartParams p)
{
    fn_info_message_queue_push(
            p.info_message_queue,
            "Not implemented yet.\n");
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile = NULL;
  FnGeometry destrect = p.general->position;

  tile = fn_tilecache_get_tile(p.tilecache, ANIM_BADGUYSCREEN);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.x += FN_TILE_WIDTH;
  tile = fn_tilecache_get_tile(p.tilecache, ANIM_BADGUYSCREEN + 1);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef struct fn_level_actor_hostileshot_data_t {
  /**
   * The tile number which is to be blitted in the level.
   */
  Uint16 tile;
  /**
   * Flag indicating if the shot is touching the hero.
   */
  Uint8 touching_hero;
  /**
   * The current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames.
   */
  Uint8 num_frames;
} fn_level_actor_hostileshot_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_hostileshot_data_t * data = malloc(
      sizeof(fn_level_actor_hostileshot_data_t));
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  *(p.specific) = data;
  data->current_frame = 0;
  data->num_frames = 2;
  if (p.general->actor_type == ActorType_HostileShotLeft) {
    data->tile = OBJ_BADSHOT;
  } else {
    data->tile = OBJ_BADSHOT + 2;
  }
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_hostileshot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_touch_start(
        FnLevelActorHeroTouchStartParams p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_touch_end(
        FnLevelActorHeroTouchEndParams p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;
  p.general->hurts_hero = false;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_act(
        FnLevelActorActParams p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;

  data->current_frame++;
  data->current_frame %= data->num_frames;

  if (p.general->actor_type == ActorType_HostileShotLeft) {
    p.general->position.x -= FN_HALFTILE_WIDTH;
  } else {
    p.general->position.x += FN_HALFTILE_WIDTH;
  }
  if (fn_level_solids_get(&(p.level_data->solids),
        p.general->position.x / FN_TILE_WIDTH,
        p.general->position.y / FN_TILE_HEIGHT)) {
    p.general->is_alive = 0;
    if (data->touching_hero) {
      p.general->hurts_hero = false;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_create(
        FnLevelActorCreateParams p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_free(
        FnLevelActorFreeParams p)
{
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_interact_start(
        FnLevelActorHeroInteractStartParams p)
{
    fn_info_message_queue_push(
            p.info_message_queue,
            "Not implemented yet.\n");
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile = NULL;
  FnGeometry destrect = p.general->position;
  tile = fn_tilecache_get_tile(p.tilecache, OBJ_NOTE);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The exitdoor struct.
 */
typedef struct fn_level_actor_exitdoor_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The counter which defines for how many steps the exitdoor
   * has to be animated until the hero disappears.
   * time the act function is called.
   */
  Uint8 counter;
  /**
   * Stores the state of the door. 0 is idle, 1 is opening,
   * 2 is closing.
   */
  Uint8 state;
} fn_level_actor_exitdoor_data_t;

/* --------------------------------------------------------------- */

/**
 * Create an exit door.
 *
 * @param  actor  The door actor.
 */
void fn_level_actor_function_exitdoor_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_exitdoor_data_t * data = malloc(
      sizeof(fn_level_actor_exitdoor_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH * 2;
  p.general->position.h = FN_TILE_HEIGHT * 2;
  data->counter = 0;
  data->state = 0;
  data->tile = ANIM_EXITDOOR;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

/**
 * Delete an exit door.
 *
 * @param  actor  The door actor.
 */
void fn_level_actor_function_exitdoor_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_exitdoor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Interaction of hero with exit door.
 *
 * @param  actor  The exitdoor actor.
 */
void fn_level_actor_function_exitdoor_interact_start(
        FnLevelActorHeroInteractStartParams p)
{
  fn_level_actor_exitdoor_data_t * data = p.specific;
  data->state = 1;
  p.level_data->level_passed = 1;
}

/* --------------------------------------------------------------- */

/**
 * Action for exit door.
 * 
 * @param  actor  The exitdoor actor.
 */
void fn_level_actor_function_exitdoor_act(
        FnLevelActorActParams p)
{
  fn_level_actor_exitdoor_data_t * data = p.specific;

  switch(data->state) {
    case 0: /* idle */
      break;
    case 1: /* door opening */
      data->counter++;
      if (data->counter == 4) {
        fn_hero_data_set_hidden(p.hero_data, true);
        data->state = 2;
        data->counter--;
      }
      break;
    case 2: /* door closing */
      if (data->counter == 0) {
        p.level_data->do_play = 0;
        fn_hero_data_set_hidden(p.hero_data, false);
      }
      data->counter--;
      break;
    default: /* error */
      fn_error_print_commandline("Exitdoor in invalid state");
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit the exitdoor.
 *
 * @param  actor  The exitdoor actor.
 */
void fn_level_actor_function_exitdoor_blit(
        FnLevelActorBlitParams p)
{
  FnGeometry destrect;
  fn_level_actor_exitdoor_data_t * data = p.specific;

  destrect.x = 0;
  destrect.y = 0;
  destrect.w = p.general->position.w;
  destrect.h = p.general->position.h;

  SDL_Surface * tile = SDL_CreateRGBSurface(
      p.target->flags,
      p.general->position.w,
      p.general->position.h,
      p.target->format->BitsPerPixel,
      0,
      0,
      0,
      0);
  const FnTexture * part = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->counter * 4);
  fn_texture_blit_to_sdl_surface(part, NULL, tile, &destrect);

  destrect.x += FN_TILE_WIDTH;
  part = fn_tilecache_get_tile(p.tilecache,
    data->tile + data->counter * 4 + 1);
  fn_texture_blit_to_sdl_surface(part, NULL, tile, &destrect);

  destrect.x = 0;
  destrect.y += FN_TILE_HEIGHT;
  part = fn_tilecache_get_tile(p.tilecache,
    data->tile + data->counter * 4 + 2);
  fn_texture_blit_to_sdl_surface(part, NULL, tile, &destrect);

  destrect.x += FN_TILE_WIDTH;
  part = fn_tilecache_get_tile(p.tilecache,
    data->tile + data->counter * 4 + 3);
  fn_texture_blit_to_sdl_surface(part, NULL, tile, &destrect);


  SDL_Rect target_rect = fn_geometry_as_sdl_rect(&p.general->position);

  SDL_BlitSurface(tile, NULL, p.target, &target_rect);

  SDL_FreeSurface(tile); tile = NULL;
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The door struct.
 */
typedef struct fn_level_actor_door_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The counter which defines for how many steps the exitdoor
   * has to be animated until the hero disappears.
   * time the act function is called.
   */
  Uint8 counter;
  /**
   * Stores the state of the door. 0 is idle, 1 is opening,
   * 2 is removed and invisible.
   */
  Uint8 state;
} fn_level_actor_door_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a door.
 *
 * @param  actor  The door actor.
 */
void fn_level_actor_function_door_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_door_data_t * data = malloc(
      sizeof(fn_level_actor_door_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->counter = 0;
  data->state = 0;
  data->tile = OBJ_DOOR;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

/**
 * Delete a door.
 *
 * @param  actor  The door actor.
 */
void fn_level_actor_function_door_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_door_data_t * data = *(p.specific);
  free(data); data = *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Action for door.
 * 
 * @param  actor  The door actor.
 */
void fn_level_actor_function_door_act(
        FnLevelActorActParams p)
{
  fn_level_actor_door_data_t * data = p.specific;

  switch(data->state) {
    case 0: /* idle */
      break;
    case 1: /* door opening */
      if (data->counter == 0) {
        fn_level_solids_set(&(p.level_data->solids),
            p.general->position.x / FN_TILE_WIDTH,
            p.general->position.y / FN_TILE_HEIGHT,
            0);
      }
      data->counter++;
      if (data->counter == 8) {
        data->counter = 0;
        data->state = 2;
        p.general->is_alive = 0;
      }
      break;
    case 2:
      /* do nothing, the door is already opened. */
      break;
    default: /* error */
      fn_error_print_commandline("Door in invalid state");
      break;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_door_receive_message(
        FnLevelActorReceiveMessageParams p)
{
    if (p.message != ActorMessageType_OpenDoor) {
        return;
    }
    fn_level_actor_door_data_t * data = p.specific;
    data->state = 1;
}

/* --------------------------------------------------------------- */

/**
 * Blit the door.
 *
 * @param  actor  The door actor.
 */
void fn_level_actor_function_door_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_door_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->counter);

  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The keyhole struct.
 */
typedef struct fn_level_actor_keyhole_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The counter which defines for how many steps the exitdoor
   * has to be animated until the hero disappears.
   * time the act function is called.
   */
  Uint8 counter;
} fn_level_actor_keyhole_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a keyhole.
 *
 * @param  actor  The keyhole actor.
 */
void fn_level_actor_function_keyhole_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_keyhole_data_t * data = malloc(
      sizeof(fn_level_actor_keyhole_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->counter = 0;
  data->tile = OBJ_KEYHOLE_BLACK;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

/**
 * Delete a keyhole.
 *
 * @param  actor  The keyhole actor.
 */
void fn_level_actor_function_keyhole_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_keyhole_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Action for keyhole.
 * 
 * @param  actor  The keyhole actor.
 */
void fn_level_actor_function_keyhole_act(
        FnLevelActorActParams p)
{
  fn_level_actor_keyhole_data_t * data = p.specific;

  if (data->counter != 5) {
    data->counter++;
    data->counter %= 4;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit the keyhole.
 *
 * @param  actor  The keyhole actor.
 */
void fn_level_actor_function_keyhole_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_keyhole_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile);

  if (data->counter > 1) {
    switch(p.general->actor_type) {
      case ActorType_KeyholeRed:
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEYHOLE_RED);
        break;
      case ActorType_KeyholeBlue:
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEYHOLE_BLUE);
        break;
      case ActorType_KeyholePink:
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEYHOLE_PINK);
        break;
      case ActorType_KeyholeGreen:
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEYHOLE_GREEN);
        break;
      default:
        fn_error_print_commandline("Invalid keyhole actor");
    }
  }

  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

/**
 * Hero interacts with keyhole.
 *
 * @param  actor  The keyhole actor.
 */
void fn_level_actor_function_keyhole_interact_start(
        FnLevelActorHeroInteractStartParams p)
{
  fn_level_actor_keyhole_data_t * data = p.specific;

  char msg[40];

  Uint8 haskey = 0;
  Uint8 needed_key = 0;
  FnLevelActorType door_to_open;

  switch(p.general->actor_type) {
    case ActorType_KeyholeRed:
      needed_key = InventoryItem_KeyRed;
      door_to_open = ActorType_DoorRed;
      snprintf(msg, 40, "You don't have the red key.\n");
      break;
    case ActorType_KeyholeBlue:
      needed_key = InventoryItem_KeyBlue;
      door_to_open = ActorType_DoorBlue;
      snprintf(msg, 40, "You don't have the blue key.\n");
      break;
    case ActorType_KeyholePink:
      needed_key = InventoryItem_KeyPink;
      door_to_open = ActorType_DoorPink;
      snprintf(msg, 40, "You don't have the pink key.\n");
      break;
    case ActorType_KeyholeGreen:
      needed_key = InventoryItem_KeyGreen;
      door_to_open = ActorType_DoorGreen;
      snprintf(msg, 40, "You don't have the green key.\n");
      break;
    default:
      fn_error_print_commandline("Invalid keyhole actor");
  }

  FnHeroInventory * hero_inventory = fn_hero_data_get_inventory(p.hero_data);
  haskey = fn_hero_inventory_is_set(hero_inventory, needed_key);

  if (haskey) {
    fn_hero_inventory_unset(hero_inventory, needed_key);
    data->counter = 5;

    fn_level_actor_message_queue_push_back(
            p.actor_message_queue,
            door_to_open,
            ActorMessageType_OpenDoor);
  } else if (data->counter != 5) {
    fn_info_message_queue_push(
            p.info_message_queue,
            msg);
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Key actor creation function.
 *
 * @param  actor  The key actor.
 */
void fn_level_actor_function_key_create(
        FnLevelActorCreateParams p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_WIDTH;
  p.general->is_in_foreground = 0;
}


/* --------------------------------------------------------------- */

/**
 * The hero touches a key actor.
 *
 * @param  actor  The key actor.
 */
void fn_level_actor_function_key_touch_start(
        FnLevelActorHeroTouchStartParams p)
{
  FnHeroInventory * inventory = fn_hero_data_get_inventory(p.hero_data);
  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  switch(p.general->actor_type) {
    case ActorType_KeyRed:
      fn_hero_inventory_set(inventory, InventoryItem_KeyRed);
      break;
    case ActorType_KeyBlue:
      fn_hero_inventory_set(inventory, InventoryItem_KeyBlue);
      break;
    case ActorType_KeyGreen:
      fn_hero_inventory_set(inventory, InventoryItem_KeyGreen);
      break;
    case ActorType_KeyPink:
      fn_hero_inventory_set(inventory, InventoryItem_KeyPink);
      break;
    default:
      printf(__FILE__ ":%d: warning: key #%d"
          " added which is not a key\n",
          __LINE__, p.general->actor_type);
      break;
  }
  fn_hero_score_add(score, 1000);
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Score1000,
      p.general->position.x,
      p.general->position.y);
  p.general->is_alive = 0;
}

/* --------------------------------------------------------------- */

/**
 * Blit the key.
 *
 * @param  actor  The key actor.
 */
void fn_level_actor_function_key_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile = NULL;
  FnGeometry destrect = p.general->position;
  switch(p.general->actor_type) {
    case ActorType_KeyRed:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEY_RED);
      break;
    case ActorType_KeyBlue:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEY_BLUE);
      break;
    case ActorType_KeyGreen:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEY_GREEN);
      break;
    case ActorType_KeyPink:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_KEY_PINK);
      break;
    default:
      printf(__FILE__ ":%d: warning: key #%d"
          " tried to blit which is not a key\n",
          __LINE__, p.general->actor_type);
      return;
      break;
  }
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Create the shootable wall.
 *
 * @param  actor  The wall actor.
 */
void fn_level_actor_function_shootable_wall_create(
        FnLevelActorCreateParams p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

/**
 * Blit the shootable wall.
 *
 * @param  actor  The wall actor.
 */
void fn_level_actor_function_shootable_wall_blit(
        FnLevelActorBlitParams p)
{
  const FnTexture * tile = NULL;

  FnGeometry destrect = p.general->position;

  tile = fn_tilecache_get_tile(p.tilecache, 0x8C0/0x20);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  tile = fn_tilecache_get_tile(p.tilecache, 0x1800/0x20);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

/**
 * Shoot the shootable wall.
 *
 * @param  actor  The wall actor.
 */
void fn_level_actor_function_shootable_wall_shot(
        FnLevelActorShotParams p)
{
  p.general->is_alive = 0;
  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  fn_hero_score_add(score, 10);
  fn_level_solids_set(&(p.level_data->solids),
      p.general->position.x / FN_TILE_WIDTH,
      p.general->position.y / FN_TILE_HEIGHT,
      0);
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Explosion,
      p.general->position.x,
      p.general->position.y);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef struct fn_level_actor_accesscard_door_data_t
{
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames for the animation.
   */
  Uint8 num_frames;
} fn_level_actor_accesscard_door_data_t;

/**
 * Create an accesscard door.
 *
 * @param  actor  The accesscard door actor.
 */
void fn_level_actor_function_access_card_door_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_accesscard_door_data_t * data = malloc(
      sizeof(fn_level_actor_accesscard_door_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;

  data->tile = OBJ_LASERBEAM;
  data->current_frame = 0;
  data->num_frames = 4;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

/**
 * Delete an accesscard door.
 *
 * @param  actor  The accesscard door actor.
 */
void fn_level_actor_function_access_card_door_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_accesscard_door_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Blit an accesscard door.
 *
 * @param  actor  The accesscard door actor.
 */
void fn_level_actor_function_access_card_door_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_accesscard_door_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

/**
 * Act an accesscard door.
 *
 * @param  actor  The accesscard door actor.
 */
void fn_level_actor_function_access_card_door_act(
        FnLevelActorActParams p)
{
  fn_level_actor_accesscard_door_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_access_card_door_receive_message(
        FnLevelActorReceiveMessageParams p)
{
    if (p.message != ActorMessageType_OpenDoor) {
        return;
    }
    p.general->is_alive = 0;
    int x = p.general->position.x / FN_TILE_WIDTH;
    int y = p.general->position.y / FN_TILE_HEIGHT;
    fn_level_solids_set(&(p.level_data->solids), x, y, 0);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The fan wheel
 */
typedef struct fn_level_actor_fan_data_t {
  /**
   * The tile number which is blitted in the level.
   */
  Uint16 tile;
  /**
   * The animation counter.
   */
  Uint8 current_frame;
  /**
   * The number of frames.
   */
  Uint8 num_frames;
  /**
   * A flag showing if the wheel is running.
   */
  Uint8 running;
} fn_level_actor_fan_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_create(
        FnLevelActorCreateParams p)
{
  fn_level_actor_fan_data_t * data = malloc(
      sizeof(fn_level_actor_fan_data_t));
  *(p.specific) = data;
  data->tile = ANIM_FAN;
  data->num_frames = 4;
  data->current_frame = 0;
  data->running = 10;
  p.general->position.h = 2 * FN_TILE_HEIGHT;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.y -= FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_free(
        FnLevelActorFreeParams p)
{
  fn_level_actor_fan_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_act(
        FnLevelActorActParams p)
{
  fn_level_actor_fan_data_t * data = p.specific;

  switch(data->running) {
    case 0:
      /* Do nothing */
      break;
    case 1:
      data->current_frame++;
      break;
    case 2:
      break;
    case 3:
      break;
    case 4:
      break;
    case 5:
      data->current_frame++;
      break;
    case 6:
      break;
    case 7:
      break;
    case 8:
      data->current_frame++;
      break;
    case 9:
      break;
    case 10:
      data->current_frame++;
      break;
    default:
      /* do nothing */
      break;
  }

  data->current_frame %= data->num_frames;
  if (data->running < 10 && data->running > 0) {
    data->running--;
  } else if (data->running == 10) {
    FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
    FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);

    if (fn_geometry_overlaps_vertically(
                hero_geometry, p.general->position)) {
      int hdistance = fn_geometry_horizontal_distance(
          hero_geometry, p.general->position);

      int fandirection = 0;
      if (p.general->actor_type == ActorType_FanLeft) {
        fandirection = -1;
      } else {
        fandirection = 1;
      }

      if (fandirection < 0 && hdistance < 0) {
        return;
      }
      if (fandirection > 0 && hdistance > 0) {
        return;
      }

      if (hdistance == 0) {
        hdistance = FN_HALFTILE_WIDTH * fandirection;
      }

      Uint16 hdistance_abs =
        (hdistance > 0 ? hdistance : -hdistance);
      if (hdistance_abs < 8 * FN_HALFTILE_WIDTH) {
          fn_hero_position_push_horizontally(
                  hero_position,
                  &(p.level_data->solids),
                  fandirection * FN_TILE_WIDTH);
      }
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_blit(
        FnLevelActorBlitParams p)
{
  fn_level_actor_fan_data_t * data = p.specific;


  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame * 2);

  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame * 2 + 1);
  destrect.y += FN_TILE_HEIGHT;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_shot(
        FnLevelActorShotParams p)
{
  fn_level_actor_fan_data_t * data = p.specific;
  data->running = 9;
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Steam,
      p.general->position.x,
      p.general->position.y);
}


/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * An array of functions to call for different actions on different
 * actors.
 */
fn_level_actor_functions_t
fn_level_actor_functions[] =
{
  [ActorType_FireWheelBot] = {
    .create = fn_level_actor_function_firewheelbot_create,
    .free = fn_level_actor_function_firewheelbot_free,
    .hero_touch_start = fn_level_actor_function_firewheelbot_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_firewheelbot_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_firewheelbot_act,
    .blit = fn_level_actor_function_firewheelbot_blit,
    .shot = fn_level_actor_function_firewheelbot_shot,
    .receive_message = NULL,
  },
  [ActorType_FlameGnomeBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_FlyingBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_FootBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_HelicopterBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_RabbitoidBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_RedBallJumping] = {
    .create = fn_level_actor_function_redball_jumping_create,
    .free = fn_level_actor_function_redball_jumping_free,
    .hero_touch_start = fn_level_actor_function_redball_jumping_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_redball_jumping_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_redball_jumping_act,
    .blit = fn_level_actor_function_redball_jumping_blit,
    .shot = fn_level_actor_function_redball_jumping_shot,
    .receive_message = NULL,
  },
  [ActorType_RedBallLying] = {
    .create = fn_level_actor_function_redball_lying_create,
    .free = fn_level_actor_function_redball_lying_free,
    .hero_touch_start = fn_level_actor_function_redball_lying_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_redball_lying_act,
    .blit = fn_level_actor_function_redball_lying_blit,
    .shot = fn_level_actor_function_redball_lying_shot,
    .receive_message = NULL,
  },
  [ActorType_Robot] = {
    .create = fn_level_actor_function_robot_create,
    .free = fn_level_actor_function_robot_free,
    .hero_touch_start = fn_level_actor_function_robot_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_robot_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_robot_act,
    .blit = fn_level_actor_function_robot_blit,
    .shot = fn_level_actor_function_robot_shot,
    .receive_message = NULL,
  },
  [ActorType_RobotDisappearing] = {
    .create = fn_level_actor_function_singleanimation_create,
    .free = fn_level_actor_function_singleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_singleanimation_act,
    .blit = fn_level_actor_function_singleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_SnakeBot] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_TankBot] = {
    .create = fn_level_actor_function_tankbot_create,
    .free = fn_level_actor_function_tankbot_free,
    .hero_touch_start = fn_level_actor_function_tankbot_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_tankbot_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_tankbot_act,
    .blit = fn_level_actor_function_tankbot_blit,
    .shot = fn_level_actor_function_tankbot_shot,
    .receive_message = NULL,
  },
  [ActorType_WallCrawlerBotLeft] = {
    .create = fn_level_actor_function_wallcrawler_create,
    .free = fn_level_actor_function_wallcrawler_free,
    .hero_touch_start = fn_level_actor_function_wallcrawler_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_wallcrawler_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_wallcrawler_act,
    .blit = fn_level_actor_function_wallcrawler_blit,
    .shot = fn_level_actor_function_wallcrawler_shot,
    .receive_message = NULL,
  },
  [ActorType_WallCrawlerBotRight] = {
    .create = fn_level_actor_function_wallcrawler_create,
    .free = fn_level_actor_function_wallcrawler_free,
    .hero_touch_start = fn_level_actor_function_wallcrawler_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_wallcrawler_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_wallcrawler_act,
    .blit = fn_level_actor_function_wallcrawler_blit,
    .shot = fn_level_actor_function_wallcrawler_shot,
    .receive_message = NULL,
  },
  [ActorType_DrProton] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_Camera] = {
    .create = fn_level_actor_function_camera_create,
    .free = NULL,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_camera_blit,
    .shot = fn_level_actor_function_camera_shot,
    .receive_message = NULL,
  },
  [ActorType_Explosion] = {
    .create = fn_level_actor_function_explosion_create,
    .free = fn_level_actor_function_explosion_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_explosion_act,
    .blit = fn_level_actor_function_explosion_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Fire] = {
    .create = fn_level_actor_function_singleanimation_create,
    .free = fn_level_actor_function_singleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_singleanimation_act,
    .blit = fn_level_actor_function_singleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_DustCloud] = {
    .create = fn_level_actor_function_singleanimation_create,
    .free = fn_level_actor_function_singleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_singleanimation_act,
    .blit = fn_level_actor_function_singleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Steam] = {
    .create = fn_level_actor_function_singleanimation_create,
    .free = fn_level_actor_function_singleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_singleanimation_act,
    .blit = fn_level_actor_function_singleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ParticlePink] = {
    .create = fn_level_actor_function_particle_create,
    .free = fn_level_actor_function_particle_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_particle_act,
    .blit = fn_level_actor_function_particle_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ParticleBlue] = {
    .create = fn_level_actor_function_particle_create,
    .free = fn_level_actor_function_particle_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_particle_act,
    .blit = fn_level_actor_function_particle_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ParticleWhite] = {
    .create = fn_level_actor_function_particle_create,
    .free = fn_level_actor_function_particle_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_particle_act,
    .blit = fn_level_actor_function_particle_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ParticleGreen] = {
    .create = fn_level_actor_function_particle_create,
    .free = fn_level_actor_function_particle_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_particle_act,
    .blit = fn_level_actor_function_particle_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Rocket] = {
    .create = fn_level_actor_function_rocket_create,
    .free = fn_level_actor_function_rocket_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_rocket_act,
    .blit = fn_level_actor_function_rocket_blit,
    .shot = fn_level_actor_function_rocket_shot,
    .receive_message = NULL,
  },
  [ActorType_Bomb] = {
    .create = fn_level_actor_bomb_create,
    .free = fn_level_actor_bomb_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_bomb_act,
    .blit = fn_level_actor_bomb_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BombFire] = {
    .create = fn_level_actor_bombfire_create,
    .free = fn_level_actor_bombfire_free,
    .hero_touch_start = fn_level_actor_bombfire_hero_touch_start,
    .hero_touch_end = fn_level_actor_bombfire_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_bombfire_act,
    .blit = fn_level_actor_bombfire_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Water] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_ExitDoor] = {
    .create = fn_level_actor_function_exitdoor_create,
    .free = fn_level_actor_function_exitdoor_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_exitdoor_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_exitdoor_act,
    .blit = fn_level_actor_function_exitdoor_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Notebook] = {
    .create = fn_level_actor_function_notebook_create,
    .free = fn_level_actor_function_notebook_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_notebook_interact_start,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_notebook_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_SurveillanceScreen] = {
    .create = fn_level_actor_function_surveillancescreen_create,
    .free = fn_level_actor_function_surveillancescreen_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_surveillancescreen_interact_start,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_surveillancescreen_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_HostileShotLeft] = {
    .create = fn_level_actor_function_hostileshot_create,
    .free = fn_level_actor_function_hostileshot_free,
    .hero_touch_start = fn_level_actor_function_hostileshot_touch_start,
    .hero_touch_end = fn_level_actor_function_hostileshot_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_hostileshot_act,
    .blit = fn_level_actor_function_hostileshot_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_HostileShotRight] = {
    .create = fn_level_actor_function_hostileshot_create,
    .free = fn_level_actor_function_hostileshot_free,
    .hero_touch_start = fn_level_actor_function_hostileshot_touch_start,
    .hero_touch_end = fn_level_actor_function_hostileshot_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_hostileshot_act,
    .blit = fn_level_actor_function_hostileshot_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Soda] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_SodaFlying] = {
    .create = fn_level_actor_function_soda_flying_create,
    .free = fn_level_actor_function_soda_flying_free,
    .hero_touch_start = fn_level_actor_function_soda_flying_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_soda_flying_act,
    .blit = fn_level_actor_function_soda_flying_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_UnstableFloor] = {
    .create = fn_level_actor_function_unstablefloor_create,
    .free = fn_level_actor_function_unstablefloor_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_unstablefloor_act,
    .blit = fn_level_actor_function_unstablefloor_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ExpandingFloor] = {
    .create = fn_level_actor_function_expandingfloor_create,
    .free = fn_level_actor_function_expandingfloor_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_expandingfloor_act,
    .blit = fn_level_actor_function_expandingfloor_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_expandingfloor_receive_message,
  },
  [ActorType_ConveyorLeftMovingRightEnd] = {
    .create = fn_level_actor_function_conveyor_create,
    .free = fn_level_actor_function_conveyor_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_conveyor_act,
    .blit = fn_level_actor_function_conveyor_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ConveyorRightMovingRightEnd] = {
    .create = fn_level_actor_function_conveyor_create,
    .free = fn_level_actor_function_conveyor_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_conveyor_act,
    .blit = fn_level_actor_function_conveyor_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_FanLeft] = {
    .create = fn_level_actor_function_fan_create,
    .free = fn_level_actor_function_fan_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_fan_act,
    .blit = fn_level_actor_function_fan_blit,
    .shot = fn_level_actor_function_fan_shot,
    .receive_message = NULL,
  },
  [ActorType_FanRight] = {
    .create = fn_level_actor_function_fan_create,
    .free = fn_level_actor_function_fan_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_fan_act,
    .blit = fn_level_actor_function_fan_blit,
    .shot = fn_level_actor_function_fan_shot,
    .receive_message = NULL,
  },
  [ActorType_BrokenWallBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_StoneBackground] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_Teleporter1] = {
    .create = fn_level_actor_function_teleporter_create,
    .free = NULL,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_teleporter_hero_interact_start,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_teleporter_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_teleporter_receive_message,
  },
  [ActorType_Teleporter2] = {
    .create = fn_level_actor_function_teleporter_create,
    .free = NULL,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_teleporter_hero_interact_start,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_teleporter_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_teleporter_receive_message,
  },
  [ActorType_FenceBackground] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_StoneWindowBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_WindowLeftBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_WindowRightBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Screen] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_BoxGreyEmpty] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyBoots] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Boots] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyClamps] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Clamps] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyGun] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Gun] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyBomb] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_BoxRedSoda] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_BoxRedChicken] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_ChickenSingle] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_ChickenDouble] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxBlueFootball] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Football] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Flag] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxBlueJoystick] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Joystick] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxBlueDisk] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Disk] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxBlueBalloon] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Balloon] = {
    .create = fn_level_actor_function_balloon_create,
    .free = fn_level_actor_function_balloon_free,
    .hero_touch_start = fn_level_actor_function_balloon_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_balloon_act,
    .blit = fn_level_actor_function_balloon_blit,
    .shot = fn_level_actor_function_balloon_shot,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyGlove] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Glove] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyFullLife] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_FullLife] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxBlueFlag] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_BlueFlag] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL,
  },
  [ActorType_BoxBlueRadio] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_Radio] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyAccessCard] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_AccessCard] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyLetterD] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_LetterD] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyLetterU] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_LetterU] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyLetterK] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_LetterK] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BoxGreyLetterE] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = fn_level_actor_function_item_shot,
    .receive_message = NULL,
  },
  [ActorType_LetterE] = {
    .create = fn_level_actor_function_item_create,
    .free = fn_level_actor_function_item_free,
    .hero_touch_start = fn_level_actor_function_item_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_item_act,
    .blit = fn_level_actor_function_item_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_AccessCardSlot] = {
    .create = fn_level_actor_function_accesscard_slot_create,
    .free = fn_level_actor_function_accesscard_slot_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_accesscard_slot_hero_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_accesscard_slot_act,
    .blit = fn_level_actor_function_accesscard_slot_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_GloveSlot] = {
    .create = fn_level_actor_function_glove_slot_create,
    .free = fn_level_actor_function_glove_slot_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_glove_slot_hero_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_glove_slot_act,
    .blit = fn_level_actor_function_glove_slot_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeyRed] = {
    .create = fn_level_actor_function_key_create,
    .free = NULL,
    .hero_touch_start = fn_level_actor_function_key_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_key_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeyholeRed] = {
    .create = fn_level_actor_function_keyhole_create,
    .free = fn_level_actor_function_keyhole_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_keyhole_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_keyhole_act,
    .blit = fn_level_actor_function_keyhole_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_DoorRed] = {
    .create = fn_level_actor_function_door_create,
    .free = fn_level_actor_function_door_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_door_act,
    .blit = fn_level_actor_function_door_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_door_receive_message,
  },
  [ActorType_KeyBlue] = {
    .create = fn_level_actor_function_key_create,
    .free = NULL,
    .hero_touch_start = fn_level_actor_function_key_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_key_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeyholeBlue] = {
    .create = fn_level_actor_function_keyhole_create,
    .free = fn_level_actor_function_keyhole_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_keyhole_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_keyhole_act,
    .blit = fn_level_actor_function_keyhole_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_DoorBlue] = {
    .create = fn_level_actor_function_door_create,
    .free = fn_level_actor_function_door_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_door_act,
    .blit = fn_level_actor_function_door_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_door_receive_message,
  },
  [ActorType_KeyPink] = {
    .create = fn_level_actor_function_key_create,
    .free = NULL,
    .hero_touch_start = fn_level_actor_function_key_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_key_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeyholePink] = {
    .create = fn_level_actor_function_keyhole_create,
    .free = fn_level_actor_function_keyhole_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_keyhole_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_keyhole_act,
    .blit = fn_level_actor_function_keyhole_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_DoorPink] = {
    .create = fn_level_actor_function_door_create,
    .free = fn_level_actor_function_door_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_door_act,
    .blit = fn_level_actor_function_door_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_door_receive_message,
  },
  [ActorType_KeyGreen] = {
    .create = fn_level_actor_function_key_create,
    .free = NULL,
    .hero_touch_start = fn_level_actor_function_key_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_key_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeyholeGreen] = {
    .create = fn_level_actor_function_keyhole_create,
    .free = fn_level_actor_function_keyhole_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_keyhole_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_keyhole_act,
    .blit = fn_level_actor_function_keyhole_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_DoorGreen] = {
    .create = fn_level_actor_function_door_create,
    .free = fn_level_actor_function_door_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_door_act,
    .blit = fn_level_actor_function_door_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_door_receive_message,
  },
  [ActorType_ShootableWall] = {
    .create = fn_level_actor_function_shootable_wall_create,
    .free = NULL,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_shootable_wall_blit,
    .shot = fn_level_actor_function_shootable_wall_shot,
    .receive_message = NULL,
  },
  [ActorType_Lift] = {
    .create = fn_level_actor_function_elevator_create,
    .free = fn_level_actor_function_elevator_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_elevator_hero_interact_start,
    .hero_interact_end = fn_level_actor_function_elevator_hero_interact_end,
    .act = fn_level_actor_function_elevator_act,
    .blit = fn_level_actor_function_elevator_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Acme] = {
    .create = fn_level_actor_function_acme_create,
    .free = fn_level_actor_function_acme_free,
    .hero_touch_start = fn_level_actor_function_acme_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_acme_act,
    .blit = fn_level_actor_function_acme_blit,
    .shot = fn_level_actor_function_acme_shot,
    .receive_message = NULL,
  },
  [ActorType_FireRight] = {
    .create = fn_level_actor_function_fire_create,
    .free = fn_level_actor_function_fire_free,
    .hero_touch_start = fn_level_actor_function_fire_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_fire_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_fire_act,
    .blit = fn_level_actor_function_fire_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_FireLeft] = {
    .create = fn_level_actor_function_fire_create,
    .free = fn_level_actor_function_fire_free,
    .hero_touch_start = fn_level_actor_function_fire_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_fire_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_fire_act,
    .blit = fn_level_actor_function_fire_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Mill] = {
    .create = fn_level_actor_function_mill_create,
    .free = fn_level_actor_function_mill_free,
    .hero_touch_start = fn_level_actor_function_mill_hero_touch_start,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_mill_act,
    .blit = fn_level_actor_function_mill_blit,
    .shot = fn_level_actor_function_mill_shot,
    .receive_message = NULL,
  },
  [ActorType_Laserbeam] = {
    .create = NULL, /* TODO */
    .free = NULL, /* TODO */
    .hero_touch_start = NULL, /* TODO */
    .hero_touch_end = NULL, /* TODO */
    .hero_interact_start = NULL, /* TODO */
    .hero_interact_end = NULL, /* TODO */
    .act = NULL, /* TODO */
    .blit = NULL, /* TODO */
    .shot = NULL, /* TODO */
    .receive_message = NULL, /* TODO */
  },
  [ActorType_AccessCardDoor] = {
    .create = fn_level_actor_function_access_card_door_create,
    .free = fn_level_actor_function_access_card_door_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_access_card_door_act,
    .blit = fn_level_actor_function_access_card_door_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_access_card_door_receive_message,
  },
  [ActorType_SpikesUp] = {
    .create = fn_level_actor_function_spikes_create,
    .free = fn_level_actor_function_spikes_free,
    .hero_touch_start = fn_level_actor_function_spikes_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_spikes_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_SpikesDown] = {
    .create = fn_level_actor_function_spikes_create,
    .free = fn_level_actor_function_spikes_free,
    .hero_touch_start = fn_level_actor_function_spikes_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_spikes_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Spike] = {
    .create = fn_level_actor_function_spikes_create,
    .free = fn_level_actor_function_spikes_free,
    .hero_touch_start = fn_level_actor_function_spikes_hero_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_hero_touch_end,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = NULL,
    .blit = fn_level_actor_function_spikes_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score100] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score200] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score500] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score1000] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score2000] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score5000] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Score10000] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus1Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus1Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus2Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus2Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus3Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus3Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus4Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus4Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus5Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus5Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus6Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus6Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus7Left] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_ScoreBonus7Right] = {
    .create = fn_level_actor_function_score_create,
    .free = fn_level_actor_function_score_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_score_act,
    .blit = fn_level_actor_function_score_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BlueLightBackground1] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BlueLightBackground2] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BlueLightBackground3] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BlueLightBackground4] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_TextOnScreenBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_HighVoltageFlashBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_RedFlashlightBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_BlueFlashlightBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_KeypanelBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_RedRotationLightBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_UpArrowBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_GreenPoisonBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_LavaBackground] = {
    .create = fn_level_actor_function_simpleanimation_create,
    .free = fn_level_actor_function_simpleanimation_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = NULL,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_simpleanimation_act,
    .blit = fn_level_actor_function_simpleanimation_blit,
    .shot = NULL,
    .receive_message = NULL,
  },
};


/* --------------------------------------------------------------- */

fn_level_actor_t * fn_level_actor_create(
        fn_level_t * level,
        FnLevelActorType type,
        Uint16 x,
        Uint16 y)
{
  fn_level_actor_create_function_t func = NULL;
  fn_level_actor_t * actor = malloc(sizeof(fn_level_actor_t));
  actor->general = fn_level_actor_data_create(type);
  actor->general->position.x = x;
  actor->general->position.y = y;
  actor->general->position.w = 0; /* should be changed by func */
  actor->general->position.h = 0; /* should be changed by func */
  actor->general->is_alive = 1;
  actor->touches_hero = 0;
  actor->general->is_in_foreground = 0;
  actor->is_visible = 0;
  actor->acts_while_invisible = 0;
  func = fn_level_actor_functions[actor->general->actor_type].create;
  if (func != NULL) {
      FnLevelActorCreateParams p ={
          .general = actor->general,
          .specific = &(actor->specific),
          .level_data = level->data,
      };

      func(p);
  }
  return actor;
}

/* --------------------------------------------------------------- */

void fn_level_actor_free(fn_level_actor_t * actor, fn_level_t * level)
{
  fn_level_actor_free_function_t func =
    fn_level_actor_functions[actor->general->actor_type].free;
  if (func != NULL) {
      FnLevelActorFreeParams p ={
          .specific = &(actor->specific),
      };
    func(p);
  }
  fn_level_actor_data_free(actor->general);
  actor->general = NULL;
  free(actor);
}

/* --------------------------------------------------------------- */

int fn_level_actor_touches_hero(fn_level_actor_t * actor, fn_hero_t * hero)
{
  return fn_geometry_overlaps(
        fn_hero_get_position(hero), actor->general->position);
}

/* --------------------------------------------------------------- */

void fn_level_actor_check_hero_touch(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnLevelActorQueue * actor_queue)
{
  if (fn_level_actor_touches_hero(actor, fn_level_get_hero(level))) {
    if (!actor->touches_hero) {
      actor->touches_hero = 1;
      fn_level_actor_hero_touch_start(
              actor, level, actor_queue);
    }
  } else {
    if (actor->touches_hero) {
      fn_level_actor_hero_touch_end(actor, level);
      actor->touches_hero = 0;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_hero_touch_start(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnLevelActorQueue * actor_queue)
{
  fn_level_actor_hero_touch_start_function_t func =
    fn_level_actor_functions[actor->general->actor_type].hero_touch_start;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);

    FnLevelActorHeroTouchStartParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .actor_queue = actor_queue,
        .hero_data = hero->data,
    };
    func(p);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_hero_touch_end(fn_level_actor_t * actor, fn_level_t * level)
{
  fn_level_actor_hero_touch_end_function_t func =
    fn_level_actor_functions[actor->general->actor_type].hero_touch_end;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);
    FnLevelActorHeroTouchEndParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .hero_data = hero->data,
    };
    func(p);
  }
}

/* --------------------------------------------------------------- */

Uint8 fn_level_actor_hero_can_interact(fn_level_actor_t * actor, fn_hero_t * hero)
{
  if (actor->general->actor_type == ActorType_Lift) {
    /* This check needs to be done for elevator only because
     * if there are two elevators next to each other, the mostleft
     * elevator would be chosen for interaction instead of the one on
     * which the hero stands.
     */
    FnGeometry heropos = fn_hero_get_position(hero);
    return (actor->general->position.x == heropos.x);
  }
  fn_level_actor_interact_start_function_t func =
    fn_level_actor_functions[actor->general->actor_type].hero_interact_start;
  return (func != NULL);
}

/* --------------------------------------------------------------- */

void fn_level_actor_hero_interact_start(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnInfoMessageQueue * info_message_queue,
        FnLevelActorMessageQueue * actor_message_queue)
{
  fn_level_actor_interact_start_function_t func =
    fn_level_actor_functions[actor->general->actor_type].hero_interact_start;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);

    FnLevelActorHeroInteractStartParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .level_data = level->data,
        .hero_data = hero->data,
        .info_message_queue = info_message_queue,
        .actor_message_queue = actor_message_queue,
    };

    func(p);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_hero_interact_stop(fn_level_actor_t * actor, fn_level_t * level)
{
  fn_level_actor_interact_end_function_t func =
    fn_level_actor_functions[actor->general->actor_type].hero_interact_end;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);

    FnLevelActorHeroInteractEndParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .level_data = level->data,
        .hero_data = hero->data,
    };

    func(p);
  }
}

/* --------------------------------------------------------------- */

int fn_level_actor_act(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnLevelActorQueue * actor_queue)
{
  fn_level_actor_check_hero_touch(actor, level, actor_queue);
  fn_level_actor_act_function_t func =
    fn_level_actor_functions[actor->general->actor_type].act;
  if (func != NULL)
  {
    fn_hero_t * hero = fn_level_get_hero(level);
    FnLevelActorActParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .level_data = level->data,
        .actor_queue = actor_queue,
        .hero_data = hero->data,
    };
    func(p);
  }
  return actor->general->is_alive;
}

/* --------------------------------------------------------------- */

void fn_level_actor_blit(fn_level_actor_t * actor, fn_level_t * level)
{
  fn_level_actor_blit_function_t func =
    fn_level_actor_functions[actor->general->actor_type].blit;
  if (func != NULL) {
    SDL_Surface * target = fn_level_get_surface(level);
    fn_hero_t * hero = fn_level_get_hero(level);
    FnLevelActorBlitParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .hero_data = hero->data,
        .tilecache = fn_level_get_tilecache(level),
        .target = target
    };
    func(p);
    Uint8 draw_collision_bounds =
      fn_environment_get_draw_collision_bounds(
          fn_level_get_environment(level));
    Uint32 collision_color = FN_COLLISION_DEBUG_COLOR(target->format);
    if (draw_collision_bounds) {
      fn_geometry_draw_outline(
              target, actor->general->position, collision_color);
    }
  }
}

/* --------------------------------------------------------------- */

Uint8 fn_level_actor_shot(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnLevelActorQueue * actor_queue)
{
  fn_level_actor_shot_function_t func =
    fn_level_actor_functions[actor->general->actor_type].shot;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);
    FnLevelActorShotParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .level_data = level->data,
        .actor_queue = actor_queue,
        .hero_data = hero->data,
    };
    func(p);
    return 1;
  }
  return 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_receive_message(
        fn_level_actor_t * actor,
        fn_level_t * level,
        FnLevelActorMessageType message)
{
  fn_level_actor_receive_message_function_t func =
    fn_level_actor_functions[actor->general->actor_type].receive_message;
  if (func != NULL) {
    fn_hero_t * hero = fn_level_get_hero(level);

    FnLevelActorReceiveMessageParams p = {
        .general = actor->general,
        .specific = actor->specific,
        .message = message,
        .hero_data = hero->data,
        .level_data = level->data,
    };

    func(p);
  }
}

/* --------------------------------------------------------------- */

Uint16 fn_level_actor_get_x(fn_level_actor_t * actor)
{
  return actor->general->position.x;
}

/* --------------------------------------------------------------- */

Uint16 fn_level_actor_get_y(fn_level_actor_t * actor)
{
  return actor->general->position.y;
}

/* --------------------------------------------------------------- */

Uint16 fn_level_actor_get_w(fn_level_actor_t * actor)
{
  return actor->general->position.w;
}

/* --------------------------------------------------------------- */

Uint16 fn_level_actor_get_h(fn_level_actor_t * actor)
{
  return actor->general->position.h;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_actor_can_get_shot(fn_level_actor_t * actor)
{
  fn_level_actor_shot_function_t func =
      fn_level_actor_functions[actor->general->actor_type].shot;
  return (func != NULL);
}

/* --------------------------------------------------------------- */

Uint8 fn_level_actor_in_foreground(fn_level_actor_t * actor)
{
  return actor->general->is_in_foreground;
}

/* --------------------------------------------------------------- */

FnGeometry fn_level_actor_get_position(fn_level_actor_t * actor)
{
  return actor->general->position;
}

/* --------------------------------------------------------------- */

void fn_level_actor_set_visible(fn_level_actor_t * actor, Uint8 visibility)
{
  actor->is_visible = visibility;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_actor_is_visible(fn_level_actor_t * actor)
{
  return actor->is_visible;
}
