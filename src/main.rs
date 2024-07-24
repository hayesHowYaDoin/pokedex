mod shell;
mod core;

use color_eyre::eyre::Result;

use crate::shell::sql::DatabaseMapper;
use crate::shell::ratatui::app::App;

async fn tokio_main() -> Result<()> {
    let db = DatabaseMapper::new("tools/sqlite3/pokedex.db")
        .expect("Failed to create database connection");

    let mut app = App::new(db).expect("Failed to create application");
    app.run().await?;
  
    Ok(())
  }

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } 
    else {
        Ok(())
    }
}
