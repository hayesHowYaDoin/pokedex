use color_eyre::Result;
use thiserror::Error;

use crate::core::ui::pages::ListPagePokemon;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ListPagePokemonRepositoryError(pub String);

pub trait ListPagePokemonRepository {
    fn fetch_all(&self) -> Result<Vec<ListPagePokemon>>;
}
