#![allow(dead_code)]

use std::fs::File;

use clap::Parser;
use color_eyre::eyre::Result;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

use database::DatabaseMapper;
use settings::Settings;
use ui_shell::app::App;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run in silent mode (no sound effects)
    #[arg(short, long)]
    silent: bool,
}

async fn tokio_main() -> Result<()> {
    log::trace!("Connecting to database...");
    let db = DatabaseMapper::new().await?;

    log::trace!("Creating application...");
    let mut app = App::new(Box::new(db)).await?;

    log::trace!("Running application...");
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?; // Enable colorized error output

    let cli = Cli::parse();
    let settings = Settings::new(cli.silent);
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        {
            std::fs::create_dir_all(Settings::get_log_path().parent().unwrap()).unwrap();
            File::create(Settings::get_log_path()).unwrap()
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
