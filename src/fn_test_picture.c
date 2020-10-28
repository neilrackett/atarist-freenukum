/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Test for Picture Drawing functions
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
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

/* --------------------------------------------------------------- */

#include "fn.h"
#include "rusted.h"
#include "fn_environment.h"

/* --------------------------------------------------------------- */

int main(int argc, char ** argv)
{
    FnFile * file;
    int res;
    int quit = 0;
    SDL_Event event;

    FnSettings settings = fn_settings_load_or_create();
    if (!fn_game_initialize_sdl()) {
        return 1;
    }
    fn_environment_t * env = fn_environment_create(settings.fullscreen);

    if (argc != 2)
    {
        fprintf(stderr, "\nShows a Duke Nukem picture of the original Duke Nukem game.\n");
        fprintf(stderr, "Usage: %s <FILENAME>\n", argv[0]);
        fprintf(stderr, "FILENAME is usually one of the following file names:\n"
                "BADGUY.DN1, CREDITS.DN1, DN.DN1, DUKE.DN1, END.DN1\n\n");
        return -1;
    }

    file = fn_file_open(argv[1]);

    SDL_Surface * screen;
    FnTexture * picture;

    screen = fn_environment_get_screen_sdl(env);
    FnTextureCreationParams params =
        fn_sdl_surface_creation_params(screen);

    picture = fn_picture_load(file, params);

    fn_texture_blit_to_sdl_surface(picture, NULL, screen, NULL);
    SDL_UpdateRect(screen, 0, 0, 0, 0);

    fn_texture_free(picture);

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

/* --------------------------------------------------------------- */

