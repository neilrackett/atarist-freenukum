project(
    'freenukum',
    'c',
    version: '0.3.0',
    default_options: ['c_std=c11'],
)

src = [
    'src/fn_borders.c',
    'src/fn_bot.c',
    'src/fn_collision.c',
    'src/fn_data.c',
    'src/fn_drop.c',
    'src/fn_effect.c',
    'src/fn_environment.c',
    'src/fn_error.c',
    'src/fn_error_cmdline.c',
    'src/fn_game.c',
    'src/fn_hero.c',
    'src/fn_infobox.c',
    'src/fn_inputbox.c',
    'src/fn_inputfield.c',
    'src/fn_level.c',
    'src/fn_level_actor.c',
    'src/fn_list.c',
    'src/fn_mainmenu.c',
    'src/fn_menu.c',
    'src/fn_msgbox.c',
    'src/fn_picture.c',
    'src/fn_picture_splash.c',
    'src/fn_settings.c',
    'src/fn_shot.c',
    'src/fn_text.c',
    'src/fn_tile.c',
    'src/fn_tilecache.c',
    'src/fnactor.c',
    'src/fnactorgroup.c',
    'src/fngeometry.c',
    'src/fngraphics.c',
    'src/fnscreen.c',
    'src/fntexture.c',
    'src/freenukum.c'
]

sdl_lib = meson.get_compiler('c').find_library('SDL')
sdl_ttf_lib = meson.get_compiler('c').find_library('SDL_ttf')
curl_lib = meson.get_compiler('c').find_library('curl')
glib = dependency('gobject-2.0', version: '>=2.36')
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

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)


test_borders = executable(
    'test_borders',
    [
        'src/fn_test_borders.c',

        'src/fn_borders.c',
        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_drop = executable(
    'test_drop',
    [
        'src/fn_test_drop.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_drop.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_effect = executable(
    'test_effect',
    [
        'src/fn_test_effect.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_effect.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_picture.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
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

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_infobox = executable(
    'test_infobox',
    [
        'src/fn_test_infobox.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_inputbox = executable(
    'test_inputbox',
    [
        'src/fn_test_inputbox.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_inputbox.c',
        'src/fn_inputfield.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_level_loader = executable(
    'test_level_loader',
    [
        'src/fn_test_level_loader.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_mainmenu = executable(
    'test_mainmenu',
    [
        'src/fn_test_mainmenu.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_mainmenu.c',
        'src/fn_menu.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_menu = executable(
    'test_menu',
    [
        'src/fn_test_menu.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_menu.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_msgbox = executable(
    'test_msgbox',
    [
        'src/fn_test_msgbox.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_picture = executable(
    'test_picture',
    [
        'src/fn_test_picture.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_picture.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_picture_splash = executable(
    'test_picture_splash',
    [
        'src/fn_test_picture_splash.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_picture.c',
        'src/fn_picture_splash.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

test_settings = executable(
    'test_settings',
    [
        'src/fn_test_settings.c',

        'src/fn_settings.c',
    ],
)

test_tilecache = executable(
    'test_tilecache',
    [
        'src/fn_test_tilecache.c',

        'src/fn_bot.c',
        'src/fn_collision.c',
        'src/fn_data.c',
        'src/fn_environment.c',
        'src/fn_error.c',
        'src/fn_error_cmdline.c',
        'src/fn_hero.c',
        'src/fn_infobox.c',
        'src/fn_level.c',
        'src/fn_level_actor.c',
        'src/fn_list.c',
        'src/fn_msgbox.c',
        'src/fn_settings.c',
        'src/fn_shot.c',
        'src/fn_text.c',
        'src/fn_tile.c',
        'src/fn_tilecache.c',
        'src/fngeometry.c',
        'src/fngraphics.c',
        'src/fntexture.c',
    ],
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
)

executable(
    'freenukum',
    src,
    dependencies: [
        curl_lib,
        glib,
        sdl_lib,
        sdl_ttf_lib,
        zip_lib,
    ],
    include_directories: include_directories('.'),
)
