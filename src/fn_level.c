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

/* --------------------------------------------------------------- */

#include "fn_level.h"
#include "fn_hero.h"
#include "fn_object.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

fn_level_t * fn_level_load(FnFile* file,
    fn_environment_t * env)
{
  size_t i = 0;
  fn_level_t * lv = malloc(sizeof(fn_level_t));
  memset(lv, 0, sizeof(fn_level_t));
  Uint16 tilenr;
  Uint8 uppertile;
  Uint8 lowertile;

  lv->data = fn_level_data_create();

  lv->environment = env;

  lv->animated_frames = 0;

  lv->data->level_passed = 0;

  lv->num_shots = 0;

  lv->actors = NULL;
  lv->bots = NULL;
  lv->shots = NULL;
  lv->interactor = NULL;

  lv->data->do_play = 1;

  lv->surface_fixed = fn_environment_create_surface(
      env,
      FN_TILE_WIDTH * FN_LEVEL_WIDTH,
      FN_TILE_HEIGHT * FN_LEVEL_HEIGHT);

  lv->surface = fn_environment_create_surface(
      env,
      FN_TILE_WIDTH * FN_LEVEL_WIDTH,
      FN_TILE_HEIGHT * FN_LEVEL_HEIGHT);

  FnLevelTiles * tiles = &(lv->data->tiles);
  FnLevelSolids * solids = &(lv->data->solids);
  FnLevelActorQueue * actor_queue = fn_level_actor_queue_create();

  while (i != FN_LEVEL_HEIGHT * FN_LEVEL_WIDTH)
  {
    size_t x = i%FN_LEVEL_WIDTH;
    size_t y = i/FN_LEVEL_WIDTH;

    /* we don't only want to run on big-endian systems,
     * so we load the bytes separately.
     */
    fn_file_read(file, &lowertile, 1);
    fn_file_read(file, &uppertile, 1);
    tilenr = (uppertile << 8) | lowertile;

    lv->raw[y][x] = tilenr;

    if ((tilenr >= 4) && (tilenr <= 0x2fe0)) {
      fn_level_tiles_set(tiles, x, y, tilenr / 0x20);
      fn_level_solids_set(solids, x, y, (tilenr >= 0x1800));
    }

    fn_hero_t * hero = fn_environment_get_hero(env);

    uint16_t tx = x * FN_TILE_WIDTH;
    uint16_t ty = y * FN_TILE_HEIGHT;

    switch(tilenr) {
      case 0x0080: /* written text on black screen */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_TextOnScreenBackground, tx, ty);
        break;
      case 0x0100: /* blue high voltage flash */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_HighVoltageFlashBackground, tx, ty);
        break;
      case 0x0180: /* red flash light */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_RedFlashlightBackground, tx, ty);
        break;
      case 0x0200: /* blue high voltage flash */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BlueFlashlightBackground, tx, ty);
        break;
      case 0x0280: /* key panel on the wall */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeypanelBackground, tx, ty);
        break;
      case 0x0300: /* red rotation light */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_RedRotationLightBackground, tx, ty);
        break;
      case 0x0380: /* flashing up arrow */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_UpArrowBackground, tx, ty);
        break;
      case 0x0400: /* background blinking blue box */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BlueLightBackground1, tx, ty);
        break;
      case 0x0420: /* background blinking blue box */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BlueLightBackground2, tx, ty);
        break;
      case 0x0440: /* background blinking blue box */
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_BlueLightBackground3, tx, ty);
        break;
      case 0x0460: /* background blinking blue box */
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_BlueLightBackground4, tx, ty);
        break;
      case 0x0480: /* background green poison liquid */
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_GreenPoisonBackground, tx, ty);
        break;
      case 0x0500: /* background lava */
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_LavaBackground, tx, ty);
        break;
      case 0x1800: /* solid wall which can be shot */
        fn_level_tiles_set(tiles, x, y, 0x17E0/0x20);
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_ShootableWall, tx, ty);
        break;
      case 0x1C00: /* center conveyor */
        fn_level_tiles_set(tiles, x, y, SOLID_BLACK);
        fn_level_solids_set(solids, x, y, true);

        /*
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_CONVEYOR_RIGHTMOVING_CENTER, tx, ty);
          */
        break;
      case 0x3000: /* grey box, empty */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_BoxGreyEmpty, tx, ty);
        break;
      case 0x3001: /* lift */
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
          ActorType_Lift, tx, ty);
        break;
      case 0x3002: /* left end of left-moving conveyor */
        fn_level_tiles_set(tiles, x, y, SOLID_CONVEYORBELT_LEFTEND);
        fn_level_solids_set(solids, x, y, true);
        break;
      case 0x3003: /* right end of left-moving conveyor */
        fn_level_tiles_set(tiles, x, y, SOLID_BLACK);
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_ConveyorLeftMovingRightEnd, tx, ty);
        break;
      case 0x3004: /* left end of right-moving conveyor */
        fn_level_tiles_set(tiles, x, y, SOLID_CONVEYORBELT_LEFTEND);
        fn_level_solids_set(solids, x, y, true);
        break;
      case 0x3005: /* right end of right-moving conveyor */
        fn_level_tiles_set(tiles, x, y, SOLID_BLACK);
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_ConveyorRightMovingRightEnd, tx, ty);
        break;
      case 0x3006: /* grey box with boots inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyBoots, tx, ty);
        break;
      case 0x3007: /* rocket which gets started if shot
                    * and leaves a blue box with a balloon */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Rocket, tx, ty);
        break;
      case 0x3008: /* grey box with clamps inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyClamps, tx, ty);
        break;
      case 0x3009: /* fire burning to the right */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FireRight, tx, ty);
        break;
      case 0x300A: /* fire burning to the left */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FireLeft, tx, ty);
        break;
      case 0x300b: /* flying techbot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FlyingBot, tx, ty);
        break;
      case 0x300c: /* footbot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        lv->bots = fn_list_append(lv->bots, fn_bot_create(
              BotType_FootBot, x*2, y*2));
        break;
      case 0x300d: /* tankbot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_TankBot, tx, ty);
        break;
      case 0x300e: /* fire wheel bot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FireWheelBot, tx, ty);
        break;
      case 0x300F: /* grey box with gun inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyGun, tx, ty);
        break;
      case 0x3010: /* robot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Robot, tx, ty);
        break;
      case 0x3011: /* exit door */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_ExitDoor, tx, ty-FN_TILE_HEIGHT);
        break;
      case 0x3012: /* grey box with bomb inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyBomb, tx, ty);
        break;
      case 0x3013: /* bot consisting of several white-blue balls */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_SnakeBot, tx, ty);
        break;
      case 0x3014: /* water mirroring everything that is above */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Water, tx, ty);
        break;
      case 0x3015: /* red box with soda inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxRedSoda, tx, ty);
        break;
      case 0x3016: /* crab bot crawling along wall left of him */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_WallCrawlerBotLeft, tx, ty);
        break;
      case 0x3017: /* crab bot crawling along wall right of him */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_WallCrawlerBotRight, tx, ty);
        break;
      case 0x3018: /* red box with chicken inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxRedChicken, tx, ty);
        break;
      case 0x3019: /* floor that breaks on second jump onto it */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_UnstableFloor, tx, ty);
        break;
      case 0x301a: /* horizontal laser beam which gets deactivated when mill is shot */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Laserbeam, tx, ty);
        break;
      case 0x301b: /* fan wheel mounted on right wall blowing to the left */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FanLeft, tx, ty);
        break;
      case 0x301c: /* fan wheel mounted on left wall blowing to the right*/
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FanRight, tx, ty);
        break;
      case 0x301d: /* blue box with football insdie */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueFootball, tx, ty);
        break;
      case 0x301e: /* blue box with joystick inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueJoystick, tx, ty);
        break;
      case 0x301f: /* blue box with disk inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueDisk, tx, ty);
        break;
      case 0x3020: /* grey box with glove inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyGlove, tx, ty);
        break;
      case 0x3021: /* laser beam which is deactivated by access card */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_AccessCardDoor, tx, ty);
        break;
      case 0x3022: /* helicopter */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_HelicopterBot, tx, ty);
        break;
      case 0x3023: /* blue box with balloon inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueBalloon, tx, ty);
        break;
      case 0x3024: /* camera */
        /*
        if (x > 0) {
          tiles[y][x] = tiles[y][x-1];
        }
        */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Camera, tx, ty);
        break;
      case 0x3025: /* broken wall background */
        /* take the part from one above */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BrokenWallBackground, tx, ty);
        break;
      case 0x3026: /* left end of background stone wall */
        /* TODO */
        break;
      case 0x3027: /* right end of background stone wall */
        /* TODO */
        break;
      case 0x3028: /* window inside background stone wall */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_StoneWindowBackground, tx, ty);
        break;
      case 0x3029: /* grey box with full life */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyFullLife, tx, ty);
        break;
      case 0x302a: /* "ACME" brick that comes falling down */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Acme, tx, ty);
        break;
      case 0x302b: /* rotating mill that can kill duke on touch */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Mill, tx, ty);
        break;
      case 0x302c: /* single spike standing out of the floor */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Spike, tx, ty);
        break;
      case 0x302d: /* blue box with flag inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueFlag, tx, ty);
        break;
      case 0x302e: /* blue box with radio inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxBlueRadio, tx, ty);
        break;
      case 0x302f: /* teleporter station */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Teleporter1, tx, ty);
        break;
      case 0x3030: /* teleporter station */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Teleporter2, tx, ty);
        break;
      case 0x3031: /* jumping mines */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_RedBallJumping, tx, ty);
        break;
      case 0x3032: /* we found our hero! */
        fn_hero_data_enter_level(hero->data,
            x * FN_TILE_WIDTH, (y - 1) * FN_TILE_HEIGHT);
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        break;
      case 0x3033: /* grey box with the access card inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyAccessCard, tx, ty);
        break;
      case 0x3034: /* slot for access card */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_AccessCardSlot, tx, ty);
        break;
      case 0x3035: /* slot for glove */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_GloveSlot, tx, ty);
        break;
      case 0x3036: /* floor which expands to right by access of glove slot */
        fn_level_solids_set(solids, x, y, 1);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_ExpandingFloor, tx, ty);
        break;
      case 0x3037: /* grey box with a D inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyLetterD, tx, ty);
        break;
      case 0x3038: /* grey box with a U inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyLetterU, tx, ty);
        break;
      case 0x3039: /* grey box with a K inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
              ActorType_BoxGreyLetterK, tx, ty);
        break;
      case 0x303a: /* grey box with a E inside */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_BoxGreyLetterE, tx, ty);
        break;
      case 0x303b: /* bunny bot */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_RabbitoidBot, tx, ty);
        break;
      case 0x303c: /* fire gnome */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FlameGnomeBot, tx, ty);
        break;
      case 0x303d: /* fence with backdrop 1 behind it */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_FenceBackground, tx, ty);
        break;
      case 0x303e: /* window - left part */
        fn_level_tiles_set(tiles, x, y, 0);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_WindowLeftBackground, tx, ty);
        break;
      case 0x303f: /* window - right part */
        fn_level_tiles_set(tiles, x, y, 0);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_WindowRightBackground, tx, ty);
        break;
      case 0x3040: /* the notebook */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Notebook, tx, ty);
        break;
      case 0x3041: /* the surveillance screen */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_SurveillanceScreen, tx, ty);
        break;
      case 0x3043: /* dr proton -the final opponent */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_DrProton, tx, ty);
        break;
      case 0x3044: /* red key */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyRed, tx, ty);
        break;
      case 0x3045: /* green key */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyGreen, tx, ty);
        break;
      case 0x3046: /* blue key */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyBlue, tx, ty);
        break;
      case 0x3047: /* pink key */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyPink, tx, ty);
        break;
      case 0x3048: /* red keyhole */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyholeRed, tx, ty);
        break;
      case 0x3049: /* green keyhole */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyholeGreen, tx, ty);
        break;
      case 0x304a: /* blue keyhole */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyholeBlue, tx, ty);
        break;
      case 0x304b: /* pink keyhole */
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_KeyholePink, tx, ty);
        break;
      case 0x304c: /* red door */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_DoorRed, tx, ty);
        break;
      case 0x304d: /* green door */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_DoorGreen, tx, ty);
        break;
      case 0x304e: /* blue door */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_DoorBlue, tx, ty);
        break;
      case 0x304f: /* pink door */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_solids_set(solids, x, y, true);
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_DoorPink, tx, ty);
        break;
      case 0x3050: /* football on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Football, tx, ty);
        break;
      case 0x3051: /* single chicken on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_ChickenSingle, tx, ty);
        break;
      case 0x3052: /* soda on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Soda, tx, ty);
        break;
      case 0x3053: /* a disk on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Disk, tx, ty);
        break;
      case 0x3054: /* a joystick on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Joystick, tx, ty);
        break;
      case 0x3055: /* a flag on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Flag, tx, ty);
        break;
      case 0x3056: /* a radio on its own */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_Radio, tx, ty);
        break;
      case 0x3057: /* the red mine lying on the ground */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_RedBallLying, tx, ty);
        break;
      case 0x3058: /* spikes showing up */
        if (y > 0) {
          fn_level_tiles_copy_from_to(tiles, x, y-1, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_SpikesUp, tx, ty);
        break;
      case 0x3059: /* spikes showing down */
        if (x > 0) {
          fn_level_tiles_copy_from_to(tiles, x-1, y, x, y);
        }
        fn_level_actor_queue_push_back(actor_queue,
            ActorType_SpikesDown, tx, ty);
        break;
      default:
        if (tilenr / 0x20 >= SOLID_END) {
          fprintf(stderr, "Unknown tile 0x%04x at x: %d, y: %d\n",
              tilenr, (int)x, (int)y);
          fn_level_tiles_set(tiles, x, y, 2);
        }
        break;
    }

    i++;
  }

  while (fn_level_actor_queue_has_items(actor_queue)) {
      FnLevelActorQueueItem item =
          fn_level_actor_queue_pop_front(actor_queue);
      fn_level_add_actor(lv, item.actor_type, item.x, item.y);
  }
  fn_level_actor_queue_free(actor_queue);

  /*
   * Blit everything fixed to lv->surface_fixed.
   */
  Uint32 transparent;
  transparent = SDL_MapRGB(lv->surface_fixed->format, 100, 1, 1);
  SDL_SetColorKey(lv->surface, SDL_SRCCOLORKEY, transparent);
  SDL_FillRect(lv->surface_fixed, NULL, transparent);

  FnGeometry r;
  r.w = FN_TILE_WIDTH;
  r.h = FN_TILE_HEIGHT;

  Uint16 y = 0;
  Uint16 x = 0;
  const FnTexture * tile = NULL;
  for (y = 0; y < FN_LEVEL_HEIGHT; y++) {
    for (x = 0; x < FN_LEVEL_WIDTH; x++) {
      tilenr = fn_level_tiles_get(tiles, x, y);
      tile = NULL;
      if (tilenr > 1 && tilenr < (48 * 8)) {
        r.x = x * FN_TILE_WIDTH;
        r.y = y * FN_TILE_HEIGHT;
        tile = fn_environment_get_tile(env, tilenr);
        fn_texture_blit_to_sdl_surface(tile, NULL, lv->surface_fixed, &r);
      }
    }
  }

  return lv;
}

/* --------------------------------------------------------------- */

void fn_level_free(fn_level_t * lv)
{
  fn_list_t * iter = NULL;

  for (iter = fn_list_first(lv->bots);
      iter != fn_list_last(lv->bots);
      iter = fn_list_next(iter)) {
    fn_bot_free((FnBot *)iter->data);
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
      fn_level_actor_free((FnLevelActor *)iter->data);
      iter->data = NULL;
    }
  }
  fn_list_free(lv->actors);

  SDL_FreeSurface(lv->surface);
  SDL_FreeSurface(lv->surface_fixed);

  fn_level_data_free(lv->data);

  free(lv);
}

/* --------------------------------------------------------------- */

Uint16 fn_level_get_raw(fn_level_t * lv, size_t x, size_t y)
{
  return lv->raw[y][x];
}

/* --------------------------------------------------------------- */

Uint8 fn_level_is_solid(const fn_level_t * lv, int x, int y)
{
  if (x < 0 || y < 0 || x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return 1;
  }
  return fn_level_solids_get(&(lv->data->solids), x, y);
}

/* --------------------------------------------------------------- */

void fn_level_set_solid(fn_level_t * lv, int x, int y, Uint8 solid)
{
  if (x < 0 || y < 0 || x > FN_LEVEL_WIDTH || y > FN_LEVEL_HEIGHT) {
    return;
  }
  fn_level_solids_set(&(lv->data->solids), x, y, solid);
}

/* --------------------------------------------------------------- */

void fn_level_blit_to_surface(fn_level_t * lv,
    SDL_Surface * target,
    FnGeometry * targetrect,
    FnGeometry * sourcerect,
    FnTexture * backdrop1,
    FnTexture * backdrop2)
{
  int x_start = 0;
  int x_end = FN_LEVEL_WIDTH;
  int y_start = 0;
  int y_end = FN_LEVEL_HEIGHT;
  fn_list_t * iter = NULL;

  fn_environment_t * env = fn_level_get_environment(lv);

  /* load the background tiles */
  /*
  SDL_FillRect(lv->surface, sourcerect, 0);
  */
  SDL_Rect srcrect = fn_geometry_as_sdl_rect(sourcerect);
  if (backdrop1 != NULL) {
    fn_texture_blit_to_sdl_surface(
        backdrop1, NULL, lv->surface, sourcerect);
  } else {
    SDL_FillRect(lv->surface, &srcrect, 0);
  }

  SDL_BlitSurface(
      lv->surface_fixed, &srcrect, lv->surface, &srcrect);

  /* calculate the bounds of the area we have to blit. */
  if (sourcerect) {
    x_start = (sourcerect->x / FN_TILE_WIDTH) - (FN_LEVELWINDOW_WIDTH / 2);
    if (x_start < 0) {
      x_start = 0;
    }
    x_end = x_start + (sourcerect->w / FN_TILE_WIDTH) * 2;
    if (x_end > FN_LEVEL_WIDTH) {
      x_end = FN_LEVEL_WIDTH;
      x_start = x_end - FN_LEVELWINDOW_WIDTH * 2;
    }

    y_start = (sourcerect->y / FN_TILE_HEIGHT) - (FN_LEVELWINDOW_HEIGHT / 2);
    if (y_start < 0) {
      y_start = 0;
    }
    y_end = y_start + (sourcerect->h / FN_TILE_HEIGHT) * 2;
    if (y_end > FN_LEVEL_HEIGHT) {
      y_end = FN_LEVEL_HEIGHT;
      y_start = y_end - FN_LEVELWINDOW_HEIGHT * 2;
    }
  }

  fn_hero_t * hero = fn_environment_get_hero(env);
  Uint8 draw_collision_bounds =
      fn_environment_get_draw_collision_bounds(env);
  const FnTileCache * tilecache = fn_environment_get_tilecache(env);

  /* blit the actors in the background */
  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    FnLevelActor * actor = (FnLevelActor *)iter->data;

    if (actor != NULL) {
      FnGeometry position = fn_level_actor_get_position(actor);
      Uint16 xl = position.x / FN_TILE_WIDTH;
      Uint16 yt = position.y / FN_TILE_HEIGHT;
      Uint16 xr = xl + position.w / FN_TILE_WIDTH;
      Uint16 yb = yt + position.h / FN_TILE_HEIGHT;

      if (xr > x_start && yb > y_start && xl < x_end && yt < y_end) {
        fn_level_actor_set_visible(actor, 1);
        if (!fn_level_actor_in_foreground(actor)) {
          fn_level_actor_blit(
                  actor,
                  hero->data,
                  tilecache,
                  lv->surface,
                  draw_collision_bounds);
        }
      } else {
        fn_level_actor_set_visible(actor, 0);
      }
    }
  }

  /* blit the hero */
  fn_hero_data_blit(hero->data,
      lv->surface,
      tilecache,
      &(lv->data->solids),
      draw_collision_bounds);

  /* blit the actors in the foreground */
  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    FnLevelActor * actor = (FnLevelActor *)iter->data;

    if (actor != NULL && fn_level_actor_is_visible(actor)) {
      if (fn_level_actor_in_foreground(actor)) {
        fn_level_actor_blit(
                actor,
                hero->data,
                tilecache,
                lv->surface,
                draw_collision_bounds);
      }
    }
  }

  /* blit the bots */
  for (iter = fn_list_first(lv->bots);
      iter != NULL;
      iter = fn_list_next(iter)) {
    const FnBot * bot = (FnBot *)iter->data;
    int x = fn_bot_get_x(bot) / 2;
    int y = fn_bot_get_y(bot) / 2;
    if (x > x_start && y > y_start && x < x_end && y < y_end) {
      fn_bot_blit(bot, lv->surface, tilecache);
    }
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
        fn_shot_blit(shot, lv->surface, tilecache);
      } else {
        fn_shot_gets_out_of_sight(shot);
      }
    }
  }

  SDL_Rect trect = fn_geometry_as_sdl_rect(targetrect);

  /* blit the whole thing to the caller */
  SDL_BlitSurface(lv->surface, &srcrect, target, &trect);
}

/* --------------------------------------------------------------- */

SDL_Surface * fn_level_get_surface(fn_level_t * lv)
{
  return lv->surface;
}

/* --------------------------------------------------------------- */

/* TODO this is deprecated! remove it. */
const FnTileCache * fn_level_get_tilecache(fn_level_t * lv)
{
  return fn_environment_get_tilecache(lv->environment);
}

/* --------------------------------------------------------------- */

int fn_level_keep_on_playing(fn_level_t * lv) {
  return lv->data->do_play;
}

/* --------------------------------------------------------------- */

fn_hero_t * fn_level_get_hero(fn_level_t * lv) {
  return fn_environment_get_hero(lv->environment);
}

/* --------------------------------------------------------------- */

int fn_level_act(
        fn_level_t * lv,
        FnLevelActorQueue * actor_queue,
        FnLevelActorMessageQueue * actor_message_queue)
{
  fn_list_t * iter = NULL;
  int res = 0;
  int cleanup = 0;

  fn_hero_t * hero = fn_environment_get_hero(lv->environment);

  lv->animated_frames ++;
  lv->animated_frames %= 1;

  for (iter = fn_list_first(lv->shots);
      iter != NULL;
      iter = fn_list_next(iter)) {
    fn_shot_t * shot = (fn_shot_t *)iter->data;

    if (shot != NULL) {
      res = fn_shot_act(shot, lv, actor_queue);
      if (res == 0) {
        /* set the cleanup flag and free the memory */
        cleanup = 1;
        iter->data = 0;
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
  size_t actors_hurting_hero = 0;

  while (fn_level_actor_message_queue_has_items(actor_message_queue)) {
      FnLevelActorMessage message =
          fn_level_actor_message_queue_pop_front(actor_message_queue);
      for (iter = fn_list_first(lv->actors);
              iter != NULL;
              iter = fn_list_next(iter))
      {
          FnLevelActor * actor = (FnLevelActor *)iter->data;
          if (fn_level_actor_type(actor) == message.receivers) {
              fn_level_actor_receive_message(
                      actor,
                      message.message,
                      hero->data,
                      lv->data);
          }
      }
  }


  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    FnLevelActor * actor = (FnLevelActor *)iter->data;

    if  (fn_level_actor_acts_while_invisible(actor) ||
            fn_level_actor_is_visible(actor)) {
      sum++;
      fn_level_actor_act(actor, lv->data, hero->data, actor_queue);
      if (!fn_level_actor_is_alive(actor)) {
        /* set the cleanup flag and free the memory */
        cleanup = 1;
        iter->data = NULL;
        fn_level_actor_free(actor); actor = NULL;
      } else if (fn_level_actor_hurts_hero(actor)) {
          actors_hurting_hero++;
      }
    }
  }

  while (fn_level_actor_queue_has_items(actor_queue)) {
      FnLevelActorQueueItem item = fn_level_actor_queue_pop_front(actor_queue);
      fn_level_add_actor(lv, item.actor_type, item.x, item.y);
  }

  if (cleanup) {
    /* clean up the actors that are finished */
    cleanup = 0;
    lv->actors = fn_list_remove_all(lv->actors, NULL);
  }

  fn_hero_data_set_gets_hurt(hero->data, actors_hurting_hero > 0);

  if (lv->animated_frames == 0) {
    /* do some action, not just animation */
    fn_hero_data_act(hero->data, &(lv->data->solids));
  }

  fn_hero_data_next_frame(hero->data);
  fn_hero_data_update_animation(hero->data);

  return 1;
}

/* --------------------------------------------------------------- */

void fn_level_hero_interact_stop(fn_level_t * lv)
{
  if (lv->interactor != NULL) {
    fn_hero_t * hero = fn_level_get_hero(lv);
    fn_level_actor_hero_interact_end(
            lv->interactor, lv->data, hero->data);
  }
  lv->interactor = NULL;
}

/* --------------------------------------------------------------- */

void fn_level_hero_interact_start(
        fn_level_t * lv,
        FnInfoMessageQueue * info_message_queue,
        FnLevelActorMessageQueue * actor_message_queue)
{
  fn_list_t * iter = NULL;
  for (iter = fn_list_first(lv->actors);
      iter != NULL;
      iter = fn_list_next(iter)) {
    FnLevelActor * actor = (FnLevelActor *)iter->data;
    fn_hero_t * hero = fn_level_get_hero(lv);

    if (fn_level_actor_hero_can_interact(actor, hero->data)) {
      FnHeroPosition * position = fn_hero_data_get_position(hero->data);

      FnGeometry heropos = fn_hero_position_get_geometry(position);

      if (fn_geometry_touches(heropos, fn_level_actor_get_position(actor)))
      {
        fn_level_hero_interact_stop(lv);

        lv->interactor = actor;
        fn_level_actor_hero_interact_start(
                actor,
                lv->data,
                hero->data,
                info_message_queue,
                actor_message_queue);
        return;
      }
    }
  }
}

/* --------------------------------------------------------------- */

FnLevelActor * fn_level_add_actor(fn_level_t * lv,
    FnLevelActorType type,
    Uint16 x,
    Uint16 y)
{
  FnLevelActor * actor = fn_level_actor_create(type, lv->data, x, y);
  lv->actors = fn_list_append(lv->actors, actor);

  return actor;
}

/* --------------------------------------------------------------- */

fn_shot_t * fn_level_add_shot(fn_level_t * lv,
    FnHorizontalDirection direction,
    Uint16 x,
    Uint16 y,
    FnLevelActorQueue * actor_queue)
{
  fn_shot_t * shot = fn_shot_create(x, y, direction);

  int addition = (direction == HorizontalDirection_Right ?
      1 : -1);

  lv->shots = fn_list_append(lv->shots, shot);

  fn_shot_push(shot, lv, addition * FN_HALFTILE_WIDTH, actor_queue);

  Uint8 draw_collision_bounds =
    fn_environment_get_draw_collision_bounds(lv->environment);

  fn_shot_set_draw_collision_bounds(shot,
      draw_collision_bounds);
  return shot;
}

/* --------------------------------------------------------------- */

void fn_level_fire_shot(fn_level_t * lv, FnLevelActorQueue * actor_queue)
{
  fn_hero_t * hero = fn_level_get_hero(lv);
  FnHeroFirepower * firepower = fn_hero_data_get_firepower(hero->data);
  FnHeroPosition * position = fn_hero_data_get_position(hero->data);

  if (lv->num_shots < fn_hero_firepower_num_shots(firepower)) {
    FnGeometry geometry = fn_hero_position_get_geometry(position);
    HorizontalDirection direction = fn_hero_data_get_direction(hero->data);

    fn_level_add_shot(lv, direction, geometry.x, geometry.y, actor_queue);
    lv->num_shots++;
  }
}

/* --------------------------------------------------------------- */

fn_list_t * fn_level_get_items_of_type(fn_level_t * lv,
    FnLevelActorType type)
{
  fn_list_t * ret = NULL;
  fn_list_t * iter = NULL;
  for (iter = fn_list_first(lv->actors);
      iter != fn_list_last(lv->actors);
      iter = fn_list_next(iter)) {
    FnLevelActor * actor = iter->data;
    if (fn_level_actor_type(actor) == type) {
      ret = fn_list_append(ret, actor);
    }
  }
  return ret;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_stands_on_solid_ground_completely(
        FnLevelSolids * solids,
        FnGeometry rect)
{
  if ((rect.y + rect.h) % FN_TILE_HEIGHT) {
    return 0;
  }
  Uint16 i = 0;
  Uint16 j = (rect.y + rect.h) / FN_TILE_HEIGHT;
  for (i = rect.x / FN_TILE_WIDTH;
      i < (rect.x + rect.w - 1) / FN_TILE_WIDTH + 1;
      i++)
  {
    if (!fn_level_solids_get(solids, i, j)) {
      return 0;
    }
  }
  return 1;
}

/* --------------------------------------------------------------- */


Uint8 fn_level_stands_on_solid_ground_partially(
        FnLevelSolids * solids, FnGeometry rect)
{
  if ((rect.y + rect.h) % FN_TILE_HEIGHT) {
    return 0;
  }
  Uint16 i = 0;
  Uint16 j = (rect.y + rect.h) / FN_TILE_HEIGHT;
  for (i = rect.x / FN_TILE_WIDTH;
      i < (rect.x + rect.w - 1) / FN_TILE_WIDTH + 1;
      i++)
  {
    if (fn_level_solids_get(solids, i, j)) {
      return 1;
    }
  }
  return 0;
}

/* --------------------------------------------------------------- */

Uint8 fn_level_push_rect_standing_on_solid_ground(
        FnLevelSolids * solids,
        FnGeometry rect,
        Sint8 offset,
        Uint8 gravity)
{
  if (fn_level_solids_collides(solids, rect)) {
    /* locked in, so don't move at all */
    return 0;
  }

  /* fall down as far as possible */
  fn_level_rect_fall_down(solids, rect, gravity);

  /* check if we stand on solid ground before movement */
  Uint8 stood_solid = fn_level_stands_on_solid_ground_completely(
      solids, rect);

  rect.x += offset;

  if (fn_level_solids_collides(solids, rect)) {
    /* we collide with something, so we revert to original position */
    rect.x -= offset;
    return 0;
  }

  if (stood_solid && fn_level_stands_on_solid_ground_completely(
        solids, rect)) {
    /* we stood on solid ground before, and still do. */
    return 1;
  } else if (stood_solid) {
    /* we stood on solid ground before, but do no longer now. */
    rect.x -= offset;
    return 0;
  } else {
    /* we walk on partial solid ground as long as possible. */
    return 1;
  }
}

/* --------------------------------------------------------------- */

Uint8 fn_level_rect_fall_down(
    FnLevelSolids * solids, FnGeometry rect, Uint8 dist)
{
  if (fn_level_solids_collides(solids, rect)) {
    /* can't fall down because collides with solid ground */
    return 0;
  }
  if (fn_level_stands_on_solid_ground_partially(solids, rect)) {
    /* stands on solid ground so can't fall down */
    return 0;
  }
  Uint8 i = 0;
  while (i < dist) {
    /* check how far we can fall down */
    rect.y++;
    if (fn_level_stands_on_solid_ground_partially(solids, rect)) {
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

