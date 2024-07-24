use rusqlite::Result;
use thiserror::Error;

#[derive(Debug)]
pub struct TypesDTO {
    pub number: i32,
    pub primary_type: String,
    pub secondary_type: Option<String>,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub enum TypesTableRepositoryError {
    ConnectionError(String),
    TypesNotFound(String),
}

impl From<rusqlite::Error> for TypesTableRepositoryError {
    fn from(err: rusqlite::Error) -> TypesTableRepositoryError {
        type Error = rusqlite::Error;
        match err {
            Error::QueryReturnedNoRows => TypesTableRepositoryError::TypesNotFound(err.to_string()),
            _ => TypesTableRepositoryError::ConnectionError(err.to_string()),
        }
    }
}

pub trait TypesTableRepository {
    fn fetch(&self, number: i32) -> Result<TypesDTO, TypesTableRepositoryError>;
    fn fetch_all(&self) -> Result<Vec<TypesDTO>, TypesTableRepositoryError>;
}