

#[derive(Debug)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
}

#[derive(Debug)]
pub enum PokedexRepositoryError {
    ConnectionError(String),
    PokemonNotFound(String),
}

pub trait PokedexRepository {
    fn fetch(&self, number: i32) -> Result<Pokemon, PokedexRepositoryError>;
    fn fetch_all(&self) -> Result<Vec<Pokemon>, PokedexRepositoryError>;
}