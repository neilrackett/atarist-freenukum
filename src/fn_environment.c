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

fn_environment_t * fn_environment_create()
{
  /* create the environment */
  fn_environment_t * env = malloc(sizeof(fn_environment_t));

  /* fill with default values */
  env->videoflags = FN_SURFACE_FLAGS;
  env->transparent = 0;
  env->screen = NULL;
  env->episode = 1;
  env->num_episodes = 0;
  env->hero = fn_hero_data_create();

  env->settings = fn_settings_load_or_create();

  if (env->settings.fullscreen) {
    env->videoflags |= SDL_FULLSCREEN;
  }

  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER) == -1) {
    fn_error_printf(1024, "Can't initialize SDL: %s", SDL_GetError());
    return env;
  }

  env->screen = SDL_SetVideoMode(
      FN_WINDOW_WIDTH,
      FN_WINDOW_HEIGHT,
      FN_COLOR_DEPTH,
      env->videoflags);
  if (env->screen == NULL) {
    fn_error_printf(1024, "Can't set video mode: %s", SDL_GetError());
    return env;
  }

  env->transparent = SDL_MapRGB(env->screen->format, 100, 1, 1);

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
  if (env->hero != NULL) {
    fn_hero_data_free(env->hero); env->hero = NULL;
  }

  free(env);
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_check_for_episodes(fn_environment_t * env)
{
  env->num_episodes = fn_data_count_installed_episodes();

  if (env->num_episodes == 0) {
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

  return env->num_episodes;
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_get_fullscreen(fn_environment_t * env)
{
  return env->settings.fullscreen;
}

/* --------------------------------------------------------------- */

void fn_environment_toggle_fullscreen(fn_environment_t * env)
{
  int res = SDL_WM_ToggleFullScreen(env->screen);
  if (res) {
    env->settings.fullscreen = (env->settings.fullscreen + 1) % 2;
  }
}

/* --------------------------------------------------------------- */

Uint32 fn_environment_get_transparent(fn_environment_t * env)
{
  return env->transparent;
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_environment_get_screen_sdl(fn_environment_t * env)
{
  return env->screen;
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_get_episode(fn_environment_t * env)
{
  return env->episode;
}

/* --------------------------------------------------------------- */

void fn_environment_set_episode(fn_environment_t * env,
    Uint8 episode)
{
  env->episode = episode;
}

/* --------------------------------------------------------------- */

const Uint8 fn_environment_get_health(fn_environment_t * env)
{
  FnHeroHealth * health = fn_hero_data_get_health(env->hero);
  return fn_hero_health_get(health);
}

/* --------------------------------------------------------------- */

Uint8 fn_environment_get_draw_collision_bounds(
    fn_environment_t * env)
{
  return env->settings.draw_collision_bounds;
}

/* --------------------------------------------------------------- */

void fn_environment_store_settings(fn_environment_t * env)
{
  fn_settings_save(env->settings);
}

/* --------------------------------------------------------------- */

FnHeroData * fn_environment_get_hero(fn_environment_t * env)
{
  return env->hero;
}

/* --------------------------------------------------------------- */

FnTextureCreationParams fn_environment_build_texture_creation_params(
        fn_environment_t * env)
{
  FnTextureCreationParams params;
  params.flags = env->screen->flags;
  params.bits_per_pixel = env->screen->format->BitsPerPixel;
  params.transparent = env->transparent;
  return params;
}

/* --------------------------------------------------------------- */
