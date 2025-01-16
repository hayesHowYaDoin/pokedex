use std::{collections::HashMap, path::Path};

use itertools::Itertools;
use rusqlite::Connection;
use thiserror::Error;

use super::tables::{
    PokemonDTO, PokemonID, PokemonTableRepository, PokemonTypeDTO, PokemonTypeTableRepository,
    TypeID, TypesDTO, TypesTableRepository,
};

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
    fn fetch(&self, id: PokemonID) -> Result<PokemonDTO, DatabaseError> {
        let sql_cmd = "SELECT id, identifier, species_id FROM pokemon WHERE id = ?";
        let mut stmt = self
            .conn
            .prepare(sql_cmd)
            .expect("Failed to prepare statement");
        let pokemon = stmt
            .query_row([id], |row| {
                let identifier: String = row.get(1)?;
                let species_id: i32 = row.get(2)?;

                Ok(PokemonDTO::new(species_id, identifier))
            })
            .expect("Failed to execute query row");

        Ok(pokemon)
    }

    fn fetch_all(&self) -> Result<HashMap<PokemonID, PokemonDTO>, DatabaseError> {
        let sql_cmd = "SELECT id, identifier, species_id FROM pokemon";
        let mut stmt = self
            .conn
            .prepare(sql_cmd)
            .expect("Failed to prepare statement");
        let pokemon = stmt
            .query_map([], |row| {
                let id: i32 = row.get(0)?;
                let identifier: String = row.get(1)?;
                let species_id: i32 = row.get(2)?;

                Ok((PokemonID(id), PokemonDTO::new(species_id, identifier)))
            })
            .expect("Failed to execute query map")
            .filter_map(|t| t.ok())
            .collect();

        Ok(pokemon)
    }
}

impl TypesTableRepository for Database {
    fn fetch(&self, id: TypeID) -> Result<TypesDTO, DatabaseError> {
        let type_id_sql_cmd = "SELECT id, identifier FROM types WHERE id = ?";
        let mut stmt = self
            .conn
            .prepare(type_id_sql_cmd)
            .expect("Failed to prepare statement");
        let type_ = stmt
            .query_row([id], |row| {
                let identifier: String = row.get(1)?;

                Ok(TypesDTO::new(identifier))
            })
            .expect("Failed to execute query map");

        Ok(type_)
    }

    fn fetch_all(&self) -> Result<HashMap<TypeID, TypesDTO>, DatabaseError> {
        let sql_cmd = "SELECT id, identifier FROM types";
        let mut stmt = self
            .conn
            .prepare(sql_cmd)
            .expect("Failed to prepare statement");
        let types = stmt
            .query_map([], |row| {
                let id: i32 = row.get(0)?;
                let identifier: String = row.get(1)?;

                Ok((TypeID(id), TypesDTO::new(identifier)))
            })
            .expect("Failed to execute query map")
            .filter_map(|t| t.ok())
            .collect();

        Ok(types)
    }
}

impl PokemonTypeTableRepository for Database {
    fn fetch(&self, pokemon_id: PokemonID) -> Result<Vec<PokemonTypeDTO>, DatabaseError> {
        let sql_cmd = "SELECT pokemon_id, type_id, slot FROM pokemon_types WHERE pokemon_id = ?";
        let mut stmt = self
            .conn
            .prepare(sql_cmd)
            .expect("Failed to prepare statement");
        let pokemon_types = stmt
            .query_map([pokemon_id], |row| {
                let type_id: i32 = row.get(1)?;
                let slot: i32 = row.get(2)?;

                Ok(PokemonTypeDTO::new(TypeID(type_id), slot))
            })
            .expect("Failed to execute query map")
            .filter_map(|t| t.ok())
            .collect();

        Ok(pokemon_types)
    }

    fn fetch_all(&self) -> Result<HashMap<PokemonID, Vec<PokemonTypeDTO>>, DatabaseError> {
        let sql_cmd = "SELECT pokemon_id, type_id, slot FROM pokemon_types";
        let mut stmt = self
            .conn
            .prepare(sql_cmd)
            .expect("Failed to prepare statement");

        let pokemon_types = stmt
            .query_map([], |row| {
                let pokemon_id: i32 = row.get(0)?;
                let type_id: i32 = row.get(1)?;
                let slot: i32 = row.get(2)?;

                Ok((PokemonID(pokemon_id), PokemonTypeDTO::new(TypeID(type_id), slot)))
            })
            .expect("Failed to execute query map")
            .filter_map(|t| t.ok())
            .fold(HashMap::new(), |mut acc, (pokemon_id, pokemon_type)| {
                acc.entry(pokemon_id).or_insert_with(Vec::new).push(pokemon_type);
                acc  
            });

        Ok(pokemon_types)
    }
}
