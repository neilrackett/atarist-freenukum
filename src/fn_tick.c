/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Game tick event source
 */

#include "fn_tick.h"
#include "fn.h"

/* --------------------------------------------------------------- */

#define FN_TICK_DURATION 80

/* --------------------------------------------------------------- */

static Uint32 next_tick = 0;

/* --------------------------------------------------------------- */

int fn_tick_pending(void)
{
  if (SDL_PollEvent(NULL)) {
    return 1;
  }
  if (next_tick == 0) {
    return 0;
  }
  return (Sint32)(SDL_GetTicks() - next_tick) >= 0;
}

/* --------------------------------------------------------------- */

int fn_wait_event_tick(SDL_Event * event)
{
  while (1) {
    if (SDL_PollEvent(event)) {
      return 1;
    }

    Uint32 now = SDL_GetTicks();
    if (next_tick == 0) {
      next_tick = now + FN_TICK_DURATION;
    }

    if ((Sint32)(now - next_tick) >= 0) {
      next_tick += FN_TICK_DURATION;
      if ((Sint32)(now - next_tick) > 4 * FN_TICK_DURATION) {
        /* Fell far behind (level loading etc.), don't try
         * to catch up with a burst of ticks. */
        next_tick = now + FN_TICK_DURATION;
      }
      event->type = SDL_USEREVENT;
      event->user.code = fn_event_timer;
      event->user.data1 = 0;
      event->user.data2 = 0;
      return 1;
    }

    SDL_Delay(10);
  }
}
