#![allow(dead_code)]

mod core;
mod shell;

use color_eyre::eyre::Result;

use crate::shell::ratatui::app::App;
use crate::shell::sql::DatabaseMapper;

async fn tokio_main() -> Result<()> {
    color_eyre::install()?; // Enable colorized error output

    let db = DatabaseMapper::new().expect("Failed to create database connection");

    let mut app = App::new(Box::new(db)).expect("Failed to create application");
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
