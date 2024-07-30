use std::collections::HashMap;

use super::database::DatabaseError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PokemonID {
    pub number: i32,
}

impl PokemonID {
    pub fn new(number: i32) -> Self {
        PokemonID { number }
    }
}

#[derive(Debug)]
pub struct PokemonDTO {
    pub name: String,
}

impl PokemonDTO {
    pub fn new(name: String) -> Self {
        PokemonDTO { name }
    }
}

pub trait PokemonTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, number: i32) -> Result<PokemonDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, PokemonDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct TypesDTO {
    pub primary_type: String,
    pub secondary_type: Option<String>,
}

impl TypesDTO {
    pub fn new(primary_type: String, secondary_type: Option<String>) -> Self {
        TypesDTO {
            primary_type,
            secondary_type,
        }
    }
}

pub trait TypesTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, number: i32) -> Result<TypesDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, TypesDTO>, DatabaseError>;
}