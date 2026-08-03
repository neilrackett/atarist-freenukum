/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Duke Nukem 1 sound effects on the YM2149, played by STDL_Sfx.
 *
 * The original game plays PC speaker square waves: each effect is a
 * sequence of 8253 timer dividers stored in DUKE1.DN1 / DUKE1-B.DN1,
 * one divider per ~11ms step. The YM2149 is also a square wave
 * generator, so the dividers translate directly into YM periods:
 *
 *   pc_freq   = 1193180 / divider
 *   ym_period = 125000 / pc_freq = divider * 125000 / 1193180
 *
 * That leaves a plain STDL_Sfx step effect per sound: STDL owns the
 * voice allocation, the 50Hz tick that advances the steps and the
 * YM register access (including the port bits TOS relies on).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <SDL.h>
#include <stdl/stdl_sfx.h>

#include "fn_sound.h"

typedef unsigned char  Byte;
typedef unsigned short Word;
typedef unsigned long  Long;

/* ---------------------------------------------------------------- */

static const char * const fn_sound_names[FN_SOUND_NUM] = {
  "PLAYERDEATH", "GETFOODITEM", "COKECANHIT", "GETBALLON",
  "GETPOWERUP", "BRIDGEXTEND", "PLAYERGUN", "TELEPORT",
  "PLAYERQUIT", "GETKEY", "BOXEXPLODE", "ENEMYSHOT",
  "SPECIALITEM", "PLAYERJUMP", "PLAYERLAND", "PLAYERHIT",
  "GETBONUSOBJ", "HITREACTOR", "SMALLDEATH", "LEVELDONE",
  "ELEVATOR", "FORCEFIELD", "WALKING", "HIGHSCORE",
  "CHEATMODE", "STARTGAME", "CLINGHOOKS", "READNOTE",
  "MONITOR", "ROCKET", "OPENKEYDOOR", "DANDERSIGN",
  "BOMBEXPLODE", "MINEBOUNCE", "RABBITGONE", "REACTORSND",
  "GETDUKESND", "HITHEAD", "DOORSND", "BADGUYGOUP",
  "BADGUYISDED", "HITABREAKER", "TORCHON", "THEND"
};

/* one effect per sound; periods point at the converted step arrays
 * and must stay allocated for as long as the effect can play */
static STDL_Sfx fn_sound_fx[FN_SOUND_NUM];

#define FN_SOUND_VOLUME 12
#define FN_SOUND_STEP_MS 11

static int fn_sound_ready = 0;

/* ---------------------------------------------------------------- */
/* data loading                                                     */

static Word fn_divider_to_period(Word divider)
{
  Long period;
  if (divider == 0) {
    return 0;
  }
  period = ((Long)divider * 6866UL) >> 16;
  if (period < 1) {
    period = 1;
  }
  if (period > 0xFFF) {
    period = 0xFFF;
  }
  return (Word)period;
}

static int fn_sound_index(const char * name)
{
  int i;
  for (i = 0; i < FN_SOUND_NUM; i++) {
    if (strcmp(name, fn_sound_names[i]) == 0) {
      return i;
    }
  }
  return -1;
}

static void fn_sound_load_file(const char * path)
{
  FILE * f = fopen(path, "rb");
  Byte * raw;
  long size;
  Word addr[25];
  char name[25][13];
  int i;

  if (f == NULL) {
    return;
  }
  fseek(f, 0, SEEK_END);
  size = ftell(f);
  fseek(f, 0, SEEK_SET);
  raw = malloc(size);
  if (raw == NULL || (long)fread(raw, 1, size, f) != size) {
    free(raw);
    fclose(f);
    return;
  }
  fclose(f);

  if (size < 400 || memcmp(raw, "SND\0", 4) != 0) {
    free(raw);
    return;
  }

  for (i = 1; i <= 24; i++) {
    long offset = 16 * i;
    addr[i] = raw[offset] | (raw[offset + 1] << 8);
    memcpy(name[i], raw + offset + 4, 12);
    name[i][12] = '\0';
  }

  for (i = 1; i < 24; i++) {
    int index = fn_sound_index(name[i]);
    long start = addr[i];
    long end = addr[i + 1];
    if (index < 0 || fn_sound_fx[index].periods != NULL
        || start < 400 || end <= start || end > size) {
      continue;
    }
    {
      Word num = (Word)((end - start) / 2);
      Word * steps = malloc(num * sizeof(Word));
      Word n;
      if (steps == NULL) {
        continue;
      }
      for (n = 0; n < num; n++) {
        Word divider = raw[start + 2 * n]
          | (raw[start + 2 * n + 1] << 8);
        steps[n] = fn_divider_to_period(divider);
      }
      fn_sound_fx[index].periods = steps;
      fn_sound_fx[index].volumes = NULL;
      fn_sound_fx[index].nsteps = num;
      fn_sound_fx[index].volume = FN_SOUND_VOLUME;
      fn_sound_fx[index].step_ms = FN_SOUND_STEP_MS;
      fn_sound_fx[index].noise = 0;
    }
  }

  free(raw);
}

/* ---------------------------------------------------------------- */

static void fn_sound_exit(void)
{
  if (fn_sound_ready) {
    fn_sound_ready = 0;
    STDL_StopSfx(-1);
  }
}

void fn_sound_init(char * datapath)
{
  char path[1024];

  if (fn_sound_ready) {
    return;
  }

  /* the shareware episode keeps its sounds in these two files */
  snprintf(path, sizeof(path), "%s/DUKE1.DN1", datapath);
  fn_sound_load_file(path);
  snprintf(path, sizeof(path), "%s/DUKE1-B.DN1", datapath);
  fn_sound_load_file(path);

  fn_sound_ready = 1;
  atexit(fn_sound_exit);
}

void fn_sound_play(fn_sound_e sound)
{
  if (!fn_sound_ready || sound >= FN_SOUND_NUM
      || fn_sound_fx[sound].periods == NULL) {
    return;
  }
  /* STDL picks a free voice, or steals the one it can spare */
  STDL_PlaySfx(&fn_sound_fx[sound], -1);
}
