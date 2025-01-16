use std::{any::Any, path::Path};

use color_eyre::Result;

use crate::core::ui::{
    pages::{DetailPagePokemon, ListPagePokemon},
    repository::{
        DetailPagePokemonRepository, DetailPagePokemonRepositoryError, ListPagePokemonRepository,
        ListPagePokemonRepositoryError,
    },
};

use super::{
    tables::{
        PokemonDTO, PokemonID, PokemonTableRepository, PokemonTypeTableRepository, TypesDTO,
        TypesTableRepository,
    },
    Database, DatabaseError,
};

pub struct DatabaseMapper {
    database: Database,
}

impl DatabaseMapper {
    pub fn new<P: AsRef<Path>>(database_path: P) -> Result<Self> {
        Ok(DatabaseMapper {
            database: Database::new(database_path)?,
        })
    }
}

impl ListPagePokemonRepository for DatabaseMapper {
    fn fetch_all(&self) -> Result<Vec<ListPagePokemon>> {
        let pokemon = PokemonTableRepository::fetch_all(&self.database)?;
        let pokemon_types = PokemonTypeTableRepository::fetch_all(&self.database)?;
        let types = TypesTableRepository::fetch_all(&self.database)?;

        let list_page_pokemon = pokemon
            .into_iter()
            .filter_map(|(id, p)| {
                let number = p.species_id;
                let name = capitalize(&p.identifier);

                let type_ids = pokemon_types.get(&id)?;
                let pokemon_types: Vec<String> = type_ids
                    .iter()
                    .filter_map(|pt| {
                        Some(capitalize(&types.get(&pt.id)?.identifier.clone()))
                    })
                    .collect();

                if (pokemon_types.len() != 1 && pokemon_types.len() != 2)
                    || pokemon_types.get(0).is_none()
                {
                    return None;
                }

                Some(ListPagePokemon::new(
                    number,
                    name,
                    pokemon_types.get(0)?.to_owned().into(),
                    pokemon_types
                        .get(1)
                        .map_or(None, |t| Some(t.to_owned().into())),
                ))
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

// impl DetailPagePokemonRepository for DatabaseMapper {
//     fn fetch(&self, number: i32) -> Result<Vec<DetailPagePokemon>> {
//         let pokemon = PokemonTableRepository::fetch(&self.database, number)?;
//         let types = TypesTableRepository::fetch(&self.database, number)?;

//         let detail_page_pokemon = pokemon.into_iter()
//             .filter_map(|(id, p)| {
//                 let t = types.get(&id)?;
//                 Some(DetailPagePokemon::new(
//                     id.number,
//                     p.name,
//                     p.description,
//                     t.primary_type.clone().into(),
//                     t.secondary_type.clone().map(|t| t.into()))
//                 )
//             })
//             .collect();

//         Ok(detail_page_pokemon)
//     }
// }

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
