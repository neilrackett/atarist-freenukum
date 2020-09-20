/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Texture
 *
 * *****************************************************************
 *
 * Copyright 2009 Wolfgang Silbermayr
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

#ifndef FNTEXTURE_H
#define FNTEXTURE_H

/* =============================================================== */

#include <SDL/SDL.h>

/* =============================================================== */

#include "rusted.h"
#include "fn_environment.h"

/* =============================================================== */

#define FN_TEXTURE_DEFAULT_WIDTH 32
#define FN_TEXTURE_DEFAULT_HEIGHT 32

/* =============================================================== */

typedef struct FnTexture FnTexture;

/* =============================================================== */

FnTexture * fn_texture_new_with_environment(
    int width,
    int height,
    fn_environment_t * env
    );

/* =============================================================== */

void fn_texture_free(FnTexture* texture);

/* =============================================================== */

void
fn_texture_set_data(
    FnTexture * texture,
    unsigned char * data,
    Uint32 transparent);

/* =============================================================== */

void
fn_texture_blit_to_sdl_surface(
    FnTexture * texture,
    SDL_Rect * srcrect,
    SDL_Surface * destination,
    SDL_Rect * dstrect);

/* =============================================================== */

/* TODO write documentation that sourcegeometry as well
   as targetgeometry can be NULL.
   */
void
fn_texture_clone_to_texture(
    FnTexture * source,
    FnGeometry * sourcegeometry,
    FnTexture * target,
    FnGeometry * targetgeometry);

/* =============================================================== */

unsigned int
fn_texture_get_width(
    FnTexture * texture);

/* =============================================================== */

unsigned int
fn_texture_get_height(
    FnTexture * texture);

/* =============================================================== */

/* TODO for api-documentation: area can be NULL, in that case
   the whole texture is filled with the color */
void
fn_texture_fill_area(
    FnTexture * texture,
    FnGeometry * area,
    unsigned char red,
    unsigned char green,
    unsigned char blue);

/* =============================================================== */

#endif /* FNTEXTURE_H */
