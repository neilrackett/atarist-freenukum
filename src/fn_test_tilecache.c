/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Test for tile cache
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

#include <SDL/SDL.h>
#include <unistd.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_object.h"
#include "fn_environment.h"
#include "fn_error.h"
#include "fn_error_cmdline.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

int sumuntil(Uint8 * ar, size_t s)
{
    int ret = 0;
    while (s > 0)
    {
        s--;
        ret += ar[s];
    }
    return ret;
}

/* --------------------------------------------------------------- */

void blithex(
    SDL_Surface * target,
    FnGeometry * r,
    const FnTileCache * tilecache,
    int x)
{
  int tilenr;
  char dst[3];
  snprintf(dst, 3, "%02X", x);
  int i;

  for (i = 0; i < strlen(dst); i++) {
    if(dst[i] >= ' ' && dst[i] <= 'Z')
      tilenr = dst[i] - ' ' + FONT_ASCII_UPPERCASE;
    else
      tilenr = dst[i] - 'a' + FONT_ASCII_LOWERCASE;
    fn_texture_blit_to_sdl_surface(
        fn_tilecache_get_tile(tilecache, tilenr),
        NULL,
        target,
        r);
    r->x += FN_FONT_WIDTH;
  }
}

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
  size_t i = 0;
  size_t j = 0;

  int res;
  int quit = 0;
  SDL_Event event;
  fn_environment_t * env;
  env = fn_environment_create();

  SDL_Surface * screen = SDL_SetVideoMode(
      FN_TILE_WIDTH * (50+1),
      FN_TILE_HEIGHT * (26+1),
      FN_COLOR_DEPTH,
      FN_SURFACE_FLAGS);

  if (screen == NULL)
  {
    fprintf(stderr, "Can't set video mode: %s\n", SDL_GetError());
    return -1;
  }

  FnTextureCreationParams texture_creation_params =
      fn_sdl_surface_creation_params(screen);
  const FnTileCache * tilecache =
      fn_tilecache_load(texture_creation_params);

  fn_error_set_handler(fn_error_print_commandline);


  Uint8 size[] = {
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    48,
    50,
    50,
    50,
    48,
    48,
    48,
    48,
    48,
    50,
    50,
    48,
    44,
    0
  };

  if (SDL_Init(SDL_INIT_VIDEO) == -1)
  {
    fprintf(stderr, "Can't init SDL: %s\n", SDL_GetError());
    return -1;
  }


  FnGeometry r;
  r.x = 0;
  r.y = 0;
  r.w = FN_TILE_WIDTH;
  r.h = FN_TILE_HEIGHT;

  for (i = 0; i != 26; i++) {
    r.x = 0;
    r.y = (i+1) * FN_TILE_HEIGHT;
    blithex(screen, &r, tilecache, i);
  }

  for (i = 0; i != 50; i++) {
    r.x = (i+1) * FN_TILE_WIDTH;
    r.y = 0;
    blithex(screen, &r, tilecache, i);
  }

  for (j = 0; j != 26; j++)
  {
    for(i = 0; i != size[j]; i++)
    {
      r.x = (i+1) * FN_TILE_WIDTH;
      r.y = (j+1) * FN_TILE_HEIGHT;
      const FnTexture * tile =
        fn_tilecache_get_tile(tilecache, sumuntil(size, j)+i);
      fn_texture_blit_to_sdl_surface(
          tile, NULL,
          screen, &r);
    }
  }
  SDL_UpdateRect(screen, 0, 0, 0, 0);

  while (quit == 0)
  {
    res = SDL_WaitEvent(&event);
    if (res == 1)
    {
      switch(event.type)
      {
        case SDL_QUIT:
          quit = 1;
          break;
        case SDL_KEYDOWN:
          switch(event.key.keysym.sym)
          {
            case SDLK_q:
            case SDLK_ESCAPE:
              quit = 1;
              break;
            default:
              /* do nothing, ignoring other keys. */
              break;
          }
        case SDL_VIDEOEXPOSE:
          SDL_UpdateRect(screen, 0, 0, 0, 0);
          break;
        default:
          /* do nothing */
          break;
      }
    }
  }

  fn_environment_delete(env);

  return 0;
}

