use crate::core::pokemon::{
    PokemonNameRepository, 
    PokemonNameRepositoryError, 
    PokemonTypesRepository, 
    PokemonTypesRepositoryError,
};
use super::{
    Database,
    tables::{
        pokemon::{PokemonTableRepository, PokemonTableRepositoryError},
        types::{TypesTableRepository, TypesTableRepositoryError},
    },
};

pub struct DatabaseMapper {
    database: Database,
}

impl DatabaseMapper {
    pub fn new(database: Database) -> Self {
        DatabaseMapper { database }
    }
}

impl PokemonNameRepository for DatabaseMapper {
    fn fetch_name(&self, number: i32) -> Result<String, PokemonNameRepositoryError> {
        Ok(PokemonTableRepository::fetch(&self.database, number)?.name)
    }
}

impl PokemonTypesRepository for DatabaseMapper {
    fn fetch_primary_type(&self, number: i32) -> Result<String, PokemonTypesRepositoryError> {
        Ok(TypesTableRepository::fetch(&self.database, number)?.primary_type)
    }

    fn fetch_secondary_type(&self, number: i32) -> Result<Option<String>, PokemonTypesRepositoryError> {
        Ok(TypesTableRepository::fetch(&self.database, number)?.secondary_type)
    }
}

impl From<PokemonTableRepositoryError> for PokemonNameRepositoryError {
    fn from(err: PokemonTableRepositoryError) -> PokemonNameRepositoryError {
        PokemonNameRepositoryError(err.to_string())
    }
}

impl From<TypesTableRepositoryError> for PokemonTypesRepositoryError {
    fn from(err: TypesTableRepositoryError) -> PokemonTypesRepositoryError {
        PokemonTypesRepositoryError(err.to_string())
    }
}