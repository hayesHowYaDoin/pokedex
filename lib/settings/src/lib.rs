use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

#[derive(Debug)]
pub struct Settings {
    log_path: PathBuf,
    database_path: PathBuf,
    assets_path: PathBuf,
    is_silent: bool,
}

impl Settings {
    pub fn init(is_silent: bool) {
        let log_path = match std::env::var("POKEDEX_LOG_PATH").ok() {
            Some(p) => PathBuf::from(p.trim_end_matches('/')),
            None => PathBuf::from("/usr/share/rich_pokedex/pokedex.log"),
        };
        let database_path = match std::env::var("POKEDEX_DATABASE_PATH").ok() {
            Some(p) => PathBuf::from(p.trim_end_matches('/')),
            None => PathBuf::from("/usr/share/rich_pokedex/pokedex.db"),
        };
        let assets_path = match std::env::var("POKEDEX_ASSETS_PATH").ok() {
            Some(p) => PathBuf::from(p.trim_end_matches('/')),
            None => PathBuf::from("/usr/share/rich_pokedex/assets"),
        };

        SETTINGS
            .set(Settings {
                log_path,
                database_path,
                assets_path,
                is_silent,
            })
            .expect("Settings can only be initialized once");
    }

    fn get() -> &'static Settings {
        SETTINGS.get().expect("Settings have not been initialized")
    }

    pub fn log_path() -> &'static Path {
        Settings::get().log_path.as_path()
    }

    pub fn database_path() -> &'static Path {
        Settings::get().database_path.as_path()
    }

    pub fn assets_path() -> &'static Path {
        Settings::get().assets_path.as_path()
    }

    pub fn is_silent() -> bool {
        Settings::get().is_silent
    }
}
