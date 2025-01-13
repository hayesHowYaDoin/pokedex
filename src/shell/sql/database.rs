use std::path::Path;
use std::collections::HashMap;

use thiserror::Error;
use rusqlite::Connection;

use super::tables::{PokemonDTO, PokemonID, PokemonTableRepository, TypesDTO, TypesTableRepository};

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
        let sql_cmd = "SELECT number, name FROM pokemon WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd).expect("Failed to prepare statement");
        let pokemon = stmt.query_row([number], |row| {
            Ok(PokemonDTO::new(row.get(1).expect("Failed to get name")))
        }).expect("Failed to execute query row");

        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<HashMap<PokemonID, PokemonDTO>, DatabaseError> {
        let sql_cmd = "SELECT number, name FROM pokemon";
        let mut stmt = self.conn.prepare(sql_cmd).expect("Failed to prepare statement");
        let pokemon = stmt.query_map([], |row| {
            let id = PokemonID::new(row.get(0).expect("Failed to get number"));
            let pokemon = PokemonDTO::new(row.get(1).expect("Failed to get name"));
            Ok((id, pokemon))
        }).expect("Failed to execute query map")
        .filter_map(|t| t.ok())
        .collect();

        Ok(pokemon)
    }
}

impl TypesTableRepository for Database {
    fn fetch(&self, number: i32) -> Result<TypesDTO, DatabaseError> {
        let sql_cmd = "SELECT number, primary_type, secondary_type FROM pokemon_types WHERE number = ?";
        let mut stmt = self.conn.prepare(sql_cmd).expect("Failed to prepare statement");
        let types = stmt.query_row([number], |row| {
            Ok(TypesDTO::new(
                row.get(1).expect("Failed to get primary type"),
                row.get(2).expect("Failed to get secondary type")
            ))
        }).expect("Failed to execute query row");

        Ok(types)
    }

    fn fetch_all(&self) -> Result<HashMap<PokemonID, TypesDTO>, DatabaseError> {
        let sql_cmd = "SELECT number, primary_type, secondary_type FROM pokemon_types";
        let mut stmt = self.conn.prepare(sql_cmd).expect("Failed to prepare statement");
        let types = stmt.query_map([], |row| {
                let id = PokemonID::new(row.get(0).expect("Failed to get number"));
                let types = TypesDTO::new(
                    row.get(1).expect("Failed to get primary type"),
                    row.get(2).expect("Failed to get secondary type")
                );
                Ok((id, types))
            }).expect("Failed to execute query map")
            .filter_map(|t| t.ok())
            .collect();

        Ok(types)
    }
}
