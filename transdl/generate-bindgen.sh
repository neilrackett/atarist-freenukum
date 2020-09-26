#!/bin/sh

# blacklisted functions want to use u128 which isn't supported by ffi
bindgen \
    --blacklist-function "strtold" \
    --blacklist-function "strtol" \
    --blacklist-function "qecvt" \
    --blacklist-function "qfcvt" \
    --blacklist-function "qgcvt" \
    --blacklist-function "qecvt_r" \
    --blacklist-function "qfcvt_r" \
    /usr/include/SDL/SDL_ttf.h > src/ll/ttf.rs

# blacklisted functions want to use u128 which isn't supported by ffi
bindgen \
    --blacklist-function "strtold" \
    --blacklist-function "strtol" \
    --blacklist-function "qecvt" \
    --blacklist-function "qfcvt" \
    --blacklist-function "qgcvt" \
    --blacklist-function "qecvt_r" \
    --blacklist-function "qfcvt_r" \
    /usr/include/SDL/SDL_video.h > src/ll/video.rs
