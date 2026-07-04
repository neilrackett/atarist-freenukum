/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Native Atari ST implementation of the SDL 1.2 subset
 *          used by the game ("nsdl")
 *
 * *****************************************************************
 *
 * Copyright 2026 Freenukum contributors
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

#include <SDL.h>
#include <mint/osbind.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>

/* ---------------------------------------------------------------- */
/* palette                                                          */

/* 16 EGA colors (index = bright<<3 | red<<2 | green<<1 | blue),
 * entry 16 is the colorkey. */
static SDL_Color nsdl_colors[17];
static SDL_Palette nsdl_palette = { 17, nsdl_colors };
static SDL_PixelFormat nsdl_format = { &nsdl_palette, 8, 1 };

static void nsdl_init_colors(void)
{
  int i;
  for (i = 0; i < 16; i++) {
    Uint8 bright = (i & 8) ? 0x54 : 0x00;
    nsdl_colors[i].r = ((i & 4) ? 0xA8 : 0x00) + bright;
    nsdl_colors[i].g = ((i & 2) ? 0xA8 : 0x00) + bright;
    nsdl_colors[i].b = ((i & 1) ? 0xA8 : 0x00) + bright;
  }
  /* EGA's brown: dark yellow has halved green */
  nsdl_colors[6].g = 0x54;
  nsdl_colors[16].r = 100;
  nsdl_colors[16].g = 1;
  nsdl_colors[16].b = 1;
}

static void nsdl_set_hardware_palette(void)
{
  int i;
  for (i = 0; i < 16; i++) {
    Setcolor(i,
        ((nsdl_colors[i].r >> 5) << 8)
        | ((nsdl_colors[i].g >> 5) << 4)
        | (nsdl_colors[i].b >> 5));
  }
}

/* ---------------------------------------------------------------- */
/* time                                                             */

static long nsdl_read_hz200(void)
{
  return *(volatile long *)0x4BA;
}

static Uint32 nsdl_ticks_base = 0;

Uint32 SDL_GetTicks(void)
{
  Uint32 now = (Uint32)Supexec(nsdl_read_hz200) * 5;
  if (nsdl_ticks_base == 0) {
    nsdl_ticks_base = now;
  }
  return now - nsdl_ticks_base;
}

void SDL_Delay(Uint32 ms)
{
  Uint32 end = SDL_GetTicks() + ms;
  while ((Sint32)(SDL_GetTicks() - end) < 0);
}

/* ---------------------------------------------------------------- */
/* event queue                                                      */

#define NSDL_QUEUE_SIZE 32

static SDL_Event nsdl_queue[NSDL_QUEUE_SIZE];
static int nsdl_queue_head = 0;
static int nsdl_queue_len = 0;

int SDL_PushEvent(SDL_Event * event)
{
  if (nsdl_queue_len >= NSDL_QUEUE_SIZE) {
    return -1;
  }
  nsdl_queue[(nsdl_queue_head + nsdl_queue_len) % NSDL_QUEUE_SIZE] =
    *event;
  nsdl_queue_len++;
  return 0;
}

static int nsdl_pop_event(SDL_Event * event)
{
  if (nsdl_queue_len == 0) {
    return 0;
  }
  *event = nsdl_queue[nsdl_queue_head];
  nsdl_queue_head = (nsdl_queue_head + 1) % NSDL_QUEUE_SIZE;
  nsdl_queue_len--;
  return 1;
}

/* ---------------------------------------------------------------- */
/* keyboard                                                         */
/*
 * Plain TOS gives us key presses (with auto repeat) through the
 * BIOS console, but no release events. We set the fastest repeat
 * rate and treat "no repeat seen for a while" as the release.
 * Modifier keys are polled exactly through Kbshift.
 */

#define NSDL_KEY_TIMEOUT 140  /* ms without repeat = released */

typedef struct {
  int active;
  SDLKey sym;
  Uint32 last_seen;
} nsdl_heldkey_t;

static nsdl_heldkey_t nsdl_held[128];
static Uint8 nsdl_shift_state = 0;
static int nsdl_repeat_enabled = 0;

/* ST scancode to SDLKey for keys without a useful ascii code */
static SDLKey nsdl_scan_to_sym(Uint8 scan, Uint8 ascii)
{
  switch (scan) {
    case 0x01: return SDLK_ESCAPE;
    case 0x0E: return SDLK_BACKSPACE;
    case 0x1C: return SDLK_RETURN;
    case 0x39: return SDLK_SPACE;
    case 0x48: return SDLK_UP;
    case 0x4B: return SDLK_LEFT;
    case 0x4D: return SDLK_RIGHT;
    case 0x50: return SDLK_DOWN;
    case 0x53: return SDLK_DELETE;
    case 0x62: return SDLK_F1; /* HELP key */
    case 0x72: return SDLK_RETURN; /* keypad enter */
    default: break;
  }
  if (scan >= 0x3B && scan <= 0x44) {
    return (SDLKey)(SDLK_F1 + (scan - 0x3B));
  }
  if (ascii >= 'A' && ascii <= 'Z') {
    return (SDLKey)(ascii + 32);
  }
  if (ascii >= 32 && ascii < 127) {
    return (SDLKey)ascii;
  }
  return SDLK_UNKNOWN;
}

static Uint16 nsdl_kmod(void)
{
  Uint16 mod = KMOD_NONE;
  Uint8 shift = nsdl_shift_state;
  if (shift & 0x03) mod |= KMOD_SHIFT;
  if (shift & 0x04) mod |= KMOD_CTRL;
  if (shift & 0x08) mod |= KMOD_ALT;
  return mod;
}

static void nsdl_push_key(Uint8 type, SDLKey sym)
{
  SDL_Event ev;
  ev.key.type = type;
  ev.key.state = (type == SDL_KEYDOWN);
  ev.key.keysym.scancode = 0;
  ev.key.keysym.sym = sym;
  ev.key.keysym.mod = nsdl_kmod();
  ev.key.keysym.unicode = 0;
  SDL_PushEvent(&ev);
}

static void nsdl_pump(void)
{
  Uint32 now = SDL_GetTicks();
  int i;

  /* modifier keys via Kbshift */
  Uint8 shift = (Uint8)Kbshift(-1);
  Uint8 changed = shift ^ nsdl_shift_state;
  nsdl_shift_state = shift;
  if (changed & 0x01) {
    nsdl_push_key((shift & 0x01) ? SDL_KEYDOWN : SDL_KEYUP,
        SDLK_RSHIFT);
  }
  if (changed & 0x02) {
    nsdl_push_key((shift & 0x02) ? SDL_KEYDOWN : SDL_KEYUP,
        SDLK_LSHIFT);
  }
  if (changed & 0x04) {
    nsdl_push_key((shift & 0x04) ? SDL_KEYDOWN : SDL_KEYUP,
        SDLK_LCTRL);
  }
  if (changed & 0x08) {
    nsdl_push_key((shift & 0x08) ? SDL_KEYDOWN : SDL_KEYUP,
        SDLK_LALT);
  }

  /* normal keys via the BIOS console */
  while (Bconstat(2)) {
    Uint32 c = (Uint32)Bconin(2);
    Uint8 scan = (c >> 16) & 0x7F;
    Uint8 ascii = c & 0xFF;
    nsdl_heldkey_t * held = &nsdl_held[scan];
    if (!held->active) {
      held->active = 1;
      held->sym = nsdl_scan_to_sym(scan, ascii);
      nsdl_push_key(SDL_KEYDOWN, held->sym);
    } else if (nsdl_repeat_enabled) {
      nsdl_push_key(SDL_KEYDOWN, held->sym);
    }
    held->last_seen = now;
  }

  /* expire keys that stopped repeating */
  for (i = 0; i < 128; i++) {
    if (nsdl_held[i].active
        && (Sint32)(now - nsdl_held[i].last_seen)
            > NSDL_KEY_TIMEOUT) {
      nsdl_held[i].active = 0;
      nsdl_push_key(SDL_KEYUP, nsdl_held[i].sym);
    }
  }
}

int SDL_PollEvent(SDL_Event * event)
{
  nsdl_pump();
  return nsdl_pop_event(event);
}

int SDL_WaitEvent(SDL_Event * event)
{
  while (1) {
    if (SDL_PollEvent(event)) {
      return 1;
    }
    SDL_Delay(5);
  }
}

int SDL_EnableKeyRepeat(int delay, int interval)
{
  (void)interval;
  nsdl_repeat_enabled = (delay != 0);
  return 0;
}

/* ---------------------------------------------------------------- */
/* surfaces                                                         */
/*
 * Plane data layout is identical to ST screen RAM: lines of
 * 16-pixel chunks, each chunk 4 words (planes 0-3). The byte
 * holding the 8 pixels at x for plane p in a line is at offset
 *   ((x >> 4) << 3) + (p << 1) + ((x >> 3) & 1)
 * The opacity mask is linear, one byte per 8 pixels.
 */

static SDL_Surface nsdl_screen;

static SDL_Surface * nsdl_alloc_surface(int width, int height)
{
  int wpad = (width + 15) & ~15;
  SDL_Surface * s = calloc(1, sizeof(SDL_Surface));
  if (s == NULL) {
    return NULL;
  }
  s->w = width;
  s->h = height;
  s->pitch = wpad / 2;
  s->maskpitch = wpad / 8;
  s->format = &nsdl_format;
  s->pixels = calloc(1, (Uint32)s->pitch * height);
  s->mask = malloc((Uint32)s->maskpitch * height);
  if (s->pixels == NULL || s->mask == NULL) {
    fprintf(stderr, "nsdl: %dx%d surface allocation failed\n",
        width, height);
    free(s->pixels);
    free(s->mask);
    free(s);
    return NULL;
  }
  /* New SDL surfaces contain opaque black pixels (zeroed memory),
   * and the game relies on that for box interiors. */
  memset(s->mask, 0xFF, (Uint32)s->maskpitch * height);
  return s;
}

SDL_Surface * SDL_CreateRGBSurface(Uint32 flags, int width,
    int height, int depth, Uint32 Rmask, Uint32 Gmask, Uint32 Bmask,
    Uint32 Amask)
{
  (void)flags; (void)depth;
  (void)Rmask; (void)Gmask; (void)Bmask; (void)Amask;
  return nsdl_alloc_surface(width, height);
}

void SDL_FreeSurface(SDL_Surface * surface)
{
  if (surface == NULL || surface == &nsdl_screen) {
    return;
  }
  free(surface->pixels);
  free(surface->mask);
  free(surface);
}

int SDL_SetColorKey(SDL_Surface * surface, Uint32 flag, Uint32 key)
{
  (void)key;
  surface->usekey = (flag & SDL_SRCCOLORKEY) ? 1 : 0;
  return 0;
}

Uint32 SDL_MapRGB(SDL_PixelFormat * format, Uint8 r, Uint8 g, Uint8 b)
{
  (void)format;
  Uint32 best = 0;
  Sint32 bestdist = 0x7FFFFFFF;
  int i;
  for (i = 0; i < 17; i++) {
    Sint32 dr = (Sint32)r - nsdl_colors[i].r;
    Sint32 dg = (Sint32)g - nsdl_colors[i].g;
    Sint32 db = (Sint32)b - nsdl_colors[i].b;
    Sint32 dist = dr * dr + dg * dg + db * db;
    if (dist < bestdist) {
      bestdist = dist;
      best = i;
    }
  }
  return best;
}

void SDL_GetRGB(Uint32 pixel, SDL_PixelFormat * format, Uint8 * r,
    Uint8 * g, Uint8 * b)
{
  (void)format;
  if (pixel > 16) {
    pixel = 0;
  }
  *r = nsdl_colors[pixel].r;
  *g = nsdl_colors[pixel].g;
  *b = nsdl_colors[pixel].b;
}

int SDL_SetColors(SDL_Surface * surface, SDL_Color * colors,
    int firstcolor, int ncolors)
{
  int i;
  for (i = 0; i < ncolors && firstcolor + i < 17; i++) {
    nsdl_colors[firstcolor + i] = colors[i];
  }
  if (surface != NULL && surface->is_screen) {
    nsdl_set_hardware_palette();
  }
  return 1;
}

/* ---------------------------------------------------------------- */
/* blitting                                                         */

int SDL_FillRect(SDL_Surface * dst, SDL_Rect * dstrect, Uint32 color)
{
  int x, y, w, h;
  if (dstrect == NULL) {
    x = 0; y = 0; w = dst->w; h = dst->h;
  } else {
    x = dstrect->x; y = dstrect->y; w = dstrect->w; h = dstrect->h;
  }

  /* clip */
  if (x < 0) { w += x; x = 0; }
  if (y < 0) { h += y; y = 0; }
  if (x + w > dst->w) { w = dst->w - x; }
  if (y + h > dst->h) { h = dst->h - y; }
  if (w <= 0 || h <= 0) {
    return 0;
  }

  {
    int gx0 = x >> 3;
    int gx1 = (x + w + 7) >> 3;
    int maxg = dst->w >> 3;
    int transparent = (color == NSDL_KEYCOLOR);
    Uint8 planebyte[4];
    int p, g;
    if (gx1 > maxg) {
      gx1 = maxg;
    }
    if (!transparent) {
      color &= 15;
    }
    for (p = 0; p < 4; p++) {
      planebyte[p] =
        (!transparent && (color & (1 << p))) ? 0xFF : 0x00;
    }
    for (; h > 0; y++, h--) {
      Uint8 * line = (Uint8 *)dst->pixels + (Uint32)y * dst->pitch;
      Uint8 * mline =
        dst->mask ? dst->mask + (Uint32)y * dst->maskpitch : NULL;
      for (g = gx0; g < gx1; g++) {
        Uint8 * chunk = line + ((g >> 1) << 3) + (g & 1);
        /* pixel-precise coverage of [x, x+w) within this group,
         * leftmost pixel is the most significant bit */
        int lo = x > (g << 3) ? x - (g << 3) : 0;
        int hi = (x + w) < ((g + 1) << 3) ? x + w - (g << 3) : 8;
        Uint8 cover = (Uint8)((0xFF >> lo) & ~(0xFF >> hi));
        if (cover == 0xFF) {
          for (p = 0; p < 4; p++) {
            chunk[p << 1] = planebyte[p];
          }
          if (mline != NULL) {
            mline[g] = transparent ? 0x00 : 0xFF;
          }
        } else {
          Uint8 keep = ~cover;
          for (p = 0; p < 4; p++) {
            chunk[p << 1] =
              (chunk[p << 1] & keep) | (planebyte[p] & cover);
          }
          if (mline != NULL) {
            if (transparent) {
              mline[g] &= keep;
            } else {
              mline[g] |= cover;
            }
          }
        }
      }
    }
  }
  return 0;
}

int SDL_BlitSurface(SDL_Surface * src, SDL_Rect * srcrect,
    SDL_Surface * dst, SDL_Rect * dstrect)
{
  int sx, sy, w, h, dx, dy;

  if (src == NULL || dst == NULL) {
    return -1;
  }

  if (srcrect != NULL) {
    sx = srcrect->x; sy = srcrect->y;
    w = srcrect->w; h = srcrect->h;
  } else {
    sx = 0; sy = 0; w = src->w; h = src->h;
  }
  if (dstrect != NULL) {
    dx = dstrect->x; dy = dstrect->y;
  } else {
    dx = 0; dy = 0;
  }

  /* keep source and destination on the same 8px phase by moving
   * the destination back to the grid */
  dx -= (dx - sx) & 7;

  /* clip source */
  if (sx < 0) { w += sx; dx -= sx; sx = 0; }
  if (sy < 0) { h += sy; dy -= sy; sy = 0; }
  if (sx + w > src->w) { w = src->w - sx; }
  if (sy + h > src->h) { h = src->h - sy; }

  /* clip destination */
  if (dx < 0) { w += dx; sx -= dx; dx = 0; }
  if (dy < 0) { h += dy; sy -= dy; dy = 0; }
  if (dx + w > dst->w) { w = dst->w - dx; }
  if (dy + h > dst->h) { h = dst->h - dy; }
  if (w <= 0 || h <= 0) {
    if (dstrect != NULL) {
      dstrect->w = 0;
      dstrect->h = 0;
    }
    return 0;
  }

  /* SDL 1.2 stores the final clipped rectangle back into dstrect,
   * and the game relies on it (e.g. stepping by the blitted tile
   * size when composing the hero from tiles). */
  if (dstrect != NULL) {
    dstrect->x = dx;
    dstrect->y = dy;
    dstrect->w = w;
    dstrect->h = h;
  }

  {
    int sg0 = sx >> 3;
    int dg0 = dx >> 3;
    int ng = (((sx & 7) + w + 7) >> 3);
    int masked = (src->usekey && src->mask != NULL);
    int y;

    if (sg0 + ng > src->w >> 3) {
      ng = (src->w >> 3) - sg0;
    }
    if (dg0 + ng > dst->w >> 3) {
      ng = (dst->w >> 3) - dg0;
    }
    if (ng <= 0) {
      return 0;
    }

    for (y = 0; y < h; y++) {
      Uint8 * sline =
        (Uint8 *)src->pixels + (Uint32)(sy + y) * src->pitch;
      Uint8 * dline =
        (Uint8 *)dst->pixels + (Uint32)(dy + y) * dst->pitch;
      Uint8 * smline = src->mask
        ? src->mask + (Uint32)(sy + y) * src->maskpitch : NULL;
      Uint8 * dmline = dst->mask
        ? dst->mask + (Uint32)(dy + y) * dst->maskpitch : NULL;
      int g;
      for (g = 0; g < ng; g++) {
        int sgg = sg0 + g;
        int dgg = dg0 + g;
        Uint8 * schunk = sline + ((sgg >> 1) << 3) + (sgg & 1);
        Uint8 * dchunk = dline + ((dgg >> 1) << 3) + (dgg & 1);
        if (masked) {
          Uint8 m = smline[sgg];
          if (m == 0) {
            continue;
          }
          if (m == 0xFF) {
            dchunk[0] = schunk[0];
            dchunk[2] = schunk[2];
            dchunk[4] = schunk[4];
            dchunk[6] = schunk[6];
          } else {
            Uint8 nm = ~m;
            dchunk[0] = (dchunk[0] & nm) | (schunk[0] & m);
            dchunk[2] = (dchunk[2] & nm) | (schunk[2] & m);
            dchunk[4] = (dchunk[4] & nm) | (schunk[4] & m);
            dchunk[6] = (dchunk[6] & nm) | (schunk[6] & m);
          }
          if (dmline != NULL) {
            dmline[dgg] |= m;
          }
        } else {
          dchunk[0] = schunk[0];
          dchunk[2] = schunk[2];
          dchunk[4] = schunk[4];
          dchunk[6] = schunk[6];
          if (dmline != NULL) {
            dmline[dgg] = smline != NULL ? smline[sgg] : 0xFF;
          }
        }
      }
    }
  }
  return 0;
}

void SDL_UpdateRect(SDL_Surface * screen, Sint32 x, Sint32 y,
    Uint32 w, Uint32 h)
{
  /* rendering goes straight to screen RAM */
  (void)screen; (void)x; (void)y; (void)w; (void)h;
}

/* ---------------------------------------------------------------- */
/* video init                                                       */

static int nsdl_old_rez = -1;
static Uint16 nsdl_old_palette[16];
static Uint16 nsdl_old_kbrate = 0xFFFF;

static void nsdl_restore(void)
{
  int i;
  if (nsdl_old_rez >= 0) {
    for (i = 0; i < 16; i++) {
      Setcolor(i, nsdl_old_palette[i]);
    }
    Setscreen((void *)-1L, (void *)-1L, nsdl_old_rez);
    nsdl_old_rez = -1;
  }
  if (nsdl_old_kbrate != 0xFFFF) {
    Kbrate((nsdl_old_kbrate >> 8) & 0xFF, nsdl_old_kbrate & 0xFF);
    nsdl_old_kbrate = 0xFFFF;
  }
}

SDL_Surface * SDL_SetVideoMode(int width, int height, int bpp,
    Uint32 flags)
{
  int i;
  (void)width; (void)height; (void)bpp; (void)flags;

  if (nsdl_old_rez < 0) {
    nsdl_old_rez = Getrez();
    for (i = 0; i < 16; i++) {
      nsdl_old_palette[i] = Setcolor(i, -1);
    }
    if (nsdl_old_rez != 0) {
      Setscreen((void *)-1L, (void *)-1L, 0);
    }
    atexit(nsdl_restore);
  }

  nsdl_set_hardware_palette();

  nsdl_screen.w = 320;
  nsdl_screen.h = 200;
  nsdl_screen.pitch = 160;
  nsdl_screen.maskpitch = 0;
  nsdl_screen.pixels = Physbase();
  nsdl_screen.mask = NULL;
  nsdl_screen.usekey = 0;
  nsdl_screen.is_screen = 1;
  nsdl_screen.format = &nsdl_format;
  nsdl_screen.flags = SDL_HWSURFACE | SDL_HWPALETTE | SDL_FULLSCREEN;

  memset(nsdl_screen.pixels, 0, 32000);

  return &nsdl_screen;
}

int SDL_Init(Uint32 flags)
{
  (void)flags;
  nsdl_init_colors();
  nsdl_old_kbrate = (Uint16)Kbrate(-1, -1);
  Kbrate(1, 1);
  return 0;
}

void SDL_Quit(void)
{
  nsdl_restore();
}

char * SDL_GetError(void)
{
  return "nsdl error";
}

/* ---------------------------------------------------------------- */
/* stubs                                                            */

void SDL_WM_SetCaption(const char * title, const char * icon)
{
  (void)title; (void)icon;
}

int SDL_WM_ToggleFullScreen(SDL_Surface * surface)
{
  (void)surface;
  return 1; /* always fullscreen */
}

SDL_TimerID SDL_AddTimer(Uint32 interval,
    SDL_NewTimerCallback callback, void * param)
{
  /* game ticks come from fn_wait_event_tick */
  (void)interval; (void)callback; (void)param;
  return NULL;
}

int SDL_RemoveTimer(SDL_TimerID id)
{
  (void)id;
  return 1;
}
