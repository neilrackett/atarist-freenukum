/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Tile loader
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

#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>

/* --------------------------------------------------------------- */

#include "fn_object.h"
#include "fn_tile.h"
#include "fn.h"

/* --------------------------------------------------------------- */

int fn_tile_loadheader(int fd, fn_tileheader_t * h)
{
    return read(fd, h, sizeof(*h)) == sizeof(*h);
}

/* --------------------------------------------------------------- */

FnTexture * fn_tile_load(
    int fd,
    fn_environment_t * env,
    fn_tileheader_t * h,
    bool has_transparency)
{
  Uint16 width = h->width * 8;
  Uint16 height = h->height;

  FnTexture * tile = fn_texture_new_with_params(
      width,
      height,
      fn_environment_build_texture_creation_params(env));

  unsigned char * data = malloc(sizeof(unsigned char) * width * height * 4);

  unsigned char readbuf[5];

  size_t num_loads = width * height / 8;
  size_t num_read = 0;
  unsigned char * iter = data;

  while (num_read < num_loads)
  {
    read(fd, readbuf, 5);
    unsigned char opaque_row = readbuf[0];
    unsigned char blue_row   = readbuf[1];
    unsigned char green_row  = readbuf[2];
    unsigned char red_row    = readbuf[3];
    unsigned char bright_row = readbuf[4];

    unsigned char i = 0;

    for (i = 0; i < 8; i++) {
      unsigned char bright_pixel = ((bright_row >> (7-i)) & 1);
      unsigned char red_pixel    = ((red_row    >> (7-i)) & 1);
      unsigned char green_pixel  = ((green_row  >> (7-i)) & 1);
      unsigned char blue_pixel   = ((blue_row   >> (7-i)) & 1);
      unsigned char opaque_pixel = (
          has_transparency ?
          ((opaque_row >> (7-i)) & 1) :
          1);
      unsigned char ugly_yellow  = (
          red_pixel == 1 &&
          green_pixel == 1 &&
          blue_pixel == 0 &&
          bright_pixel == 0) ? 1 : 0;

      unsigned char * red    = iter;
      unsigned char * green  = iter + 1;
      unsigned char * blue   = iter + 2;
      unsigned char * opaque = iter + 3;

      *red =    0x54 * (red_pixel   * 2 + bright_pixel);
      *green =  0x54 * (green_pixel * 2 + bright_pixel - ugly_yellow);
      *blue  =  0x54 * (blue_pixel  * 2 + bright_pixel);
      *opaque = opaque_pixel * 0xFF;
      iter += 4;
    }

    num_read++;
  }

  fn_texture_set_data(tile, data, fn_environment_get_transparent(env));

  free(data);
  return tile;
}

/* --------------------------------------------------------------- */

int fn_tile_is_solid(
    Uint16 tile)
{
  return tile >= SOLID_START && tile < ANIM_START;
}

/* --------------------------------------------------------------- */

