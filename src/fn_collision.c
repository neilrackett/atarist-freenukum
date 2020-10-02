/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Collision functions
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

#include "fn.h"
#include "fn_collision.h"

/* --------------------------------------------------------------- */

int fn_collision_overlap_rect_rect(FnGeometry r1, FnGeometry r2)
{
  if (r1.x + r1.w <= r2.x)
  {
    return 0;
  }
  if (r2.x + r2.w <= r1.x)
  {
    return 0;
  }
  if (r1.y + r1.h <= r2.y)
  {
    return 0;
  }
  if (r2.y + r2.h <= r1.y)
  {
    return 0;
  }
  return 1;
}

/* --------------------------------------------------------------- */

void fn_collision_rect_draw(SDL_Surface * destination, FnGeometry rect)
{
  Uint32 color = FN_COLLISION_DEBUG_COLOR(destination->format);

  FnGeometry destrect;

  destrect.x = rect.x;
  destrect.y = rect.y;
  destrect.w = 1;
  destrect.h = rect.h;

  {
      SDL_Rect r = fn_geometry_as_sdl_rect(&destrect);
      SDL_FillRect(destination, &r, color);
  }
  destrect.x += rect.w - 1;
  {
      SDL_Rect r = fn_geometry_as_sdl_rect(&destrect);
      SDL_FillRect(destination, &r, color);
  }
  destrect.x = rect.x;
  destrect.y = rect.y;
  destrect.h = 1;
  destrect.w = rect.w;
  {
      SDL_Rect r = fn_geometry_as_sdl_rect(&destrect);
      SDL_FillRect(destination, &r, color);
  }
  destrect.y = rect.y + rect.h - 1;
  {
      SDL_Rect r = fn_geometry_as_sdl_rect(&destrect);
      SDL_FillRect(destination, &r, color);
  }
}

/* --------------------------------------------------------------- */

int fn_collision_distance_horizontal_rect_rect(
    FnGeometry r1, FnGeometry r2)
{
  if (r1.x + r1.w < r2.x) {
    return r2.x - r1.w - r1.x;
  }
  if (r2.x + r2.w < r1.x) {
    return - (r1.x - r2.w - r2.x);
  }
  return 0;
}

/* --------------------------------------------------------------- */

int fn_collision_overlap_vertical_rect_rect(
    FnGeometry r1, FnGeometry r2)
{
  if (r1.y + r1.h <= r2.y) {
    return 0;
  }
  if (r2.y + r2.h <= r1.y) {
    return 0;
  }
  return 1;
}

/* --------------------------------------------------------------- */

int fn_collision_distance_vertical_rect_rect(
    FnGeometry r1, FnGeometry r2)
{
  if (r1.y + r1.h < r2.y) {
    return r2.y - r1.h - r1.y;
  }
  if (r2.y + r2.h < r1.y) {
    return -(r1.y - r1.h - r2.y);
  }
  return 0;
}

/* --------------------------------------------------------------- */

int fn_collision_touch_rect_rect(FnGeometry rect1, FnGeometry rect2)
{
    rect1.w += 1;
    rect1.h += 1;
    rect2.w += 1;
    rect2.h += 1;
  return fn_collision_overlap_rect_rect(rect1, rect2);
}

/* --------------------------------------------------------------- */
