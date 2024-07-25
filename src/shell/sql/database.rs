use std::path::Path;

use thiserror::Error;
use rusqlite::Connection;

use super::tables::{PokemonDTO, PokemonTableRepository, TypesDTO, TypesTableRepository};

pub struct Database {
    conn: Connection,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum DatabaseError {
    ConnectionError(String),
    FetchError(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(err: rusqlite::Error) -> DatabaseError {
        type Error = rusqlite::Error;
        match err {
            Error::QueryReturnedNoRows => DatabaseError::FetchError(err.to_string()),
            _ => DatabaseError::ConnectionError(err.to_string()),
        }
    }
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Database, DatabaseError> {
        let conn = Connection::open(path)?;
        Ok(Database { conn })
    }
}

impl PokemonTableRepository for Database {
    fn fetch(&self, number: i32) -> Result<PokemonDTO, DatabaseError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon WHERE number = ?")?;
        let pokemon = stmt.query_row([number], |row| {
            Ok(PokemonDTO {
                number: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<Vec<PokemonDTO>, DatabaseError> {
        let mut stmt = self.conn.prepare("SELECT number, name FROM pokemon")?;
        let pokemon_iter = stmt.query_map([], |row| {
            Ok(PokemonDTO {
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

impl TypesTableRepository for Database {
    fn fetch(&self, number: i32) -> Result<TypesDTO, DatabaseError> {
        let sql_cmd = "SELECT number, primary_type, secondary_type FROM pokemon_types WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd)?;
        let types = stmt.query_row([number], |row| {
            Ok(TypesDTO {
                number: row.get(0)?,
                primary_type: row.get(1)?,
                secondary_type: row.get(2)?,
            })
        })?;

        Ok(types)
    }

    fn fetch_all(&self) -> Result<Vec<TypesDTO>, DatabaseError> {
        let sql_cmd = "SELECT number, primary_type, secondary_type FROM pokemon_types WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd)?;
        let types_iter = stmt.query_map([], |row| {
                Ok(TypesDTO {
                    number: row.get(0)?,
                    primary_type: row.get(1)?,
                    secondary_type: row.get(2)?,
                })
            })?
            .map(|t| t.unwrap())
            .collect();

        Ok(types_iter)
    }
}
