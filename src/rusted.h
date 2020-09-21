#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FONT_HEIGHT 8

#define FONT_WIDTH 8

#define HALFTILE_HEIGHT 8

#define HALFTILE_WIDTH 8

#define HEALTH_COUNT 8

#define INVENTORY_WIDTH (HEALTH_COUNT / 2)

#define MAX_TILES_PER_FILE 50

#define TILE_HEIGHT (HALFTILE_HEIGHT * 2)

#define TILE_WIDTH (HALFTILE_WIDTH * 2)

typedef struct File File;

typedef struct Texture Texture;

typedef struct {
    int16_t x;
    int16_t y;
    uint16_t w;
    uint16_t h;
} Geometry;

typedef Geometry FnGeometry;

typedef File FnFile;

typedef Texture FnTexture;

typedef struct {
    uint32_t flags;
    int32_t bits_per_pixel;
    uint32_t transparent;
} TextureCreationParams;

typedef TextureCreationParams FnTextureCreationParams;

SDL_Rect fn_geometry_as_sdl_rect(const FnGeometry *ptr);

FnGeometry fn_geometry_create(int16_t x, int16_t y, uint16_t w, uint16_t h);

FnFile *fn_open_file(void);

void fn_texture_blit_to_sdl_surface(const FnTexture *ptr,
                                    const Geometry *srcrect,
                                    SDL_Surface *destination,
                                    const Geometry *dstrect);

void fn_texture_clone_to_texture(const FnTexture *ptr,
                                 const Geometry *srcrect,
                                 FnTexture *destination,
                                 const Geometry *dstrect);

void fn_texture_fill_area(FnTexture *ptr,
                          const Geometry *area,
                          uint8_t red,
                          uint8_t green,
                          uint8_t blue);

void fn_texture_free(FnTexture *ptr);

uint16_t fn_texture_get_height(const FnTexture *ptr);

uint16_t fn_texture_get_width(const FnTexture *ptr);

FnTexture *fn_texture_new_with_params(uint16_t w,
                                      uint16_t h,
                                      FnTextureCreationParams params);

void fn_texture_set_data(FnTexture *ptr,
                         const uint8_t *data,
                         uint32_t transparent);
