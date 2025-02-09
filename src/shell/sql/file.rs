use std::path::PathBuf;
use std::sync::LazyLock;

pub static DATABASE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_DATABASE_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex"),
    });

pub static ASSETS_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_ASSETS_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/assets"),
    });
