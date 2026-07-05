/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Level functions
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

#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/* --------------------------------------------------------------- */

#include "fn_level.h"
#include "fn_sound.h"
#include "fn_actor.h"
#include "fn_hero.h"
#include "fn_object.h"
#include "fn_collision.h"

/* --------------------------------------------------------------- */

fn_level_t * fn_level_load(int fd,
    fn_environment_t * env)
{
  size_t i = 0;
  fn_level_t * lv = malloc(sizeof(fn_level_t));
  memset(lv, 0, sizeof(fn_level_t));
  Uint16 tilenr;
  Uint8 uppertile;
  Uint8 lowertile;

  lv->environment = env;

  lv->animated_frames = 0;

  lv->levelpassed = 0;

  lv->num_shots = 0;

  lv->actors = NULL;
  lv->bots = NULL;
  lv->shots = NULL;
  lv->interactor = NULL;

  lv->prev_valid = 0;
  lv->screen_refresh = 0;
  lv->num_carry_dirty = 0;

  lv->do_play = 1;

  /* The level is composed in a full-width stripe that follows the
   * camera vertically (surface->ybias); a full-level surface would
   * need almost 3MB which does not fit small machines. */
  lv->surface_fixed = NULL;
  lv->surface = fn_environment_create_surface(
      env,
      FN_TILE_WIDTH * FN_LEVEL_WIDTH,
      FN_TILE_HEIGHT * (FN_LEVELWINDOW_HEIGHT + 4));

  /* bulk-read the level map: two 1-byte reads per tile cost a
   * system call each, which alone took seconds per level */
  Uint8 * levelmap = malloc(FN_LEVEL_HEIGHT * FN_LEVEL_WIDTH * 2);
  if (levelmap != NULL) {
    size_t total = FN_LEVEL_HEIGHT * FN_LEVEL_WIDTH * 2;
    size_t got = 0;
    while (got < total) {
      ssize_t n = read(fd, levelmap + got, total - got);
      if (n <= 0) {
        break;
      }
      got += (size_t)n;
    }
  }

  while (i != FN_LEVEL_HEIGHT * FN_LEVEL_WIDTH)
  {
    size_t x = i%FN_LEVEL_WIDTH;
    size_t y = i/FN_LEVEL_WIDTH;

    /* we don't only want to run on big-endian systems,
     * so we load the bytes separately.
     */
    if (levelmap != NULL) {
      lowertile = levelmap[i * 2];
      uppertile = levelmap[i * 2 + 1];
    } else {
      read(fd, &lowertile, 1);
      read(fd, &uppertile, 1);
    }
    tilenr = (uppertile << 8) | lowertile;

    lv->raw[y][x] = tilenr;

    if ((tilenr >= 4) && (tilenr <= 0x2fe0)) {
      lv->tiles[y][x] = tilenr / 0x20;
      lv->solid[y][x] = (tilenr >= 0x1800);
    }

    fn_hero_t * hero = fn_environment_get_hero(env);

    switch(tilenr) {
      case 0x0080: /* written text on black screen */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_TEXT_ON_SCREEN_BACKGROUND, x, y);
        break;
      case 0x0100: /* blue high voltage flash */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_HIGH_VOLTAGE_FLASH_BACKGROUND, x, y);
        break;
      case 0x0180: /* red flash light */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_RED_FLASHLIGHT_BACKGROUND, x, y);
        break;
      case 0x0200: /* blue high voltage flash */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BLUE_FLASHLIGHT_BACKGROUND, x, y);
        break;
      case 0x0280: /* key panel on the wall */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEYPANEL_BACKGROUND, x, y);
        break;
      case 0x0300: /* red rotation light */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_RED_ROTATIONLIGHT_BACKGROUND, x, y);
        break;
      case 0x0380: /* flashing up arrow */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_UPARROW_BACKGROUND, x, y);
        break;
      case 0x0400: /* background blinking blue box */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BLUE_LIGHT_BACKGROUND1, x, y);
        break;
      case 0x0420: /* background blinking blue box */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BLUE_LIGHT_BACKGROUND2, x, y);
        break;
      case 0x0440: /* background blinking blue box */
        fn_level_add_initial_actor(lv,
          FN_ACTOR_BLUE_LIGHT_BACKGROUND3, x, y);
        break;
      case 0x0460: /* background blinking blue box */
        fn_level_add_initial_actor(lv,
          FN_ACTOR_BLUE_LIGHT_BACKGROUND4, x, y);
        break;
      case 0x0480: /* background green poison liquid */
        fn_level_add_initial_actor(lv,
          FN_ACTOR_GREEN_POISON_BACKGROUND, x, y);
        break;
      case 0x0500: /* background lava */
        fn_level_add_initial_actor(lv,
          FN_ACTOR_LAVA_BACKGROUND, x, y);
        break;
      case 0x1800: /* solid wall which can be shot */
        lv->tiles[y][x] = 0x17E0/0x20;
        fn_level_add_initial_actor(lv,
          FN_ACTOR_SHOOTABLE_WALL, x, y);
        break;
      case 0x1C00: /* center conveyor */
        lv->tiles[y][x] = SOLID_BLACK; 
        lv->solid[y][x] = 1;

        /*
        fn_level_add_initial_actor(lv,
          FN_ACTOR_CONVEYOR_RIGHTMOVING_CENTER, x, y);
          */
        break;
      case 0x3000: /* grey box, empty */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
          FN_ACTOR_BOX_GREY_EMPTY, x, y);
        break;
      case 0x3001: /* lift */
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
          FN_ACTOR_LIFT, x, y);
        break;
      case 0x3002: /* left end of left-moving conveyor */
        lv->tiles[y][x] = SOLID_CONVEYORBELT_LEFTEND;
        lv->solid[y][x] = 1;
        break;
      case 0x3003: /* right end of left-moving conveyor */
        lv->tiles[y][x] = SOLID_BLACK; 
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_CONVEYOR_LEFTMOVING_RIGHTEND, x, y);
        break;
      case 0x3004: /* left end of right-moving conveyor */
        lv->tiles[y][x] = SOLID_CONVEYORBELT_LEFTEND;
        lv->solid[y][x] = 1;
        break;
      case 0x3005: /* right end of right-moving conveyor */
        lv->tiles[y][x] = SOLID_BLACK; 
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_CONVEYOR_RIGHTMOVING_RIGHTEND, x, y);
        break;
      case 0x3006: /* grey box with boots inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_BOOTS, x, y);
        break;
      case 0x3007: /* rocket which gets started if shot
                    * and leaves a blue box with a balloon */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_ROCKET, x, y);
        break;
      case 0x3008: /* grey box with clamps inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_CLAMPS, x, y);
        break;
      case 0x3009: /* fire burning to the right */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FIRE_RIGHT, x, y);
        break;
      case 0x300A: /* fire burning to the left */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FIRE_LEFT, x, y);
        break;
      case 0x300b: /* flying techbot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FLYINGBOT, x, y);
        break;
      case 0x300c: /* footbot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->bots = fn_list_append(lv->bots, fn_bot_create(
              FN_BOT_TYPE_FOOTBOT, hero, env,
              x*2, y*2));
        break;
      case 0x300d: /* tankbot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_TANKBOT, x, y);
        break;
      case 0x300e: /* fire wheel bot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FIREWHEELBOT, x, y);
        break;
      case 0x300F: /* grey box with gun inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_GUN, x, y);
        break;
      case 0x3010: /* robot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_ROBOT, x, y);
        break;
      case 0x3011: /* exit door */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_EXITDOOR, x, y-1);
        break;
      case 0x3012: /* grey box with bomb inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_BOMB, x, y);
        break;
      case 0x3013: /* bot consisting of several white-blue balls */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SNAKEBOT, x, y);
        break;
      case 0x3014: /* water mirroring everything that is above */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_WATER, x, y);
        break;
      case 0x3015: /* red box with soda inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_RED_SODA, x, y);
        break;
      case 0x3016: /* crab bot crawling along wall left of him */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_WALLCRAWLERBOT_LEFT, x, y);
        break;
      case 0x3017: /* crab bot crawling along wall right of him */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_WALLCRAWLERBOT_RIGHT, x, y);
        break;
      case 0x3018: /* red box with chicken inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_RED_CHICKEN, x, y);
        break;
      case 0x3019: /* floor that breaks on second jump onto it */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_UNSTABLEFLOOR, x, y);
        break;
      case 0x301a: /* horizontal laser beam which gets deactivated when mill is shot */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_LASERBEAM, x, y);
        break;
      case 0x301b: /* fan wheel mounted on right wall blowing to the left */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FAN_LEFT, x, y);
        break;
      case 0x301c: /* fan wheel mounted on left wall blowing to the right*/
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FAN_RIGHT, x, y);
        break;
      case 0x301d: /* blue box with football insdie */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_FOOTBALL, x, y);
        break;
      case 0x301e: /* blue box with joystick inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_JOYSTICK, x, y);
        break;
      case 0x301f: /* blue box with disk inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_DISK, x, y);
        break;
      case 0x3020: /* grey box with glove inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_GLOVE, x, y);
        break;
      case 0x3021: /* laser beam which is deactivated by access card */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_ACCESS_CARD_DOOR, x, y);
        break;
      case 0x3022: /* helicopter */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_HELICOPTERBOT, x, y);
        break;
      case 0x3023: /* blue box with balloon inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_BALLOON, x, y);
        break;
      case 0x3024: /* camera */
        /*
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_CAMERA, x, y);
        break;
      case 0x3025: /* broken wall background */
        /* take the part from one above */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BROKENWALL_BACKGROUND, x, y);
        break;
      case 0x3026: /* left end of background stone wall */
        /* TODO */
        break;
      case 0x3027: /* right end of background stone wall */
        /* TODO */
        break;
      case 0x3028: /* window inside background stone wall */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_STONEWINDOW_BACKGROUND, x, y);
        break;
      case 0x3029: /* grey box with full life */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_FULL_LIFE, x, y);
        break;
      case 0x302a: /* "ACME" brick that comes falling down */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_ACME, x, y);
        break;
      case 0x302b: /* rotating mill that can kill duke on touch */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_MILL, x, y);
        break;
      case 0x302c: /* single spike standing out of the floor */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SPIKE, x, y);
        break;
      case 0x302d: /* blue box with flag inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_FLAG, x, y);
        break;
      case 0x302e: /* blue box with radio inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_BLUE_RADIO, x, y);
        break;
      case 0x302f: /* teleporter station */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_TELEPORTER1, x, y);
        break;
      case 0x3030: /* teleporter station */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_TELEPORTER2, x, y);
        break;
      case 0x3031: /* jumping mines */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_REDBALL_JUMPING, x, y);
        break;
      case 0x3032: /* we found our hero! */
        fn_hero_enterlevel(hero,
            x * FN_TILE_WIDTH, (y - 1) * FN_TILE_HEIGHT);
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        break;
      case 0x3033: /* grey box with the access card inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_ACCESS_CARD, x, y);
        break;
      case 0x3034: /* slot for access card */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_ACCESS_CARD_SLOT, x, y);
        break;
      case 0x3035: /* slot for glove */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_GLOVE_SLOT, x, y);
        break;
      case 0x3036: /* floor which expands to right by access of glove slot */
        fn_level_set_solid(lv, x, y, 1);
        fn_level_add_initial_actor(lv,
            FN_ACTOR_EXPANDINGFLOOR, x, y);
        break;
      case 0x3037: /* grey box with a D inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_LETTER_D, x, y);
        break;
      case 0x3038: /* grey box with a U inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_LETTER_U, x, y);
        break;
      case 0x3039: /* grey box with a K inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
              FN_ACTOR_BOX_GREY_LETTER_K, x, y);
        break;
      case 0x303a: /* grey box with a E inside */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_BOX_GREY_LETTER_E, x, y);
        break;
      case 0x303b: /* bunny bot */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_RABBITOIDBOT, x, y);
        break;
      case 0x303c: /* fire gnome */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FLAMEGNOMEBOT, x, y);
        break;
      case 0x303d: /* fence with backdrop 1 behind it */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FENCE_BACKGROUND, x, y);
        break;
      case 0x303e: /* window - left part */
        lv->tiles[y][x] = 0;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_WINDOWLEFT_BACKGROUND, x, y);
        break;
      case 0x303f: /* window - right part */
        lv->tiles[y][x] = 0;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_WINDOWRIGHT_BACKGROUND, x, y);
        break;
      case 0x3040: /* the notebook */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_NOTEBOOK, x, y);
        break;
      case 0x3041: /* the surveillance screen */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SURVEILLANCESCREEN, x, y);
        break;
      case 0x3043: /* dr proton -the final opponent */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DRPROTON, x, y);
        break;
      case 0x3044: /* red key */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEY_RED, x, y);
        break;
      case 0x3045: /* green key */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEY_GREEN, x, y);
        break;
      case 0x3046: /* blue key */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEY_BLUE, x, y);
        break;
      case 0x3047: /* pink key */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEY_PINK, x, y);
        break;
      case 0x3048: /* red keyhole */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEYHOLE_RED, x, y);
        break;
      case 0x3049: /* green keyhole */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEYHOLE_GREEN, x, y);
        break;
      case 0x304a: /* blue keyhole */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEYHOLE_BLUE, x, y);
        break;
      case 0x304b: /* pink keyhole */
        fn_level_add_initial_actor(lv,
            FN_ACTOR_KEYHOLE_PINK, x, y);
        break;
      case 0x304c: /* red door */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DOOR_RED, x, y);
        break;
      case 0x304d: /* green door */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DOOR_GREEN, x, y);
        break;
      case 0x304e: /* blue door */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DOOR_BLUE, x, y);
        break;
      case 0x304f: /* pink door */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        lv->solid[y][x] = 1;
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DOOR_PINK, x, y);
        break;
      case 0x3050: /* football on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FOOTBALL, x, y);
        break;
      case 0x3051: /* single chicken on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_CHICKEN_SINGLE, x, y);
        break;
      case 0x3052: /* soda on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SODA, x, y);
        break;
      case 0x3053: /* a disk on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_DISK, x, y);
        break;
      case 0x3054: /* a joystick on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_JOYSTICK, x, y);
        break;
      case 0x3055: /* a flag on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_FLAG, x, y);
        break;
      case 0x3056: /* a radio on its own */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_RADIO, x, y);
        break;
      case 0x3057: /* the red mine lying on the ground */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_REDBALL_LYING, x, y);
        break;
      case 0x3058: /* spikes showing up */
        if (y > 0) {
          lv->tiles[y][x] = lv->tiles[y-1][x];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SPIKES_UP, x, y);
        break;
      case 0x3059: /* spikes showing down */
        if (x > 0) {
          lv->tiles[y][x] = lv->tiles[y][x-1];
        }
        fn_level_add_initial_actor(lv,
            FN_ACTOR_SPIKES_DOWN, x, y);
        break;
      default:
        if (tilenr / 0x20 >= SOLID_END) {
          fprintf(stderr, "Unknown tile 0x%04x at x: %d, y: %d\n",
              tilenr, (int)x, (int)y);
          lv->tiles[y][x] = 2;
        }
        break;
    }

    i++;
  }

  free(levelmap);

  /* Put the correct tile behind the cameras. */
  fn_list_t * cameras =
    fn_level_get_items_of_type(lv,
        FN_ACTOR_CAMERA);
  fn_list_t * iter = NULL;
  for (iter = fn_list_first(cameras);
      iter != fn_list_last(cameras);
      iter = fn_list_next(iter)) {
    fn_actor_t * camera = iter->data;
    Uint16 cameray = camera->position.y / FN_TILE_HEIGHT;
    Uint16 camerax = camera->position.x / FN_TILE_WIDTH;
    lv->tiles[cameray][camerax] =
      lv->tiles[cameray+1][camerax];
  }
  fn_list_free(cameras);

  return lv;
}

/* --------------------------------------------------------------- */

void fn_level_free(fn_level_t * lv)
{
  fn_list_t * iter = NULL;

  for (iter = fn_list_first(lv->bots);
      iter != fn_list_last(lv->bots);
      iter = fn_list_next(iter)) {
    fn_bot_free((fn_bot_t *)iter->data);
  }
  fn_list_free(lv->bots);

  for (iter = fn_list_first(lv->shots);
      iter != fn_list_last(lv->shots);
      iter = fn_list_next(iter)) {
    if (iter->data != NULL) {
      fn_shot_free((fn_shot_t *)iter->data);
    }
  }
  fn_list_free(lv->shots);

  for (iter = fn_list_first(lv->actors);
      iter != fn_list_last(lv->actors);
      iter = fn_list_next(iter)) {
    if (iter->data != NULL) {
      fn_actor_free((fn_actor_t *)iter->data);
      iter->data = NULL;
    }
  }
  fn_list_free(lv->actors);

  SDL_FreeSurface(lv->surface);
  SDL_FreeSurface(lv->surface_fixed);

  free(lv);
}

/* --------------------------------------------------------------- */

void fn_level_set_tile(
    fn_level_t * lv,
    size_t x,
    size_t y,
    Uint16 tile)
{
  if (x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return;
  }
  lv->tiles[y][x] = tile;
}

/* --------------------------------------------------------------- */

Uint16 fn_level_get_tile(fn_level_t * lv, size_t x, size_t y)
{
  if (x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return 0;
  }
  return lv->tiles[y][x];
}

/* --------------------------------------------------------------- */

Uint16 fn_level_get_raw(fn_level_t * lv, size_t x, size_t y)
{
  return lv->raw[y][x];
}

/* --------------------------------------------------------------- */

Uint8 fn_level_is_solid(fn_level_t * lv, int x, int y)
{
  if (x < 0 || y < 0 || x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return 1;
  }
  return lv->solid[y][x];
}

/* --------------------------------------------------------------- */

void fn_level_set_solid(fn_level_t * lv, int x, int y, Uint8 solid)
{
  if (x < 0 || y < 0 || x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return;
  }
  lv->solid[y][x] = solid;
}

/* --------------------------------------------------------------- */

#ifdef FN_ATARI_PROFILE
extern volatile Uint32 fn_prof[12];
#define FN_HZ200 (*(volatile Uint32 *)0x4baUL)
#endif

/* Does r overlap any of the collected dirty rects? Used to decide
 * whether a never-changing decoration actor must be redrawn. */
static int fn_level_rect_dirty(const SDL_Rect * r,
    const SDL_Rect * dirty, int num_dirty)
{
  int i;
  for (i = 0; i < num_dirty; i++) {
    if (dirty[i].w != 0
        && r->x < dirty[i].x + dirty[i].w
        && r->x + r->w > dirty[i].x
        && r->y < dirty[i].y + dirty[i].h
        && r->y + r->h > dirty[i].y) {
      return 1;
    }
  }
  return 0;
}

/* Compose backdrop + static tiles for the cells under rect d.
 * Backdrop only gets painted where it can show through, and runs
 * of see-through cells are batched into one blit per row. With
 * skip_opaque set, cells whose tile is fully opaque are left
 * entirely untouched - used after horizontal scrolls, where those
 * cells are still valid in the level-anchored stripe and only
 * cells showing the window-anchored backdrop need repainting. */
static void fn_level_compose_cells(fn_level_t * lv,
    fn_environment_t * env,
    SDL_Surface * backdrop1,
    const SDL_Rect * sourcerect,
    Uint16 tilesize,
    SDL_Rect d,
    int skip_opaque)
{
  int tx, ty;
  int tx0, tx1;
  if (d.w == 0) {
    return;
  }
  tx0 = d.x / tilesize;
  tx1 = (d.x + d.w - 1) / tilesize;
  if (tx1 >= FN_LEVEL_WIDTH) {
    tx1 = FN_LEVEL_WIDTH - 1;
  }
  for (ty = d.y / tilesize;
      ty <= (d.y + d.h - 1) / tilesize
      && ty < FN_LEVEL_HEIGHT; ty++) {
    int run_start = -1;
    for (tx = tx0; tx <= tx1 + 1; tx++) {
      SDL_Surface * tile = NULL;
      int opaque = 0;
      if (tx <= tx1) {
        Uint16 tilenr = fn_level_get_tile(lv, tx, ty);
        if (tilenr > 1 && tilenr < (48 * 8)) {
          tile = fn_environment_get_tile(env, tilenr);
          opaque = nsdl_surface_is_opaque(tile);
        }
      }
      /* backdrop shows through where there is no tile or the
       * tile has transparent pixels */
      if (tx <= tx1 && !opaque && run_start < 0) {
        run_start = tx;
      }
      /* flush the pending backdrop run when the see-through
       * stretch ends, or up to and including this cell when a
       * transparent tile needs backdrop beneath it now */
      if (run_start >= 0 && (tx > tx1 || opaque || tile != NULL)) {
        int run_end = (tx > tx1 || opaque) ? tx - 1 : tx;
        SDL_Rect bdst;
        bdst.x = run_start * tilesize;
        bdst.y = ty * tilesize;
        bdst.w = (run_end + 1 - run_start) * tilesize;
        bdst.h = tilesize;
        if (backdrop1 != NULL) {
          SDL_Rect bsrc;
          bsrc.x = bdst.x - sourcerect->x;
          bsrc.y = bdst.y - sourcerect->y;
          bsrc.w = bdst.w;
          bsrc.h = bdst.h;
          SDL_BlitSurface(backdrop1, &bsrc, lv->surface, &bdst);
        } else {
          SDL_FillRect(lv->surface, &bdst, 0);
        }
        run_start = -1;
      }
      if (tile != NULL && !(skip_opaque && opaque)) {
        SDL_Rect tr;
        tr.x = tx * tilesize;
        tr.y = ty * tilesize;
        tr.w = tilesize;
        tr.h = tilesize;
        SDL_BlitSurface(tile, NULL, lv->surface, &tr);
      }
    }
  }
}

void fn_level_blit_to_surface(fn_level_t * lv,
    SDL_Surface * target,
    SDL_Rect * targetrect,
    SDL_Rect * sourcerect,
    SDL_Surface * backdrop1,
    SDL_Surface * backdrop2)
{
  int x_start = 0;
  int x_end = FN_LEVEL_WIDTH;
  int y_start = 0;
  int y_end = FN_LEVEL_HEIGHT;
  fn_list_t * iter = NULL;

  /* dirty rectangle bookkeeping: only the parts of the window that
   * changed get recomposed and blitted. A camera move invalidates
   * everything because the backdrop is anchored to the window. */
#define FN_LEVEL_MAXDIRTY 64
  SDL_Rect dirty[FN_LEVEL_MAXDIRTY];
  int num_dirty = 0;
  int full_compose;
  int full_blit;
  int hscroll = 0;
  int i;

  fn_environment_t * env = fn_level_get_environment(lv);
  Uint8 pixelsize = fn_environment_get_pixelsize(env);
  Uint16 tilesize = FN_TILE_WIDTH * pixelsize;
  fn_hero_t * hero = fn_environment_get_hero(env);
  SDL_Rect herorect;

  if (sourcerect == NULL) {
    return;
  }

  /* let the stripe follow the camera */
  lv->surface->ybias =
    sourcerect->y - 2 * FN_TILE_HEIGHT * pixelsize;

  full_compose = !lv->prev_valid
    || sourcerect->y != lv->prev_sourcerect.y;
#ifdef FN_ATARI_PROFILE
  if (!lv->prev_valid) fn_prof[10]++;
  else if (sourcerect->y != lv->prev_sourcerect.y) fn_prof[11]++;
#endif
  if (!full_compose && sourcerect->x != lv->prev_sourcerect.x) {
    Sint32 dx = sourcerect->x - lv->prev_sourcerect.x;
    if (dx >= -2 * (Sint32)tilesize && dx <= 2 * (Sint32)tilesize) {
      /* small horizontal step: most composed cells stay valid */
      hscroll = 1;
    } else {
      full_compose = 1;
    }
  }
  full_blit = full_compose || hscroll || lv->screen_refresh;

  /* fn_hero_blit paints a 2x2 tile grid stepped by w and h, so
   * the drawn area is twice the position size in each direction */
  herorect.x = pixelsize *
    (fn_hero_get_x(hero) - FN_HALFTILE_WIDTH);
  herorect.y = pixelsize * fn_hero_get_y(hero);
  herorect.w = pixelsize * fn_hero_get_w(hero) * 2;
  herorect.h = pixelsize * fn_hero_get_h(hero) * 2;

  /* visibility bounds in tiles (with margin) for the actor loops */
  x_start = sourcerect->x / tilesize - 2;
  if (x_start < 0) {
    x_start = 0;
  }
  x_end = (sourcerect->x + sourcerect->w) / tilesize + 3;
  if (x_end > FN_LEVEL_WIDTH) {
    x_end = FN_LEVEL_WIDTH;
  }
  y_start = sourcerect->y / tilesize - 2;
  if (y_start < 0) {
    y_start = 0;
  }
  y_end = (sourcerect->y + sourcerect->h) / tilesize + 3;
  if (y_end > FN_LEVEL_HEIGHT) {
    y_end = FN_LEVEL_HEIGHT;
  }

  if (full_compose) {
    dirty[0] = *sourcerect;
    num_dirty = 1;
  } else {
    /* collect the rects of everything that may have changed */
    num_dirty = 0;

    if (hscroll) {
      /* the column of cells that just scrolled into the window */
      SDL_Rect strip;
      Sint32 dx = sourcerect->x - lv->prev_sourcerect.x;
      strip.y = sourcerect->y;
      strip.h = sourcerect->h;
      if (dx > 0) {
        strip.x = sourcerect->x + sourcerect->w - dx;
        strip.w = dx;
      } else {
        strip.x = sourcerect->x;
        strip.w = -dx;
      }
      dirty[num_dirty++] = strip;
    }

    for (i = 0; i < lv->num_carry_dirty
        && num_dirty < FN_LEVEL_MAXDIRTY; i++) {
      /* carried rects are lastdrawn rects of removed actors and
       * shots; pad like the live ones (blits can overshoot) */
      SDL_Rect pad = lv->carry_dirty[i];
      pad.w += FN_TILE_WIDTH * pixelsize;
      pad.h += FN_TILE_HEIGHT * pixelsize;
      dirty[num_dirty++] = pad;
    }

    if (num_dirty + 2 <= FN_LEVEL_MAXDIRTY) {
      dirty[num_dirty++] = lv->prev_herorect;
      dirty[num_dirty++] = herorect;
    } else {
      full_compose = 1;
    }

    for (iter = fn_list_first(lv->actors);
        !full_compose && iter != NULL;
        iter = fn_list_next(iter)) {
      fn_actor_t * actor = (fn_actor_t *)iter->data;
      if (actor != NULL && actor->is_visible) {
        /* most actors just animate in place - one rect covers
         * both the erase and the redraw then, which keeps the
         * dirty list from overflowing into a full recompose */
        int moved = actor->lastdrawn.x != actor->position.x
          || actor->lastdrawn.y != actor->position.y
          || actor->lastdrawn.w != actor->position.w
          || actor->lastdrawn.h != actor->position.h;
        if (actor->appearance_static && !moved) {
          /* never changes; nothing to erase or redraw */
          continue;
        }
        if (num_dirty + 1 + moved <= FN_LEVEL_MAXDIRTY) {
          /* some blit functions paint up to a tile beyond their
           * position rect (multi-tile sprites anchored at the
           * top left), so cover that too */
          SDL_Rect pad = actor->lastdrawn;
          pad.w += FN_TILE_WIDTH * pixelsize;
          pad.h += FN_TILE_HEIGHT * pixelsize;
          dirty[num_dirty++] = pad;
          if (moved) {
            pad = actor->position;
            pad.w += FN_TILE_WIDTH * pixelsize;
            pad.h += FN_TILE_HEIGHT * pixelsize;
            dirty[num_dirty++] = pad;
          }
        } else {
          full_compose = 1;
        }
      }
    }

    for (iter = fn_list_first(lv->shots);
        !full_compose && iter != NULL;
        iter = fn_list_next(iter)) {
      fn_shot_t * shot = (fn_shot_t *)iter->data;
      if (shot != NULL) {
        if (num_dirty + 2 <= FN_LEVEL_MAXDIRTY) {
          SDL_Rect pad = shot->lastdrawn;
          pad.w += FN_TILE_WIDTH * pixelsize;
          pad.h += FN_TILE_HEIGHT * pixelsize;
          dirty[num_dirty++] = pad;
          pad = shot->position;
          pad.w += FN_TILE_WIDTH * pixelsize;
          pad.h += FN_TILE_HEIGHT * pixelsize;
          dirty[num_dirty++] = pad;
        } else {
          full_compose = 1;
        }
      }
    }

    for (iter = fn_list_first(lv->bots);
        !full_compose && iter != NULL;
        iter = fn_list_next(iter)) {
      fn_bot_t * bot = (fn_bot_t *)iter->data;
      if (bot != NULL) {
        if (num_dirty + 2 <= FN_LEVEL_MAXDIRTY) {
          /* bots draw up to 2x2 tiles, one tile above their
           * anchor (footbot), so cover that whole area */
          SDL_Rect botrect;
          botrect.x = fn_bot_get_x(bot) * pixelsize
            * FN_HALFTILE_WIDTH;
          botrect.y = (fn_bot_get_y(bot) - 2) * pixelsize
            * FN_HALFTILE_HEIGHT;
          botrect.w = 2 * FN_TILE_WIDTH * pixelsize;
          botrect.h = 2 * FN_TILE_HEIGHT * pixelsize;
          dirty[num_dirty++] = bot->lastdrawn;
          dirty[num_dirty++] = botrect;
        } else {
          full_compose = 1;
        }
      }
    }

    if (full_compose) {
      dirty[0] = *sourcerect;
      num_dirty = 1;
      full_blit = 1;
    }
  }
  lv->num_carry_dirty = 0;

  /* align the dirty rects to the tile grid and clip them to the
   * window; empty ones get zero size */
  for (i = 0; i < num_dirty; i++) {
    Sint16 rx0 = dirty[i].x;
    Sint16 ry0 = dirty[i].y;
    Sint32 rx1 = rx0 + dirty[i].w;
    Sint32 ry1 = ry0 + dirty[i].h;
    if (rx0 < sourcerect->x) rx0 = sourcerect->x;
    if (ry0 < sourcerect->y) ry0 = sourcerect->y;
    if (rx1 > sourcerect->x + sourcerect->w)
      rx1 = sourcerect->x + sourcerect->w;
    if (ry1 > sourcerect->y + sourcerect->h)
      ry1 = sourcerect->y + sourcerect->h;
    if (rx1 <= rx0 || ry1 <= ry0) {
      dirty[i].w = 0;
      dirty[i].h = 0;
      continue;
    }
    /* align outward to whole tiles */
    rx0 -= rx0 % tilesize;
    ry0 -= ry0 % tilesize;
    if (rx1 % tilesize) rx1 += tilesize - rx1 % tilesize;
    if (ry1 % tilesize) ry1 += tilesize - ry1 % tilesize;
    dirty[i].x = rx0;
    dirty[i].y = ry0;
    dirty[i].w = rx1 - rx0;
    dirty[i].h = ry1 - ry0;
  }

#ifdef FN_ATARI_PROFILE
  Uint32 prof_t0 = FN_HZ200;
  Uint32 prof_t1, prof_t2;
  if (full_compose) fn_prof[8]++;
  if (hscroll) fn_prof[9]++;
#endif

  /* recompose the base (backdrop + static tiles) under each rect.
   * Backdrop only gets painted where it can show through: cells
   * whose tile is fully opaque skip it (most of a level is solid
   * tiles, so this saves the bulk of the compose work). Runs of
   * see-through cells are batched into one backdrop blit per row. */
  for (i = 0; i < num_dirty; i++) {
    fn_level_compose_cells(lv, env, backdrop1, sourcerect,
        tilesize, dirty[i], 0);
  }

  /* after a horizontal scroll the level-anchored stripe still
   * holds valid cells; only the ones showing the window-anchored
   * backdrop need repainting (plus the fresh column, which is in
   * the dirty list) */
  if (hscroll && !full_compose) {
    fn_level_compose_cells(lv, env, backdrop1, sourcerect,
        tilesize, *sourcerect, 1);
  }

#ifdef FN_ATARI_PROFILE
  prof_t1 = FN_HZ200;
  fn_prof[5] += prof_t1 - prof_t0;
#endif

  /* blit the actors: a single pass with direct field access -
   * this loop visits every actor in the level, so per-actor call
   * overhead adds up. The (rare) foreground actors are stashed
   * and drawn after the hero to keep the original layering. */
  {
    fn_actor_t * foreground[24];
    int num_foreground = 0;

    for (iter = fn_list_first(lv->actors);
        iter != NULL;
        iter = fn_list_next(iter)) {
      fn_actor_t * actor = (fn_actor_t *)iter->data;

      if (actor != NULL) {
        Uint16 xl = (Uint16)actor->position.x / FN_TILE_WIDTH;
        Uint16 yt = (Uint16)actor->position.y / FN_TILE_HEIGHT;
        Uint16 xr = xl + actor->position.w / FN_TILE_WIDTH;
        Uint16 yb = yt + actor->position.h / FN_TILE_HEIGHT;

        if (xr > x_start && yb > y_start
            && xl < x_end && yt < y_end) {
          actor->is_visible = 1;
          /* a never-changing decoration only needs redrawing
           * when something repainted the area underneath */
          if (actor->appearance_static
              && !full_compose && !hscroll
              && actor->lastdrawn.x == actor->position.x
              && actor->lastdrawn.y == actor->position.y
              && actor->lastdrawn.w == actor->position.w
              && actor->lastdrawn.h == actor->position.h
              && !fn_level_rect_dirty(&actor->position,
                  dirty, num_dirty)) {
            /* untouched since last frame */
          } else if (actor->is_in_foreground
              && num_foreground < 24) {
            foreground[num_foreground++] = actor;
            actor->lastdrawn = actor->position;
          } else {
            fn_actor_blit(actor);
            actor->lastdrawn = actor->position;
          }
        } else {
          actor->is_visible = 0;
        }
      }
    }

    /* blit the hero */
    fn_hero_blit(hero,
        lv->surface,
        lv);
    lv->prev_herorect = herorect;

    /* the foreground actors go over the hero */
    for (i = 0; i < num_foreground; i++) {
      fn_actor_blit(foreground[i]);
    }
  }

  /* blit the bots */
  for (iter = fn_list_first(lv->bots);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_bot_t * bot = (fn_bot_t *)iter->data;
    int x = fn_bot_get_x(bot) / 2;
    int y = fn_bot_get_y(bot) / 2;
    if (x > x_start && y > y_start && x < x_end && y < y_end) {
      fn_bot_blit(bot, lv->surface);
    }
    bot->lastdrawn.x = fn_bot_get_x(bot) * pixelsize
      * FN_HALFTILE_WIDTH;
    bot->lastdrawn.y = (fn_bot_get_y(bot) - 2) * pixelsize
      * FN_HALFTILE_HEIGHT;
    bot->lastdrawn.w = 2 * FN_TILE_WIDTH * pixelsize;
    bot->lastdrawn.h = 2 * FN_TILE_HEIGHT * pixelsize;
  }

  /* blit the shots */
  for (iter = fn_list_first(lv->shots);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_shot_t * shot = (fn_shot_t *)iter->data;

    if (shot != NULL) {
      Uint16 x = fn_shot_get_x(shot) / FN_TILE_WIDTH;
      Uint16 y = fn_shot_get_y(shot) / FN_TILE_HEIGHT;
      if (x > x_start && y > y_start && x < x_end && y < y_end) {
        fn_shot_blit(shot);
      } else {
        fn_shot_gets_out_of_sight(shot);
      }
      shot->lastdrawn = shot->position;
    }
  }

#ifdef FN_ATARI_PROFILE
  prof_t2 = FN_HZ200;
  fn_prof[6] += prof_t2 - prof_t1;
#endif

  /* bring the composed image to the screen; the composed stripe
   * is fully opaque, so skip the colorkey for a faster blit */
  lv->surface->usekey = 0;
  if (full_blit) {
    SDL_Rect src = *sourcerect;
    SDL_Rect dst = *targetrect;
    SDL_BlitSurface(lv->surface, &src, target, &dst);
  } else {
    for (i = 0; i < num_dirty; i++) {
      SDL_Rect src = dirty[i];
      SDL_Rect dst;
      Sint32 sx1, sy1;
      if (src.w == 0) {
        continue;
      }
      /* tile alignment may overhang the window; clamp so the
       * screen blit cannot touch the HUD around it */
      sx1 = src.x + src.w;
      sy1 = src.y + src.h;
      if (src.x < sourcerect->x) src.x = sourcerect->x;
      if (src.y < sourcerect->y) src.y = sourcerect->y;
      if (sx1 > sourcerect->x + sourcerect->w)
        sx1 = sourcerect->x + sourcerect->w;
      if (sy1 > sourcerect->y + sourcerect->h)
        sy1 = sourcerect->y + sourcerect->h;
      if (sx1 <= src.x || sy1 <= src.y) {
        continue;
      }
      src.w = sx1 - src.x;
      src.h = sy1 - src.y;
      dst.x = targetrect->x + (src.x - sourcerect->x);
      dst.y = targetrect->y + (src.y - sourcerect->y);
      dst.w = src.w;
      dst.h = src.h;
      SDL_BlitSurface(lv->surface, &src, target, &dst);
    }
  }

  lv->surface->usekey = 1;

#ifdef FN_ATARI_PROFILE
  fn_prof[7] += FN_HZ200 - prof_t2;
#endif

  lv->prev_sourcerect = *sourcerect;
  lv->prev_valid = 1;
  lv->screen_refresh = 0;
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_level_get_surface(fn_level_t * lv)
{
  return lv->surface;
}

/* --------------------------------------------------------------- */

/* TODO this is deprecated! remove it. */
fn_tilecache_t * fn_level_get_tilecache(fn_level_t * lv)
{
  return fn_environment_get_tilecache(lv->environment);
}

/* --------------------------------------------------------------- */

/* TODO this is deprecated! remove it. */
Uint8 fn_level_get_pixelsize(fn_level_t * lv)
{
  return fn_environment_get_pixelsize(lv->environment);
}

/* --------------------------------------------------------------- */

void fn_level_request_refresh(fn_level_t * lv)
{
  lv->screen_refresh = 1;
}

/* --------------------------------------------------------------- */

int fn_level_keep_on_playing(fn_level_t * lv) {
  return lv->do_play;
}

/* --------------------------------------------------------------- */

fn_hero_t * fn_level_get_hero(fn_level_t * lv) {
  return fn_environment_get_hero(lv->environment);
}

/* --------------------------------------------------------------- */

int fn_level_act(fn_level_t * lv) {
  fn_list_t * iter = NULL;
  int res = 0;
  int cleanup = 0;

  fn_hero_t * hero = fn_environment_get_hero(lv->environment);

  lv->animated_frames ++;
  lv->animated_frames %= 1;
  if (lv->animated_frames == 0) {
    /* do some action, not just animation */
    fn_hero_act(hero, lv);
  }

  for (iter = fn_list_first(lv->shots);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_shot_t * shot = (fn_shot_t *)iter->data;

    if (shot != NULL) {
      res = fn_shot_act(shot);
      if (res == 0) {
        /* set the cleanup flag and free the memory */
        cleanup = 1;
        iter->data = 0;
        if (lv->num_carry_dirty < FN_LEVEL_MAXCARRY) {
          lv->carry_dirty[lv->num_carry_dirty++] = shot->lastdrawn;
        } else {
          lv->prev_valid = 0;
        }
        fn_shot_free(shot); shot = NULL;
        lv->num_shots--;
      }
    }
  }

  if (cleanup) {
    /* clean up the shots that are finished */
    cleanup = 0;
    lv->shots = fn_list_remove_all(lv->shots, NULL);
  }

  int sum = 0;

  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_actor_t * actor = (fn_actor_t *)iter->data;

    if  (actor->acts_while_invisible || actor->is_visible) {
      sum++;
      res = fn_actor_act(actor);
      if (res == 0) {
        /* set the cleanup flag and free the memory */
        cleanup = 1;
        iter->data = NULL;
        if (lv->num_carry_dirty < FN_LEVEL_MAXCARRY) {
          lv->carry_dirty[lv->num_carry_dirty++] = actor->lastdrawn;
        } else {
          lv->prev_valid = 0;
        }
        fn_actor_free(actor); actor = NULL;
      }
    }
  }

  if (cleanup) {
    /* clean up the actors that are finished */
    cleanup = 0;
    lv->actors = fn_list_remove_all(lv->actors, NULL);
  }

  fn_hero_next_animationframe(hero);
  fn_hero_update_animation(hero);

  return 1;
};

/* --------------------------------------------------------------- */

void fn_level_hero_interact_stop(fn_level_t * lv)
{
  if (lv->interactor != NULL) {
    fn_actor_hero_interact_stop(lv->interactor);
  }
  lv->interactor = NULL;
}

/* --------------------------------------------------------------- */

int fn_level_hero_interact_start(fn_level_t * lv)
{
  fn_list_t * iter = NULL;
  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_actor_t * actor = (fn_actor_t *)iter->data;

    if (fn_actor_hero_can_interact(actor)) {

      fn_hero_t * hero = fn_level_get_hero(lv);
      SDL_Rect * heropos = fn_hero_get_position(hero);

      if (fn_collision_touch_rect_rect(heropos,
            fn_actor_get_position(actor)))
      {
        fn_level_hero_interact_stop(lv);

        lv->interactor = actor;
        fn_actor_hero_interact_start(actor);
        return 1;
      }
    }
  }
  return 0;
}

/* --------------------------------------------------------------- */

fn_actor_t * fn_level_add_actor(fn_level_t * lv,
    fn_actor_type_e type,
    Uint16 x,
    Uint16 y)
{
  fn_actor_t * actor = fn_actor_create(lv, type, x, y);
  lv->actors = fn_list_append(lv->actors, actor);

  return actor;
}

/* --------------------------------------------------------------- */

fn_actor_t * fn_level_add_initial_actor(fn_level_t * lv,
    fn_actor_type_e type,
    Uint16 x,
    Uint16 y)
{
  fn_actor_t * actor = fn_level_add_actor(
      lv,
      type,
      x * FN_TILE_WIDTH,
      y * FN_TILE_HEIGHT);
  return actor;
}

/* --------------------------------------------------------------- */

fn_shot_t * fn_level_add_shot(fn_level_t * lv,
    fn_horizontal_direction_e direction,
    Uint16 x,
    Uint16 y)
{
  fn_shot_t * shot = fn_shot_create(lv,
      x, y, direction);

  int addition = (direction == fn_horizontal_direction_right ?
      1 : -1);

  lv->shots = fn_list_append(lv->shots, shot);

  fn_shot_push(shot, addition * FN_HALFTILE_WIDTH);

  Uint8 draw_collision_bounds =
    fn_environment_get_draw_collision_bounds(lv->environment);

  fn_shot_set_draw_collision_bounds(shot,
      draw_collision_bounds);
  return shot;
}

/* --------------------------------------------------------------- */

void fn_level_add_particle_firework(fn_level_t * lv,
    Uint16 x, Uint16 y, Uint8 num_particles)
{
  Uint8 i;
  for (i = 0; i < num_particles; i++) {
    switch(i % 4) {
      case 0:
        fn_level_add_actor(lv, FN_ACTOR_PARTICLE_PINK,
            x, y);
        break;
      case 1:
        fn_level_add_actor(lv, FN_ACTOR_PARTICLE_BLUE,
            x, y);
        break;
      case 2:
        fn_level_add_actor(lv, FN_ACTOR_PARTICLE_WHITE,
            x, y);
        break;
      case 3:
        fn_level_add_actor(lv, FN_ACTOR_PARTICLE_GREEN,
            x, y);
        break;
      default:
        /* Do nothing, cannot be entered. */
        break;
    }
  }
}

/* --------------------------------------------------------------- */

void fn_level_fire_shot(fn_level_t * lv)
{
  fn_hero_t * hero = fn_level_get_hero(lv);

  if (lv->num_shots < fn_hero_get_firepower(hero)) {
    SDL_Rect * position = fn_hero_get_position(hero);

    fn_level_add_shot(lv, hero->direction, position->x, position->y);
    lv->num_shots++;
    fn_sound_play(FN_SOUND_PLAYERGUN);
  }
}

/* --------------------------------------------------------------- */

fn_list_t * fn_level_get_items_of_type(fn_level_t * lv,
    fn_actor_type_e type)
{
  fn_list_t * ret = NULL;
  fn_list_t * iter = NULL;
  for (iter = fn_list_first(lv->actors);
      iter != fn_list_last(lv->actors);
      iter = fn_list_next(iter)) {
    fn_actor_t * actor = iter->data;
    if (actor->type == type) {
      ret = fn_list_append(ret, actor);
    }
  }
  return ret;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_solid_collides(fn_level_t * lv,
    SDL_Rect * rect)
{
  Uint16 i = 0;
  Uint16 j = 0;
  SDL_Rect solidrect;
  solidrect.w = FN_TILE_WIDTH;
  solidrect.h = FN_TILE_HEIGHT;

  for (i = rect->x / FN_TILE_WIDTH;
      i < (rect->x + rect->w) / FN_TILE_WIDTH + 1;
      i++) {
    for (j = rect->y / FN_TILE_WIDTH;
        j < (rect->y + rect->h) / FN_TILE_HEIGHT + 1;
        j++)
    {
      if (fn_level_is_solid(lv, i, j)) {
        solidrect.x = i * FN_TILE_WIDTH;
        solidrect.y = j * FN_TILE_HEIGHT;
        if (fn_collision_overlap_rect_rect(&solidrect, rect)) {
          return 1;
        }
      }
    }
  }
  return 0;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_stands_on_solid_ground_completely(fn_level_t * lv,
    SDL_Rect * rect)
{
  if ((rect->y + rect->h) % FN_TILE_HEIGHT) {
    return 0;
  }
  Uint16 i = 0;
  Uint16 j = (rect->y + rect->h) / FN_TILE_HEIGHT;
  for (i = rect->x / FN_TILE_WIDTH;
      i < (rect->x + rect->w - 1) / FN_TILE_WIDTH + 1;
      i++)
  {
    if (!fn_level_is_solid(lv, i, j)) {
      return 0;
    }
  }
  return 1;
}

/* --------------------------------------------------------------- */


Uint8 fn_level_stands_on_solid_ground_partially(fn_level_t * lv,
    SDL_Rect * rect)
{
  if ((rect->y + rect->h) % FN_TILE_HEIGHT) {
    return 0;
  }
  Uint16 i = 0;
  Uint16 j = (rect->y + rect->h) / FN_TILE_HEIGHT;
  for (i = rect->x / FN_TILE_WIDTH;
      i < (rect->x + rect->w - 1) / FN_TILE_WIDTH + 1;
      i++)
  {
    if (fn_level_is_solid(lv, i, j)) {
      return 1;
    }
  }
  return 0;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_push_rect_standing_on_solid_ground(
    fn_level_t * level, SDL_Rect * rect, Sint8 offset,
    Uint8 gravity)
{
  if (fn_level_solid_collides(level, rect)) {
    /* locked in, so don't move at all */
    return 0;
  }

  /* fall down as far as possible */
  fn_level_rect_fall_down(level, rect, gravity);

  /* check if we stand on solid ground before movement */
  Uint8 stood_solid = fn_level_stands_on_solid_ground_completely(
      level, rect);

  rect->x += offset;

  if (fn_level_solid_collides(level, rect)) {
    /* we collide with something, so we revert to original position */
    rect->x -= offset;
    return 0;
  }

  if (stood_solid && fn_level_stands_on_solid_ground_completely(
        level, rect)) {
    /* we stood on solid ground before, and still do. */
    return 1;
  } else if (stood_solid) {
    /* we stood on solid ground before, but do no longer now. */
    rect->x -= offset;
    return 0;
  } else {
    /* we walk on partial solid ground as long as possible. */
    return 1;
  }
}

/* --------------------------------------------------------------- */

Uint8 fn_level_rect_fall_down(
    fn_level_t * level, SDL_Rect * rect, Uint8 dist)
{
  if (fn_level_solid_collides(level, rect)) {
    /* can't fall down because collides with solid ground */
    return 0;
  }
  if (fn_level_stands_on_solid_ground_partially(level, rect)) {
    /* stands on solid ground so can't fall down */
    return 0;
  }
  Uint8 i = 0;
  while (i < dist) {
    /* check how far we can fall down */
    rect->y++;
    if (fn_level_stands_on_solid_ground_partially(level, rect)) {
      return i;
    }
    i++;
  }
  return i;
}

/* --------------------------------------------------------------- */

fn_environment_t * fn_level_get_environment(fn_level_t * level)
{
  return level->environment;
}

/* --------------------------------------------------------------- */

