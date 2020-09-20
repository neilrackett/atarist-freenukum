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

typedef struct {
    int32_t x;
    int32_t y;
    uint32_t w;
    uint32_t h;
} Geometry;

typedef Geometry FnGeometry;

FnGeometry fn_geometry_create(int32_t x, int32_t y, uint32_t w, uint32_t h);
