/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Level functions
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

#ifndef FN_LEVEL_H
#define FN_LEVEL_H

/* --------------------------------------------------------------- */

typedef struct fn_level_t fn_level_t;

/* --------------------------------------------------------------- */

/*
 * includes below use fn_level_t this is why they are not
 * at the beginning of this file because fn_level_t is
 * not yet defined there.
 */
#include "rusted.h"

/* --------------------------------------------------------------- */

/**
 * A struct representing a level.
 */
struct fn_level_t {
  /**
   * Level data
   */
  FnLevelData * data;

  /**
   * The surface which contains the always fixed tiles.
   */
  SDL_Surface * surface_fixed;

  /**
   * The surface for blitting the level.
   */
  SDL_Surface * surface;

  /**
   * The shots inside the level.
   */
  FnShotList * shots;

  /**
   * The number of animated frames since last action.
   */
  size_t animated_frames;
};

/* --------------------------------------------------------------- */

/**
 * Load a level from a file.
 *
 * @param  file  An already opened file descriptor to the level file.
 * @param  env   The environment of the game.
 *
 * @return  The fully loaded level. If it was not possible to load
 *          the level, NULL is returned. Examine errno in order
 *          to find out what error occured.
 */
fn_level_t * fn_level_load(
        FnFile * file,
        FnHeroData * hero,
        const FnTileCache * tilecache,
        FnTextureCreationParams texture_creation_params,
        FnLevelRaw * out_param_raw_to_fill);

/* --------------------------------------------------------------- */

/**
 * Destroy a level.
 *
 * @param  level  The level to destroy.
 */
void fn_level_free(fn_level_t * lv);

/* --------------------------------------------------------------- */

/**
 * Blit the current state of the level to an SDL Surface.
 *
 * @param  lv         The level to blit.
 * @param  target     The target SDL Surface.
 * @param  targetrect The target area to which to blit.
 * @param  sourcerect The source rectangle.
 * @param  backdrop1  The first backdrop type.
 * @param  backdrop2  The second backdrop type.
 */
void fn_level_blit_to_surface(
        fn_level_t * lv,
        const FnTileCache * tilecache,
        FnHeroData * hero,
        bool draw_collision_bounds,
        SDL_Surface * target,
        FnGeometry * targetrect,
        FnGeometry * sourcerect,
        FnTexture * backdrop1,
        FnTexture * backdrop2);

/* --------------------------------------------------------------- */

/**
 * Indicate if we are still playing this level.
 * 
 * @param  lv    The level we want to play.
 * 
 * @return  non-zero if we are still playing, otherwise zero.
 */
int fn_level_keep_on_playing(fn_level_t * lv);

/* --------------------------------------------------------------- */

/**
 * Call this function make the game act one step further.
 *
 * @param  lv  The level which should step.
 *
 * @return zero if the level is finished, otherwise non-zero.
 */
int fn_level_act(
        fn_level_t * lv,
        FnHeroData * hero,
        FnLevelActorQueue * actor_queue,
        FnLevelActorMessageQueue * actor_message_queue);

/* --------------------------------------------------------------- */

/**
 * Call this function to make the hero interact with items around.
 *
 * @param  lv  The level whose hero should interact.
 */
void fn_level_hero_interact_start(
        fn_level_t * lv,
        FnHeroData * hero,
        FnInfoMessageQueue * info_message_queue,
        FnLevelActorMessageQueue * actor_message_queue);

/* --------------------------------------------------------------- */

/**
 * Call this function to make the hero stop interacting with an item.
 *
 * @param  lv  The level whose hero should interact.
 */
void fn_level_hero_interact_stop(fn_level_t * lv, FnHeroData * hero);

/* --------------------------------------------------------------- */

/**
 * Let the hero fire a shot.
 *
 * If there are already the maximum of shots in the level,
 * no shot is created and this function returns.
 * 
 * @param  lv  The level.
 */
void fn_level_fire_shot(
        fn_level_t * lv,
        FnHeroData * hero,
        FnLevelActorQueue * actor_queue);

/* --------------------------------------------------------------- */

/**
 * Check if a rectangle stands completely on solid ground in a level.
 * Completely means that the whole width of the rectangle has solid
 * ground directly below.
 *
 * @param  lv     The level.
 * @param  rect   The rectangle.
 * 
 * @return 1 if the rectangle stands on solid ground, otherwise 0.
 */
Uint8 fn_level_stands_on_solid_ground_completely(
        FnLevelSolids * solids, FnGeometry rect);

/* --------------------------------------------------------------- */

/**
 * Check if a rectangle stands partially on solid ground in a level.
 * Partially means that not the whole width of the rectangle must
 * stand on ground, but only part of it.
 *
 * @param  lv     The level.
 * @param  rect   The rectangle.
 * 
 * @return 1 if the rectangle stands on solid ground, otherwise 0.
 */
Uint8 fn_level_stands_on_solid_ground_partially(FnLevelSolids * solids,
    FnGeometry rect);

/* --------------------------------------------------------------- */

/**
 * Push a rectangle which is standing on solid ground.
 * If the rectangle would no longer stand on solid ground,
 * this is shown in the return value.
 *
 * @param  level    The level inside which the rectangle is.
 * @param  rect     The rectangle.
 * @param  offset   The horizontal offset by which to push.
 * @param  gravity  The vertical speed by which the rect falls down.
 *
 * @return 1 if the push was successful, otherwise 0.
 */
Uint8 fn_level_push_rect_standing_on_solid_ground(
    FnLevelSolids * solids, FnGeometry rect, Sint8 offset, Uint8 gravity);

/* --------------------------------------------------------------- */

/**
 * Let a rectangle fall down within a level.
 * This is possible, if it overlaps with no solid parts and if
 * it does not stand on any ground at all.
 *
 * @param  level     The level inside which the rectangle is.
 * @param  rect      The rectangle.
 * @param  dist      The distance for which to check.
 *
 * @return The number of pixels by which the rect can fall down
 *         (maximally the same value as dist).
 */
Uint8 fn_level_rect_fall_down(
    FnLevelSolids * solids, FnGeometry rect, Uint8 dist);

/* --------------------------------------------------------------- */

#endif /* FN_LEVEL_H */
