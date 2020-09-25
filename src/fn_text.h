/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Text Drawing functions
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

#ifndef FN_TEXT_H
#define FN_TEXT_H

/* --------------------------------------------------------------- */

#include <SDL/SDL.h>

/* --------------------------------------------------------------- */

#include "fn_environment.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

void fn_text_printletter(
    FnTexture * target,
    FnGeometry * r,
    const FnTileCache * tilecache,
    char c);

/* --------------------------------------------------------------- */

void fn_text_print(
    FnTexture * target,
    FnGeometry * r,
    const FnTileCache * tilecache,
    char * text);

/* --------------------------------------------------------------- */

#endif /* FN_TEXT_H */
