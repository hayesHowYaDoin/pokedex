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
    fn resolve_paths() -> (PathBuf, PathBuf, PathBuf) {
        // Try environment variables first (development/custom setups)
        if let Ok(db) = std::env::var("POKEDEX_DATABASE_PATH") {
            let log = std::env::var("POKEDEX_LOG_PATH")
                .unwrap_or_else(|_| "/tmp/pokedex.log".to_string());
            let assets = std::env::var("POKEDEX_ASSETS_PATH")
                .unwrap_or_else(|_| "/usr/share/pokedex/assets".to_string());

            return (
                PathBuf::from(log.trim_end_matches('/')),
                PathBuf::from(db.trim_end_matches('/')),
                PathBuf::from(assets.trim_end_matches('/')),
            );
        }

        // Check if running from a Nix store path
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_str) = exe_path.to_str() {
                if exe_str.contains("/nix/store/") {
                    // Running from Nix store, use relative paths
                    // Binary is at /nix/store/.../bin/pokedex
                    // Data is at /nix/store/.../share/pokedex/
                    if let Some(store_path) = exe_path.parent().and_then(|p| p.parent()) {
                        let share_dir = store_path.join("share/pokedex");
                        let log_dir = std::env::temp_dir().join("pokedex");
                        let _ = std::fs::create_dir_all(&log_dir); // Best effort

                        return (
                            log_dir.join("pokedex.log"),
                            share_dir.join("pokedex.db"),
                            share_dir.join("assets"),
                        );
                    }
                }
            }
        }

        // Fallback to standard Unix paths (e.g., Debian package)
        (
            PathBuf::from("/tmp/pokedex.log"),
            PathBuf::from("/usr/share/pokedex/pokedex.db"),
            PathBuf::from("/usr/share/pokedex/assets"),
        )
    }

    pub fn init(is_silent: bool) {
        let (log_path, database_path, assets_path) = Self::resolve_paths();

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
