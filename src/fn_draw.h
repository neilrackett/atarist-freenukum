/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Pixel Drawing functions
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

#ifndef FN_DRAW
#define FN_DRAW

/* --------------------------------------------------------------- */

#include <SDL.h>

/* --------------------------------------------------------------- */

typedef struct {
    Uint8 brighten;
    Uint8 red;
    Uint8 green;
    Uint8 blue;
    Uint8 trans;
} fn_byterow_t;

/* --------------------------------------------------------------- */

int fn_draw_byterow(
        SDL_Surface * target,
        SDL_Rect r,
        fn_byterow_t * br,
        Uint32 transcolor,
        Uint8 pixelsize);

/* --------------------------------------------------------------- */

/**
 * Decode a run of 8-pixel groups (5 bytes each: trans, blue,
 * green, red, brighten) in one call, wrapping to the next row
 * after rowgroups groups. Much faster than calling
 * fn_draw_byterow per group.
 */
int fn_draw_byterow_run(
        SDL_Surface * target,
        int x,
        int y,
        const Uint8 * data,
        size_t ngroups,
        size_t rowgroups,
        Uint32 transcolor,
        Uint8 pixelsize);

/* --------------------------------------------------------------- */

#endif /* FN_DRAW */
