use super::database::DatabaseError;

#[derive(Debug)]
pub struct PokemonDTO {
    pub id: i32,
    pub identifier: String,
    pub species_id: i32,
}

impl PokemonDTO {
    pub fn new(id: i32, identifier: String, species_id: i32) -> Self {
        PokemonDTO { id, identifier, species_id }
    }
}

pub trait PokemonTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, number: i32) -> Result<PokemonDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<Vec<PokemonDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct TypesDTO {
    pub id: i32,
    pub identifier: String,
}

impl TypesDTO {
    pub fn new(id: i32, identifier: String) -> Self {
        TypesDTO { id, identifier }
    }
}

pub trait TypesTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: i32) -> Result<TypesDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<Vec<TypesDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonTypeDTO {
    pub pokemon_id: i32,
    pub type_id: i32,
    pub slot: i32,
}

impl PokemonTypeDTO {
    pub fn new(pokemon_id: i32, type_id: i32, slot: i32) -> Self {
        PokemonTypeDTO { pokemon_id, type_id, slot }
    }
}

pub trait PokemonTypeTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, pokemon_id: i32) -> Result<Vec<PokemonTypeDTO>, DatabaseError>;
    fn fetch_all(&self) -> Result<Vec<Vec<PokemonTypeDTO>>, DatabaseError>;
}
