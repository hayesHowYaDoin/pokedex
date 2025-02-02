use std::collections::HashMap;

use rusqlite::ToSql;

use super::database::DatabaseError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PokemonID(pub u32);

impl ToSql for PokemonID {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.0.into()),
        ))
    }
}

impl Into<u32> for PokemonID {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeID(pub u32);

impl ToSql for TypeID {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.0.into()),
        ))
    }
}

impl Into<u32> for TypeID {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatID(pub u32);

impl ToSql for StatID {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.0.into()),
        ))
    }
}

impl Into<u32> for StatID {
    fn into(self) -> u32 {
        self.0
    }
}

#[derive(Debug)]
pub struct PokemonDTO {
    pub species_id: u32,
    pub identifier: String,
}

impl PokemonDTO {
    pub fn new(species_id: u32, identifier: String) -> Self {
        PokemonDTO {
            species_id,
            identifier,
        }
    }
}

pub trait PokemonTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &PokemonID) -> Result<PokemonDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, PokemonDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct TypesDTO {
    pub identifier: String,
}

impl TypesDTO {
    pub fn new(identifier: String) -> Self {
        TypesDTO { identifier }
    }
}

pub trait TypesTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &TypeID) -> Result<TypesDTO, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<TypeID, TypesDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonTypeDTO {
    pub id: TypeID,
    pub slot: u32,
}

impl PokemonTypeDTO {
    pub fn new(id: TypeID, slot: u32) -> Self {
        PokemonTypeDTO { id, slot }
    }
}

pub trait PokemonTypeTableRepository {
    #[allow(dead_code)]
    fn fetch(&self, id: &PokemonID) -> Result<Vec<PokemonTypeDTO>, DatabaseError>;
    fn fetch_all(&self) -> Result<HashMap<PokemonID, Vec<PokemonTypeDTO>>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonSizeDTO {
    pub height_dm: u32,
    pub weight_hg: u32,
}

impl PokemonSizeDTO {
    pub fn new(height_dm: u32, weight_hg: u32) -> Self {
        PokemonSizeDTO {
            height_dm,
            weight_hg,
        }
    }
}

pub trait PokemonSizeTableRepository {
    fn fetch(&self, id: &PokemonID) -> Result<PokemonSizeDTO, DatabaseError>;
}

#[derive(Debug)]
pub struct StatNamesDTO {
    pub name: String,
}

impl StatNamesDTO {
    pub fn new(name: String) -> StatNamesDTO {
        StatNamesDTO { name }
    }
}

pub trait StatNamesRepository {
    fn fetch_all(&self) -> Result<HashMap<StatID, StatNamesDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonStatsDTO {
    pub base_stat: u32,
    pub effort: u32,
}

impl PokemonStatsDTO {
    pub fn new(base_stat: u32, effort: u32) -> PokemonStatsDTO {
        PokemonStatsDTO { base_stat, effort }
    }
}

pub trait PokemonStatsRepository {
    fn fetch(
        &self,
        pokemon_id: &PokemonID,
    ) -> Result<HashMap<StatID, PokemonStatsDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonDescriptionDTO {
    pub text: String,
}

impl PokemonDescriptionDTO {
    pub fn new(text: String) -> PokemonDescriptionDTO {
        PokemonDescriptionDTO { text }
    }
}

pub trait PokemonDescriptionsRepository {
    fn fetch(&self, pokemon_id: &PokemonID) -> Result<PokemonDescriptionDTO, DatabaseError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilityID(pub u32);

#[derive(Debug)]
pub struct AbilityDTO {
    pub identifier: String,
}

impl AbilityDTO {
    pub fn new(identifier: String) -> AbilityDTO {
        AbilityDTO { identifier }
    }
}

pub trait AbilitiesRepository {
    fn fetch_all(&self) -> Result<HashMap<AbilityID, AbilityDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonAbilitiesDTO {
    pub id: AbilityID,
}

impl PokemonAbilitiesDTO {
    pub fn new(id: AbilityID) -> PokemonAbilitiesDTO {
        PokemonAbilitiesDTO {id}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbilitySlot(pub u32);

pub trait PokemonAbilitiesRepository {
    fn fetch(&self, id: &PokemonID) -> Result<HashMap<AbilitySlot, PokemonAbilitiesDTO>, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonSpeciesNamesDTO {
  pub genus: String,
}

impl PokemonSpeciesNamesDTO {
  pub fn new(genus: String) -> PokemonSpeciesNamesDTO {
    PokemonSpeciesNamesDTO { genus }
  }
}

pub trait PokemonSpeciesNamesRepository {
  fn fetch(&self, id: &PokemonID) -> Result<PokemonSpeciesNamesDTO, DatabaseError>;
}

#[derive(Debug)]
pub struct PokemonGenderDTO {
  pub rate: i32,
}

impl PokemonGenderDTO {
  pub fn new(rate: i32) -> PokemonGenderDTO {
    PokemonGenderDTO { rate }
  }
}

pub trait PokemonGenderRepository {
  fn fetch(&self, id: &PokemonID) -> Result<PokemonGenderDTO, DatabaseError>;
}
