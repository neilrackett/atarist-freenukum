/*******************************************************************
 *
 * Project: FreeNukum 2D Jump'n Run
 * File:    Main menu
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

#include "fn_mainmenu.h"

/* --------------------------------------------------------------- */

int fn_mainmenu(
        SDL_Surface * screen,
        const FnTileCache * tilecache,
        FnTextureCreationParams texture_creation_params)
{
  int choice = 0;
  char * msg =
    "\n"
    "  FREENUKUM MAIN MENU \n"
    "  -------------------";
  FnMenu * menu = fn_menu_create(msg);
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_START,
      "S)tart a new game");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_RESTORE,
      "R)estore an old game");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_INSTRUCTIONS,
      "I)nstructions");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_ORDERINGINFO,
      "O)rdering information");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_FULLSCREENTOGGLE,
      "F)ullscreen toggle");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_EPISODECHANGE,
      "E)pisode change");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_HIGHSCORES,
      "H)igh scores");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_PREVIEWS,
      "P)reviews/Main Demo!");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_VIEWUSERDEMO,
      "V)iew user demo");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_TITLESCREEN,
      "T)itle screen");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_CREDITS,
      "C)redits");
  fn_menu_append_entry(
      menu,
      FN_MENUCHOICE_QUIT,
      "Q)it to DOS");

  choice = fn_menu_get_choice(
          menu, screen, tilecache, texture_creation_params);

  if (choice == '\0') {
    choice = FN_MENUCHOICE_QUIT;
  }

  return choice;
}
