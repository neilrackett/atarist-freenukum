/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Test for Messagebox Drawing functions
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
#include <SDL/SDL_ttf.h>
#include <stdlib.h>

/* --------------------------------------------------------------- */

#include "config.h"
#include "fn.h"
#include "rusted.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
    FnTexture * msgbox;
    int res;
    int quit = 0;
    SDL_Event event;
    char * homedir;
    char tilespath[1024];

    FnSettings settings = fn_settings_load_or_create();
    SDL_Surface * screen = fn_game_initialize_and_get_window(
            FN_WINDOW_WIDTH,
            FN_WINDOW_HEIGHT,
            settings.fullscreen,
            "Freenukum " VERSION,
            "Freenukum " VERSION
            );
    if (!screen) {
        return 1;
    }
    FnTextureCreationParams texture_creation_params =
        fn_sdl_surface_creation_params(screen);
    const FnTileCache * tilecache =
        fn_tilecache_load(texture_creation_params);

    char * msg =
        " FREENUKUM MAIN MENU\n"
        " ------------------- \n"
        "\n"
        "S)tart a new game\n"
        "R)estore an old game\n"
        "I)nstructions\n"
        "O)rdering information\n"
        "G)ame setup\n"
        "H)igh scores\n"
        "P)reviews/Main Demo!\n"
        "V)iew user demo\n"
        "T)itle screen\n"
        "C)redits\n"
        "Q)it to DOS\n";

    homedir = getenv("HOME");

    if (homedir == NULL) {
      printf("%s\n", "HOME directory path not set.");
      exit(1);
    }

    snprintf(tilespath, 1024, "%s%s", homedir, "/.freenukum/data/");

    msgbox = fn_messagebox(msg, tilecache, texture_creation_params);

    fn_texture_blit_to_sdl_surface(msgbox, NULL, screen, NULL);
    fn_texture_free(msgbox);
    SDL_UpdateRect(screen, 0, 0, 0, 0);

    while (quit == 0)
    {
        res = SDL_WaitEvent(&event);
        if (res == 1)
        {
            switch(event.type)
            {
                case SDL_QUIT:
                    quit = 1;
                    break;
                case SDL_KEYDOWN:
                    switch(event.key.keysym.sym)
                    {
                        case SDLK_q:
                        case SDLK_ESCAPE:
                            quit = 1;
                            break;
                        default:
                            /* do nothing, ignoring other keys. */
                            break;
                    }
                case SDL_VIDEOEXPOSE:
                    SDL_UpdateRect(screen, 0, 0, 0, 0);
                    break;
                default:
                    /* do nothing */
                    break;
            }
        }
    }


    return 0;
}

