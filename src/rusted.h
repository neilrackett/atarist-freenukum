#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

static const uintptr_t FONT_HEIGHT = 8;

static const uintptr_t FONT_WIDTH = 8;

static const uintptr_t HALFTILE_HEIGHT = 8;

static const uintptr_t HALFTILE_WIDTH = 8;

static const uintptr_t HEALTH_COUNT = 8;

static const uintptr_t INVENTORY_WIDTH = (HEALTH_COUNT / 2);

static const uintptr_t MAX_TILES_PER_FILE = 50;

static const uintptr_t TILE_HEIGHT = (HALFTILE_HEIGHT * 2);

static const uintptr_t TILE_WIDTH = (HALFTILE_WIDTH * 2);

struct Geometry {
  int32_t x;
  int32_t y;
  uint32_t w;
  uint32_t h;
};

extern "C" {

uint32_t geometry_h(const Geometry *geometry);

void geometry_set_data(Geometry *geometry, int32_t x, int32_t y, uint32_t w, uint32_t h);

void geometry_set_h(Geometry *geometry, uint32_t h);

void geometry_set_w(Geometry *geometry, uint32_t w);

void geometry_set_x(Geometry *geometry, int32_t x);

void geometry_set_y(Geometry *geometry, int32_t y);

uint32_t geometry_w(const Geometry *geometry);

int32_t geometry_x(const Geometry *geometry);

int32_t geometry_y(const Geometry *geometry);

} // extern "C"
