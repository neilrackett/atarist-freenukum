/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Main game starting file
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

#include "config.h"

/* --------------------------------------------------------------- */

#include <SDL/SDL_ttf.h>
#include <SDL/SDL.h>
#include <stdlib.h>
#include <sys/types.h>
#include <dirent.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_error_cmdline.h"
#include "fn_error.h"
#include "fn_picture_splash.h"
#include "fn_game.h"
#include "fn_environment.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
  int res = 0; /* results are stored here */

  int retval = 1; /* The return value from the program. */

  int choice = 0; /* choice of the main menu */

  /* TODO move this into fn_environment. */
  char backgroundfile[10] = "dn.dn1";
/* --------------------------------------------------------------- */

  fn_error_set_handler(fn_error_print_commandline);

  fn_environment_t * env = fn_environment_create();

/* --------------------------------------------------------------- */

  /* check if all data is present */
  int episodes = fn_environment_check_for_episodes(env);
  if (episodes == 0) {
    exit(retval);
  }

  FnHeroData * hero_data = fn_environment_get_hero(env);

  SDL_Surface * screen = env->screen;

  FnTextureCreationParams texture_creation_params =
      fn_sdl_surface_creation_params(screen);
  const FnTileCache * tilecache =
      fn_tilecache_load(texture_creation_params);


  /* show the splash screen */
  res = fn_picture_splash_show(
      tilecache,
      texture_creation_params,
      env,
      backgroundfile);
  if (!res) {
    fn_error_printf(1024, "Could not show splash screen.\n");
    exit(retval);
  }

  /* show the main menu */
  while (choice != MainMenuEntry_Quit) {
    choice = fn_mainmenu(screen, tilecache, texture_creation_params);
    switch(choice) {
      case MainMenuEntry_Start:
        fn_game_start(
            tilecache,
            hero_data,
            texture_creation_params,
            env);
        res = fn_picture_splash_show(
            tilecache,
            texture_creation_params,
            env,
            backgroundfile);
        break;
      case MainMenuEntry_Restore:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Restore not implemented yet.\n");
        break;
      case MainMenuEntry_Instructions:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Instructions not implemented yet.\n");
        break;
      case MainMenuEntry_OrderingInfo:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Orderinginfo not implemented yet.\n");
        break;
      case MainMenuEntry_FullScreenToggle:
        {
          fn_environment_toggle_fullscreen(env);
        }
        break;
      case MainMenuEntry_EpisodeChange:
        {
          Uint8 episode = fn_environment_get_episode(env);
          episode++;
          if (episode > 3) {
            episode = 1;
          }
          snprintf(backgroundfile,
              7, "dn.dn%d", episode);
          res = fn_picture_splash_show(
                  tilecache, texture_creation_params, env, backgroundfile);
          if (res == 0) {
            fn_infobox_show(screen, tilecache, texture_creation_params,
                "You don't have this episode installed.\n"
                "We stay in episode 1\n");
            episode = 1;
            snprintf(backgroundfile,
                7, "dn.dn%d", episode);
            res = fn_picture_splash_show(
                    tilecache, texture_creation_params, env, backgroundfile);
          } else {
            fn_environment_set_episode(env, episode);
          }
        }
        break;
      case MainMenuEntry_HighScores:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Highscores not implemented yet.\n");
        break;
      case MainMenuEntry_Previews:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Previews not implemented yet.\n");
        break;
      case MainMenuEntry_ViewUserDemo:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Userdemo not implemented yet.\n");
        break;
      case MainMenuEntry_TitleScreen:
        res = fn_picture_splash_show(tilecache, texture_creation_params, env,
            backgroundfile);
        break;
      case MainMenuEntry_Credits:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Credits not implemented yet.\n");
        break;
      default:
        break;
    }
  }

  fn_environment_store_settings(env);

  retval = 0;

  fn_environment_delete(env);

  return retval;
}
