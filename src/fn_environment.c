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
  if (env->screen != NULL) {
    SDL_FreeSurface(env->screen); env->screen = NULL;
  }

  free(env);
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_environment_get_screen_sdl(fn_environment_t * env)
{
  return env->screen;
}
