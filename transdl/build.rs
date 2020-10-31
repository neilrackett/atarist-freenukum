fn pkg_config_print(lib_name: &str) {
    pkg_config::Config::new()
        .statik(false)
        .probe(lib_name)
        .unwrap();
}

fn main() {
    pkg_config_print("sdl");
    pkg_config_print("SDL_ttf");
}
