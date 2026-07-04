/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Native Atari ST replacement for the SDL 1.2 subset
 * used by the game ("nsdl")
 */
/*
 * Surfaces are stored in Atari ST 4-bitplane word-interleaved
 * format (the screen's native layout), plus a separate 1-bit
 * opacity mask for colorkeyed surfaces. Pixel values are palette
 * indices 0-15; value 16 is the transparent colorkey.
 *
 * Horizontal blit/fill coordinates are rounded to the 8-pixel
 * grid (1 byte per bitplane). The game positions everything on
 * the half-tile (8px) grid, so this loses nothing in practice.
 *
 */

#ifndef NSDL_SDL_H
#define NSDL_SDL_H

#include <stdint.h>
#include <stdlib.h>

typedef uint8_t  Uint8;
typedef int8_t   Sint8;
typedef uint16_t Uint16;
typedef int16_t  Sint16;
typedef uint32_t Uint32;
typedef int32_t  Sint32;
typedef uint64_t Uint64;
typedef int64_t  Sint64;

/* ---------------------------------------------------------------- */
/* general */

#define SDL_INIT_VIDEO 0x00000020
#define SDL_INIT_TIMER 0x00000001

int SDL_Init(Uint32 flags);
void SDL_Quit(void);
char * SDL_GetError(void);

/* ---------------------------------------------------------------- */
/* video */

typedef struct SDL_Rect {
  Sint16 x, y;
  Uint16 w, h;
} SDL_Rect;

typedef struct SDL_Color {
  Uint8 r;
  Uint8 g;
  Uint8 b;
  Uint8 unused;
} SDL_Color;

typedef struct SDL_Palette {
  int ncolors;
  SDL_Color * colors;
} SDL_Palette;

typedef struct SDL_PixelFormat {
  SDL_Palette * palette;
  Uint8 BitsPerPixel;
  Uint8 BytesPerPixel;
} SDL_PixelFormat;

typedef struct SDL_Surface {
  Uint32 flags;
  SDL_PixelFormat * format;
  int w, h;
  Uint16 pitch;        /* bytes per line of interleaved plane data */
  void * pixels;       /* 4-bitplane word-interleaved data */
  Uint8 * mask;        /* 1 bit per pixel opacity, or NULL */
  Uint16 maskpitch;    /* bytes per line of mask data */
  int usekey;          /* honor mask as colorkey when blitting */
  int is_screen;       /* pixels point at video RAM */
  int ybias;           /* logical y of the surface's first line;
                        * lets a short stripe stand in for a tall
                        * virtual surface (windowed level render) */
} SDL_Surface;

#define SDL_SWSURFACE   0x00000000
#define SDL_HWSURFACE   0x00000001
#define SDL_ANYFORMAT   0x10000000
#define SDL_HWPALETTE   0x20000000
#define SDL_FULLSCREEN  0x80000000
#define SDL_HWACCEL     0x00000100
#define SDL_SRCCOLORKEY 0x00001000

/* pixel value used for transparent pixels */
#define NSDL_KEYCOLOR 16

SDL_Surface * SDL_SetVideoMode(int width, int height, int bpp,
    Uint32 flags);
SDL_Surface * SDL_CreateRGBSurface(Uint32 flags, int width,
    int height, int depth, Uint32 Rmask, Uint32 Gmask, Uint32 Bmask,
    Uint32 Amask);
void SDL_FreeSurface(SDL_Surface * surface);

int SDL_BlitSurface(SDL_Surface * src, SDL_Rect * srcrect,
    SDL_Surface * dst, SDL_Rect * dstrect);
int SDL_FillRect(SDL_Surface * dst, SDL_Rect * dstrect, Uint32 color);
void SDL_UpdateRect(SDL_Surface * screen, Sint32 x, Sint32 y,
    Uint32 w, Uint32 h);

Uint32 SDL_MapRGB(SDL_PixelFormat * format, Uint8 r, Uint8 g, Uint8 b);
void SDL_GetRGB(Uint32 pixel, SDL_PixelFormat * format, Uint8 * r,
    Uint8 * g, Uint8 * b);
int SDL_SetColors(SDL_Surface * surface, SDL_Color * colors,
    int firstcolor, int ncolors);
int SDL_SetColorKey(SDL_Surface * surface, Uint32 flag, Uint32 key);

/* nsdl extension: write one 8px group of a decoded graphic */
void nsdl_put_group(SDL_Surface * s, int x, int y,
    const Uint8 * planes, Uint8 mask);

void SDL_WM_SetCaption(const char * title, const char * icon);
int SDL_WM_ToggleFullScreen(SDL_Surface * surface);

/* ---------------------------------------------------------------- */
/* keys */

typedef enum {
  SDLK_UNKNOWN = 0,
  SDLK_BACKSPACE = 8,
  SDLK_RETURN = 13,
  SDLK_ESCAPE = 27,
  SDLK_SPACE = 32,
  SDLK_EXCLAIM = 33,
  SDLK_QUOTEDBL = 34,
  SDLK_HASH = 35,
  SDLK_DOLLAR = 36,
  SDLK_AMPERSAND = 38,
  SDLK_QUOTE = 39,
  SDLK_LEFTPAREN = 40,
  SDLK_RIGHTPAREN = 41,
  SDLK_ASTERISK = 42,
  SDLK_PLUS = 43,
  SDLK_COMMA = 44,
  SDLK_MINUS = 45,
  SDLK_PERIOD = 46,
  SDLK_SLASH = 47,
  SDLK_0 = 48, SDLK_1, SDLK_2, SDLK_3, SDLK_4,
  SDLK_5, SDLK_6, SDLK_7, SDLK_8, SDLK_9,
  SDLK_COLON = 58,
  SDLK_SEMICOLON = 59,
  SDLK_LESS = 60,
  SDLK_EQUALS = 61,
  SDLK_GREATER = 62,
  SDLK_QUESTION = 63,
  SDLK_AT = 64,
  SDLK_a = 97, SDLK_b, SDLK_c, SDLK_d, SDLK_e, SDLK_f, SDLK_g,
  SDLK_h, SDLK_i, SDLK_j, SDLK_k, SDLK_l, SDLK_m, SDLK_n, SDLK_o,
  SDLK_p, SDLK_q, SDLK_r, SDLK_s, SDLK_t, SDLK_u, SDLK_v, SDLK_w,
  SDLK_x, SDLK_y, SDLK_z,
  SDLK_DELETE = 127,
  SDLK_UP = 273,
  SDLK_DOWN = 274,
  SDLK_RIGHT = 275,
  SDLK_LEFT = 276,
  SDLK_LSHIFT = 304,
  SDLK_RSHIFT = 303,
  SDLK_LCTRL = 306,
  SDLK_RCTRL = 305,
  SDLK_LALT = 308,
  SDLK_RALT = 307,
  SDLK_F1 = 282,
  SDLK_F2, SDLK_F3, SDLK_F4, SDLK_F5, SDLK_F6, SDLK_F7,
  SDLK_F8, SDLK_F9, SDLK_F10,
  SDLK_LAST = 323
} SDLKey;

#define KMOD_NONE   0x0000
#define KMOD_LSHIFT 0x0001
#define KMOD_RSHIFT 0x0002
#define KMOD_SHIFT  0x0003
#define KMOD_LCTRL  0x0040
#define KMOD_RCTRL  0x0080
#define KMOD_CTRL   0x00C0
#define KMOD_LALT   0x0100
#define KMOD_RALT   0x0200
#define KMOD_ALT    0x0300
/* nsdl extension: the event was synthesized from the joystick */
#define KMOD_JOYSTICK 0x8000

typedef struct SDL_keysym {
  Uint8 scancode;
  SDLKey sym;
  Uint16 mod;
  Uint16 unicode;
} SDL_keysym;

/* ---------------------------------------------------------------- */
/* events */

enum {
  SDL_NOEVENT = 0,
  SDL_KEYDOWN = 2,
  SDL_KEYUP = 3,
  SDL_MOUSEMOTION = 4,
  SDL_MOUSEBUTTONDOWN = 5,
  SDL_MOUSEBUTTONUP = 6,
  SDL_QUIT = 12,
  SDL_VIDEOEXPOSE = 17,
  SDL_USEREVENT = 24
};

#define SDL_BUTTON_LEFT   1
#define SDL_BUTTON_MIDDLE 2
#define SDL_BUTTON_RIGHT  3

typedef struct SDL_KeyboardEvent {
  Uint8 type;
  Uint8 state;
  SDL_keysym keysym;
} SDL_KeyboardEvent;

typedef struct SDL_MouseButtonEvent {
  Uint8 type;
  Uint8 button;
  Uint8 state;
  Uint16 x, y;
} SDL_MouseButtonEvent;

typedef struct SDL_MouseMotionEvent {
  Uint8 type;
  Uint8 state;
  Uint16 x, y;
  Sint16 xrel, yrel;
} SDL_MouseMotionEvent;

typedef struct SDL_UserEvent {
  Uint8 type;
  int code;
  void * data1;
  void * data2;
} SDL_UserEvent;

typedef union SDL_Event {
  Uint8 type;
  SDL_KeyboardEvent key;
  SDL_MouseButtonEvent button;
  SDL_MouseMotionEvent motion;
  SDL_UserEvent user;
} SDL_Event;

int SDL_PollEvent(SDL_Event * event);
int SDL_WaitEvent(SDL_Event * event);
int SDL_PushEvent(SDL_Event * event);

#define SDL_DEFAULT_REPEAT_DELAY 500
#define SDL_DEFAULT_REPEAT_INTERVAL 30
int SDL_EnableKeyRepeat(int delay, int interval);

/* ---------------------------------------------------------------- */
/* time and timers */

Uint32 SDL_GetTicks(void);
void SDL_Delay(Uint32 ms);

typedef void * SDL_TimerID;
typedef Uint32 (*SDL_NewTimerCallback)(Uint32 interval, void * param);
SDL_TimerID SDL_AddTimer(Uint32 interval,
    SDL_NewTimerCallback callback, void * param);
int SDL_RemoveTimer(SDL_TimerID id);

#endif /* NSDL_SDL_H */
