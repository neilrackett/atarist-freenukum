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

#include "config.h"

/* --------------------------------------------------------------- */

#include <errno.h>
#include <dirent.h>
#include <sys/stat.h>
#include <SDL/SDL_ttf.h>

/* --------------------------------------------------------------- */

#include "fn_environment.h"
#include "fn_error.h"

/* --------------------------------------------------------------- */

char * font_base_directories[] = {
  "/usr/share/fonts",
  "/usr/share/fonts/truetype",
  "/usr/local/share/fonts",
  "/usr/local/share/fonts/truetype",
  "/usr/share/fonts/TTF",
  0
};

char * fonts[] = {
  "/DejaVuSans.ttf",
  "/DejavuSerif.ttf",
  "/linux-libertine/LinLibertine.ttf",
  "/linux-libertine/LinLibertine_Re.ttf",
  "/freefont/FreeMono.ttf",
  "/freefont/FreeSans.ttf",
  "/freefont/FreeSerif.ttf",
  "/dustin/Balker.ttf",
  "/dustin/Dustismo.ttf",
  "/kochi/kochi-gothic.ttf",
  "/kochi/kochi-mincho.ttf",
  "/msttcorefonts/arial.ttf",
  "/msttcorefonts/Arial.ttf",
  "/msttcorefonts/Arial_Black.ttf",
  "/msttcorefonts/times.ttf",
  "/msttcorefonts/verdana.ttf",
  "/ttf-bitstream-vera/Vera.ttf",
  "/ttf-dejavu/DejaVuSans.ttf",
  "/ttf-dejavu/DejaVuSansMono.ttf",
  "/unfonts/UnBatang.ttf",
  "/unfonts/UnDotum.ttf",
  0
};

/* --------------------------------------------------------------- */

TTF_Font * fn_environment_loadfont(const int fontsize)
{
  TTF_Font * font = NULL;
  char path[256] = "";

  int i = 0;
  int j = 0;
  while (fonts[i] != 0) {
    while (font_base_directories[j] != 0) {
      snprintf(path, 256, "%s%s",
          font_base_directories[j], fonts[i]);
      font = TTF_OpenFont(path, fontsize);
      if (font) {
        return font;
      }
      j++;
    }
    i++;
    j=0;
  }

  printf("Could not find any font.");
  return font;
}

/* --------------------------------------------------------------- */

fn_environment_t * fn_environment_create(bool fullscreen)
{
  /* create the environment */
  fn_environment_t * env = malloc(sizeof(fn_environment_t));

  /* fill with default values */
  env->screen = NULL;
  env->episodes = NULL;

  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER) == -1) {
    fn_error_printf(1024, "Can't initialize SDL: %s", SDL_GetError());
    return env;
  }

  env->screen = fn_sdl_create_screen(
          FN_WINDOW_WIDTH, FN_WINDOW_HEIGHT, fullscreen);
  if (env->screen == NULL) {
    fn_error_printf(1024, "Can't set video mode: %s", SDL_GetError());
    return env;
  }

  SDL_WM_SetCaption("Freenukum " VERSION, "Freenukum " VERSION);

  env->initialized = 1;
  return env;
}

/* --------------------------------------------------------------- */

void fn_environment_delete(fn_environment_t * env)
{
  if (env->episodes != NULL) {
    fn_episodes_free(env->episodes); env->episodes = NULL;
  }
  if (env->screen != NULL) {
    SDL_FreeSurface(env->screen); env->screen = NULL;
  }

  free(env);
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_check_for_episodes(fn_environment_t * env)
{
  env->episodes = fn_episodes_find_installed();

  if (fn_episodes_count(env->episodes) == 0) {
      /* we found no episodes */
      char * message =
          "Could not load data level and graphics files.\n"
          "Please use the accompanied freenukum-data-tool\n"
          "for installing the game data files.\n";
      printf(message);
      TTF_Font * font = NULL;
      int fontsize = 10;
      if (TTF_Init() != -1) {
        font = fn_environment_loadfont(fontsize);
      }
      if (font) {
        fn_data_display_text(env->screen, 0, 0, font, message);
        SDL_UpdateRect(
            env->screen, 0, 0, env->screen->w, env->screen->h);

        SDL_Event event;

        char input = 0;
        int res = 0;
        while (input == 0) {
          res = SDL_WaitEvent(&event);
          if (res == 1) {
            switch(event.type) {
              case SDL_QUIT:
                input = 'q';
                break;
              case SDL_KEYDOWN:
                switch(event.key.keysym.sym) {
                  case SDLK_RETURN:
                    input = 'd';
                    break;
                  case SDLK_ESCAPE:
                    input = 'q';
                    break;
                  default:
                    /* do nothing */
                    break;
                }
              default:
                /* do nothing on other events */
                break;
            }
          }
        }
        TTF_CloseFont(font);
        font = NULL;
      }
  }

  return fn_episodes_count(env->episodes);
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_environment_get_screen_sdl(fn_environment_t * env)
{
  return env->screen;
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_get_episode(fn_environment_t * env)
{
  if (env->episodes != NULL) {
      return fn_episodes_current(env->episodes) + 1;
  } else {
      return 0;
  }
}
