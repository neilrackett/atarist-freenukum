/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Game tick event source
 */

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

/**
 * Check whether work is already waiting: an event is queued or the
 * next timer tick is due. Lets render loops skip drawing frames
 * while the game logic is behind (frameskip) so the game keeps
 * real-time speed on slow machines.
 *
 * @return 1 if fn_wait_event_tick would return without waiting.
 */
int fn_tick_pending(void);

#endif /* FN_TICK_H */
