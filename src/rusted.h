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

#define PICTURE_HEIGHT 200

#define PICTURE_WIDTH 40

#define TILE_HEIGHT (HALFTILE_HEIGHT * 2)

#define TILE_WIDTH (HALFTILE_WIDTH * 2)

#define WINDOW_HEIGHT 200

#define WINDOW_WIDTH 320

typedef struct File File;

typedef struct Texture Texture;

typedef struct TileCache TileCache;

typedef File FnFile;

typedef struct {
    int16_t x;
    int16_t y;
    uint16_t w;
    uint16_t h;
} Geometry;

typedef Geometry FnGeometry;

typedef Texture FnTexture;

typedef struct {
    uint32_t flags;
    int32_t bits_per_pixel;
    uint32_t transparent;
} TextureCreationParams;

typedef TextureCreationParams FnTextureCreationParams;

typedef struct {
    uint8_t tiles;
    uint8_t width;
    uint8_t height;
} TileHeader;

typedef TileHeader FnTileHeader;

typedef TileCache FnTileCache;

void fn_file_free(FnFile *ptr);

FnFile *fn_file_open(const char *path);

void fn_file_read(FnFile *ptr, void *buffer, size_t length);

SDL_Rect fn_geometry_as_sdl_rect(const FnGeometry *ptr);

FnGeometry fn_geometry_create(int16_t x, int16_t y, uint16_t w, uint16_t h);

FnTexture *fn_picture_load(FnFile *file, FnTextureCreationParams params);

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

bool fn_tile_is_solid(uint16_t index);

FnTexture *fn_tile_load(FnFile *file,
                        FnTextureCreationParams params,
                        FnTileHeader header,
                        bool has_transparency);

void fn_tilecache_free(FnTileCache *ptr);

const FnTexture *fn_tilecache_get_tile(const FnTileCache *ptr,
                                       uintptr_t index);

FnTileCache *fn_tilecache_load(const char *path,
                               FnTextureCreationParams params);

FnTileHeader fn_tileheader_load(FnFile *file);
