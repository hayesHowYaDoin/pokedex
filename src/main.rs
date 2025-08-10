#![allow(dead_code)]

use std::fs::File;
use std::path::PathBuf;
use std::sync::LazyLock;

use color_eyre::eyre::Result;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

use database::DatabaseMapper;
use ui_shell::app::App;

pub static LOG_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| match std::env::var("POKEDEX_LOG_PATH").ok() {
        Some(p) => PathBuf::from(p.trim_end_matches('/')),
        None => PathBuf::from("/usr/share/rich_pokedex/pokedex.log"),
    });

async fn tokio_main() -> Result<()> {
    log::trace!("Connecting to database...");
    let db = DatabaseMapper::new().expect("Failed to create database connection");

    log::trace!("Starting application...");
    let mut app = App::new(Box::new(db)).expect("Failed to create application");
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?; // Enable colorized error output
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        {
            std::fs::create_dir_all(LOG_PATH.parent().unwrap()).unwrap();
            File::create(&*LOG_PATH).unwrap()
        },
    )])
    .unwrap();

    if let Err(e) = tokio_main().await {
        log::error!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
