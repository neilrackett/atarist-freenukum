/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Infobox drawing function
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

#include "fn_infobox.h"

/* --------------------------------------------------------------- */

void fn_infobox_show(
    fn_environment_t * env,
    char * msg)
{
  FnTexture * msgbox;
  SDL_Surface * temp;
  FnGeometry destrect;

  int res;

  SDL_Event event;

  msgbox = fn_messagebox(
          msg,
          fn_environment_get_tilecache(env),
          fn_environment_build_texture_creation_params(env));

  SDL_Surface * screen = fn_environment_get_screen_sdl(env);
  destrect.x = ((screen->w) - fn_texture_get_width(msgbox)) / 2;
  destrect.y = ((screen->h) - fn_texture_get_height(msgbox)) / 2;
  destrect.w = fn_texture_get_width(msgbox);
  destrect.h = fn_texture_get_height(msgbox);

  SDL_Rect dstrect = fn_geometry_as_sdl_rect(&destrect);

  /* backup the background */
  temp = fn_environment_create_surface(env,
      fn_texture_get_width(msgbox), fn_texture_get_height(msgbox));
  SDL_BlitSurface(screen, &dstrect, temp, NULL);

  fn_texture_blit_to_sdl_surface(msgbox, NULL, screen, &destrect);
  fn_texture_free(msgbox);
  SDL_UpdateRect(screen, 0, 0, 0, 0);

  while (1) {
    res = SDL_WaitEvent(&event);
    if (res == 1) {
      switch(event.type) {
        case SDL_KEYDOWN:
          SDL_BlitSurface(temp, NULL, screen, &dstrect);
          SDL_FreeSurface(temp);
          return;
          break;
        case SDL_MOUSEBUTTONDOWN:
          if (event.button.button == SDL_BUTTON_LEFT) {
            SDL_BlitSurface(temp, NULL, screen, &dstrect);
            SDL_FreeSurface(temp);
            return;
          }
          break;
        case SDL_VIDEOEXPOSE:
          SDL_UpdateRect(screen, 0, 0, 0, 0);
          break;
        default:
          /* ignore other events */
          break;
      }
    }
  }
}

