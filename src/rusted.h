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

Geometry *geometry_clone(const Geometry *geometry);

Geometry geometry_create(int32_t x, int32_t y, uint32_t w, uint32_t h);

uint32_t geometry_h(const Geometry *geometry);

void geometry_set_data(Geometry *geometry,
                       int32_t x,
                       int32_t y,
                       uint32_t w,
                       uint32_t h);

void geometry_set_h(Geometry *geometry, uint32_t h);

void geometry_set_w(Geometry *geometry, uint32_t w);

void geometry_set_x(Geometry *geometry, int32_t x);

void geometry_set_y(Geometry *geometry, int32_t y);

uint32_t geometry_w(const Geometry *geometry);

int32_t geometry_x(const Geometry *geometry);

int32_t geometry_y(const Geometry *geometry);
