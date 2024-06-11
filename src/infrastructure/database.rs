use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Pokemon {
    number: i32,
    name: String,
}

pub fn get_pokemon() -> Result<Vec<Pokemon>> {
    let conn = Connection::open("tools/sqlite3/pokedex.db")?;
    let mut stmt = conn.prepare("SELECT number, name FROM pokemon")?;
    let pokemon_iter = stmt.query_map([], |row| {
        Ok(Pokemon {
            number: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut pokemon = Vec::new();
    for p in pokemon_iter {
        pokemon.push(p?);
    }

    Ok(pokemon)
}