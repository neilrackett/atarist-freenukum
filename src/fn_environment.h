/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Setting up the freenukum environment
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

#ifndef FN_ENVIRONMENT_H
#define FN_ENVIRONMENT_H

/* --------------------------------------------------------------- */

#include <unistd.h>
#include <SDL/SDL.h>
#include <SDL/SDL_ttf.h>

/* --------------------------------------------------------------- */

typedef struct fn_environment_t fn_environment_t;

/* --------------------------------------------------------------- */

#include "fn.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

struct fn_environment_t {
  /**
   * Was the initialization successful?
   */
  Uint8 initialized;

  /**
   * The game settings.
   */
  FnSettings settings;

  /**
   * SDL screen
   */
  SDL_Surface * screen;

  /**
   * The episode number.
   */
  Uint8 episode;

  /**
   * The number of episodes available.
   */
  Uint8 num_episodes;
};

/* --------------------------------------------------------------- */

/**
 * Create an environment using the default settings.
 *
 * @return  The completely setup environment. If something goes
 *          wrong, NULL is returned.
 */
fn_environment_t * fn_environment_create();

/* --------------------------------------------------------------- */

/**
 * Delete an allocated environment.
 *
 * @param  env  The environment.
 */
void fn_environment_delete(fn_environment_t * env);

/* --------------------------------------------------------------- */

/**
 * Check if there are any episodes available for this environment.
 *
 * @param  env  The environment.
 *
 * @return The number of available episodes.
 */
Uint8 fn_environment_check_for_episodes(fn_environment_t * env);

/* --------------------------------------------------------------- */

/**
 * Get the information if the environment is in fullscreen mode.
 *
 * @param  env  The environment.
 *
 * @return  1 if the game is in fullscreen mode, 0 if windowed.
 */
Uint8 fn_environment_get_fullscreen(fn_environment_t * env);

/* --------------------------------------------------------------- */

/**
 * Get the screen to which the environment displays the game.
 *
 * @param  env    The environment.
 *
 * @return  The screen.
 */
SDL_Surface * fn_environment_get_screen_sdl(fn_environment_t * env);

/* --------------------------------------------------------------- */

/**
 * Get the number of the current episode.
 *
 * @param  env  The environment.
 *
 * @return  The number of the episode.
 */
Uint8 fn_environment_get_episode(fn_environment_t * env);

/* --------------------------------------------------------------- */

/**
 * Switch the environment to an episode.
 *
 * @param  env      The environment.
 * @param  episode  The episode number.
 */
void fn_environment_set_episode(fn_environment_t * env,
    Uint8 episode);

/* --------------------------------------------------------------- */

/**
 * Get the information if collision bounds should be draws.
 */
Uint8 fn_environment_get_draw_collision_bounds(
    fn_environment_t * env);

/* --------------------------------------------------------------- */

void fn_environment_store_settings(fn_environment_t * env);

/* --------------------------------------------------------------- */

#endif /* FN_ENVIRONMENT_H */
