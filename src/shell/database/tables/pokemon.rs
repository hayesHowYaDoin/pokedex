use rusqlite::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct PokemonDTO {
    pub number: i32,
    pub name: String,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum PokemonTableRepositoryError {
    ConnectionError(String),
    PokemonNotFound(String),
}

impl From<rusqlite::Error> for PokemonTableRepositoryError {
    fn from(err: rusqlite::Error) -> PokemonTableRepositoryError {
        type Error = rusqlite::Error;
        match err {
            Error::QueryReturnedNoRows => PokemonTableRepositoryError::PokemonNotFound(err.to_string()),
            _ => PokemonTableRepositoryError::ConnectionError(err.to_string()),
        }
    }
}

pub trait PokemonTableRepository {
    fn fetch(&self, number: i32) -> Result<PokemonDTO, PokemonTableRepositoryError>;
    fn fetch_all(&self) -> Result<Vec<PokemonDTO>, PokemonTableRepositoryError>;
}
