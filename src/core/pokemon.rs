use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
    pub types: PokemonTypes,
}

impl Pokemon {
    pub fn new(number: i32, name: String, primary_type: String, secondary_type: Option<String>) -> Self {
        Pokemon {
            number,
            name,
            types: PokemonTypes {
                primary: primary_type,
                secondary: secondary_type,
            },
        }
    }
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct PokemonNumberRepositoryError(pub String);

#[derive(Debug, Error)]
#[error("{0}")]
pub struct PokemonNameRepositoryError(pub String);

#[derive(Clone, Debug)]
pub struct PokemonTypes {
    pub primary: String,
    pub secondary: Option<String>,
}

impl PokemonTypes {
    pub fn new(primary: String, secondary: Option<String>) -> Self {
        PokemonTypes {
            primary,
            secondary,
        }
    }
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct PokemonTypesRepositoryError(pub String);

pub trait PokemonNumberRepository {
    fn fetch_all_numbers(&self) -> Result<Vec<i32>, PokemonNumberRepositoryError>;
}

pub trait PokemonNameRepository {
    fn fetch_name(&self, number: i32) -> Result<String, PokemonNameRepositoryError>;
    fn fetch_all_names(&self) -> Result<Vec<String>, PokemonNameRepositoryError>;
}

pub trait PokemonTypesRepository {
    fn fetch_primary_type(&self, number: i32) -> Result<String, PokemonTypesRepositoryError>;
    fn fetch_all_primary_types(&self) -> Result<Vec<String>, PokemonTypesRepositoryError>;
    fn fetch_secondary_type(&self, number: i32) -> Result<Option<String>, PokemonTypesRepositoryError>;
    fn fetch_all_secondary_types(&self) -> Result<Vec<Option<String>>, PokemonTypesRepositoryError>;
}