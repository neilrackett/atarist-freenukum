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

#include "fn_draw.h"

/* --------------------------------------------------------------- */

/* The 16 EGA colors indexed by brighten|red|green|blue bits,
 * mapped to pixel values once per pixel format. Mapping colors
 * per-pixel via SDL_MapRGB is prohibitively expensive on paletted
 * displays (linear palette search), which the Atari ST port hits
 * for every decoded graphics byte.
 */
static Uint32 fn_draw_colorcache[16];
static SDL_PixelFormat * fn_draw_colorcache_fmt = NULL;

static void fn_draw_fill_colorcache(SDL_PixelFormat * fmt)
{
    int i;
    for (i = 0; i < 16; i++) {
        Uint8 bright = (i & 1) ? 0x54 : 0x00;
        Uint8 red    = ((i & 8) ? 0xA8 : 0x00) + bright;
        Uint8 green  = ((i & 4) ? 0xA8 : 0x00) + bright;
        Uint8 blue   = ((i & 2) ? 0xA8 : 0x00) + bright;

        /* Workaround for beautiful brown instead of
         * strange yellow */
        if (red == 0xA8 && green == 0xA8 && blue == 0x00) {
            green = 0x54;
        }

        fn_draw_colorcache[i] = SDL_MapRGB(fmt, red, green, blue);
    }
    fn_draw_colorcache_fmt = fmt;
}

/* --------------------------------------------------------------- */

int fn_draw_byterow(
        SDL_Surface * target,
        SDL_Rect r,
        fn_byterow_t * br,
        Uint32 transcolor,
        Uint8 pixelsize)
{
    SDL_PixelFormat * fmt = target->format;
    Uint32 color;
    size_t i = 0;
    r.h = pixelsize;
    r.w = pixelsize;

    if (fmt != fn_draw_colorcache_fmt) {
        fn_draw_fill_colorcache(fmt);
    }

    for (i = 0; i != 8; i++)
    {
        Uint8 filled = (br->trans >> (7-i)) & 1;
        if (filled == 0)
        {
            color = transcolor;
        }
        else
        {
            color = fn_draw_colorcache[
                (((br->brighten >> (7-i)) & 1))
                | (((br->blue     >> (7-i)) & 1) << 1)
                | (((br->green    >> (7-i)) & 1) << 2)
                | (((br->red      >> (7-i)) & 1) << 3)];
        }
        SDL_FillRect(target, &r, color);

        r.x+= pixelsize;
    }
    return 0;
}

/* --------------------------------------------------------------- */

