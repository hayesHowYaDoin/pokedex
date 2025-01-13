use std::path::Path;

use color_eyre::Result;

use crate::core::ui::{
    pages::ListPagePokemon,
    repository::{ListPagePokemonRepository, ListPagePokemonRepositoryError},
};

use super::{
    Database,
    DatabaseError,
    tables::{PokemonTableRepository, TypesTableRepository},
};

pub struct DatabaseMapper {
    database: Database,
}

impl DatabaseMapper {
    pub fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        Ok(DatabaseMapper { database: Database::new(database_path)? })
    }
}

impl ListPagePokemonRepository for DatabaseMapper {
    fn fetch_all(&self) -> Result<Vec<ListPagePokemon>> {
        let pokemon = PokemonTableRepository::fetch_all(&self.database)?;
        let types = TypesTableRepository::fetch_all(&self.database)?;

        let list_page_pokemon = pokemon.into_iter()
            .filter_map(|(id, p)| {
                let t = types.get(&id)?;
                Some(ListPagePokemon::new(
                    id.number,
                    p.name,
                    t.primary_type.clone().into(),
                    t.secondary_type.clone().map(|t| t.into()))
                )
            })
            .collect();

        Ok(list_page_pokemon)
    }
}

impl From<DatabaseError> for ListPagePokemonRepositoryError {
    fn from(err: DatabaseError) -> ListPagePokemonRepositoryError {
        ListPagePokemonRepositoryError(err.to_string())
    }
}
