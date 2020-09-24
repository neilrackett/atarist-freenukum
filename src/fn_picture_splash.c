/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Splash a picture to the screen
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
#include <stdio.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>
#include <errno.h>
#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

#include "fn_picture_splash.h"
#include "fn_error.h"
#include "fn_error_cmdline.h"
#include "fn.h"
#include "fn_msgbox.h"

/* --------------------------------------------------------------- */

int fn_picture_splash_show(
    fn_environment_t * env,
    char * filename)
{
  return fn_picture_splash_show_with_message(
      env,
      filename,
      NULL,
      0,0);
}

/* --------------------------------------------------------------- */

int fn_picture_splash_show_with_message(
    fn_environment_t * env,
    char * filename,
    char * msg,
    Uint8 x,
    Uint8 y)
{
  char * path;
  FnFile * file;
  int res;
  SDL_Event event;
  FnTexture * picture;

  char * datapath = fn_environment_get_datapath(env);
  path = malloc(strlen(datapath) + strlen(filename) + 1);
  sprintf(path, "%s/%s", datapath, filename);
  file = fn_file_open(path);

  if (file == NULL) {
    fn_error_printf(1024, "Could not open file %s for reading: %s",
        path,strerror(errno));
    free(path);
    return 0;
  }
  free(path);

  FnTextureCreationParams params = fn_environment_build_texture_creation_params(env);
  picture = fn_picture_load(file, params);

  SDL_Surface * screen = fn_environment_get_screen_sdl(env);
  fn_texture_blit_to_sdl_surface(picture, NULL, screen, NULL);


  if (fn_environment_tilecache_loaded(env) && msg != NULL) {
    FnTexture * msgbox;
    FnGeometry dstrect;

    msgbox = fn_msgbox(env, msg);

    dstrect.x = x;
    dstrect.y = y;

    fn_texture_blit_to_sdl_surface(msgbox, NULL, screen, &dstrect);
    fn_texture_free(msgbox);
  }

  SDL_UpdateRect(screen, 0, 0, 0, 0);
  fn_texture_free(picture);

  while (1) {
    res = SDL_WaitEvent(&event);
    if (res == 1) {
      switch(event.type) {
        case SDL_QUIT:
          return 1;
          break;
        case SDL_KEYDOWN:
          switch(event.key.keysym.sym) {
            case SDLK_ESCAPE:
            case SDLK_RETURN:
              return 1;
            default:
              /* ignore other keys */
              break;
          }
        case SDL_MOUSEBUTTONDOWN:
          if (event.button.button == SDL_BUTTON_LEFT) {
            return 1;
          }
          break;
        case SDL_VIDEOEXPOSE:
          SDL_UpdateRect(screen, 0, 0, 0, 0);
          break;
        default:
          /* ignore unknown events */
          break;
      }
    }
  }
  return 0;
}
