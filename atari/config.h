/* Hand-written config.h for the Atari ST / MiNT cross build.
 * Replaces the autotools-generated one; SDL_ttf, libcurl and libzip
 * are deliberately left undefined so the optional shareware
 * downloader and TTF text rendering compile out.
 */

#ifndef FN_ATARI_CONFIG_H
#define FN_ATARI_CONFIG_H

#define PACKAGE "freenukum"
#define PACKAGE_NAME "freenukum"
#define PACKAGE_VERSION "0.3.0-atari"
#define PACKAGE_STRING "freenukum 0.3.0-atari"
#define VERSION "0.3.0-atari"

#define STDC_HEADERS 1
#define HAVE_STDINT_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_STRINGS_H 1
#define HAVE_MEMORY_H 1
#define HAVE_UNISTD_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TYPES_H 1

#endif /* FN_ATARI_CONFIG_H */
