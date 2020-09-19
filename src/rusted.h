#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define FnFONT_HEIGHT 8

#define FnFONT_WIDTH 8

#define FnHALFTILE_HEIGHT 8

#define FnHALFTILE_WIDTH 8

#define FnHEALTH_COUNT 8

#define FnINVENTORY_WIDTH (FnHEALTH_COUNT / 2)

#define FnMAX_TILES_PER_FILE 50

#define FnTILE_HEIGHT (FnHALFTILE_HEIGHT * 2)

#define FnTILE_WIDTH (FnHALFTILE_WIDTH * 2)

typedef struct {
    int32_t x;
    int32_t y;
    uint32_t w;
    uint32_t h;
} FnGeometry;

FnGeometry *fn_geometry_clone(const FnGeometry *geometry);

FnGeometry fn_geometry_create(int32_t x, int32_t y, uint32_t w, uint32_t h);

uint32_t fn_geometry_h(const FnGeometry *geometry);

void fn_geometry_set_data(FnGeometry *geometry,
                          int32_t x,
                          int32_t y,
                          uint32_t w,
                          uint32_t h);

void fn_geometry_set_h(FnGeometry *geometry, uint32_t h);

void fn_geometry_set_w(FnGeometry *geometry, uint32_t w);

void fn_geometry_set_x(FnGeometry *geometry, int32_t x);

void fn_geometry_set_y(FnGeometry *geometry, int32_t y);

uint32_t fn_geometry_w(const FnGeometry *geometry);

int32_t fn_geometry_x(const FnGeometry *geometry);

int32_t fn_geometry_y(const FnGeometry *geometry);
