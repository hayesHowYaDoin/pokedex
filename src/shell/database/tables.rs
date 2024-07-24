use super::database::DatabaseError;

#[derive(Debug)]
pub struct PokemonDTO {
    pub number: i32,
    pub name: String,
}

pub trait PokemonTableRepository {
    fn fetch(&self, number: i32) -> Result<PokemonDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<Vec<PokemonDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct TypesDTO {
    pub number: i32,
    pub primary_type: String,
    pub secondary_type: Option<String>,
}

pub trait TypesTableRepository {
    fn fetch(&self, number: i32) -> Result<TypesDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<Vec<TypesDTO>, DatabaseError>;
}