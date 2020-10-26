/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Test for Level loading functions
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

#include <SDL/SDL.h>
#include <unistd.h>
#include <string.h>
#include <fcntl.h>

/* --------------------------------------------------------------- */

#include "fn.h"
#include "fn_environment.h"
#include "fn_level.h"

/* --------------------------------------------------------------- */

void scroll(
        int xdist,
        int ydist,
        FnGeometry * r,
        SDL_Surface * level,
        SDL_Surface * screen)
{
    r->x += xdist;
    r->y += ydist;

    if (r->x < 0)
        r->x = 0;
    if (r->y < 0)
        r->y = 0;
    if (r->x + r->w > level->w)
        r->x = level->w - r->w;
    if (r->y + r->h > level->h)
        r->y = level->h - r->h;

    SDL_Rect rect = fn_geometry_as_sdl_rect(r);
    SDL_BlitSurface(level, &rect, screen, NULL);
    SDL_UpdateRect(screen, 0, 0, 0, 0);
}

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
    fn_level_t * lv = NULL;
    FnFile * file;
    int quit = 0;
    int res;
    SDL_Surface * screen;
    SDL_Surface * level;
    SDL_Event event;
    char * homedir;
    char levelfile[100];
    fn_environment_t * env = fn_environment_create();

    FnTextureCreationParams texture_creation_params =
        fn_sdl_surface_creation_params(env->screen);
    const FnTileCache * tilecache =
        fn_tilecache_load(texture_creation_params);

    fn_environment_check_for_episodes(env);

    int argok = 0;
    int levelnumber = 0;

    if (argc == 2) {
      if (strlen(argv[1]) == 1) {
        char c = argv[1][0];
        argok = 1;
        if (c >= '1' && c <= '9') {
            levelnumber = c - '0';
        } else if (c >= 'a' && c <= 'c') {
            levelnumber = c - 'a' + 10;
        } else if (c >= 'A' && c <= 'C') {
            levelnumber = c - 'A' + 10;
        } else {
          argok = 0;
        }
      }
    }

    if (!argok)
    {
        fprintf(stderr, "\nShows a Duke Nukem original level.\n");
        fprintf(stderr, "Usage: %s <LEVELNUMBER>\n", argv[0]);
        fprintf(stderr,
                "LEVELNUMBER is the number of the level.\n"
                "          This is usually a number between 1 and 9\n"
                "          or one of a, b, c.\n\n");

        return -1;
    }

    homedir = getenv("HOME");

    if (homedir == NULL) {
      printf("%s\n", "HOME directory path not set.");
      exit(1);
    }

    snprintf(levelfile, 100, "worldal%x.dn1", levelnumber);

    printf("Use the arrow keys to navigate through the level\n");

    file = fn_data_open_file(levelfile);

    if (file == NULL)
    {
        perror("Can't open file");
        return -1;
    }

    screen = fn_environment_get_screen_sdl(env);
    FnHeroData * hero = env->hero;
    bool draw_collision_bounds =
        fn_environment_get_draw_collision_bounds(env);

    lv = fn_level_load(
            file, hero, tilecache, texture_creation_params);
    if (lv == NULL)
    {
        fn_file_free(file);
        fprintf(stderr, "Could not load level from file %s\n", levelfile);
        return -1;
    }

    fn_file_free(file);

    level = SDL_CreateRGBSurface(
        texture_creation_params.flags,
        FN_TILE_WIDTH * FN_LEVEL_WIDTH,
        FN_TILE_HEIGHT * FN_LEVEL_HEIGHT,
        texture_creation_params.bits_per_pixel,
        0,
        0,
        0,
        0);
    SDL_SetColorKey(
            level,
            SDL_SRCCOLORKEY,
            texture_creation_params.transparent);

    SDL_WM_SetCaption("FreeNukum Level Tester", "");

    FnGeometry r;
    r.x = 0;
    r.y = 0;
    r.w = FN_TILE_WIDTH * FN_LEVEL_WIDTH;
    r.h = FN_TILE_HEIGHT * FN_LEVEL_HEIGHT;

    fn_level_blit_to_surface(
            lv,
            tilecache,
            hero,
            draw_collision_bounds,
            level,
            &r,
            &r,
            NULL,
            NULL);

    SDL_BlitSurface(level, NULL, screen, NULL);

    SDL_UpdateRect(screen, 0, 0, 0, 0);

    r.x = 0;
    r.y = 0;
    r.w = screen->w;
    r.h = screen->h;

    while (quit == 0)
    {
        res = SDL_WaitEvent(&event);
        if (res == 1)
        {
            int multiplier = 1;
            switch(event.type)
            {
                case SDL_QUIT:
                    quit = 1;
                    break;
                case SDL_KEYDOWN:
                    if (event.key.keysym.mod & KMOD_CTRL)
                    {
                        multiplier = 160;
                    }
                    else
                    {
                        multiplier = 16;
                    }
                    switch(event.key.keysym.sym)
                    {
                        case SDLK_q:
                        case SDLK_ESCAPE:
                            quit = 1;
                            break;
                        case SDLK_DOWN:
                            scroll(0, multiplier, &r, level, screen);
                            break;
                        case SDLK_UP:
                            scroll(0, -1 * multiplier, &r, level, screen);
                            break;
                        case SDLK_LEFT:
                            scroll(-1 * multiplier, 0, &r, level, screen);
                            break;
                        case SDLK_RIGHT:
                            scroll(multiplier, 0, &r, level, screen);
                            break;
                        default:
                            /* do nothing, ignoring other keys. */
                            break;
                    }
                    break;
                case SDL_MOUSEBUTTONDOWN:
                    if (event.button.button == SDL_BUTTON_LEFT) {
                      Uint16 click_x = 0;
                      Uint16 click_y = 0;
                      Uint16 global_x = 0;
                      Uint16 global_y = 0;
                      Uint16 tile_x = 0;
                      Uint16 tile_y = 0;
                      int tilenr = 0;
                      Uint8 is_solid = 0;
                      click_x = event.button.x;
                      click_y = event.button.y;

                      global_x = (r.x) + (click_x);
                      global_y = (r.y) + (click_y);
                      tile_x = global_x / FN_TILE_WIDTH;
                      tile_y = global_y / FN_TILE_HEIGHT;

                      tilenr = fn_level_get_raw(lv, tile_x, tile_y);
                      FnLevelSolids * solids =
                          fn_level_data_get_solids(lv->data);
                      is_solid = fn_level_solids_get(
                              solids, tile_x, tile_y);
                      printf("Tile number x=%d y=%d: 0x%04x; Solid: %s\n",
                          tile_x, tile_y, tilenr, (is_solid? "yes" : "no"));
                    }
                    break;
                case SDL_VIDEOEXPOSE:
                    SDL_UpdateRect(screen, 0, 0, 0, 0);
                    break;
                default:
                    /* do nothing */
                    break;
            }
        }
    }

    fn_level_free(lv);

    return 0;
}

/* --------------------------------------------------------------- */
