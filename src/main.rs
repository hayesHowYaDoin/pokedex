mod shell;
mod core;

use color_eyre::eyre::Result;

use crate::shell::database::{Database, DatabaseMapper};
use crate::shell::ui::app::App;

async fn tokio_main() -> Result<()> {
    let db = Database::new().expect("Failed to create database connection");
    let db_mapper = DatabaseMapper::new(db);

    let mut app = App::new(&pokemon);
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
