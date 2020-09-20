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

#include <SDL/SDL.h>

/* =============================================================== */

#include "fn.h"
#include "fn_environment.h"
#include "fntexture.h"

/* =============================================================== */

struct FnTexture
{
  int width;
  int height;
  SDL_Surface * surface;
};

/* =============================================================== */

FnTexture * fn_texture_new_with_environment(
    int width,
    int height,
    fn_environment_t * env
    )
{
  FnTexture * texture = malloc(sizeof(FnTexture));
  texture->width = width;
  texture->height = height;


  SDL_Surface * surface = SDL_CreateRGBSurface(
          env->screen->flags,
          width,
          height,
          env->screen->format->BitsPerPixel,
          0,
          0,
          0,
          0);
  SDL_SetColorKey(surface, SDL_SRCCOLORKEY, env->transparent);

  SDL_FillRect(
      surface,
      NULL,
      0);

  texture->surface = surface;
  return texture;
}

/* =============================================================== */

void fn_texture_free(FnTexture* texture)
{
    SDL_FreeSurface(texture->surface);
    free(texture);
}

/* =============================================================== */

void
fn_texture_set_data(
    FnTexture * texture,
    unsigned char * data,
    Uint32 transparent)
{
  SDL_Surface * surface = texture->surface;

  unsigned int i = 0;
  unsigned int j = 0;
  unsigned char * iter = data;

  SDL_Rect r;
  r.x = 0;
  r.y = 0;
  r.w = 1;
  r.h = 1;

  SDL_PixelFormat * fmt = surface->format;
  Uint32 color;

  for (i = 0; i < texture->height; i++)
  {
    for (j = 0; j < texture->width; j++)
    {
      unsigned char red    = iter[0];
      unsigned char green  = iter[1];
      unsigned char blue   = iter[2];
      unsigned char opaque = iter[3];

      if (opaque == 0) {
        color = transparent;
      } else {
        color = SDL_MapRGB(fmt, red, green, blue);
      }

      r.x = j;
      r.y = i;

      SDL_FillRect(surface, &r, color);

      iter += 4;
    }
  }

  texture->surface = surface;
}

/* =============================================================== */

void
fn_texture_blit_to_sdl_surface(
    FnTexture * texture,
    SDL_Rect * srcrect,
    SDL_Surface * destination,
    SDL_Rect * dstrect)
{
  SDL_Surface * src = texture->surface;

  SDL_BlitSurface(src, srcrect, destination, dstrect);
}

/* =============================================================== */

void
fn_texture_clone_to_texture(
    FnTexture * source,
    FnGeometry * sourcegeometry,
    FnTexture * target,
    FnGeometry * targetgeometry)
{
  SDL_Rect * sourcerect = NULL;
  if (sourcegeometry != NULL) {
    sourcerect = g_malloc0(sizeof(SDL_Rect));
    sourcerect->x = sourcegeometry->x;
    sourcerect->y = sourcegeometry->y;
    sourcerect->w = sourcegeometry->w;
    sourcerect->h = sourcegeometry->h;
  }

  SDL_Rect * targetrect = NULL;
  if (targetgeometry != NULL) {
    targetrect = g_malloc0(sizeof(SDL_Rect));
    targetrect->x = targetgeometry->x;
    targetrect->y = targetgeometry->y;
    targetrect->w = targetgeometry->w;
    targetrect->h = targetgeometry->h;
  }

  SDL_BlitSurface(
      source->surface, sourcerect,
      target->surface, targetrect);

  if (sourcerect != NULL) {
    free(sourcerect);
  }
  if (targetrect != NULL) {
    free(targetrect);
  }
}

/* =============================================================== */

unsigned int
fn_texture_get_width(FnTexture * texture)
{
  return texture->width;
}

/* =============================================================== */

unsigned int
fn_texture_get_height(FnTexture * texture)
{
  return texture->height;
}

/* =============================================================== */

void
fn_texture_fill_area(
    FnTexture * texture,
    FnGeometry * area,
    unsigned char red,
    unsigned char green,
    unsigned char blue)
{
  int x;
  int y;
  unsigned int width;
  unsigned int height;

  if (area != NULL) {
    x = area->x;
    y = area->y;
    width = area->w;
    height = area->h;
  } else {
    x = 0;
    y = 0;
    width = texture->width;
    height = texture->height;
  }
  SDL_Rect rect = { x, y, width, height };

  Uint32 cursorcolor = SDL_MapRGB(
      texture->surface->format, red, green, blue);
  SDL_FillRect(texture->surface, &rect, cursorcolor);
}

/* =============================================================== */
