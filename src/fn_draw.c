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

/* Set when the pixel values in the cache are exactly the source
 * bits rearranged (cache index brighten|blue<<1|green<<2|red<<3
 * maps to pixel value blue|green<<1|red<<2|brighten<<3, i.e. the
 * native EGA palette). Then the five bytes of a decoded row ARE
 * the four bitplanes plus the mask, and fn_draw_byterow collapses
 * to a handful of ANDs. */
static int fn_draw_cache_direct = 0;

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

    fn_draw_cache_direct = 1;
    for (i = 0; i < 16; i++) {
        Uint32 want = ((i & 1) << 3)    /* brighten */
            | ((i >> 1) & 1)            /* blue */
            | (((i >> 2) & 1) << 1)     /* green */
            | (((i >> 3) & 1) << 2);    /* red */
        if (fn_draw_colorcache[i] != want) {
            fn_draw_cache_direct = 0;
            break;
        }
    }
}

/* --------------------------------------------------------------- */

static int fn_draw_byterow_slow(
        SDL_Surface * target,
        SDL_Rect r,
        fn_byterow_t * br,
        Uint32 transcolor,
        Uint8 pixelsize);

/* --------------------------------------------------------------- */

int fn_draw_byterow(
        SDL_Surface * target,
        SDL_Rect r,
        fn_byterow_t * br,
        Uint32 transcolor,
        Uint8 pixelsize)
{
    SDL_PixelFormat * fmt = target->format;
    size_t i = 0;
    r.h = pixelsize;
    r.w = pixelsize;

    if (fmt != fn_draw_colorcache_fmt) {
        fn_draw_fill_colorcache(fmt);
    }

    /* fastest path: the source bytes are the bitplanes (see
     * fn_draw_cache_direct); pixels without their trans bit are
     * background - transparent (mask 0) or black (planes 0) */
    if (pixelsize == 1 && (r.x & 7) == 0 && fn_draw_cache_direct
            && (transcolor == 0 || transcolor > 15)) {
        Uint8 planes[4];
        Uint8 t = br->trans;
        planes[0] = br->blue & t;
        planes[1] = br->green & t;
        planes[2] = br->red & t;
        planes[3] = br->brighten & t;
        /* STDL masks are transparency (bit set = destination
         * preserved), the inverse of the decoder's opacity byte */
        STDL_PutGroup8(target, r.x, r.y, planes,
            transcolor > 15 ? (Uint8)~t : 0x00);
        return 0;
    }

    /* fast path: build the whole 8px group and write it once
     * instead of one FillRect per pixel */
    if (pixelsize == 1 && (r.x & 7) == 0) {
        Uint8 planes[4] = { 0, 0, 0, 0 };
        Uint8 mask = 0;
        for (i = 0; i != 8; i++) {
            Uint8 bit = 0x80 >> i;
            Uint32 c;
            if ((br->trans >> (7 - i)) & 1) {
                c = fn_draw_colorcache[
                    (((br->brighten >> (7 - i)) & 1))
                    | (((br->blue     >> (7 - i)) & 1) << 1)
                    | (((br->green    >> (7 - i)) & 1) << 2)
                    | (((br->red      >> (7 - i)) & 1) << 3)];
            } else {
                c = transcolor;
            }
            if (c > 15) {
                continue;       /* transparent pixel */
            }
            mask |= bit;
            if (c & 1) planes[0] |= bit;
            if (c & 2) planes[1] |= bit;
            if (c & 4) planes[2] |= bit;
            if (c & 8) planes[3] |= bit;
        }
        STDL_PutGroup8(target, r.x, r.y, planes, (Uint8)~mask);
        return 0;
    }

    return fn_draw_byterow_slow(target, r, br, transcolor,
        pixelsize);
}

/* --------------------------------------------------------------- */

int fn_draw_byterow_run(
        SDL_Surface * target,
        int x,
        int y,
        const Uint8 * data,
        size_t ngroups,
        size_t rowgroups,
        Uint32 transcolor,
        Uint8 pixelsize)
{
    /* whole-image decode in one call: the per-group function call
     * and address setup dominate tile loading otherwise */
    if (pixelsize == 1 && (x & 7) == 0
            && fn_draw_cache_direct
            && (transcolor == 0 || transcolor > 15)
            && (target->mask != NULL || transcolor == 0)
            && x + (int)rowgroups * 8 <= target->w
            && y + (int)(ngroups / rowgroups) <= target->h) {
        int g0 = x >> 3;
        Uint8 * prow = (Uint8 *)target->pixels
            + (Uint32)(Uint16)y * target->pitch
            + ((Uint32)(g0 >> 1) << 3) + (g0 & 1);
        int transparent = (transcolor > 15);
        /* an opaque surface (no mask) writes its all-zero mask
         * bytes to a scratch byte with a zero stride, which keeps
         * the inner loop free of a per-group branch */
        Uint8 sink = 0;
        Uint8 * mrow = (target->mask != NULL)
            ? target->mask
              + (Uint32)(Uint16)y * target->maskstride + g0
            : &sink;
        int mstep = (target->mask != NULL) ? 1 : 0;
        int mpitch = (target->mask != NULL) ? target->maskstride : 0;
        size_t done = 0;

        target->opaque_state = 0;
        while (done < ngroups) {
            Uint8 * p = prow;
            Uint8 * m = mrow;
            int step = (g0 & 1) ? 7 : 1;
            size_t g;
            for (g = 0; g < rowgroups; g++) {
                Uint8 t = data[0];
                p[0] = data[1] & t;   /* blue  = plane 0 */
                p[2] = data[2] & t;   /* green = plane 1 */
                p[4] = data[3] & t;   /* red   = plane 2 */
                p[6] = data[4] & t;   /* brighten = plane 3 */
                /* transparency mask: bit set = destination kept */
                *m = transparent ? (Uint8)~t : 0x00;
                m += mstep;
                data += 5;
                p += step;
                step = 8 - step;
            }
            prow += target->pitch;
            mrow += mpitch;
            done += rowgroups;
        }
        return 0;
    }

    /* general case: one row at a time */
    {
        SDL_Rect r;
        fn_byterow_t br;
        size_t done;
        r.w = 8 * pixelsize;
        r.h = pixelsize;
        for (done = 0; done < ngroups; done++) {
            size_t col = done % rowgroups;
            r.x = (x + (int)col * 8) * pixelsize;
            r.y = (y + (int)(done / rowgroups)) * pixelsize;
            br.trans    = data[0];
            br.blue     = data[1];
            br.green    = data[2];
            br.red      = data[3];
            br.brighten = data[4];
            data += 5;
            fn_draw_byterow(target, r, &br, transcolor, pixelsize);
        }
    }
    return 0;
}

/* --------------------------------------------------------------- */

static int fn_draw_byterow_slow(
        SDL_Surface * target,
        SDL_Rect r,
        fn_byterow_t * br,
        Uint32 transcolor,
        Uint8 pixelsize)
{
    Uint32 color;
    size_t i;

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

