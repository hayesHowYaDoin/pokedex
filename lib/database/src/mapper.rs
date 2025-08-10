use std::collections::HashMap;
use std::future::Future;

use color_eyre::{eyre::eyre, Result};

use crate::{
    file::{ASSETS_PATH, DATABASE_PATH},
    tables::*,
};
use pokemon::{
    PokemonAttributes, PokemonCry, PokemonDescription, PokemonGenderRates, PokemonStats,
    PokemonTypes,
};
use string::{capitalize, capitalize_words};
use ui_core::{
    pages::{DetailPagePokemon, ListPagePokemon},
    repository::{
        DetailPagePokemonRepository, ListPagePokemonRepository, ListPagePokemonRepositoryError,
    },
};

pub struct DatabaseMapper {
    pool: sqlx::SqlitePool,
}

impl DatabaseMapper {
    pub async fn new() -> Result<Self> {
        let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}", DATABASE_PATH.display()))
            .await
            .map_err(|e| eyre!("Failed to connect to database: {}", e))?;
        Ok(Self { pool })
    }
}

impl ListPagePokemonRepository for DatabaseMapper {
    fn fetch_all(&self) -> impl Future<Result<Vec<ListPagePokemon>>> + Send {
        let pokemon = PokemonDTO::read_all(&self.pool).await?;
        let pokemon_types = PokemonTypeDTO::read_all(&self.pool).await?;
        let types = TypeDTO::read_all(&self.pool).await?;

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
                    || pokemon_types.is_empty()
                {
                    return None;
                }

                Some(ListPagePokemon::new(
                    number,
                    name,
                    PokemonTypes::new(
                        pokemon_types.first()?.to_owned().into(),
                        pokemon_types.get(1).map(|t| t.to_owned().into()),
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
    async fn fetch(&self, number: u32) -> impl Future<Result<DetailPagePokemon>> + Send {
        let pokemon = PokemonDTO::read(&self.pool, number as i64).await?;
        let pokemon_types = PokemonTypeDTO::read(&self.pool, number as i64).await?;
        let pokemon_size = PokemonSizeDTO::read(&self.pool, number as i64).await?;
        let pokemon_stats_dto = PokemonStatsDTO::read(&self.pool, number as i64).await?;
        let types = TypeDTO::read_all(&self.pool).await?;
        let pokemon_description = PokemonDescriptionDTO::read(&self.pool, number as i64).await?;
        let abilities = AbilityDTO::read_all(&self.pool).await?;
        let pokemon_abilities = PokemonAbilitiesDTO::read(&self.pool, number as i64).await?;
        let pokemon_species = PokemonSpeciesNamesDTO::read(&self.pool, number as i64).await?;
        let pokemon_gender = PokemonGenderDTO::read(&self.pool, number as i64).await?;

        let detail_page_pokemon = DetailPagePokemon::new(
            pokemon.species_id,
            capitalize(&pokemon.identifier),
            build_image(number)?,
            build_types(pokemon_types, types)?,
            build_description(pokemon_description),
            build_attributes(
                pokemon_size,
                abilities,
                pokemon_abilities,
                pokemon_species,
                pokemon_gender,
            )?,
            build_stats(pokemon_stats_dto),
            build_cry(number)?,
        );

        Ok(detail_page_pokemon)
    }
}

fn build_image(id: u32) -> Result<image::DynamicImage> {
    let image_path = ASSETS_PATH.join(format!("{}/bw_front.png", Into::<u32>::into(id)));
    Ok(image::ImageReader::open(image_path)
        .expect("Unable to open image.")
        .decode()?)
}

fn build_cry(id: u32) -> Result<PokemonCry> {
    let cry_path = ASSETS_PATH.join(format!("{}/cry.wav", Into::<u32>::into(id)));
    let cry_bytes = std::fs::read(cry_path)?;
    Ok(PokemonCry::new(cry_bytes))
}

fn build_stats(pokemon_stats_dto: PokemonStatsDTO) -> PokemonStats {
    PokemonStats::new(
        pokemon_stats_dto.get(&StatID(1)).map_or(0, |s| s.base_stat),
        pokemon_stats_dto.get(&StatID(2)).map_or(0, |s| s.base_stat),
        pokemon_stats_dto.get(&StatID(3)).map_or(0, |s| s.base_stat),
        pokemon_stats_dto.get(&StatID(4)).map_or(0, |s| s.base_stat),
        pokemon_stats_dto.get(&StatID(5)).map_or(0, |s| s.base_stat),
        pokemon_stats_dto.get(&StatID(6)).map_or(0, |s| s.base_stat),
    )
}

fn build_types(
    pokemon_types: Vec<PokemonTypeDTO>,
    types: HashMap<TypeID, TypesDTO>,
) -> Result<PokemonTypes, DatabaseError> {
    let pokemon_types_names: Vec<String> = pokemon_types
        .iter()
        .filter_map(|pt| Some(capitalize(&types.get(&pt.id)?.identifier.clone())))
        .collect();

    if (pokemon_types_names.len() != 1 && pokemon_types_names.len() != 2)
        || pokemon_types_names.is_empty()
    {
        return Err(DatabaseError::FetchError(
            "Pokemon has invalid number of types.".to_string(),
        ));
    }

    Ok(PokemonTypes::new(
        pokemon_types_names
            .first()
            .ok_or(DatabaseError::FetchError(
                "Pokemon must have a primary type.".to_string(),
            ))?
            .to_owned()
            .into(),
        pokemon_types_names.get(1).map(|t| t.to_owned().into()),
    ))
}

fn build_description(description: PokemonDescriptionDTO) -> PokemonDescription {
    // Correct malformed database rows
    PokemonDescription::new(description.text.replace("\n", " ").replace("", " "))
}

fn build_attributes(
    pokemon_size: PokemonSizeDTO,
    abilities: HashMap<AbilityID, AbilityDTO>,
    pokemon_abilities: HashMap<AbilitySlot, PokemonAbilitiesDTO>,
    pokemon_species: PokemonSpeciesNamesDTO,
    pokemon_gender: PokemonGenderDTO,
) -> Result<PokemonAttributes> {
    if !pokemon_abilities.contains_key(&AbilitySlot(1)) {
        return Err(eyre!(
            "Failed to build attributes: no ability in first slot".to_string()
        ));
    }

    let abilities_vec: Vec<_> = [
        pokemon_abilities.get(&AbilitySlot(0)),
        pokemon_abilities.get(&AbilitySlot(1)),
    ]
    .iter()
    .filter_map(|a| a.and_then(|a| abilities.get(&a.id)))
    .map(|a| capitalize_words(&a.identifier.replace("-", " ")))
    .collect();

    if !abilities_vec.len() == 0 {
        return Err(eyre!(
            "Failed to build attributes: no abilities found".to_string()
        ));
    }

    let gender_rate = match pokemon_gender.rate {
        -1 => None,
        _ => {
            let female_rate = pokemon_gender.rate as f32 / 8.0;
            Some(PokemonGenderRates::new(1.0 - female_rate, female_rate)?)
        }
    };

    Ok(PokemonAttributes::new(
        (pokemon_size.height_dm as f32 / 10.0).to_string(),
        (pokemon_size.weight_hg as f32 / 10.0).to_string(),
        pokemon_species.genus,
        abilities_vec,
        gender_rate,
    ))
}
