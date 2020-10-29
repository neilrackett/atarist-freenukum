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
#include "fn_game.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
  int res = 0; /* results are stored here */

  int retval = 1; /* The return value from the program. */

  int choice = 0; /* choice of the main menu */

  char backgroundfile[10] = "dn.dn1";
/* --------------------------------------------------------------- */

  fn_error_set_handler(fn_error_print_commandline);

  FnSettings settings = fn_settings_load_or_create();
  
  SDL_Surface * screen = fn_game_initialize_and_get_window(
          FN_WINDOW_WIDTH,
          FN_WINDOW_HEIGHT,
          settings.fullscreen,
          "Freenukum " VERSION,
          "Freenukum " VERSION
          );
  if (!screen) {
      return 1;
  }

/* --------------------------------------------------------------- */

  /* check if all data is present */
  FnEpisodes * episodes = fn_game_check_episodes(screen);
  if (fn_episodes_count(episodes) == 0) {
      fn_episodes_free(episodes);
      exit(retval);
  }

  FnHeroData * hero_data = fn_hero_data_create();

  FnTextureCreationParams texture_creation_params =
      fn_sdl_surface_creation_params(screen);
  const FnTileCache * tilecache =
      fn_tilecache_load(texture_creation_params);


  /* show the splash screen */
  FnFile * file = fn_data_open_file(backgroundfile);
  res = fn_picture_splash_show(
      tilecache,
      texture_creation_params,
      screen,
      file);
  fn_file_free(file);
  if (!res) {
    fn_error_printf(1024, "Could not show splash screen.\n");
    exit(retval);
  }

  /* show the main menu */
  while (choice != MainMenuEntry_Quit) {
    choice = fn_mainmenu(screen, tilecache, texture_creation_params);
    switch(choice) {
      case MainMenuEntry_Start:
        {
          fn_game_start(
              tilecache,
              hero_data,
              texture_creation_params,
              screen,
              &settings,
              fn_episodes_current(episodes) + 1);
          FnFile * file = fn_data_open_file(backgroundfile);
          res = fn_picture_splash_show(
              tilecache,
              texture_creation_params,
              screen,
              file);
          fn_file_free(file);
        }
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
        int res = SDL_WM_ToggleFullScreen(screen);
        if (res) {
            fn_settings_toggle_fullscreen(&settings);
            fn_settings_save(settings);
        }
        }
        break;
      case MainMenuEntry_EpisodeChange:
        {
          Uint8 old = fn_episodes_current(episodes);
          Uint8 new = fn_episodes_switch(episodes);
          Uint8 episode = new + 1;

          snprintf(backgroundfile,
              7, "dn.dn%d", episode);
          if (old == new) {
            fn_infobox_show(screen, tilecache, texture_creation_params,
                "You don't have another\n"
                "episode installed.\n"
                "\n"
                "We stay in this episode\n");
            snprintf(backgroundfile,
                7, "dn.dn%d", episode);
            FnFile * file = fn_data_open_file(backgroundfile);
            res = fn_picture_splash_show(
                    tilecache, texture_creation_params, screen, file);
            fn_file_free(file);
          } else {
            FnFile * file = fn_data_open_file(backgroundfile);
            res = fn_picture_splash_show(
                    tilecache, texture_creation_params, screen, file);
            fn_file_free(file);
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
        {
            FnFile * file = fn_data_open_file(backgroundfile);
            res = fn_picture_splash_show(tilecache, texture_creation_params, screen,
                    file);
            fn_file_free(file);
        }
        break;
      case MainMenuEntry_Credits:
        fn_infobox_show(screen, tilecache, texture_creation_params,
            "Credits not implemented yet.\n");
        break;
      default:
        break;
    }
  }

  retval = 0;

  fn_hero_data_free(hero_data); hero_data = NULL;
  fn_episodes_free(episodes);

  return retval;
}
