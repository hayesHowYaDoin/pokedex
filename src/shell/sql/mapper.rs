use std::path::Path;

use color_eyre::Result;

use crate::core::pokemon::{
    PokemonNameRepository, 
    PokemonNameRepositoryError,
    PokemonNumberRepository,
    PokemonNumberRepositoryError,
    PokemonTypesRepository, 
    PokemonTypesRepositoryError,
};
use super::{
    Database,
    DatabaseError,
    tables::{PokemonTableRepository, TypesTableRepository},
};

pub struct DatabaseMapper {
    database: Database,
}

impl DatabaseMapper {
    pub fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        Ok(DatabaseMapper { database: Database::new(database_path)? })
    }
}

impl PokemonNameRepository for DatabaseMapper {
    fn fetch_name(&self, number: i32) -> Result<String, PokemonNameRepositoryError> {
        Ok(PokemonTableRepository::fetch(&self.database, number)?.name)
    }

    fn fetch_all_names(&self) -> Result<Vec<String>, PokemonNameRepositoryError> {
        Ok(PokemonTableRepository::fetch_all(&self.database)?
            .into_iter()
            .map(|pokemon| pokemon.name)
            .collect())
    }
}

impl PokemonNumberRepository for DatabaseMapper {
    fn fetch_all_numbers(&self) -> Result<Vec<i32>, PokemonNumberRepositoryError> {
        Ok(PokemonTableRepository::fetch_all(&self.database)?
            .into_iter()
            .map(|pokemon| pokemon.number)
            .collect())
    }
}

impl PokemonTypesRepository for DatabaseMapper {
    fn fetch_primary_type(&self, number: i32) -> Result<String, PokemonTypesRepositoryError> {
        let types = TypesTableRepository::fetch(&self.database, number).expect("Failed to fetch types");
        Ok(types.primary_type)
    }

    fn fetch_all_primary_types(&self) -> std::result::Result<Vec<String>, PokemonTypesRepositoryError> {
        Ok(TypesTableRepository::fetch_all(&self.database).expect("Failed to fetch all types")
            .into_iter()
            .map(|types| types.primary_type)
            .collect())
    }

    fn fetch_secondary_type(&self, number: i32) -> Result<Option<String>, PokemonTypesRepositoryError> {
        let types = TypesTableRepository::fetch(&self.database, number).expect("Failed to fetch types");
        Ok(types.secondary_type)
    }

    fn fetch_all_secondary_types(&self) -> std::result::Result<Vec<Option<String>>, PokemonTypesRepositoryError> {
        Ok(TypesTableRepository::fetch_all(&self.database).expect("Failed to fetch all types")
            .into_iter()
            .map(|types| types.secondary_type)
            .collect())
    }
}

impl From<DatabaseError> for PokemonNameRepositoryError {
    fn from(err: DatabaseError) -> PokemonNameRepositoryError {
        PokemonNameRepositoryError(err.to_string())
    }
}

impl From<DatabaseError> for PokemonNumberRepositoryError {
    fn from(err: DatabaseError) -> PokemonNumberRepositoryError {
        PokemonNumberRepositoryError(err.to_string())
    }
}

impl From<DatabaseError> for PokemonTypesRepositoryError {
    fn from(err: DatabaseError) -> PokemonTypesRepositoryError {
        PokemonTypesRepositoryError(err.to_string())
    }
}