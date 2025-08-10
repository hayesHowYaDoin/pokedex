use std::future::Future;
use std::pin::Pin;

use color_eyre::Result;

use crate::pages::{DetailPagePokemon, ListPagePokemon};

pub trait ListPagePokemonRepository {
    fn fetch_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<ListPagePokemon>>> + Send>>;
}

pub trait DetailPagePokemonRepository {
    fn fetch(&self, number: u32)
        -> Pin<Box<dyn Future<Output = Result<DetailPagePokemon>> + Send>>;
}
