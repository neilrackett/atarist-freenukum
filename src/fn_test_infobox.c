/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Infobox drawing function test
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
#include <SDL/SDL_ttf.h>

/* --------------------------------------------------------------- */

#include "fn_environment.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv) {
  FnSettings settings = fn_settings_load_or_create();
  if (!fn_game_initialize_sdl()) {
      return 1;
  }
  fn_environment_t * env = fn_environment_create(settings.fullscreen);

  SDL_Surface * screen = env->screen;

  FnTextureCreationParams texture_creation_params =
      fn_sdl_surface_creation_params(screen);
  const FnTileCache * tilecache =
      fn_tilecache_load(texture_creation_params);

  fn_infobox_show(
      screen,
      tilecache,
      texture_creation_params,
      "This is...\n");
  fn_infobox_show(
      screen,
      tilecache,
      texture_creation_params,
      "...the great\nInfobox test case.\n");
  fn_infobox_show(
      screen,
      tilecache,
      texture_creation_params,
      "now\nwith\neven\nmore\nlines.\n");
  
  return 0;
}

