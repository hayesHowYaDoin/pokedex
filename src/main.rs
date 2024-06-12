mod infrastructure;
mod shell;

use crate::infrastructure::database::Database;
use crate::infrastructure::tui::run_tui;
use crate::shell::repositories::pokedex::PokedexRepository;

fn main() {
    println!("Gotta' fetch 'em all!");

    let db = Database::new().expect("Failed to create database connection");
    let _pokemon = db.fetch_all().expect("Failed to fetch all Pokemon");

    run_tui().expect("Failed to run TUI");
}
