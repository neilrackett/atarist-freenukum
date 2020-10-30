/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Gameplay functions
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

#include <unistd.h>
#include <string.h>
#include <fcntl.h>
#include <stdlib.h>
#include <time.h>

/* --------------------------------------------------------------- */

#include "fn_game.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

Uint32 fn_game_timer_triggered(
    Uint32 interval,
    void * param)
{
  SDL_Event event;
  event.type = SDL_USEREVENT;
  event.user.code = UserEvent_Timer;
  event.user.data1 = 0;
  event.user.data2 = 0;
  SDL_PushEvent(&event);
  return interval;
}

/* --------------------------------------------------------------- */

void fn_game_start(
    const FnTileCache * tilecache,
    FnHeroData * hero_data,
    FnTextureCreationParams texture_creation_params,
    SDL_Surface * target,
    FnSettings * settings,
    FnEpisodes * episodes)
{
  /* Initialize Random Number Generator. */
  srand(time(NULL));

  char * msg1 =
    "So you're the pitiful\n"
    "hero they sent to stop\n"
    "me.  I, Dr. Proton, will\n"
    "soon rule the world!\n";
  char * msg2 =
    "You're wrong, Proton\n"
    "breath.  I'll be done\n"
    "with you and still have\n"
    "time to watch Oprah!\n";


  FnFile * file = NULL;

  char filename[30];
  snprintf(
      filename, 30, "badguy.dn%ld", fn_episodes_current(episodes) + 1);
  file = fn_data_open_file(filename);
  fn_picture_splash_show_with_message(
      tilecache,
      texture_creation_params,
      target,
      file,
      msg1,
      0,
      144);
  fn_file_free(file);

  snprintf(
      filename, 30, "duke.dn%ld", fn_episodes_current(episodes) + 1);
  file = fn_data_open_file(filename);
  fn_picture_splash_show_with_message(
      tilecache,
      texture_creation_params,
      target,
      file,
      msg2,
      79,
      144);
  fn_file_free(file);

  fn_hero_data_reset(hero_data);
  FnHeroInventory * inventory = fn_hero_data_get_inventory(hero_data);
  FnHeroFirepower * firepower = fn_hero_data_get_firepower(hero_data);
  FnHeroScore * score = fn_hero_data_get_score(hero_data);
  FnHeroHealth * health = fn_hero_data_get_health(hero_data);

  SDL_FillRect(target, NULL, 0);

  FnTextureCreationParams surface_creation_params =
      fn_sdl_surface_creation_params(target);

  fn_borders_blit(
          target, surface_creation_params, tilecache);

  fn_borders_blit_life(
          target,
          surface_creation_params,
          tilecache,
          fn_hero_health_get(health));

  fn_borders_blit_score(
          target,
          surface_creation_params,
          tilecache,
          fn_hero_score_get(score));

  fn_borders_blit_firepower(
          target,
          surface_creation_params,
          tilecache,
          firepower);

  fn_borders_blit_inventory(
          target,
          surface_creation_params,
          tilecache,
          inventory);

  SDL_UpdateRect(target, 0, 0, 0, 0);

  { /* start the game itself */

    int level = 1;
    int interlevel = 0;
    int success = 1;

    fn_infobox_show(
        target,
        tilecache,
        surface_creation_params,
        "Get ready FreeNukum,\nyou are going in.\n");

    while (success && level < 13) {
      if (interlevel) {
        /* interlevel */
        success = fn_game_start_in_level(
                2,
                tilecache,
                hero_data,
                texture_creation_params,
                target,
                settings,
                episodes);
        level++;
        if (level == 2) {
          level++;
        }
        interlevel = 0;
      } else {
        /* real level */
        success = fn_game_start_in_level(
                level,
                tilecache,
                hero_data,
                texture_creation_params,
                target,
                settings,
                episodes);
        interlevel = 1;
      }
    }

    if (success) {
      /* the player finished, so we show the end sequence */
      /* TODO */
    }
  }

}
