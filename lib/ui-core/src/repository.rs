use color_eyre::Result;
use thiserror::Error;

use crate::pages::{DetailPagePokemon, ListPagePokemon};

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ListPagePokemonRepositoryError(pub String);

pub trait ListPagePokemonRepository {
    fn fetch_all(&self) -> Result<Vec<ListPagePokemon>>;
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct DetailPagePokemonRepositoryError(pub String);

pub trait DetailPagePokemonRepository {
    fn fetch(&self, number: u32) -> Result<DetailPagePokemon>;
}
