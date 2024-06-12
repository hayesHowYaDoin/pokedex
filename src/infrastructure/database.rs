use rusqlite::{Connection, Result};
use crate::shell::repositories::pokedex::{Pokemon, PokedexRepository, PokedexRepositoryError};

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

impl From<rusqlite::Error> for PokedexRepositoryError {
    fn from(err: rusqlite::Error) -> PokedexRepositoryError {
        type Error = rusqlite::Error;
        match err {
            Error::QueryReturnedNoRows => {
                return PokedexRepositoryError::PokemonNotFound(err.to_string());
            }
            _ => PokedexRepositoryError::ConnectionError(err.to_string()),
        }
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Database, PokedexRepositoryError> {
        let conn = Connection::open("tools/sqlite3/pokedex.db")?;
        Ok(Database { conn })
    }
}

impl PokedexRepository for Database {
    fn fetch(&self, number: i32) -> Result<Pokemon, PokedexRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon WHERE number = ?")?;
        let pokemon = stmt.query_row([number], |row| {
            Ok(Pokemon {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(Pokemon::from(pokemon))
    }

    fn fetch_all(&self) -> Result<Vec<Pokemon>, PokedexRepositoryError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon")?;
        let pokemon_iter = stmt.query_map([], |row| {
            Ok(Pokemon {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut pokemon = Vec::new();
        for p in pokemon_iter {
            pokemon.push(Pokemon::from(p?));
        }

        Ok(pokemon)
    }
}
