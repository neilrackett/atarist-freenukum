/*
 * Copyright (C) 2026 Neil Rackett
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
/*
 * Duke Nukem 1 sound effects on the YM2149.
 *
 * The original game plays PC speaker square waves: each effect is a
 * sequence of 8253 timer dividers stored in DUKE1.DN1 / DUKE1-B.DN1,
 * one divider per ~11ms step. The YM2149 is also a square wave
 * generator, so the dividers translate directly into YM periods:
 *
 *   pc_freq   = 1193180 / divider
 *   ym_period = 125000 / pc_freq = divider * 125000 / 1193180
 *
 * Up to three effects play at once on YM channels A, B and C.
 * Steps are advanced from the VBL queue, which keeps all YM access
 * serialized with the OS's own use of the sound chip.
 */

#include <mint/osbind.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

/* YM periods per effect, 0 = silent step */
static Word * fn_sound_steps[FN_SOUND_NUM];
static Word fn_sound_len[FN_SOUND_NUM];

#define FN_SOUND_VOICES 3
#define FN_SOUND_VOLUME 12

typedef struct {
  volatile int active;
  const Word * steps;
  Word num;
  volatile Word pos;
} fn_voice_t;

static fn_voice_t fn_voices[FN_SOUND_VOICES];
static int fn_sound_ready = 0;
static int fn_vbl_slot = -1;
static int fn_old_conterm = -1;

/* ---------------------------------------------------------------- */
/* YM access (supervisor mode only)                                 */

#define YM_SELECT (*(volatile Byte *)0xFF8800UL)
#define YM_DATA   (*(volatile Byte *)0xFF8802UL)

static void fn_ym_write(int reg, int val)
{
  YM_SELECT = (Byte)reg;
  YM_DATA = (Byte)val;
}

/* ---------------------------------------------------------------- */
/* VBL tick: 20ms per frame, effects step every 11ms                */

static void fn_sound_vbl(void)
{
  static int ms_acc = 0;
  int v;

  ms_acc += 20;
  while (ms_acc >= 11) {
    ms_acc -= 11;
    for (v = 0; v < FN_SOUND_VOICES; v++) {
      fn_voice_t * voice = &fn_voices[v];
      if (!voice->active) {
        continue;
      }
      if (voice->pos >= voice->num) {
        voice->active = 0;
        fn_ym_write(8 + v, 0);
        continue;
      }
      {
        Word period = voice->steps[voice->pos++];
        if (period == 0) {
          fn_ym_write(8 + v, 0);
        } else {
          fn_ym_write(2 * v, period & 0xFF);
          fn_ym_write(2 * v + 1, (period >> 8) & 0x0F);
          fn_ym_write(8 + v, FN_SOUND_VOLUME);
        }
      }
    }
  }
}

/* ---------------------------------------------------------------- */
/* supervisor helpers                                               */

static long fn_sup_install(void)
{
  volatile Word * nvbls = (volatile Word *)0x454UL;
  void (**queue)(void) = *(void (***)(void))0x456UL;
  int i;

  /* silence the console key click (bit 0 of conterm) so it cannot
   * fight over the sound chip; key repeat (bit 1) stays on */
  fn_old_conterm = *(volatile Byte *)0x484UL;
  *(volatile Byte *)0x484UL = fn_old_conterm & ~1;

  /* enable tone on channels A-C, no noise, keep the I/O port
   * direction bits the OS relies on */
  YM_SELECT = 7;
  fn_ym_write(7, (YM_SELECT & 0xC0) | 0x38);

  for (i = 0; i < *nvbls; i++) {
    if (queue[i] == NULL) {
      queue[i] = fn_sound_vbl;
      return i;
    }
  }
  return -1;
}

static long fn_sup_remove(void)
{
  void (**queue)(void) = *(void (***)(void))0x456UL;
  int v;

  if (fn_vbl_slot >= 0) {
    queue[fn_vbl_slot] = NULL;
    fn_vbl_slot = -1;
  }
  for (v = 0; v < FN_SOUND_VOICES; v++) {
    fn_ym_write(8 + v, 0);
  }
  if (fn_old_conterm >= 0) {
    *(volatile Byte *)0x484UL = (Byte)fn_old_conterm;
    fn_old_conterm = -1;
  }
  return 0;
}

static void fn_sound_exit(void)
{
  if (fn_sound_ready) {
    fn_sound_ready = 0;
    Supexec(fn_sup_remove);
  }
}

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
    if (index < 0 || fn_sound_steps[index] != NULL
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
      fn_sound_steps[index] = steps;
      fn_sound_len[index] = num;
    }
  }

  free(raw);
}

/* ---------------------------------------------------------------- */

void fn_sound_init(char * datapath)
{
  char path[1024];
  long slot;

  if (fn_sound_ready) {
    return;
  }

  /* the shareware episode keeps its sounds in these two files */
  snprintf(path, sizeof(path), "%s/DUKE1.DN1", datapath);
  fn_sound_load_file(path);
  snprintf(path, sizeof(path), "%s/DUKE1-B.DN1", datapath);
  fn_sound_load_file(path);

  slot = Supexec(fn_sup_install);
  if (slot < 0) {
    return;
  }
  fn_vbl_slot = (int)slot;
  fn_sound_ready = 1;
  atexit(fn_sound_exit);
}

void fn_sound_play(fn_sound_e sound)
{
  fn_voice_t * voice = NULL;
  Word best = 0;
  int v;

  if (!fn_sound_ready || sound >= FN_SOUND_NUM
      || fn_sound_steps[sound] == NULL) {
    return;
  }

  /* prefer a silent voice, else steal the one closest to its end */
  for (v = 0; v < FN_SOUND_VOICES; v++) {
    if (!fn_voices[v].active) {
      voice = &fn_voices[v];
      break;
    }
    {
      Word remaining = fn_voices[v].num - fn_voices[v].pos;
      if (voice == NULL || remaining < best) {
        voice = &fn_voices[v];
        best = remaining;
      }
    }
  }

  voice->active = 0;
  voice->steps = fn_sound_steps[sound];
  voice->num = fn_sound_len[sound];
  voice->pos = 0;
  voice->active = 1;
}
