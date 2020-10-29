/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Tests for picture splash functions
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

#include "config.h"
#include "fn.h"
#include "fn_error.h"
#include "fn_error_cmdline.h"
#include "fn_picture_splash.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
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

  fn_error_set_handler(fn_error_print_commandline);

  FnTextureCreationParams texture_creation_params =
      fn_sdl_surface_creation_params(screen);
  const FnTileCache * tilecache =
      fn_tilecache_load(texture_creation_params);

  FnFile * file = fn_data_open_file("dn.dn1");
  fn_picture_splash_show(
          tilecache, texture_creation_params, screen, file);
  fn_file_free(file);

  SDL_FreeSurface(screen);
  
  return 0;
}
