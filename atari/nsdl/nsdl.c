/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Native Atari ST implementation of the SDL 1.2 subset
 * used by the game ("nsdl")
 */

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

/* The whole game runs in supervisor mode (entered in SDL_Init),
 * which gives direct access to the 200Hz counter, the sound chip
 * and the blitter without trap overhead. */

static Uint32 nsdl_ticks_base = 0;

Uint32 SDL_GetTicks(void)
{
  Uint32 now = (Uint32)(*(volatile long *)0x4BAUL) * 5;
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
/* Mega STE 16MHz + cache                                           */

static int nsdl_old_cpuspeed = -1;
static long nsdl_old_ssp = 0;

static long nsdl_sup_speedup(void)
{
  /* find the _MCH cookie; only the Mega STE has the speed
   * register at $FF8E21, touching it elsewhere bus-errors */
  long * jar = *(long **)0x5A0UL;
  if (jar != NULL) {
    for (; jar[0] != 0; jar += 2) {
      if (jar[0] == 0x5F4D4348L) {          /* '_MCH' */
        if (jar[1] == 0x00010010L) {        /* Mega STE */
          volatile Uint8 * ctl = (volatile Uint8 *)0xFFFF8E21UL;
          Uint8 old = *ctl;
          *ctl = 3;                         /* 16MHz, cache on */
          return old;
        }
        break;
      }
    }
  }
  return -1;
}

static long nsdl_sup_speedrestore(void)
{
  if (nsdl_old_cpuspeed >= 0) {
    *(volatile Uint8 *)0xFFFF8E21UL = (Uint8)nsdl_old_cpuspeed;
    nsdl_old_cpuspeed = -1;
  }
  return 0;
}

/* ---------------------------------------------------------------- */
/* joystick                                                         */
/*
 * The IKBD reports joystick 1 as event packets which the OS routes
 * through the joyvec vector. The handler just stores the state
 * byte: bit 0 up, 1 down, 2 left, 3 right, 7 fire. The event pump
 * turns state changes into key events (fire = Alt, i.e. the fire
 * key), marked with KMOD_JOYSTICK so the game can give joystick-up
 * its own meaning.
 */

volatile Uint8 nsdl_joy_state;
static Uint8 nsdl_joy_prev;
static long nsdl_old_joyvec;

typedef struct {
  long midivec, vkbderr, vmiderr, statvec;
  long mousevec, clockvec, joyvec, midisys, ikbdsys;
} nsdl_kbdvecs_t;

/* called with a0 pointing at the packet: 0xFF for joystick 1,
 * then the state byte */
void nsdl_joyvec_handler(void);
__asm__(
  "\t.text\n"
  "_nsdl_joyvec_handler:\n"
  "\tcmp.b #-1,(%a0)\n"
  "\tbne.s 1f\n"
  "\tmove.b 1(%a0),_nsdl_joy_state\n"
  "1:\trts\n");

static void nsdl_joy_install(void)
{
  nsdl_kbdvecs_t * kv = (nsdl_kbdvecs_t *)Kbdvbase();
  nsdl_old_joyvec = kv->joyvec;
  kv->joyvec = (long)nsdl_joyvec_handler;
  /* make sure the IKBD reports joystick events */
  Ikbdws(0, "\x14");
}

static void nsdl_joy_remove(void)
{
  if (nsdl_old_joyvec != 0) {
    nsdl_kbdvecs_t * kv = (nsdl_kbdvecs_t *)Kbdvbase();
    kv->joyvec = nsdl_old_joyvec;
    nsdl_old_joyvec = 0;
  }
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

static Uint16 nsdl_push_mod_extra = 0;

static void nsdl_push_key(Uint8 type, SDLKey sym)
{
  SDL_Event ev;
  ev.key.type = type;
  ev.key.state = (type == SDL_KEYDOWN);
  ev.key.keysym.scancode = 0;
  ev.key.keysym.sym = sym;
  ev.key.keysym.mod = nsdl_kmod() | nsdl_push_mod_extra;
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

  /* joystick state changes become key events */
  {
    static const struct {
      Uint8 bit;
      SDLKey sym;
    } joymap[] = {
      { 0x01, SDLK_UP },
      { 0x02, SDLK_DOWN },
      { 0x04, SDLK_LEFT },
      { 0x08, SDLK_RIGHT },
      { 0x80, SDLK_LALT },  /* fire */
    };
    Uint8 js = nsdl_joy_state;
    Uint8 jchanged = js ^ nsdl_joy_prev;
    if (jchanged) {
      nsdl_joy_prev = js;
      nsdl_push_mod_extra = KMOD_JOYSTICK;
      for (i = 0; i < 5; i++) {
        if (jchanged & joymap[i].bit) {
          nsdl_push_key((js & joymap[i].bit)
              ? SDL_KEYDOWN : SDL_KEYUP, joymap[i].sym);
        }
      }
      nsdl_push_mod_extra = 0;
    }
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
/* BLiTTER                                                          */
/*
 * The BLiTTER moves one bitplane rectangle per operation. Our
 * surfaces share the screen's word-interleaved layout, so a plane
 * is walked with an x increment of 8 bytes; the separate 1-bit
 * mask is contiguous, and conveniently one mask word covers the
 * same 16 pixels as one plane word. Masked blits take two passes
 * per plane: dst &= ~mask, then dst |= source (transparent source
 * pixels are already zero in the plane data).
 * Detected with Blitmode(-1), so Mega STs count too.
 */

typedef struct {
  Uint16 halftone[16];
  Sint16 src_xinc;
  Sint16 src_yinc;
  Uint32 src_addr;
  Uint16 endmask1;
  Uint16 endmask2;
  Uint16 endmask3;
  Sint16 dst_xinc;
  Sint16 dst_yinc;
  Uint32 dst_addr;
  Uint16 xcount;
  Uint16 ycount;
  Uint8 hop;
  Uint8 op;
  volatile Uint8 ctrl;
  Uint8 skew;
} nsdl_blitregs_t;

#define NSDL_BLIT ((volatile nsdl_blitregs_t *)0xFFFF8A00UL)

static int nsdl_have_blitter = 0;

/* run one plane-rectangle operation and wait for completion */
static void nsdl_blit_go(Uint32 src, Sint16 sxinc, Sint16 syinc,
    Uint32 dst, Sint16 dxinc, Sint16 dyinc,
    Uint16 em1, Uint16 em3, Uint16 nwords, Uint16 nlines,
    Uint8 hop, Uint8 op)
{
  volatile nsdl_blitregs_t * b = NSDL_BLIT;
  b->src_xinc = sxinc;
  b->src_yinc = syinc;
  b->src_addr = src;
  b->endmask1 = (nwords == 1) ? (em1 & em3) : em1;
  b->endmask2 = 0xFFFF;
  b->endmask3 = em3;
  b->dst_xinc = dxinc;
  b->dst_yinc = dyinc;
  b->dst_addr = dst;
  b->xcount = nwords;
  b->ycount = nlines;
  b->hop = hop;
  b->op = op;
  b->skew = 0;
  b->ctrl = 0xC0;                 /* start, hog mode */
  while (b->ctrl & 0x80);
}

/* geometry of a group-aligned rectangle for plane operations */
typedef struct {
  Uint16 w0;        /* first word index in the line */
  Uint16 nwords;
  Uint16 em1, em3;
} nsdl_blitspan_t;

static void nsdl_blit_span(int g0, int ng, nsdl_blitspan_t * sp)
{
  int glast = g0 + ng - 1;
  sp->w0 = g0 >> 1;
  sp->nwords = (glast >> 1) - sp->w0 + 1;
  sp->em1 = (g0 & 1) ? 0x00FF : 0xFFFF;
  sp->em3 = (glast & 1) ? 0xFFFF : 0xFF00;
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
    y -= dst->ybias;
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

    /* big edge-aligned fills go to the blitter */
    if (nsdl_have_blitter && (x & 7) == 0 && ((x + w) & 7) == 0
        && gx1 - gx0 >= 4 && h >= 4) {
      nsdl_blitspan_t sp;
      Uint32 base;
      nsdl_blit_span(gx0, gx1 - gx0, &sp);
      base = (Uint32)dst->pixels + (Uint32)y * dst->pitch
        + ((Uint32)sp.w0 << 3);
      for (p = 0; p < 4; p++) {
        /* HOP 0 = all ones; OP 3 writes them, OP 0 writes zeros */
        nsdl_blit_go(0, 0, 0,
            base + (p << 1),
            8, dst->pitch - ((sp.nwords - 1) << 3),
            sp.em1, sp.em3, sp.nwords, h,
            0, planebyte[p] ? 3 : 0);
      }
      if (dst->mask != NULL) {
        nsdl_blit_go(0, 0, 0,
            (Uint32)dst->mask + (Uint32)y * dst->maskpitch
              + ((Uint32)sp.w0 << 1),
            2, dst->maskpitch - ((sp.nwords - 1) << 1),
            sp.em1, sp.em3, sp.nwords, h,
            0, transparent ? 0 : 3);
      }
      return 0;
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
  sy -= src->ybias;
  dy -= dst->ybias;

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
    dstrect->y = dy + dst->ybias;
    dstrect->w = w;
    dstrect->h = h;
  }

  {
    int sg0 = sx >> 3;
    int dg0 = dx >> 3;
    int ng = (((sx & 7) + w + 7) >> 3);
    int masked = (src->usekey && src->mask != NULL);
    int y;

    /* the blitter beats the CPU once the area is big enough to
     * amortize the register setup; small sprites stay on the CPU */
    if (nsdl_have_blitter && (sg0 & 1) == (dg0 & 1)
        && ng >= (masked ? 8 : 4) && h >= 4) {
      nsdl_blitspan_t ssp, dsp;
      Uint32 sbase, dbase;
      int p;
      nsdl_blit_span(sg0, ng, &ssp);
      nsdl_blit_span(dg0, ng, &dsp);
      sbase = (Uint32)src->pixels + (Uint32)sy * src->pitch
        + ((Uint32)ssp.w0 << 3);
      dbase = (Uint32)dst->pixels + (Uint32)dy * dst->pitch
        + ((Uint32)dsp.w0 << 3);
      if (masked) {
        Uint32 mbase = (Uint32)src->mask
          + (Uint32)sy * src->maskpitch + ((Uint32)ssp.w0 << 1);
        for (p = 0; p < 4; p++) {
          /* dst &= ~mask */
          nsdl_blit_go(mbase,
              2, src->maskpitch - ((ssp.nwords - 1) << 1),
              dbase + (p << 1),
              8, dst->pitch - ((dsp.nwords - 1) << 3),
              dsp.em1, dsp.em3, dsp.nwords, h, 2, 4);
          /* dst |= source plane (transparent bits are zero) */
          nsdl_blit_go(sbase + (p << 1),
              8, src->pitch - ((ssp.nwords - 1) << 3),
              dbase + (p << 1),
              8, dst->pitch - ((dsp.nwords - 1) << 3),
              dsp.em1, dsp.em3, dsp.nwords, h, 2, 7);
        }
        if (dst->mask != NULL) {
          /* dstmask |= srcmask */
          nsdl_blit_go(mbase,
              2, src->maskpitch - ((ssp.nwords - 1) << 1),
              (Uint32)dst->mask + (Uint32)dy * dst->maskpitch
                + ((Uint32)dsp.w0 << 1),
              2, dst->maskpitch - ((dsp.nwords - 1) << 1),
              dsp.em1, dsp.em3, dsp.nwords, h, 2, 7);
        }
      } else {
        for (p = 0; p < 4; p++) {
          nsdl_blit_go(sbase + (p << 1),
              8, src->pitch - ((ssp.nwords - 1) << 3),
              dbase + (p << 1),
              8, dst->pitch - ((dsp.nwords - 1) << 3),
              dsp.em1, dsp.em3, dsp.nwords, h, 2, 3);
        }
        if (dst->mask != NULL) {
          if (src->mask != NULL) {
            nsdl_blit_go((Uint32)src->mask
                  + (Uint32)sy * src->maskpitch
                  + ((Uint32)ssp.w0 << 1),
                2, src->maskpitch - ((ssp.nwords - 1) << 1),
                (Uint32)dst->mask + (Uint32)dy * dst->maskpitch
                  + ((Uint32)dsp.w0 << 1),
                2, dst->maskpitch - ((dsp.nwords - 1) << 1),
                dsp.em1, dsp.em3, dsp.nwords, h, 2, 3);
          } else {
            /* opaque source: set mask bits */
            nsdl_blit_go(0,
                0, 0,
                (Uint32)dst->mask + (Uint32)dy * dst->maskpitch
                  + ((Uint32)dsp.w0 << 1),
                2, dst->maskpitch - ((dsp.nwords - 1) << 1),
                dsp.em1, dsp.em3, dsp.nwords, h, 0, 3);
          }
        }
      }
      return 0;
    }

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
  Cconws("\033e");  /* cursor back on */
  nsdl_joy_remove();
  nsdl_sup_speedrestore();
  if (nsdl_old_ssp != 0) {
    Super((void *)nsdl_old_ssp);
    nsdl_old_ssp = 0;
  }
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

  /* clear/home the console, hide the blinking cursor and greet the
   * player in white while the game data loads (VT52 sequences) */
  Cconws("\033E\033f\033b\017");
  Cconws("\r\nCome get STome!\r\n\r\nLoading...\r\n");

  return &nsdl_screen;
}

int SDL_Init(Uint32 flags)
{
  (void)flags;
  nsdl_init_colors();
  nsdl_old_kbrate = (Uint16)Kbrate(-1, -1);
  Kbrate(1, 1);
  nsdl_joy_install();

  /* run in supervisor mode from here on: direct access to the
   * 200Hz counter, sound chip and blitter without traps */
  nsdl_old_ssp = Super(0L);

  nsdl_old_cpuspeed = (int)nsdl_sup_speedup();

  {
    long bm = Blitmode(-1);
    nsdl_have_blitter = (bm >= 0 && (bm & 2)) ? 1 : 0;
  }
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
