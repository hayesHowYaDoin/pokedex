mod shell;
mod core;

use color_eyre::eyre::Result;

use crate::shell::database::Database;
use crate::core::repositories::pokedex::PokedexRepository;
use crate::shell::ui::app::App;

async fn tokio_main() -> Result<()> {
    let db = Database::new().expect("Failed to create database connection");
    let _pokemon = db.fetch_all().expect("Failed to fetch all Pokemon");

    let mut app = App::new();
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
