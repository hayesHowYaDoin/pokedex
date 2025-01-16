use std::collections::HashMap;

use rusqlite::ToSql;

use super::database::DatabaseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PokemonID(pub u32);

impl ToSql for PokemonID {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(self.0.into())))
    }
}

impl Into<u32> for PokemonID {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeID(pub u32);

impl ToSql for TypeID {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(rusqlite::types::Value::Integer(self.0.into())))
    }
}

impl Into<u32> for TypeID {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct PokemonDTO {
    pub species_id: u32,
    pub identifier: String,
}

impl PokemonDTO {
    pub fn new(species_id: u32, identifier: String) -> Self {
        PokemonDTO { species_id, identifier }
    }
}

pub trait PokemonTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &PokemonID) -> Result<PokemonDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, PokemonDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct TypesDTO {
    pub identifier: String,
}

impl TypesDTO {
    pub fn new(identifier: String) -> Self {
        TypesDTO { identifier }
    }
}

pub trait TypesTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &TypeID) -> Result<TypesDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<TypeID, TypesDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonTypeDTO {
    pub id: TypeID,
    pub slot: u32,
}

impl PokemonTypeDTO {
    pub fn new(id: TypeID, slot: u32) -> Self {
        PokemonTypeDTO { id, slot }
    }
}

pub trait PokemonTypeTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &PokemonID) -> Result<Vec<PokemonTypeDTO>, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, Vec<PokemonTypeDTO>>, DatabaseError>;
}

pub struct PokemonSizeDTO {
    pub height_dm: u32,
    pub weight_hg: u32,
}

impl PokemonSizeDTO {
    pub fn new(height_dm: u32, weight_hg: u32) -> Self {
        PokemonSizeDTO { height_dm, weight_hg }
    }
}

pub trait PokemonSizeTableRepository {
    fn fetch(&self, id: &PokemonID) -> Result<PokemonSizeDTO, DatabaseError>;
}