project(
    'freenukum',
    'c',
    version: '0.3.0',
    default_options: ['c_std=c11'],
)

cargo = find_program('cargo', required: true)
cargo_script = find_program('scripts/cargo.sh')

profile = get_option('profile')

cargo_manifest = files(
    'Cargo.toml',
)

rust_src = files(
    'src/game.rs',
    'src/lib.rs',
    'src/sprite.rs',
    'src/text.rs',
    'src/tile.rs',
    'src/transition/geometry.rs',
    'src/transition/mod.rs',
    'src/transition/texture.rs',
)

cargo_build = custom_target(
    'cargo-build',
    build_by_default: true,
    command: [
        cargo_script, 
        '@SOURCE_ROOT@',
        '@OUTPUT@',
        meson.build_root(),
        profile,
    ],
    console: true,
    input: [
        rust_src,
        cargo_manifest,
    ],
    output: [
        'libfreenukum.so',
    ]
)

sdl_lib = meson.get_compiler('c').find_library('SDL')
sdl_ttf_lib = meson.get_compiler('c').find_library('SDL_ttf')
curl_lib = meson.get_compiler('c').find_library('curl')
zip_lib = meson.get_compiler('c').find_library('zip')

conf = configuration_data()
conf.set('version', meson.project_version())

configure_file(
    input: 'src/config.h.in',
    output: 'config.h',
    configuration: conf
)

configure_file(
    input: 'freenukum.desktop.in',
    output: 'freenukum.desktop',
    configuration: conf
)

test_tile = executable(
    'test_tile',
    [
        'src/fn_test_tile.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_picture = executable(
    'test_picture',
    [
        'src/fn_test_picture.c',

    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_picture_splash = executable(
    'test_picture_splash',
    [
        'src/fn_test_picture_splash.c',

    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_tilecache = executable(
    'test_tilecache',
    [
        'src/fn_test_tilecache.c',

    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum.so',
    ],
    link_depends: [
        cargo_build,
    ],
)
