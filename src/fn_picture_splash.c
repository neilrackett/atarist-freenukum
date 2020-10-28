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

/* --------------------------------------------------------------- */

int fn_picture_splash_show(
    const FnTileCache * tilecache,
    FnTextureCreationParams texture_creation_params,
    SDL_Surface * target,
    char * filename)
{
  return fn_picture_splash_show_with_message(
      tilecache,
      texture_creation_params,
      target,
      filename,
      NULL,
      0,0);
}

/* --------------------------------------------------------------- */

int fn_picture_splash_show_with_message(
    const FnTileCache * tilecache,
    FnTextureCreationParams texture_creation_params,
    SDL_Surface * target,
    char * filename,
    char * msg,
    Uint8 x,
    Uint8 y)
{
  FnFile * file;
  int res;
  SDL_Event event;
  FnTexture * picture;

  file = fn_data_open_file(filename);

  picture = fn_picture_load(file, texture_creation_params);

  fn_texture_blit_to_sdl_surface(picture, NULL, target, NULL);

  if (msg != NULL) {
    FnTexture * msgbox;
    FnGeometry dstrect;

    msgbox = fn_messagebox(msg, tilecache, texture_creation_params);

    dstrect.x = x;
    dstrect.y = y;

    fn_texture_blit_to_sdl_surface(msgbox, NULL, target, &dstrect);
    fn_texture_free(msgbox);
  }

  SDL_UpdateRect(target, 0, 0, 0, 0);
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
          SDL_UpdateRect(target, 0, 0, 0, 0);
          break;
        default:
          /* ignore unknown events */
          break;
      }
    }
  }
  return 0;
}
