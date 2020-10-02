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
    'src/borders.rs',
    'src/game.rs',
    'src/lib.rs',
    'src/settings.rs',
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
        'libfreenukum_rs.so',
    ]
)

c_src = [
    'src/fn_data.c',
    'src/fn_effect.c',
    'src/fn_environment.c',
    'src/fn_error.c',
    'src/fn_error_cmdline.c',
    'src/fn_game.c',
    'src/fn_hero.c',
    'src/fn_level.c',
    'src/fn_level_actor.c',
    'src/fn_list.c',
    'src/fn_picture_splash.c',
    'src/fn_shot.c',
    'src/freenukum.c'
]

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

test_list = executable(
    'test_list',
    [
        'src/fn_test_list.c',
        'src/fn_list.c',
    ]
)

test_tile = executable(
    'test_tile',
    [
        'src/fn_test_tile.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)


test_borders = executable(
    'test_borders',
    [
        'src/fn_test_borders.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_drop = executable(
    'test_drop',
    [
        'src/fn_test_drop.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_effect = executable(
    'test_effect',
    [
        'src/fn_test_effect.c',

        'src/fn_data.c',
        'src/fn_effect.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_error = executable(
    'test_error',
    [
        'src/fn_test_error.c',

        'src/fn_error.c',
    ],
)

test_hero = executable(
    'test_hero',
    [
        'src/fn_test_hero.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_infobox = executable(
    'test_infobox',
    [
        'src/fn_test_infobox.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_inputbox = executable(
    'test_inputbox',
    [
        'src/fn_test_inputbox.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_level_loader = executable(
    'test_level_loader',
    [
        'src/fn_test_level_loader.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_menu = executable(
    'test_menu',
    [
        'src/fn_test_menu.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_msgbox = executable(
    'test_msgbox',
    [
        'src/fn_test_msgbox.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_picture = executable(
    'test_picture',
    [
        'src/fn_test_picture.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_picture_splash = executable(
    'test_picture_splash',
    [
        'src/fn_test_picture_splash.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_picture_splash.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

test_tilecache = executable(
    'test_tilecache',
    [
        'src/fn_test_tilecache.c',

        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_shot.c',
    ],
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)

executable(
    'freenukum',
    c_src,
    dependencies: [
        curl_lib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    include_directories: include_directories('.'),
    objects: [
        'build/libfreenukum_rs.so',
    ],
    link_depends: [
        cargo_build,
    ],
)
