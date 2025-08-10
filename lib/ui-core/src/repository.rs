use std::future::Future;

use color_eyre::Result;
use thiserror::Error;

use crate::pages::{DetailPagePokemon, ListPagePokemon};

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ListPagePokemonRepositoryError(pub String);

pub trait ListPagePokemonRepository {
    fn fetch_all(&self) -> impl Future<Output = Result<Vec<ListPagePokemon>>> + Send;
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct DetailPagePokemonRepositoryError(pub String);

pub trait DetailPagePokemonRepository {
    fn fetch(&self, number: u32) -> impl Future<Output = Result<DetailPagePokemon>> + Send;
}
