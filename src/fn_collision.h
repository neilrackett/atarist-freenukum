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

#ifndef FN_COLLISION_H
#define FN_COLLISION_H

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

#include "rusted.h"

/* --------------------------------------------------------------- */

/**
 * Check if two rectangles overlap.
 *
 * @param  r1  The first rectangle.
 * @param  r2  The second rectangle.
 *
 * @return 1 if the rectangles overlap, otherwise 0.
 */
int fn_collision_overlap_rect_rect(FnGeometry r1, FnGeometry r2);

/* --------------------------------------------------------------- */

/**
 * Check if two rectangles touch.
 *
 * @param  rect1    The first rectangle.
 * @param  rect2    The first rectangle.
 *
 * @return 1 if the rectangles touch, otherwise 0.
 */
int fn_collision_touch_rect_rect(FnGeometry rect1, FnGeometry rect2);

/* --------------------------------------------------------------- */

/**
 * Get the horizontal distance of an area and a rectangle.
 *
 * @param  rect  The rectangle.
 * @param  x     The x coordinate of the area.
 * @param  w     The width of the area.
 *
 * @return The distance between the two areas.
 */
int fn_collision_distance_horizontal_rect_rect(
    FnGeometry r1, FnGeometry r2);

/* --------------------------------------------------------------- */

/**
 * Check if two rectangles overlap in horizontal direction.
 *
 * @param  r1  Rectangle 1.
 * @param  r2  Rectangle 2.
 *
 * @return The distance between the two rectangles.
 */
int fn_collision_overlap_vertical_rect_rect(
    FnGeometry r1, FnGeometry r2);

/* --------------------------------------------------------------- */

/**
 * Get the vertical distance of an area and a rectangle.
 *
 * @param  rect  The rectangle.
 * @param  y     The y coordinate of the area.
 * @param  h     The height of the area.
 *
 * @return The distance between the two areas.
 */
int fn_collision_distance_vertical_rect_rect(
    FnGeometry r1, FnGeometry r2);

/* --------------------------------------------------------------- */

/**
 * Debug drawing function for a collision rectangle.
 *
 * @param  destination  The surface on which to draw.
 * @param  rect  The rectangle.
 */
void fn_collision_rect_draw(SDL_Surface * destination,
    FnGeometry rect);

/* --------------------------------------------------------------- */

#endif /* FN_COLLISION_H */
