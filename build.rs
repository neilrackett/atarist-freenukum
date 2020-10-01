extern crate cbindgen;
use cbindgen::{Builder, Config, EnumConfig};

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut enum_config = EnumConfig::default();
    enum_config.prefix_with_name = true;
    let mut config = Config::default();
    config.enumeration = enum_config;

    Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_pragma_once(true)
        .with_language(cbindgen::Language::C)
        .with_tab_width(4)
        .with_line_length(78)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/rusted.h");
}
