use std::path::Path;

use color_eyre::Result;

use crate::core::{
    pokemon::{Pokemon, PokemonAttributes, PokemonCry, PokemonDescription, PokemonGenders, PokemonStats, PokemonTypes},
    ui::{
        pages::{DetailPagePokemon, ListPagePokemon},
        repository::{
            DetailPagePokemonRepository, DetailPagePokemonRepositoryError,
            ListPagePokemonRepository, ListPagePokemonRepositoryError,
        },
    },
};

use super::{
    tables::{
        PokemonID, PokemonSizeTableRepository, PokemonTableRepository,
        PokemonTypeTableRepository, TypesTableRepository,
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
                    .filter_map(|pt| Some(capitalize(&types.get(&pt.id)?.identifier.clone())))
                    .collect();

                if (pokemon_types.len() != 1 && pokemon_types.len() != 2)
                    || pokemon_types.get(0).is_none()
                {
                    return None;
                }

                Some(ListPagePokemon::new(
                    number,
                    name,
                    PokemonTypes::new(
                        pokemon_types.get(0)?.to_owned().into(),
                        pokemon_types
                            .get(1)
                            .map_or(None, |t| Some(t.to_owned().into())),
                    ),
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

impl DetailPagePokemonRepository for DatabaseMapper {
    fn fetch(&self, number: u32) -> Result<DetailPagePokemon> {
        let id = PokemonID(number);
        let pokemon = PokemonTableRepository::fetch(&self.database, &id)?;
        let pokemon_types = PokemonTypeTableRepository::fetch(&self.database, &id)?;
        let types = TypesTableRepository::fetch_all(&self.database)?;
        let pokemon_size = PokemonSizeTableRepository::fetch(&self.database, &id)?;

        let image_path = format!("./data/assets/{}/bw_front.png", Into::<u32>::into(id));
        let image = image::ImageReader::open(image_path)
            .expect("Unable to open image.")
            .decode()
            .unwrap()
            .resize(3000, 3000, image::imageops::FilterType::Nearest);

        let cry_bytes = std::fs::read(format!("./data/assets/{}/cry.wav", Into::<u32>::into(id)))
            .expect("Unable to locate cry resource.");
        let cry = PokemonCry::new(cry_bytes);

        let pokemon_types: Vec<String> = pokemon_types
            .iter()
            .filter_map(|pt| Some(capitalize(&types.get(&pt.id)?.identifier.clone())))
            .collect();

        if (pokemon_types.len() != 1 && pokemon_types.len() != 2) || pokemon_types.get(0).is_none()
        {
            return Err(DetailPagePokemonRepositoryError(
                "Pokemon has invalid number of types.".to_string(),
            )
            .into());
        }

        let description = PokemonDescription::new(
            "A strange seed was planted on its back at birth. The plant sprouts and grows with this POKéMON.".to_string()
        );

        let attributes = PokemonAttributes::new(
            pokemon_size.height_dm.to_string(),
            pokemon_size.weight_hg.to_string(),
            "Seed".to_string(),
            vec!["Overgrow".to_string()],
            [PokemonGenders::Male, PokemonGenders::Female]
                .into_iter()
                .collect(),
        );

        let stats = PokemonStats::new(45, 49, 49, 65, 65, 45);

        let detail_page_pokemon = DetailPagePokemon::new(
            pokemon.species_id,
            capitalize(&pokemon.identifier),
            image,
            PokemonTypes::new(
                pokemon_types.get(0).unwrap().to_owned().into(),
                pokemon_types
                    .get(1)
                    .map_or(None, |t| Some(t.to_owned().into())),
            ),
            description,
            attributes,
            stats,
            cry,
        );

        Ok(detail_page_pokemon)
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
