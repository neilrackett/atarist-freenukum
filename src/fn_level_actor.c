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

typedef struct fn_level_actor_create_params_t {
    FnLevelActorData * general;
    void ** specific;
    FnLevelData * level_data;
} fn_level_actor_create_params_t;

typedef void (* fn_level_actor_create_function_t)(
        fn_level_actor_create_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_free_params_t {
    void ** specific;
} fn_level_actor_free_params_t;

typedef void (* fn_level_actor_free_function_t)(
        fn_level_actor_free_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_hero_touch_start_params_t {
    FnLevelActorData * general;
    void * specific;
    FnLevelActorQueue * actor_queue;
    FnHeroData * hero_data;
} fn_level_actor_hero_touch_start_params_t;

typedef void (* fn_level_actor_hero_touch_start_function_t)(
        fn_level_actor_hero_touch_start_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_hero_touch_end_params_t {
    FnLevelActorData * general;
    void * specific;
    FnHeroData * hero_data;
} fn_level_actor_hero_touch_end_params_t;

typedef void (* fn_level_actor_hero_touch_end_function_t)(
        fn_level_actor_hero_touch_end_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_interact_start_params_t {
    FnLevelActorData * general;
    void * specific;
    FnLevelData * level_data;
    FnHeroData * hero_data;
    FnInfoMessageQueue * info_message_queue;
    FnLevelActorMessageQueue * actor_message_queue;
} fn_level_actor_interact_start_params_t;

typedef void (* fn_level_actor_interact_start_function_t)(
        fn_level_actor_interact_start_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_interact_end_params_t {
    FnLevelActorData * general;
    void * specific;
    FnLevelData * level_data;
    FnHeroData * hero_data;
} fn_level_actor_interact_end_params_t;

typedef void (* fn_level_actor_interact_end_function_t)(
        fn_level_actor_interact_end_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_act_params_t {
    FnLevelActorData * general;
    void * specific;
    fn_level_t * level;
    FnLevelData * level_data;
    FnLevelActorQueue * actor_queue;
    FnHeroData * hero_data;
} fn_level_actor_act_params_t;

typedef void (* fn_level_actor_act_function_t)(
        fn_level_actor_act_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_blit_params_t {
    FnLevelActorData * general;
    void * specific;
    FnHeroData * hero_data;
    const FnTileCache * tilecache;
    SDL_Surface * target;
} fn_level_actor_blit_params_t;


typedef void (* fn_level_actor_blit_function_t)(
        fn_level_actor_blit_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_shot_params_t {
    FnLevelActorData * general;
    void * specific;
    FnLevelData * level_data;
    FnLevelActorQueue * actor_queue;
    FnHeroData * hero_data;
} fn_level_actor_shot_params_t;

typedef void (* fn_level_actor_shot_function_t)(
        fn_level_actor_shot_params_t p);

/* --------------------------------------------------------------- */

typedef struct fn_level_actor_receive_message_params_t {
    FnLevelActorData * general;
    void * specific;
    FnLevelActorMessageType message;
    FnHeroData * hero_data;
    FnLevelData * level_data;
} fn_level_actor_receive_message_params_t;

typedef void (* fn_level_actor_receive_message_function_t)(
        fn_level_actor_receive_message_params_t p);

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

/**
 * The simple animation struct.
 * A simple animation is an animation which is one part high,
 * one part wide and has a fixed number of frames that occur
 * one after each other and are lined up in a row inside
 * the tilecache.
 */
typedef struct fn_level_actor_simpleanimation_data_t {
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
} fn_level_actor_simpleanimation_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a simple animation.
 *
 * @param  actor The animation actor.
 */
void fn_level_actor_function_simpleanimation_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_simpleanimation_data_t * data = malloc(
      sizeof(fn_level_actor_simpleanimation_data_t));
  *(p.specific) = data;

  p.general->is_in_foreground = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  switch(p.general->actor_type) {
    case ActorType_TextOnScreenBackground:
      data->tile = 0x0004;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_HighVoltageFlashBackground:
      data->tile = 0x0008;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_RedFlashlightBackground:
      data->tile = 0x000C;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_BlueFlashlightBackground:
      data->tile = 0x0010;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_KeypanelBackground:
      data->tile = 0x0014;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_RedRotationLightBackground:
      data->tile = 0x0018;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_UpArrowBackground:
      data->tile = 0x001C;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_BlueLightBackground1:
      data->tile = 0x0020;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_BlueLightBackground2:
      data->tile = 0x0021;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_BlueLightBackground3:
      data->tile = 0x0022;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_BlueLightBackground4:
      data->tile = 0x0023;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_GreenPoisonBackground:
      data->tile = 0x0028;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_LavaBackground:
      data->tile = 0x002C;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_WindowLeftBackground:
      data->tile = ANIM_WINDOWBG;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_WindowRightBackground:
      data->tile = ANIM_WINDOWBG + 1;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_StoneWindowBackground:
      data->tile = ANIM_STONEWINDOWBG;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_BrokenWallBackground:
      data->tile = ANIM_BROKENWALLBG;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    default:
      /* we got a type which should not be an animation. */
      printf(__FILE__ ":%d: warning: animation #%d"
          " added which is not an animation\n",
          __LINE__, p.general->actor_type);
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Delete a simple animation.
 *
 * @param  actor  The animation actor.
 */
void fn_level_actor_function_simpleanimation_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_simpleanimation_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}


/* --------------------------------------------------------------- */

/**
 * Action for simple animation.
 *
 * @param  actor  The animation actor.
 */
void fn_level_actor_function_simpleanimation_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_simpleanimation_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;
}

/* --------------------------------------------------------------- */

/**
 * Blit the simple animation.
 *
 * @param  actor  The animation actor.
 */
void fn_level_actor_function_simpleanimation_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_simpleanimation_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The jumping red ball.
 */
typedef struct fn_level_actor_redball_jumping_data_t {
  /**
   * The tile number which is to be blitted in the level.
   */
  Uint16 tile;
  /**
   * The counter for the position inside the loop.
   */
  Uint8 counter;
  /**
   * The base position of the ball.
   */
  Uint16 base_y;
} fn_level_actor_redball_jumping_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_redball_jumping_data_t * data = malloc(
      sizeof(fn_level_actor_redball_jumping_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->counter = 0;
  data->tile = ANIM_MINE;
  data->base_y = p.general->position.y;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_redball_jumping_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  p.general->hurts_hero = true;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_hero_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  p.general->hurts_hero = false;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_redball_jumping_data_t * data = p.specific;

  Uint8 distance = 0;
  switch(data->counter) {
    case 0:
      distance = 0;
      break;
    case 1:
    case 11:
      distance = 16;
      break;
    case 2:
    case 10:
      distance = 28;
      break;
    case 3:
    case 9:
      distance = 36;
      break;
    case 4:
    case 8:
      distance = 40;
      break;
    case 5:
    case 7:
      distance = 41;
      break;
    case 6:
      distance = 42;
      break;
    default:
      distance = 0;
      break;
  }
  p.general->position.y = data->base_y - distance;

  data->counter++;
  data->counter %= 12;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_redball_jumping_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_jumping_shot(
        fn_level_actor_shot_params_t p)
{
  /*
   * Do nothing. This function just exists so that the red
   * ball absorbs the bullet when it is shot.
   */
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The lying red ball.
 */
typedef struct fn_level_actor_redball_lying_data_t {
  /**
   * The tile number which is to be blitted in the level.
   */
  Uint16 tile;
  /**
   * Flag that stores if the redball is touching the hero.
   */
  Uint8 touching_hero;
} fn_level_actor_redball_lying_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_lying_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_redball_lying_data_t * data = malloc(
      sizeof(fn_level_actor_redball_lying_data_t));
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->tile = ANIM_MINE;
  data->touching_hero = 0;
}


/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_lying_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_redball_lying_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_lying_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_redball_lying_data_t * data = p.specific;
  if (!data->touching_hero) {
    data->touching_hero = 1;
    p.general->hurts_hero = true;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_lying_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_redball_lying_data_t * data = p.specific;

  if (!fn_level_solids_get(&(p.level_data->solids),
        (p.general->position.x) / FN_TILE_WIDTH,
        (p.general->position.y) / FN_TILE_HEIGHT + 1)) {
    p.general->position.y += FN_HALFTILE_HEIGHT;
  }

  if (data->touching_hero == 1) {
    data->touching_hero++;
  } else if (data->touching_hero > 1) {
    p.general->hurts_hero = false;
    p.general->is_alive = 0;
    fn_level_actor_queue_push_back(
            p.actor_queue,
            ActorType_Fire,
            p.general->position.x,
            p.general->position.y);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_redball_lying_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_redball_lying_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The robot.
 */
typedef struct fn_level_actor_robot_data_t {
  /**
   * The direction to which the robot moves.
   */
  FnHorizontalDirection direction;
  /**
   * The tile number.
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
   * A flag indicating if the robot is currently touching the hero.
   */
  Uint8 touching_hero;
} fn_level_actor_robot_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_robot_data_t * data = malloc(
      sizeof(fn_level_actor_robot_data_t));
  *(p.specific) = data;
  p.general->is_in_foreground = true;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->direction = HorizontalDirection_Left;
  data->tile = ANIM_ROBOT;
  data->current_frame = 0;
  data->num_frames = 3;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_robot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_robot_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_robot_data_t * data = p.specific;
  p.general->hurts_hero = false;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_robot_data_t * data = p.specific;
  data->current_frame++;

  data->current_frame %= data->num_frames;
  if (!fn_level_solids_get(&(p.level_data->solids),
        (p.general->position.x) / FN_TILE_WIDTH,
        (p.general->position.y) / FN_TILE_HEIGHT + 1)) {
    /* still in the air, so let the robot fall down. */
    p.general->position.y += FN_HALFTILE_HEIGHT;
  } else {
    /* on the floor, so let's walk */
    if (data->current_frame == 0) {
      int direction = (
          data->direction == HorizontalDirection_Left ?
          -1 :
          2);
      if (
          /* Check if the place next to the bot is free */
          !fn_level_solids_get(&(p.level_data->solids),
            (
             p.general->position.x +
             direction * FN_HALFTILE_WIDTH) /
            FN_TILE_WIDTH,
            (p.general->position.y) / FN_TILE_HEIGHT) &&
          /* Check if it is solid below this place */
          fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x + direction * FN_HALFTILE_WIDTH) /
            FN_TILE_WIDTH,
            (p.general->position.y+FN_TILE_HEIGHT) / FN_TILE_HEIGHT)
         )
      {
        if (direction == 2) direction = 1;
        p.general->position.x += direction * FN_HALFTILE_WIDTH;
      } else {
        data->direction = (
            data->direction == HorizontalDirection_Left ?
            HorizontalDirection_Right :
            HorizontalDirection_Left);

        if (direction == 2) direction = 1;
        direction *= (-1);
        p.general->position.x += direction * FN_HALFTILE_WIDTH;
      }
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_robot_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_robot_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_robot_data_t * data = p.specific;

  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  fn_hero_score_add(score, 100);
  if (data->touching_hero) {
    p.general->hurts_hero = false;
    data->touching_hero = 0;
  }
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_RobotDisappearing,
      p.general->position.x,
      p.general->position.y);
  p.general->is_alive = 0;
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The tankbot.
 */
typedef struct fn_level_actor_tankbot_data_t {
  /**
   * The direction to which the robot moves.
   */
  FnHorizontalDirection direction;
  /**
   * The tile number.
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
   * A counter counting how often the bot was hit by the hero.
   * If 0, the bot is healthy. If 1, the bot was hit once and is
   * damaged (pushes steam clouds out), if 2 the bot is killed.
   */
  Uint8 was_shot;
  /**
   * A flag indicating if the robot is currently touching the hero.
   */
  Uint8 touching_hero;
} fn_level_actor_tankbot_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_tankbot_data_t * data = malloc(
      sizeof(fn_level_actor_tankbot_data_t));
  *(p.specific) = data;
  p.general->is_in_foreground = true;
  p.general->position.w = FN_TILE_WIDTH * 2;
  p.general->position.h = FN_TILE_HEIGHT;
  data->direction = HorizontalDirection_Left;
  data->tile = ANIM_CARBOT;
  data->current_frame = 0;
  data->num_frames = 4;
  data->was_shot = 0;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_tankbot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_tankbot_data_t * data = p.specific;
  if (data->was_shot < 2) {
    p.general->hurts_hero = true;
    data->touching_hero = 1;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_hero_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_tankbot_data_t * data = p.specific;
  if (data->was_shot < 2) {
    p.general->hurts_hero = false;
    data->touching_hero = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_tankbot_data_t * data = p.specific;
  data->current_frame++;
  if (data->was_shot == 2) {
    /* create explosion */
    p.general->is_alive = false;
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Explosion,
        p.general->position.x + FN_HALFTILE_WIDTH,
        p.general->position.y);
    FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
    fn_hero_score_add(score, 2500);
    fn_level_actor_queue_push_particle_firework(
        p.actor_queue,
        p.general->position.x,
        p.general->position.y,
        4);
  } else {
    data->current_frame %= data->num_frames;
    if (!fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x) / FN_TILE_WIDTH,
          (p.general->position.y) / FN_TILE_HEIGHT + 1) &&
        !fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x) / FN_TILE_WIDTH + 1,
          (p.general->position.y) / FN_TILE_HEIGHT + 1)) {
      /* still in the air, so let the robot fall down */
      p.general->position.y += FN_HALFTILE_HEIGHT;
    } else {
      /* on the floor, so let's walk */
      int direction = (data->direction == HorizontalDirection_Left ?
          -1 : 4);
      if (
          /* check if the place next to the bot is free */
          !fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x +
             direction * FN_HALFTILE_WIDTH) / FN_TILE_WIDTH,
            (p.general->position.y) / FN_TILE_HEIGHT) &&
          /* check if it is solid below this place */
          fn_level_solids_get(&(p.level_data->solids),
            (p.general->position.x +
             direction * FN_HALFTILE_WIDTH) / FN_TILE_WIDTH,
            (p.general->position.y + FN_TILE_HEIGHT) /
            FN_TILE_HEIGHT)
         )
      {
        if (direction > 0) {
          direction = 1;
        }
        p.general->position.x += direction * FN_HALFTILE_WIDTH * 0.7;
      } else {
        /* Reached the end, so turn around */
        data->direction = (
            data->direction == HorizontalDirection_Left ?
            HorizontalDirection_Right :
            HorizontalDirection_Left);
        if (direction > 0) direction = 1;
        direction *= -1;
        p.general->position.x += direction * FN_HALFTILE_WIDTH;
        data->tile += 4 * direction;

        if (direction > 0) {
          fn_level_actor_queue_push_back(p.actor_queue,
             ActorType_HostileShotRight,
             p.general->position.x,
             p.general->position.y - 6);
        } else {
          fn_level_actor_queue_push_back(p.actor_queue,
             ActorType_HostileShotLeft,
             p.general->position.x,
             p.general->position.y - 6);
        }
      }
    }
  }
  if (data->was_shot == 1) {
    /* create steam clouds */
    if (data->current_frame == 0) {
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Steam,
          p.general->position.x + FN_HALFTILE_WIDTH,
          p.general->position.y - FN_TILE_HEIGHT);
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_tankbot_data_t * data = p.specific;

  
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame/2) * 2);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame/2) * 2 + 1);
  destrect.x += FN_TILE_WIDTH;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_tankbot_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_tankbot_data_t * data = p.specific;

  if (data->was_shot == 1 && data->touching_hero) {
    p.general->hurts_hero = false;
    data->touching_hero = 0;
  }
  if (!(data->was_shot == 2)) {
    data->was_shot++;
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The firewheel bot.
 */
typedef struct fn_level_actor_firewheelbot_data_t {
  /**
   * The direction to which the robot moves.
   */
  FnHorizontalDirection direction;
  /**
   * The tile number.
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
   * A counter counting how often the bot was hit by the hero.
   * If 0, the bot is healthy. If 1, the bot was hit once and is
   * damaged (pushes steam clouds out), if 2 the bot is killed.
   */
  Uint8 was_shot;
  /**
   * A flag indicating if the robot is currently touching the hero.
   */
  Uint8 touching_hero;
  /**
   * A flag indicating if the robot currently has fire on.
   */
  Uint8 fire_is_on;
  /**
   * A counter for the number of cycles how long the fire is on.
   */
  Uint8 counter;
} fn_level_actor_firewheelbot_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = malloc(
      sizeof(fn_level_actor_firewheelbot_data_t));
  *(p.specific) = data;
  data->direction = HorizontalDirection_Left;
  data->tile = ANIM_FIREWHEEL_OFF;
  data->current_frame = 0;
  data->num_frames = 4;
  data->was_shot = 0;
  data->touching_hero = 0;
  data->fire_is_on = 0;
  data->counter = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = p.specific;
  if (data->was_shot < 2) {
    p.general->hurts_hero = true;
    data->touching_hero = 1;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = p.specific;
  if (data->was_shot < 2) {
    p.general->hurts_hero = false;
    data->touching_hero = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = p.specific;
  if (data->was_shot == 2) {
    /* create explosion */
    p.general->is_alive = 0;
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Explosion,
        p.general->position.x + FN_HALFTILE_WIDTH,
        p.general->position.y);
    fn_level_actor_queue_push_particle_firework(
        p.actor_queue,
        p.general->position.x,
        p.general->position.y,
        8);
    FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
    fn_hero_score_add(score, 2500);
  } else {
    data->counter++;
    if (data->counter % 2) {
      data->current_frame++;
      data->current_frame %= data->num_frames;
    }

    if (data->counter == 50) {
      data->counter = 0;
      data->fire_is_on = !(data->fire_is_on);
      if (data->fire_is_on) {
        data->tile = ANIM_FIREWHEEL_ON;
      } else {
        data->tile = ANIM_FIREWHEEL_OFF;
      }
    }

    int direction = (data->direction == HorizontalDirection_Left ?
          -1 : 1);
    if (!fn_level_push_rect_standing_on_solid_ground(
        &(p.level_data->solids),
        p.general->position,
        direction * FN_HALFTILE_WIDTH / 2,
        FN_HALFTILE_HEIGHT))
    {
      /* push was not successful, so we reverse the direction */
      data->direction =
        (data->direction == HorizontalDirection_Left ?
         HorizontalDirection_Right :
         HorizontalDirection_Left);
    }

    if (data->was_shot == 1) {
      /* create steam clouds */
      if (data->current_frame == 0) {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_Steam,
            p.general->position.x + FN_HALFTILE_WIDTH,
            p.general->position.y - FN_TILE_HEIGHT);
      }
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = p.specific;

  FnGeometry destrect;
  
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame) * 4);
  destrect.x =
      p.general->position.x +
      p.general->position.w / 2 -
      FN_TILE_WIDTH;
  destrect.y = p.general->position.y - FN_TILE_HEIGHT;
  destrect.w = FN_TILE_WIDTH * 2;
  destrect.h = p.general->position.h;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame) * 4 + 1);
  destrect.x += FN_TILE_WIDTH;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.x -= FN_TILE_WIDTH;
  destrect.y += FN_TILE_HEIGHT;
  tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame) * 4 + 2);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.x += FN_TILE_WIDTH;
  tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + (data->current_frame) * 4 + 3);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_firewheelbot_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_firewheelbot_data_t * data = p.specific;

  if (!(data->fire_is_on)) {
    if (data->was_shot == 1 && data->touching_hero) {
      p.general->hurts_hero = false;
      data->touching_hero = 0;
    }
    if (!(data->was_shot == 2)) {
      data->was_shot++;
    }
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The wallcrawler bot.
 */
typedef struct fn_level_actor_wallcrawler_data_t {
  /**
   * The direction to which the wallcrawler is moving.
   */
  FnVerticalDirection direction;
  /**
   * The direction to which the wallcrawler is orientated.
   */
  FnHorizontalDirection orientation;
  /**
   * The tile number.
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
   * A flag indicating if the robot was shot.
   */
  Uint8 was_shot;
  /**
   * A flag indicating if the robot is currently touching the hero.
   */
  Uint8 touching_hero;
} fn_level_actor_wallcrawler_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_create(
        fn_level_actor_create_params_t p)
{

  fn_level_actor_wallcrawler_data_t * data = malloc(
      sizeof(fn_level_actor_wallcrawler_data_t));
  *(p.specific) = data;
  p.general->is_in_foreground = 1;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->direction = VerticalDirection_Up;
  if (p.general->actor_type == ActorType_WallCrawlerBotLeft) {
    data->tile = ANIM_WALLCRAWLERBOT_LEFT;
    data->orientation = HorizontalDirection_Left;
  } else {
    data->tile = ANIM_WALLCRAWLERBOT_RIGHT;
    data->orientation = HorizontalDirection_Right;
  }

  data->current_frame = 0;
  data->num_frames = 4;
  data->was_shot = 0;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = p.specific;
  if (!data->was_shot) {
    p.general->hurts_hero = true;
    data->touching_hero = 1;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = p.specific;
  if (!data->was_shot) {
    p.general->hurts_hero = false;
    data->touching_hero = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = p.specific;

  int direction = (data->direction == VerticalDirection_Up ?
      1 :
      -1);
  int orientation = (data->orientation == HorizontalDirection_Left ?
      -1 :
      1);
  if (direction > 0) {
    /* going up */
    data->current_frame++;
    data->current_frame %= data->num_frames;

    if (
        /* bot collides with solid tile */
        fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x) / FN_TILE_WIDTH,
          (p.general->position.y - 1) / FN_TILE_HEIGHT) ||
        /* bot has no more wall to stick upon */
        !fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x +
           orientation * FN_TILE_WIDTH) /
          FN_TILE_WIDTH,
          (p.general->position.y - 1) / FN_TILE_HEIGHT)
       ) {
      p.general->position.y++;
      data->direction = VerticalDirection_Down;
    } else {
      p.general->position.y--;
    }

  } else {
    /* going down */
    if (data->current_frame == 0) {
      data->current_frame = data->num_frames;
    }
    data->current_frame--;

    if (
        /* bot collides with solid tile */
        fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x) / FN_TILE_WIDTH,
          (p.general->position.y + FN_TILE_HEIGHT) /
          FN_TILE_HEIGHT) ||
        /* bot has no more wall to stick upon */
        !fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x + orientation * FN_TILE_WIDTH) /
          FN_TILE_WIDTH,
          (p.general->position.y + FN_TILE_HEIGHT) / FN_TILE_HEIGHT)
       ) {
      p.general->position.y--;
      data->direction = VerticalDirection_Up;
    } else {
      p.general->position.y++;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = p.specific;

  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_wallcrawler_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_wallcrawler_data_t * data = p.specific;

  if (!data->was_shot) {
    if (data->touching_hero) {
      p.general->hurts_hero = false;
      data->touching_hero = 0;
      data->current_frame = 0;
    }
    p.general->is_alive = false;
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Steam,
        p.general->position.x,
        p.general->position.y);
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Explosion,
        p.general->position.x,
        p.general->position.y);
    FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
    fn_hero_score_add(score, 100);
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef enum fn_level_actor_lift_state_e {
  fn_level_actor_lift_state_idle,
  fn_level_actor_lift_state_ascending,
  fn_level_actor_lift_state_descending,
} fn_level_actor_lift_state_e;

/* --------------------------------------------------------------- */

/**
 * The lift.
 */
typedef struct fn_level_actor_lift_data_t {
  /**
   * The state of the lift.
   */
  fn_level_actor_lift_state_e state;
} fn_level_actor_lift_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a lift.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_lift_data_t * data = malloc(
      sizeof(fn_level_actor_lift_data_t));
  data->state = fn_level_actor_lift_state_idle;
  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = false;
}

/* --------------------------------------------------------------- */

/**
 * Delete the lift.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_lift_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Hero starts to interact with lift.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_interact_start(
        fn_level_actor_interact_start_params_t p)
{
  fn_level_actor_lift_data_t * data = p.specific;

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);
  if (fn_geometry_touches(hero_geometry, p.general->position) &&
      hero_geometry.y + hero_geometry.h == p.general->position.y) {
    data->state = fn_level_actor_lift_state_ascending;
  }
}

/* --------------------------------------------------------------- */

/**
 * Hero stops to interact with lift.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_interact_end(
        fn_level_actor_interact_end_params_t p)
{
  fn_level_actor_lift_data_t * data = p.specific;

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);
  if (fn_geometry_touches(hero_geometry, p.general->position) &&
      hero_geometry.x == p.general->position.x) {
    data->state = fn_level_actor_lift_state_idle;
  } else {
    data->state = fn_level_actor_lift_state_descending;
  }
}

/* --------------------------------------------------------------- */

/**
 * Lift acts.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_lift_data_t * data = p.specific;

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  if (data->state == fn_level_actor_lift_state_ascending ||
      (data->state == fn_level_actor_lift_state_idle &&
       p.general->position.h > FN_TILE_HEIGHT))
  {
    /* check if hero leaves elevator. */
    FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);
    if (!fn_geometry_touches(hero_geometry, p.general->position) ||
        p.general->position.x != hero_geometry.x) {
      data->state = fn_level_actor_lift_state_descending;
    }
  }

  switch(data->state)
  {
    case fn_level_actor_lift_state_ascending:
      if (fn_level_solids_get(&(p.level_data->solids),
            p.general->position.x/FN_TILE_WIDTH,
            p.general->position.y/FN_TILE_HEIGHT-3)) {
        data->state = fn_level_actor_lift_state_idle;
      } else {
        Sint16 offset = fn_hero_position_push_vertically(
            hero_position, &(p.level_data->solids), -FN_TILE_HEIGHT);
        if (-offset < FN_TILE_HEIGHT) {
          offset = fn_hero_position_push_vertically(
                  hero_position, &(p.level_data->solids), -offset);
          data->state = fn_level_actor_lift_state_idle;
        } else {
          p.general->position.h -= offset;
          p.general->position.y += offset;

          fn_level_solids_set(&(p.level_data->solids),
              p.general->position.x/FN_TILE_WIDTH,
              p.general->position.y/FN_TILE_HEIGHT,
              1);
        }

      }
      break;
    case fn_level_actor_lift_state_descending:
      {
        int i = 0;
        for (i = 0; i < 2; i++) {
          if (p.general->position.h > FN_TILE_HEIGHT) {
            fn_level_solids_set(&(p.level_data->solids),
                p.general->position.x/FN_TILE_WIDTH,
                p.general->position.y/FN_TILE_HEIGHT,
                0);
            p.general->position.y += FN_TILE_HEIGHT;
            p.general->position.h -= FN_TILE_HEIGHT;
          } else {
            data->state = fn_level_actor_lift_state_idle;
          }
        }
      }
      break;
    case fn_level_actor_lift_state_idle:
      /* nothing to do, we stay where we are */
      break;
    default:
      /* we are in an invalid state. */
      printf(__FILE__ ":%d: warning: lift "
          "is in invalid state.\n",
          __LINE__);
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit the lift.
 *
 * @param  actor  The lift actor.
 */
void fn_level_actor_function_lift_blit(
        fn_level_actor_blit_params_t p)
{
  FnGeometry destrect = p.general->position;

  int i = 0;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache, SOLID_START + 23);
  for (i = 0;
      i < p.general->position.h - FN_TILE_HEIGHT;
      i += FN_HALFTILE_HEIGHT) {
    destrect.y += FN_HALFTILE_HEIGHT;
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
  }

  tile = fn_tilecache_get_tile(p.tilecache,
      OBJ_ELEVATOR);
  destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The acme stone.
 */
typedef struct fn_level_actor_acme_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The counter for the state.
   */
  Uint8 counter;
  /**
   * A flag indicating if the hero is being touched.
   */
  Uint8 touching_hero;
} fn_level_actor_acme_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_acme_data_t * data = malloc(
      sizeof(fn_level_actor_acme_data_t));
  *(p.specific) = data;
  data->tile = OBJ_FALLINGBLOCK;
  data->counter = 0;
  data->touching_hero = 0;
  p.general->position.w = FN_TILE_WIDTH * 2;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_acme_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_acme_data_t * data = p.specific;

  FnHeroPosition * hero_position = fn_hero_data_get_position(p.hero_data);
  FnGeometry hero_geometry = fn_hero_position_get_geometry(hero_position);

  switch(data->counter) {
    case 0:
      {
        Uint16 xl = p.general->position.x;
        Uint16 xr = xl + p.general->position.w;
        Uint16 y = p.general->position.y;
        Uint32 hxl = hero_geometry.x;
        Uint32 hxr = hxl + FN_TILE_WIDTH;
        Uint32 hy = hero_geometry.y;

        if (y < hy && /* actor higher than hero */
            xl < hxr &&
            xr > hxl) {
          /* check if there are solid parts between hero and acme */
          Uint16 i = 0;
          Uint8 solidbetween = 0;
          for (i = y + FN_TILE_HEIGHT;
              i < hy && !solidbetween;
              i += FN_TILE_HEIGHT) {
            if (fn_level_solids_get(&(p.level_data->solids),
                  xl / FN_TILE_WIDTH, i / FN_TILE_WIDTH) ||
                fn_level_solids_get(&(p.level_data->solids),
                  xl / FN_TILE_WIDTH + 1, i / FN_TILE_WIDTH)) {
              solidbetween = 1;
            }
          }
          if (!solidbetween) {
            data->counter++;
          }
        }
      }
      break;
    case 1:
    case 3:
    case 5:
    case 7:
    case 9:
      p.general->position.y++;
      data->counter++;
      break;
    case 2:
    case 4:
    case 6:
    case 8:
    case 10:
      p.general->position.y--;
      data->counter++;
      break;
    default:
      if (fn_level_solids_get(&(p.level_data->solids),
            p.general->position.x / FN_TILE_WIDTH,
            (p.general->position.y / FN_TILE_HEIGHT) + 1))
      {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_Steam,
            p.general->position.x + FN_HALFTILE_WIDTH,
            p.general->position.y);
        fn_level_actor_queue_push_particle_firework(
            p.actor_queue,
            p.general->position.x,
            p.general->position.y,
            4);
        p.general->is_alive = 0;
      } else {
        p.general->position.y += FN_TILE_HEIGHT;
      }
      break;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_acme_data_t * data = p.specific;

  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  tile = fn_tilecache_get_tile(p.tilecache, data->tile+1);
  destrect.x += FN_TILE_WIDTH;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_acme_data_t * data = p.specific;

  if (data->counter > 0) {
    FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
    fn_hero_score_add(score, 500);
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Score500,
        p.general->position.x,
        p.general->position.y);
    p.general->is_alive = 0;
    fn_level_actor_queue_push_particle_firework(
        p.actor_queue,
        p.general->position.x,
        p.general->position.y,
        4);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_acme_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_acme_data_t * data = p.specific;

  if (data->counter > 10 && !data->touching_hero) {
    data->touching_hero = 1;
    p.general->hurts_hero = true;
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

typedef enum fn_level_actor_fire_state_e {
  fn_level_actor_fire_state_off,
  fn_level_actor_fire_state_ignition,
  fn_level_actor_fire_state_burning
} fn_level_actor_fire_state_e;

/* --------------------------------------------------------------- */

/**
 * The burning fire.
 */
typedef struct fn_level_actor_fire_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The direction to which the fire burns.
   */
  FnHorizontalDirection direction;
  /**
   * The state of the fire.
   */
  fn_level_actor_fire_state_e state;
  /**
   * Counter for animation.
   */
  Uint8 counter;
  /**
   * Flag indicating if the hero is currently being touched.
   */
  Uint8 touching_hero;
} fn_level_actor_fire_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_fire_data_t * data = malloc(
      sizeof(fn_level_actor_fire_data_t));
  *(p.specific) = data;

  p.general->position.w = FN_TILE_WIDTH * 3;
  p.general->position.h = FN_TILE_HEIGHT;

  if (p.general->actor_type == ActorType_FireRight) {
    data->tile = OBJ_FIRERIGHT;
    data->direction = HorizontalDirection_Right;
  } else {
    data->tile = OBJ_FIRELEFT;
    data->direction = HorizontalDirection_Left;
    p.general->position.x -= 2 * FN_TILE_WIDTH;
  }
  data->counter = 0;
  data->state = fn_level_actor_fire_state_off;
  data->touching_hero = 0;
  p.general->is_in_foreground = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_fire_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_fire_data_t * data = p.specific;
  data->touching_hero = 1;

  if (data->state == fn_level_actor_fire_state_burning) {
    p.general->hurts_hero = true;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_hero_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_fire_data_t * data = p.specific;
  data->touching_hero = 0;

  if (data->state == fn_level_actor_fire_state_burning) {
    p.general->hurts_hero = false;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_fire_data_t * data = p.specific;
  switch(data->state)
  {
    case fn_level_actor_fire_state_off:
      if (data->counter == 40) {
        data->counter = 0;
        data->state = fn_level_actor_fire_state_ignition;
      }
      break;
    case fn_level_actor_fire_state_ignition:
      if (data->counter == 20) {
        data->counter = 0;
        data->state = fn_level_actor_fire_state_burning;
        if (data->touching_hero) {
          p.general->hurts_hero = true;
        }
      }
      break;
    case fn_level_actor_fire_state_burning:
      if (data->counter == 20) {
        data->counter = 0;
        data->state = fn_level_actor_fire_state_off;
        if (data->touching_hero) {
          p.general->hurts_hero = false;
        }
      }
      break;
    default:
      printf(__FILE__ ":%d: warning: fire "
          "is in invalid state.\n",
          __LINE__);
      break;
  }
  data->counter++;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fire_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_fire_data_t * data = p.specific;
  const FnTexture * tile0 = NULL;
  const FnTexture * tile1 = NULL;
  const FnTexture * tile2 = NULL;

  switch(data->state)
  {
    case fn_level_actor_fire_state_off:
      break;
    case fn_level_actor_fire_state_ignition:
      if (data->counter % 2) {
        if (data->direction == HorizontalDirection_Left) {
          tile2 = fn_tilecache_get_tile(p.tilecache, data->tile);
        } else {
          tile0 = fn_tilecache_get_tile(p.tilecache, data->tile);
        }
      }
      break;
    case fn_level_actor_fire_state_burning:
      tile1 = fn_tilecache_get_tile(
              p.tilecache,
              data->tile + 1 + (data->counter % 2));
      if (data->direction == HorizontalDirection_Left) {
        tile2 = tile1;
        tile0 = fn_tilecache_get_tile(p.tilecache, data->tile + 3 +
            (data->counter % 2));
      } else {
        tile0 = tile1;
        tile2 = fn_tilecache_get_tile(p.tilecache, data->tile + 3 +
            (data->counter % 2));
      }
      break;
    default:
      printf(__FILE__ ":%d: warning: fire "
          "is in invalid state.\n",
          __LINE__);
      break;
  }
  FnGeometry destrect = p.general->position;
  if (tile0 != NULL) {
    fn_texture_blit_to_sdl_surface(tile0, NULL, p.target, &destrect);
  }
  destrect.x += FN_TILE_WIDTH;
  if (tile1 != NULL) {
    fn_texture_blit_to_sdl_surface(tile1, NULL, p.target, &destrect);
  }
  destrect.x += FN_TILE_WIDTH;
  if (tile2 != NULL) {
    fn_texture_blit_to_sdl_surface(tile2, NULL, p.target, &destrect);
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The rotating mill.
 */
typedef struct fn_level_actor_mill_data_t {
  /**
   * The tile number which is to be blitted to the level.
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
   * The number of lives that the mill still has.
   */
  Uint8 lives;
} fn_level_actor_mill_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_mill_data_t * data = malloc(
      sizeof(fn_level_actor_mill_data_t));
  *(p.specific) = data;
  p.general->is_in_foreground = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  data->tile = OBJ_ROTATECYLINDER;
  data->current_frame = 0;
  data->num_frames = 5;
  data->lives = 10;

  while (!fn_level_solids_get(&(p.level_data->solids),
        p.general->position.x / FN_TILE_WIDTH,
        p.general->position.y / FN_TILE_HEIGHT - 1)) {
    p.general->position.y -= FN_TILE_HEIGHT;
    p.general->position.h += FN_TILE_HEIGHT;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_mill_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_mill_data_t * data = p.specific;
  if (data->lives > 0) {
    /* TODO check if this is okay or if we need to go beyond 0 */
    FnHeroHealth * health = fn_hero_data_get_health(p.hero_data);
    fn_hero_health_kill(health);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_mill_data_t * data = p.specific;
  if (data->lives > 0) {
    data->current_frame++;
    data->current_frame %= data->num_frames;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_mill_data_t * data = p.specific;

  FnGeometry destrect;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  destrect.x = p.general->position.x;
  destrect.y = p.general->position.y;
  destrect.w = FN_TILE_WIDTH;
  destrect.h = FN_TILE_HEIGHT;

  int i = 0;
  for (i = 0; i < (p.general->position.h / FN_TILE_HEIGHT); i++) {
    fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
    destrect.y += FN_TILE_HEIGHT;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_mill_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_mill_data_t * data = p.specific;
  
  data->lives--;
  if (data->lives > 0) {
    fn_level_actor_queue_push_particle_firework(
        p.actor_queue,
        p.general->position.x + p.general->position.w / 2,
        p.general->position.y + p.general->position.h / 2,
        4);
  } else {
    /* TODO add removal animation (destroyed body) */
    p.general->is_alive = 0;
    FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
    fn_hero_score_add(score, 20000);
    fn_level_actor_queue_push_particle_firework(
        p.actor_queue,
        p.general->position.x + p.general->position.w / 2,
        p.general->position.y + p.general->position.h / 2,
        20);
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Score10000,
        p.general->position.x,
        p.general->position.y + p.general->position.h / 2 - FN_TILE_HEIGHT);
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Score10000,
        p.general->position.x,
        p.general->position.y + p.general->position.h / 2);
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The accesscard slot.
 */
typedef struct fn_level_actor_acces_card_slot_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames.
   */
  Uint8 num_frames;
} fn_level_actor_access_card_slot_data_t;

/* --------------------------------------------------------------- */

/**
 * Create an accesscard slot.
 *
 * @param  actor  The accesscard slot actor.
 */
void fn_level_actor_function_accesscard_slot_create(
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  fn_level_actor_access_card_slot_data_t * data = malloc(
      sizeof(fn_level_actor_access_card_slot_data_t));
  data->tile = OBJ_ACCESS_CARD_SLOT;
  data->current_frame = 0;
  data->num_frames = 8;
  *(p.specific) = data;
  p.general->is_in_foreground = false;
}

/* --------------------------------------------------------------- */

/**
 * Delete an accesscard slot.
 *
 * @param  actor  The accesscard slot actor.
 */
void fn_level_actor_function_accesscard_slot_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_access_card_slot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Interact with an accesscard slot.
 *
 * @param  actor  The accesscard slot actor.
 */
void fn_level_actor_function_accesscard_slot_interact_start(
        fn_level_actor_interact_start_params_t p)
{
  fn_level_actor_access_card_slot_data_t * data = p.specific;

  FnHeroInventory * inventory = fn_hero_data_get_inventory(p.hero_data);
  if (fn_hero_inventory_is_set(inventory, InventoryItem_AccessCard)) {
    fn_level_actor_message_queue_push_back(
            p.actor_message_queue,
            ActorType_AccessCardDoor,
            ActorMessageType_OpenDoor);
    data->current_frame = 0;
    data->num_frames = 1;
    data->tile = OBJ_ACCESS_CARD_SLOT + 8;
    fn_hero_inventory_unset(inventory, InventoryItem_AccessCard);
  } else {
    fn_info_message_queue_push(
            p.info_message_queue,
            "You don't have the access card\n");
  }
}

/* --------------------------------------------------------------- */

/**
 * Let the accesscard slot act.
 *
 * @param  actor  The accesscard slot actor.
 */
void fn_level_actor_function_accesscard_slot_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_access_card_slot_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;
}

/* --------------------------------------------------------------- */

/**
 * Blit the accesscard slot.
 *
 * @param  actor  The accesscard slot actor.
 */
void fn_level_actor_function_accesscard_slot_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_access_card_slot_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The states in which the golve slot can be.
 */
typedef enum fn_level_actor_glove_slot_state_e {
  fn_level_actor_glove_slot_state_idle,
  fn_level_actor_glove_slot_state_shooting,
  fn_level_actor_glove_slot_state_expanded
} fn_level_actor_glove_slot_state_e;

/* --------------------------------------------------------------- */

/**
 * The glove slot.
 */
typedef struct fn_level_actor_glove_slot_data_t {
  /**
   * The tile number for the tilecache.
   */
  Uint16 tile;
  /**
   * The number of the current frame.
   */
  Uint8 current_frame;
  /**
   * The number of frames.
   */
  Uint8 num_frames;
  /**
   * The state of the slot.
   */
  fn_level_actor_glove_slot_state_e state;
  /**
   * The countdown for state shooting.
   */
  Uint8 countdown;
} fn_level_actor_glove_slot_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_glove_slot_create(
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  fn_level_actor_glove_slot_data_t * data = malloc(
      sizeof(fn_level_actor_glove_slot_data_t));
  data->tile = OBJ_GLOVE_SLOT;
  data->current_frame = 0;
  data->num_frames = 4;
  data->state = fn_level_actor_glove_slot_state_idle;
  data->countdown = 0;
  *(p.specific) = data;
  p.general->is_in_foreground = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_glove_slot_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_glove_slot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_glove_slot_interact_start(
        fn_level_actor_interact_start_params_t p)
{
  fn_level_actor_glove_slot_data_t * data = p.specific;
  FnHeroInventory * inventory = fn_hero_data_get_inventory(p.hero_data);
  switch(data->state)
  {
    case fn_level_actor_glove_slot_state_idle:
      if (fn_hero_inventory_is_set(inventory, InventoryItem_Glove)) {
          fn_level_actor_message_queue_push_back(
                  p.actor_message_queue,
                  ActorType_ExpandingFloor,
                  ActorMessageType_Expand);
          data->state = fn_level_actor_glove_slot_state_expanded;
      } else {
        data->state = fn_level_actor_glove_slot_state_shooting;
        data->countdown = 20;
      }
      break;
    case fn_level_actor_glove_slot_state_shooting:
      /* nothing to do */
      break;
    case fn_level_actor_glove_slot_state_expanded:
      /* nothing to do */
      break;
    default:
      /* we got an invalid state. */
      printf(__FILE__ ":%d: warning: glove slot"
          " is in state %d which is invalud.\n",
          __LINE__, data->state);
      break;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_glove_slot_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_glove_slot_data_t * data = p.specific;


  switch(data->state)
  {
    case fn_level_actor_glove_slot_state_idle:
      data->current_frame++;
      data->current_frame %= data->num_frames;
      break;
    case fn_level_actor_glove_slot_state_shooting:
      data->current_frame++;
      data->current_frame %= data->num_frames;

      data->countdown--;
      if (data->countdown % 4 == 0) {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_HostileShotRight,
            p.general->position.x, p.general->position.y);
      } else if (data->countdown % 4 == 2) {
        fn_level_actor_queue_push_back(p.actor_queue,
            ActorType_HostileShotLeft,
            p.general->position.x, p.general->position.y);
      }
      if (data->countdown == 0) {
        data->state = fn_level_actor_glove_slot_state_idle;
      }
      break;
    case fn_level_actor_glove_slot_state_expanded:
      /* nothing to do */
      break;
    default:
      /* we got an invalid state. */
      printf(__FILE__ ":%d: warning: glove slot"
          " is in state %d which is invalud.\n",
          __LINE__, data->state);
      break;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_glove_slot_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_glove_slot_data_t * data = p.specific;
  Uint8 adder = (data->current_frame == 0 ? 0 : 1);
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + adder);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.x -= FN_TILE_WIDTH;
  tile = fn_tilecache_get_tile(p.tilecache, data->tile + 2);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.x += 2 * FN_TILE_WIDTH;
  tile = fn_tilecache_get_tile(p.tilecache, data->tile + 3);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The item struct.
 * Items are elements in the game which fall to the floor.
 * They are one part high, one part wide and
 * have either a single frame or a fixed number
 * of frames that appear in order and are lined up in a row
 * inside the tilecache.
 */
typedef struct fn_level_actor_item_data_t {
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
   * Are we standing on the ground? If no, this is zero, otherwise 1.
   */
  Uint8 standing_on_ground;
} fn_level_actor_item_data_t;

/* --------------------------------------------------------------- */

/**
 * Create an item.
 *
 * @param  actor The item actor.
 */
void fn_level_actor_function_item_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_item_data_t * data = malloc(
      sizeof(fn_level_actor_item_data_t));

  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 0;
  switch(p.general->actor_type) {
    case ActorType_BoxRedSoda:
    case ActorType_BoxRedChicken:
      data->tile = OBJ_BOX_RED;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_BoxBlueFootball:
    case ActorType_BoxBlueJoystick:
    case ActorType_BoxBlueDisk:
    case ActorType_BoxBlueBalloon:
    case ActorType_BoxBlueFlag:
    case ActorType_BoxBlueRadio:
      data->tile = OBJ_BOX_BLUE;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_BoxGreyEmpty:
    case ActorType_BoxGreyBoots:
    case ActorType_BoxGreyClamps:
    case ActorType_BoxGreyGun:
    case ActorType_BoxGreyBomb:
    case ActorType_BoxGreyGlove:
    case ActorType_BoxGreyFullLife:
    case ActorType_BoxGreyAccessCard:
    case ActorType_BoxGreyLetterD:
    case ActorType_BoxGreyLetterU:
    case ActorType_BoxGreyLetterK:
    case ActorType_BoxGreyLetterE:
      data->tile = OBJ_BOX_GREY;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_Joystick:
      data->tile = OBJ_JOYSTICK;
      data->current_frame = 0;
      data->num_frames  = 1;
      break;
    case ActorType_Football:
      data->tile = OBJ_FOOTBALL;
      data->current_frame = 0;
      data->num_frames  = 1;
      break;
    case ActorType_Flag:
      data->tile = OBJ_FLAG;
      data->current_frame = 0;
      data->num_frames  = 3;
      break;
    case ActorType_Disk:
      data->tile = OBJ_DISK;
      data->current_frame = 0;
      data->num_frames  = 1;
      break;
    case ActorType_Radio:
      data->tile = OBJ_RADIO;
      data->current_frame = 0;
      data->num_frames = 3;
      break;
    case ActorType_Soda:
      data->tile = ANIM_SODA;
      data->current_frame = 0;
      data->num_frames = 4;
      break;
    case ActorType_Boots:
      data->tile = OBJ_BOOT;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_Gun:
      data->tile = OBJ_GUN;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_FullLife:
      data->tile = OBJ_NUCLEARMOLECULE;
      data->current_frame = 0;
      data->num_frames = 8;
      break;
    case ActorType_ChickenSingle:
      data->tile = OBJ_CHICKEN_SINGLE;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_ChickenDouble:
      data->tile = OBJ_CHICKEN_DOUBLE;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_LetterD:
      data->tile = OBJ_LETTER_D;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_LetterU:
      data->tile = OBJ_LETTER_U;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_LetterK:
      data->tile = OBJ_LETTER_K;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_LetterE:
      data->tile = OBJ_LETTER_E;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_AccessCard:
      data->tile = OBJ_ACCESS_CARD;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_Glove:
      data->tile = OBJ_ROBOHAND;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    case ActorType_Clamps:
      data->tile = OBJ_CLAMP;
      data->current_frame = 0;
      data->num_frames = 1;
      break;
    default:
      /* we got a type which should not be an item. */
      printf(__FILE__ ":%d: warning: item #%d"
          " added which is not an item\n",
          __LINE__, p.general->actor_type);
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Delete an item.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_item_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Hero starts to touch item.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_item_data_t * data = p.specific;

  FnHeroFetchedLetterState * state =
      fn_hero_data_get_fetched_letter_state(p.hero_data);;
  FnHeroScore * score =
      fn_hero_data_get_score(p.hero_data);;
  FnHeroHealth * health =
      fn_hero_data_get_health(p.hero_data);;
  FnHeroFirepower * firepower =
      fn_hero_data_get_firepower(p.hero_data);;
  FnHeroInventory * inventory =
      fn_hero_data_get_inventory(p.hero_data);;

  switch(p.general->actor_type) {
    case ActorType_LetterD:
      p.general->is_alive = 0;
      fn_hero_fetched_letter_state_picked(state, 'D');
      fn_hero_score_add(score, 500);
      fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score500,
              p.general->position.x,
              p.general->position.y);
      break;
    case ActorType_LetterU:
      p.general->is_alive = 0;
      fn_hero_fetched_letter_state_picked(state, 'U');
      fn_hero_score_add(score, 500);
      fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score500,
              p.general->position.x,
              p.general->position.y);
      break;
    case ActorType_LetterK:
      p.general->is_alive = 0;
      fn_hero_fetched_letter_state_picked(state, 'K');
      fn_hero_score_add(score, 500);
      fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score500,
              p.general->position.x,
              p.general->position.y);
      break;
    case ActorType_LetterE:
      p.general->is_alive = 0;
      fn_hero_fetched_letter_state_picked(state, 'E');
      if (fn_hero_fetched_letter_state_succeeded(state)) {
          fn_hero_fetched_letter_state_reset(state);
          fn_hero_score_add(score, 10000);
          fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score10000,
              p.general->position.x,
              p.general->position.y);
      } else {
          fn_hero_score_add(score, 500);
          fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score500,
              p.general->position.x,
              p.general->position.y);
      }
      break;
    case ActorType_FullLife:
      fn_hero_health_fill_max(health);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Gun:
      fn_hero_firepower_increase(firepower, 1);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_AccessCard:
      fn_hero_inventory_set(inventory, InventoryItem_AccessCard);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Glove:
      fn_hero_inventory_set(inventory, InventoryItem_Glove);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Boots:
      fn_hero_inventory_set(inventory, InventoryItem_Boot);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Clamps:
      fn_hero_inventory_set(inventory, InventoryItem_Clamp);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 1000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score1000,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Football:
      fn_hero_score_add(score, 100);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score100,
          p.general->position.x,
          p.general->position.y);
      p.general->is_alive = 0;
      break;
    case ActorType_Disk:
      fn_hero_score_add(score, 5000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score5000,
          p.general->position.x,
          p.general->position.y);
      p.general->is_alive = 0;
      break;
    case ActorType_Joystick:
      fn_hero_score_add(score, 2000);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score2000,
          p.general->position.x,
          p.general->position.y);
      p.general->is_alive = 0;
      break;
    case ActorType_Radio:
    case ActorType_Flag:
      switch(data->current_frame) {
        case 0:
          fn_hero_score_add(score, 100);
          fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score100,
              p.general->position.x,
              p.general->position.y);
          break;
        case 1:
          fn_hero_score_add(score, 2000);
          fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score2000,
              p.general->position.x,
              p.general->position.y);
          break;
        case 2:
          fn_hero_score_add(score, 5000);
          fn_level_actor_queue_push_back(p.actor_queue,
              ActorType_Score5000,
              p.general->position.x,
              p.general->position.y);
          break;
      }
      p.general->is_alive = 0;
      break;
    case ActorType_Soda:
      fn_hero_health_increase(health, 1);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 200);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score200,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_ChickenSingle:
      fn_hero_health_increase(health, 1);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 100);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score100,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_ChickenDouble:
      fn_hero_health_increase(health, 2);
      p.general->is_alive = 0;
      fn_hero_score_add(score, 200);
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Score200,
          p.general->position.x,
          p.general->position.y);
      break;
    default:
      /* do nothing about other items */
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Hero stops to touch item.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  /* Nothing to do here */
}

/* --------------------------------------------------------------- */

/**
 * Action for item.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_item_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;
  if (!fn_level_solids_get(&(p.level_data->solids),
        (p.general->position.x) / FN_TILE_WIDTH,
        (p.general->position.y) / FN_TILE_HEIGHT + 1)) {
    p.general->position.y += FN_HALFTILE_HEIGHT;
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit the item.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_item_data_t * data = p.specific;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

/**
 * An item gets shot.
 *
 * @param  actor  The item actor.
 */
void fn_level_actor_function_item_shot(
        fn_level_actor_shot_params_t p)
{
  switch(p.general->actor_type) {
    case ActorType_BoxBlueFootball:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Football,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxBlueJoystick:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Joystick,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxBlueDisk:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Disk,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxBlueBalloon:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Balloon,
          p.general->position.x,
          p.general->position.y - FN_TILE_HEIGHT);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxBlueFlag:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Flag,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxBlueRadio:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Radio,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxRedSoda:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Soda,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxRedChicken:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_ChickenSingle,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyEmpty:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyBoots:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Boots,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyClamps:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Clamps,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyGun:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Gun,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyBomb:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Bomb,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyGlove:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_Glove,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyFullLife:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_FullLife,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyAccessCard:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_AccessCard,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y,
          4);
      break;
    case ActorType_BoxGreyLetterD:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_LetterD,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxGreyLetterU:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_LetterU,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxGreyLetterK:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_LetterK,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_BoxGreyLetterE:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_LetterE,
          p.general->position.x,
          p.general->position.y);
      fn_level_actor_queue_push_particle_firework(
          p.actor_queue,
          p.general->position.x,
          p.general->position.y, 4);
      break;
    case ActorType_ChickenSingle:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_ChickenDouble,
          p.general->position.x,
          p.general->position.y);
      break;
    case ActorType_Soda:
      p.general->is_alive = 0;
      fn_level_actor_queue_push_back(p.actor_queue, ActorType_SodaFlying,
          p.general->position.x,
          p.general->position.y);
      break;
    default:
      break;
  }
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

void fn_level_actor_function_soda_flying_create(
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_soda_flying_free(
        fn_level_actor_free_params_t p)
{
  /* nothing to do here */
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_soda_flying_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  fn_hero_score_add(score, 1000);
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Score1000,
      p.general->position.x,
      p.general->position.y);
  p.general->is_alive = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_soda_flying_act(
        fn_level_actor_act_params_t p)
{
  p.general->position.y -= FN_HALFTILE_HEIGHT;
  if (fn_level_solids_get(&(p.level_data->solids),
        (p.general->position.x) / FN_TILE_WIDTH,
        (p.general->position.y) / FN_TILE_HEIGHT)) {
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Explosion,
        p.general->position.x,
        p.general->position.y);
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_soda_flying_blit(
        fn_level_actor_blit_params_t p)
{
  const FnTexture * tile = fn_tilecache_get_tile(
          p.tilecache, ANIM_SODAFLY +
          (p.general->position.y/FN_HALFTILE_HEIGHT) % 4);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}


/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The balloon struct.
 */
typedef struct fn_level_actor_balloon_data_t {
  /**
   * A flag indicating if the balloon was destroyed.
   */
  Uint8 destroyed;
  /**
   * The current frame number for the cord animation.
   */
  Uint8 current_frame;
} fn_level_actor_balloon_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_balloon_data_t * data =
    malloc(sizeof(fn_level_actor_balloon_data_t));
  *(p.specific) = data;
  data->destroyed = 0;
  data->current_frame = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT * 2;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_balloon_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_balloon_data_t * data = p.specific;
  FnHeroScore * score = fn_hero_data_get_score(p.hero_data);
  if (!data->destroyed) {
    p.general->is_alive = 0;
    fn_hero_score_add(score, 10000);
    fn_level_actor_queue_push_back(p.actor_queue,
        ActorType_Score10000,
        p.general->position.x,
        p.general->position.y);
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_balloon_data_t * data = p.specific;

  data->current_frame++;
  data->current_frame %= 9;
  if (data->destroyed) {
    p.general->is_alive = 0;
  } else {
    p.general->position.y--;
    if (
        /* balloon bumps against wall */
        fn_level_solids_get(&(p.level_data->solids),
          (p.general->position.x) / FN_TILE_WIDTH,
          (p.general->position.y -1) / FN_TILE_HEIGHT)
       )
    {
      data->destroyed = 1;
      fn_level_actor_queue_push_back(p.actor_queue,
          ActorType_Steam,
          p.general->position.x,
          p.general->position.y);
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_balloon_data_t * data = p.specific;

  const FnTexture * tile;

  FnGeometry destrect = p.general->position;

  if (data->destroyed) {
    tile = fn_tilecache_get_tile(p.tilecache, OBJ_BALLOON + 4);
  } else {
    tile = fn_tilecache_get_tile(p.tilecache, OBJ_BALLOON);
  }
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);

  destrect.y += FN_TILE_HEIGHT;

  tile = fn_tilecache_get_tile(p.tilecache,
      OBJ_BALLOON + 1 + data->current_frame / 3);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_balloon_shot(
        fn_level_actor_shot_params_t p)
{
  fn_level_actor_balloon_data_t * data = p.specific;
  data->destroyed = 1;
  fn_level_actor_queue_push_back(p.actor_queue,
      ActorType_Steam,
      p.general->position.x,
      p.general->position.y);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Create a teleporter.
 *
 * @param  actor  The teleporter actor.
 */
void fn_level_actor_function_teleporter_create(
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = true;
}

/* --------------------------------------------------------------- */

/**
 * Interact with a teleporter.
 *
 * @param  actor  The teleporter actor.
 */
void fn_level_actor_function_teleporter_interact_start(
        fn_level_actor_interact_start_params_t p)
{
  FnLevelActorType othertype;
  if (p.general->actor_type == ActorType_Teleporter1) {
    othertype = ActorType_Teleporter2;
  } else {
    othertype = ActorType_Teleporter1;
  }

  fn_level_actor_message_queue_push_back(
          p.actor_message_queue,
          othertype,
          ActorMessageType_Teleport);
}

/* --------------------------------------------------------------- */

/**
 * The teleporter acts.
 *
 * @param  actor  The teleporter actor.
 */
void fn_level_actor_function_teleporter_act(
        fn_level_actor_act_params_t p)
{
}

/* --------------------------------------------------------------- */

/**
 * Blit a teleporter.
 *
 * @param  actor  The teleporter actor.
 */
void fn_level_actor_function_teleporter_blit(
        fn_level_actor_blit_params_t p)
{
  const FnTexture * tile;

  FnGeometry destrect = p.general->position;

  int i = 0;
  for (i = 0; i < 3; i++) {
    int j = 0;
    for (j = 0; j < 3; j++) {
      destrect.x =
          p.general->position.x - (1 - j) * FN_TILE_WIDTH;
      destrect.y =
          p.general->position.y - (2 - i) * FN_TILE_HEIGHT;
      tile = fn_tilecache_get_tile(p.tilecache,
          ANIM_TELEPORTER1 + i * 3 + j
          );
      fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_teleporter_receive_message(
        fn_level_actor_receive_message_params_t p)
{
    if (p.message != ActorMessageType_Teleport) {
        return;
    }
    FnHeroPosition * hero_position = fn_hero_data_get_position(
            p.hero_data);
    fn_hero_position_move_to(hero_position,
            p.general->position.x,
            p.general->position.y - FN_TILE_HEIGHT);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * Singleanimation data struct.
 */
typedef struct fn_level_actor_singleanimation_data_t {
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
} fn_level_actor_singleanimation_data_t;

/* --------------------------------------------------------------- */

/**
 * Create a singleanimation.
 *
 * @param  actor  The singleanimation actor.
 */
void fn_level_actor_function_singleanimation_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_singleanimation_data_t * data = malloc(
      sizeof(fn_level_actor_singleanimation_data_t));

  *(p.specific) = data;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
  p.general->is_in_foreground = 1;

  switch(p.general->actor_type) {
    case ActorType_Fire:
      data->tile = ANIM_BOMBFIRE;
      data->current_frame = 0;
      data->num_frames = 6;
      break;
    case ActorType_DustCloud:
      data->tile = OBJ_DUST;
      data->current_frame = 0;
      data->num_frames = 5;
      break;
    case ActorType_Steam:
      data->tile = OBJ_STEAM;
      data->current_frame = 0;
      data->num_frames = 5;
      break;
    case ActorType_RobotDisappearing:
      data->tile = ANIM_ROBOT + 3;
      data->current_frame = 0;
      data->num_frames = 7;
      break;
    default:
      printf(__FILE__ ":%d: warning: singleanimation #%d"
          " added which is not a singleanimation\n",
          __LINE__, p.general->actor_type);
      break;
  }
}

/* --------------------------------------------------------------- */

/**
 * Delete a singleanimation.
 *
 * @param  actor  The singleanimation actor.
 */
void fn_level_actor_function_singleanimation_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_singleanimation_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Act a singleanimation.
 *
 * @param  actor  The singleanimation actor.
 */
void fn_level_actor_function_singleanimation_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_singleanimation_data_t * data = p.specific;

  data->current_frame++;
  if (data->current_frame == data->num_frames) {
    p.general->is_alive = 0;
    if (p.general->actor_type == ActorType_RobotDisappearing) {
        fn_level_actor_queue_push_back(
                p.actor_queue,
                ActorType_Explosion,
                p.general->position.x,
                p.general->position.y);
    }
  }
}

/* --------------------------------------------------------------- */

/**
 * Blit an singleanimation.
 *
 * @param  actor  The singleanimation actor.
 */
void fn_level_actor_function_singleanimation_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_singleanimation_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;
  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache,
      data->tile + data->current_frame);
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

/* --------------------------------------------------------------- */
/* --------------------------------------------------------------- */

/**
 * The particle information data.
 */
typedef struct fn_level_actor_particle_data_t {
  /**
   * A countdown for the living time of the particle.
   */
  Uint8 countdown;
  /**
   * The tile to blit.
   */
  Uint16 tile;
  /**
   * The horizontal speed of the particle.
   */
  int hspeed;
  /**
   * The vertical initial speed of the particle.
   */
  int vspeed;
} fn_level_actor_particle_data_t;

/* --------------------------------------------------------------- */

void fn_level_actor_function_particle_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_particle_data_t * data = malloc(
      sizeof(fn_level_actor_particle_data_t));
  *(p.specific) = data;
  data->countdown = 20;
  p.general->is_in_foreground = 1;

  Uint16 hrand = rand();
  Uint16 vrand = rand();

  int const hrand_max = 15;
  int const vrand_max = 15;

  hrand %= hrand_max;
  vrand %= vrand_max;
  data->hspeed = hrand - hrand_max / 2;
  data->vspeed = vrand - vrand_max / 2;

  switch(p.general->actor_type) {
    case ActorType_ParticlePink:
      data->tile = OBJ_SPARK_PINK;
      break;
    case ActorType_ParticleBlue:
      data->tile = OBJ_SPARK_BLUE;
      break;
    case ActorType_ParticleWhite:
      data->tile = OBJ_SPARK_WHITE;
      break;
    case ActorType_ParticleGreen:
      data->tile = OBJ_SPARK_GREEN;
      break;
    default:
      printf(__FILE__ ":%d: warning: particle #%d"
          " added which is not a particle\n",
          __LINE__, p.general->actor_type);
      break;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_particle_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_particle_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_particle_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_particle_data_t * data = p.specific;
  if (data->countdown) {
    data->countdown--;
    p.general->position.x += data->hspeed * 2;
    p.general->position.y += data->vspeed * 2;
    data->vspeed++;
  } else {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_particle_blit(
        fn_level_actor_blit_params_t p)
{
  fn_level_actor_particle_data_t * data = p.specific;

  const FnTexture * tile = fn_tilecache_get_tile(p.tilecache, data->tile);
  FnGeometry destrect = p.general->position;
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
}

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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_rocket_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_rocket_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_shot_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_bomb_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bomb_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_bombfire_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_hero_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_bombfire_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_hero_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  p.general->hurts_hero = false;
  fn_level_actor_bombfire_data_t * data = p.specific;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_act(
        fn_level_actor_act_params_t p)
{
  fn_level_actor_bomb_data_t * data = p.specific;
  data->current_frame++;
  if (data->current_frame == data->num_frames) {
    p.general->is_alive = 0;
  }
}

/* --------------------------------------------------------------- */

void fn_level_actor_bombfire_blit(
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_shot_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_unstablefloor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_unstablefloor_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_expandingfloor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_expandingfloor_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_receive_message_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_conveyor_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_conveyor_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH * 2;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_free(
        fn_level_actor_free_params_t p)
{
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_interact_start(
        fn_level_actor_interact_start_params_t p)
{
    fn_info_message_queue_push(
            p.info_message_queue,
            "Not implemented yet.\n");
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_surveillancescreen_blit(
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_hostileshot_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  fn_level_actor_hostileshot_data_t * data = p.specific;
  p.general->hurts_hero = false;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_hostileshot_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
{
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_HEIGHT;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_free(
        fn_level_actor_free_params_t p)
{
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_interact_start(
        fn_level_actor_interact_start_params_t p)
{
    fn_info_message_queue_push(
            p.info_message_queue,
            "Not implemented yet.\n");
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_notebook_blit(
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_interact_start_params_t p)
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
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_act_params_t p)
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
        fn_level_actor_receive_message_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_interact_start_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_hero_touch_start_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_shot_params_t p)
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_act_params_t p)
{
  fn_level_actor_accesscard_door_data_t * data = p.specific;
  data->current_frame++;
  data->current_frame %= data->num_frames;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_access_card_door_receive_message(
        fn_level_actor_receive_message_params_t p)
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
 * The spikes data structure.
 */
typedef struct fn_level_actor_spike_data_t {
  /**
   * A flag indicating if the hero is currently touching the spike.
   */
  Uint8 touching_hero;
} fn_level_actor_spike_data_t;

/* --------------------------------------------------------------- */

/**
 * Spikes actor creation function.
 *
 * @param  actor  The spikes actor.
 */
void fn_level_actor_function_spikes_create(
        fn_level_actor_create_params_t p)
{
  fn_level_actor_spike_data_t * data = malloc(
      sizeof(fn_level_actor_spike_data_t));
  *(p.specific) = data;
  data->touching_hero = 0;
  p.general->position.w = FN_TILE_WIDTH;
  p.general->position.h = FN_TILE_WIDTH;
  p.general->is_in_foreground = 1;
}

/* --------------------------------------------------------------- */


/**
 * Spikes actor deletion function.
 *
 * @param  actor  The spikes actor.
 */
void fn_level_actor_function_spikes_free(
        fn_level_actor_free_params_t p)
{
  fn_level_actor_spike_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

/**
 * Hero touches spikes actor.
 *
 * @param  actor  The spikes actor.
 */
void fn_level_actor_function_spikes_touch_start(
        fn_level_actor_hero_touch_start_params_t p)
{
  fn_level_actor_spike_data_t * data = p.specific;
  p.general->hurts_hero = true;
  data->touching_hero = 1;
}

/* --------------------------------------------------------------- */

/**
 * Hero stops to touch spikes actor.
 *
 * @param  actor  The spikes actor.
 */
void fn_level_actor_function_spikes_touch_end(
        fn_level_actor_hero_touch_end_params_t p)
{
  p.general->hurts_hero = false;
  fn_level_actor_spike_data_t * data = p.specific;
  data->touching_hero = 0;
}

/* --------------------------------------------------------------- */

/**
 * Blit the spikes.
 *
 * @param  actor  The spikes actor.
 */
void fn_level_actor_function_spikes_blit(
        fn_level_actor_blit_params_t p)
{
  const FnTexture * tile = NULL;
  fn_level_actor_spike_data_t * data = p.specific;

  FnGeometry destrect = p.general->position;
  switch(p.general->actor_type) {
    case ActorType_SpikesUp:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_SPIKES_UP);
      break;
    case ActorType_SpikesDown:
      tile = fn_tilecache_get_tile(p.tilecache, OBJ_SPIKES_DOWN);
      break;
    case ActorType_Spike:
      if (data->touching_hero) {
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_SPIKE + 1);
      } else {
        tile = fn_tilecache_get_tile(p.tilecache, OBJ_SPIKE);
      }

      break;
    default:
      printf(__FILE__ ":%d: warning: spike #%d"
          " tried to blit which is not a spike\n",
          __LINE__, p.general->actor_type);
      return;
      break;
  }
  fn_texture_blit_to_sdl_surface(tile, NULL, p.target, &destrect);
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
        fn_level_actor_create_params_t p)
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
        fn_level_actor_free_params_t p)
{
  fn_level_actor_fan_data_t * data = *(p.specific);
  free(data); *(p.specific) = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_actor_function_fan_act(
        fn_level_actor_act_params_t p)
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
        fn_level_actor_blit_params_t p)
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
        fn_level_actor_shot_params_t p)
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
    .hero_touch_start = fn_level_actor_function_firewheelbot_touch_start,
    .hero_touch_end = fn_level_actor_function_firewheelbot_touch_end,
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
    .shot = NULL,
    .receive_message = NULL,
  },
  [ActorType_Robot] = {
    .create = fn_level_actor_function_robot_create,
    .free = fn_level_actor_function_robot_free,
    .hero_touch_start = fn_level_actor_function_robot_touch_start,
    .hero_touch_end = fn_level_actor_function_robot_touch_end,
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
    .hero_touch_start = fn_level_actor_function_wallcrawler_touch_start,
    .hero_touch_end = fn_level_actor_function_wallcrawler_touch_end,
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
    .hero_touch_start = fn_level_actor_function_wallcrawler_touch_start,
    .hero_touch_end = fn_level_actor_function_wallcrawler_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_interact_start = fn_level_actor_function_teleporter_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_teleporter_act,
    .blit = fn_level_actor_function_teleporter_blit,
    .shot = NULL,
    .receive_message = fn_level_actor_function_teleporter_receive_message,
  },
  [ActorType_Teleporter2] = {
    .create = fn_level_actor_function_teleporter_create,
    .free = NULL,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_teleporter_interact_start,
    .hero_interact_end = NULL,
    .act = fn_level_actor_function_teleporter_act,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_balloon_touch_start,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_touch_start = fn_level_actor_function_item_touch_start,
    .hero_touch_end = fn_level_actor_function_item_touch_end,
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
    .hero_interact_start = fn_level_actor_function_accesscard_slot_interact_start,
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
    .hero_interact_start = fn_level_actor_function_glove_slot_interact_start,
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
    .create = fn_level_actor_function_lift_create,
    .free = fn_level_actor_function_lift_free,
    .hero_touch_start = NULL,
    .hero_touch_end = NULL,
    .hero_interact_start = fn_level_actor_function_lift_interact_start,
    .hero_interact_end = fn_level_actor_function_lift_interact_end,
    .act = fn_level_actor_function_lift_act,
    .blit = fn_level_actor_function_lift_blit,
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
    .hero_touch_start = fn_level_actor_function_spikes_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_touch_end,
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
    .hero_touch_start = fn_level_actor_function_spikes_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_touch_end,
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
    .hero_touch_start = fn_level_actor_function_spikes_touch_start,
    .hero_touch_end = fn_level_actor_function_spikes_touch_end,
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
      struct fn_level_actor_create_params_t p ={
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
      struct fn_level_actor_free_params_t p ={
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

    struct fn_level_actor_hero_touch_start_params_t p = {
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
    struct fn_level_actor_hero_touch_end_params_t p = {
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
    /* This check needs to be done for lift only because
     * if there are two lifts next to each other, the mostleft
     * lift would be chosen for interaction instead of the one on
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

    struct fn_level_actor_interact_start_params_t p = {
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

    struct fn_level_actor_interact_end_params_t p = {
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
    fn_level_actor_act_params_t p = {
        .general = actor->general,
        .specific = actor->specific,
        .level = level,
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
    fn_level_actor_blit_params_t p = {
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
    fn_level_actor_shot_params_t p = {
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

    struct fn_level_actor_receive_message_params_t p = {
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
