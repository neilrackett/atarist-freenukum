/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Game tick event source
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

#include "fn_tick.h"
#include "fn.h"

/* --------------------------------------------------------------- */

#define FN_TICK_DURATION 80

/* --------------------------------------------------------------- */

int fn_wait_event_tick(SDL_Event * event)
{
  static Uint32 next_tick = 0;

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
