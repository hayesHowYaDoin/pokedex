use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static LOG_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_LOG_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/pokedex.log"),
    });

pub static DATABASE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_DATABASE_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/pokedex.db"),
    });

pub static ASSETS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_ASSETS_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/assets"),
    });

pub struct Settings {
    is_silent: bool,
}

impl Settings {
    pub fn new(is_silent: bool) -> Self {
        Settings { is_silent }
    }

    pub fn get_log_path() -> &'static Path {
        (*LOG_PATH).as_path()
    }

    pub fn get_database_path() -> &'static Path {
        (*DATABASE_PATH).as_path()
    }

    pub fn get_assets_path() -> &'static Path {
        (*ASSETS_PATH).as_path()
    }

    pub fn is_silent(&self) -> bool {
        self.is_silent
    }
}
