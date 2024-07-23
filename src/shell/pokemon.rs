use rusqlite::{Connection, Result};
use crate::core::pokemon::{Pokemon, PokemonRepository, PokemonRepositoryError};

#[derive(Debug)]
struct PokemonDTO {
    pub number: i32,
    pub name: String,
}

impl From<PokemonDTO> for Pokemon {
    fn from(dto: PokemonDTO) -> Pokemon {
        Pokemon {
            number: dto.number,
            name: dto.name,
        }
    }
}

impl From<rusqlite::Error> for PokemonRepositoryError {
    fn from(err: rusqlite::Error) -> PokemonRepositoryError {
        type Error = rusqlite::Error;
        match err {
            Error::QueryReturnedNoRows => PokemonRepositoryError::PokemonNotFound(err.to_string()),
            _ => PokemonRepositoryError::ConnectionError(err.to_string()),
        }
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, PokemonRepositoryError> {
        let conn = Connection::open("tools/sqlite3/pokedex.db")?;
        Ok(Database { conn })
    }
}

impl PokemonRepository for Database {
    fn fetch(&self, number: i32) -> Result<Pokemon, PokemonRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon WHERE number = ?")?;
        let pokemon = stmt.query_row([number], |row| {
            Ok(Pokemon {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, PokemonRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon")?;
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
}
