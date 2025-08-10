use std::future::Future;
use std::pin::Pin;

use color_eyre::{eyre::eyre, Result};
use futures::{Stream, StreamExt};

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
    repository::{DetailPagePokemonRepository, ListPagePokemonRepository},
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
    fn fetch_all(&self) -> Pin<Box<dyn Future<Output = Result<Vec<ListPagePokemon>>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let pokemon = PokemonDTO::read_all(&pool).await?;
            let pokemon_types = PokemonTypeDTO::read_all(&pool).await?;
            let types = TypeDTO::read_all(&pool).await?;

            let list_page_pokemon = pokemon
                .into_iter()
                .filter_map(|p| {
                    let number = p.species_id.try_into().ok()?;
                    let name = capitalize(&p.identifier);

                    let type_ids = pokemon_types
                        .iter()
                        .filter(|t| t.pokemon_id == number as i64)
                        .collect::<Vec<_>>();
                    let pokemon_types: Vec<String> = type_ids
                        .iter()
                        .filter_map(|pt| {
                            types
                                .iter()
                                .find(|t| t.id == pt.type_id)
                                .map(|t| capitalize(&t.identifier))
                        })
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
        })
    }
}

impl DetailPagePokemonRepository for DatabaseMapper {
    fn fetch(
        &self,
        number: u32,
    ) -> Pin<Box<dyn Future<Output = Result<DetailPagePokemon>> + Send>> {
        let pool = self.pool.clone();
        Box::pin(async move {
            let pokemon = PokemonDTO::read_one(&pool, number as i64).await?;

            let detail_page_pokemon = DetailPagePokemon::new(
                pokemon.id as u32,
                capitalize(&pokemon.identifier),
                build_image(number)?,
                build_types(&pool, number as i64).await?,
                build_description(&pool, number as i64).await?,
                build_attributes(&pool, number as i64).await?,
                build_stats(number as i64, &pool).await?,
                build_cry(number)?,
            );

            Ok(detail_page_pokemon)
        })
    }
}

async fn stream_to_vec<T, E, S>(stream: S) -> Result<Vec<T>>
where
    S: Stream<Item = std::result::Result<T, E>> + Send,
    E: std::error::Error + Send + Sync + 'static,
    T: Send,
{
    let mut vec = Vec::new();
    futures::pin_mut!(stream);
    while let Some(item) = stream.next().await {
        let item = item.map_err(|e| eyre!(e))?;
        vec.push(item);
    }
    Ok(vec)
}

fn build_image(id: u32) -> Result<image::DynamicImage> {
    let image_path = ASSETS_PATH.join(format!("{}/bw_front.png", Into::<u32>::into(id)));
    Ok(image::ImageReader::open(image_path)
        .expect("Unable to open image.")
        .decode()?)
}

async fn build_types(pool: &sqlx::SqlitePool, number: i64) -> Result<PokemonTypes> {
    let pokemon_types = stream_to_vec(PokemonTypeDTO::read(&pool, number)).await?;
    let types = TypeDTO::read_all(&pool).await?;

    let pokemon_types_names: Vec<String> = pokemon_types
        .iter()
        .filter_map(|pt| {
            Some(capitalize(
                &types
                    .iter()
                    .find(|t| pt.type_id == t.id)?
                    .identifier
                    .clone(),
            ))
        })
        .collect();

    if (pokemon_types_names.len() != 1 && pokemon_types_names.len() != 2)
        || pokemon_types_names.is_empty()
    {
        return Err(eyre!("Pokemon has invalid number of types.".to_string()));
    }

    Ok(PokemonTypes::new(
        pokemon_types_names
            .first()
            .ok_or(eyre!("Pokemon must have a primary type.".to_string(),))?
            .to_owned()
            .into(),
        pokemon_types_names.get(1).map(|t| t.to_owned().into()),
    ))
}

async fn build_description(pool: &sqlx::SqlitePool, number: i64) -> Result<PokemonDescription> {
    let pokemon_description =
        stream_to_vec(PokemonDescriptionDTO::read(&pool, number as i64)).await?;
    let description = pokemon_description
        .into_iter()
        .filter(|d| d.language_id == 9) // English
        .filter(|d| d.version_id == 9) // Black/White
        .next()
        .ok_or_else(|| eyre!("No English Black/White description found"))?;

    // Correct malformed database rows
    Ok(PokemonDescription::new(
        description.flavor_text.replace("\n", " ").replace("", " "),
    ))
}

async fn build_attributes(pool: &sqlx::SqlitePool, number: i64) -> Result<PokemonAttributes> {
    let abilities = AbilityDTO::read_all(&pool).await?;
    let pokemon_size = PokemonSizeDTO::read_one(&pool, number as i64).await?;
    let mut pokemon_abilities =
        stream_to_vec(PokemonAbilitiesDTO::read(&pool, number as i64)).await?;
    let pokemon_species = PokemonSpeciesNamesDTO::read_one(&pool, number as i64).await?;
    let pokemon_gender = PokemonGenderDTO::read_one(&pool, number as i64).await?;

    if 1 > pokemon_abilities.len() || pokemon_abilities.len() > 3 {
        return Err(eyre!(format!(
            "Failed to build attributes: expected 1 or 2 abilities in database, found {}",
            pokemon_abilities.len()
        )
        .to_string()));
    }

    pokemon_abilities.sort_by_key(|a| a.slot);
    let abilities_vec: Vec<_> = pokemon_abilities
        .iter()
        .filter_map(|pa| abilities.iter().find(|a| a.id == pa.ability_id))
        .map(|a| capitalize_words(&a.identifier.replace("-", " ")))
        .collect();

    if !abilities_vec.len() == 0 {
        return Err(eyre!(
            "Failed to build attributes: no abilities found".to_string()
        ));
    }

    let gender_rate = match pokemon_gender.gender_rate {
        -1 => None,
        _ => {
            let female_rate = pokemon_gender.gender_rate as f32 / 8.0;
            Some(PokemonGenderRates::new(1.0 - female_rate, female_rate)?)
        }
    };

    Ok(PokemonAttributes::new(
        (pokemon_size.height_dm as f32 / 10.0).to_string(),
        (pokemon_size.weight_dg as f32 / 10.0).to_string(),
        pokemon_species.genus,
        abilities_vec,
        gender_rate,
    ))
}

async fn build_stats(number: i64, pool: &sqlx::SqlitePool) -> Result<PokemonStats> {
    let pokemon_stats = stream_to_vec(PokemonStatsDTO::read(pool, number as i64)).await?;
    let convert = |id: i64, ps: &[PokemonStatsDTO]| {
        ps.iter()
            .find(|s| s.stat_id == id)
            .map_or(0, |s| s.base_stat as u32)
    };
    Ok(PokemonStats::new(
        convert(1, &pokemon_stats),
        convert(2, &pokemon_stats),
        convert(3, &pokemon_stats),
        convert(4, &pokemon_stats),
        convert(5, &pokemon_stats),
        convert(6, &pokemon_stats),
    ))
}

fn build_cry(id: u32) -> Result<PokemonCry> {
    let cry_path = ASSETS_PATH.join(format!("{}/cry.wav", Into::<u32>::into(id)));
    let cry_bytes = std::fs::read(cry_path)?;
    Ok(PokemonCry::new(cry_bytes))
}
