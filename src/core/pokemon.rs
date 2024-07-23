use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum PokemonRepositoryError {
    ConnectionError(String),
    PokemonNotFound(String),
}

pub trait PokemonRepository {
    fn fetch(&self, number: i32) -> Result<Pokemon, PokemonRepositoryError>;
    fn fetch_all(&self) -> Result<Vec<Pokemon>, PokemonRepositoryError>;
}