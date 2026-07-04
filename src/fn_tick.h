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

#ifndef FN_TICK_H
#define FN_TICK_H

#include <SDL.h>

/**
 * Wait for the next event, generating fn_event_timer user events
 * every FN_TICK_DURATION milliseconds while waiting.
 *
 * Replacement for SDL_WaitEvent in loops that used to rely on
 * SDL_AddTimer, which never fires on plain TOS (no timer thread).
 *
 * @param event  Where the event gets stored.
 *
 * @return 1 (an event was stored; the function waits until one is
 *         available or a tick becomes due).
 */
int fn_wait_event_tick(SDL_Event * event);

#endif /* FN_TICK_H */
