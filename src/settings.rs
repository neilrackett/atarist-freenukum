use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pixelsize: u8,
    pub fullscreen: bool,
    pub draw_collision_bounds: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            pixelsize: 1,
            fullscreen: false,
            draw_collision_bounds: false,
        }
    }
}

impl Settings {
    fn savepath() -> PathBuf {
        crate::config_dir().join("settings.toml")
    }

    pub fn load_or_create() -> Settings {
        let path = Self::savepath();
        match std::fs::read_to_string(&path) {
            Ok(s) => match toml::de::from_str(&s) {
                Ok(settings) => settings,
                Err(e) => {
                    eprintln!(
                        "Error reading from settings file {:?}: {:?}",
                        path.to_string_lossy(),
                        e
                    );
                    eprintln!("Using default settings");
                    Settings::default()
                }
            },
            Err(e) => {
                eprintln!(
                    "Error opening settings file {:?}: {:?}",
                    path.to_string_lossy(),
                    e
                );
                eprintln!("Using default settings");
                Settings::default()
            }
        }
    }

    pub fn save(&self) {
        let config_dir = crate::config_dir();
        match std::fs::create_dir_all(&config_dir) {
            Ok(()) => {}
            Err(e) => {
                eprintln!(
                    "Error creating config directory {:?}: {:?}",
                    config_dir.to_string_lossy(),
                    e
                );
                return;
            }
        }

        let savepath = Self::savepath();
        match std::fs::write(
            &savepath,
            toml::ser::to_string(self).unwrap().as_bytes(),
        ) {
            Ok(()) => {}
            Err(e) => {
                eprintln!(
                    "Error saving config file {:?}: {:?}",
                    savepath.to_string_lossy(),
                    e
                );
            }
        }
    }
}
