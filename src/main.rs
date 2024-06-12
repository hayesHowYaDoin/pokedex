mod infrastructure;
mod shell;

use crate::infrastructure::database::Database;
use crate::shell::repositories::pokedex::PokedexRepository;

fn main() {
    println!("Gotta' fetch 'em all!");

    let db = Database::new().expect("Failed to create database connection");
    db.fetch_all().expect("Failed to fetch all Pokemon")
        .iter()
        .for_each(|pokemon| println!("{:?}", pokemon));
}
