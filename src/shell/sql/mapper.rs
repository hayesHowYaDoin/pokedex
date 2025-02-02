use std::collections::HashMap;
use std::path::Path;

use color_eyre::{eyre, Result};

use crate::core::{
    pokemon::{
        PokemonAttributes, PokemonCry, PokemonDescription, PokemonGenderRates, PokemonStats,
        PokemonTypes,
    },
    ui::{
        pages::{DetailPagePokemon, ListPagePokemon},
        repository::{
            DetailPagePokemonRepository, ListPagePokemonRepository, ListPagePokemonRepositoryError,
        },
        string::{capitalize, capitalize_words},
    },
};

use super::{
    tables::{
        AbilitiesRepository, AbilityDTO, AbilityID, AbilitySlot, PokemonAbilitiesDTO,
        PokemonAbilitiesRepository, PokemonDescriptionDTO, PokemonDescriptionsRepository,
        PokemonGenderDTO, PokemonGenderRepository, PokemonID, PokemonSizeDTO,
        PokemonSizeTableRepository, PokemonSpeciesNamesDTO, PokemonSpeciesNamesRepository,
        PokemonStatsDTO, PokemonStatsRepository, PokemonTableRepository, PokemonTypeDTO,
        PokemonTypeTableRepository, StatID, TypeID, TypesDTO, TypesTableRepository,
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
        let pokemon_size = PokemonSizeTableRepository::fetch(&self.database, &id)?;
        let pokemon_stats_dto = PokemonStatsRepository::fetch(&self.database, &id)?;
        let types = TypesTableRepository::fetch_all(&self.database)?;
        let pokemon_description = PokemonDescriptionsRepository::fetch(&self.database, &id)?;
        let abilities = AbilitiesRepository::fetch_all(&self.database)?;
        let pokemon_abilities = PokemonAbilitiesRepository::fetch(&self.database, &id)?;
        let pokemon_species = PokemonSpeciesNamesRepository::fetch(&self.database, &id)?;
        let pokemon_gender = PokemonGenderRepository::fetch(&self.database, &id)?;

        let detail_page_pokemon = DetailPagePokemon::new(
            pokemon.species_id,
            capitalize(&pokemon.identifier),
            build_image(id),
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
            build_cry(id),
        );

        Ok(detail_page_pokemon)
    }
}

fn build_image(id: PokemonID) -> image::DynamicImage {
    let image_path = format!("./data/assets/{}/bw_front.png", Into::<u32>::into(id));
    image::ImageReader::open(image_path)
        .expect("Unable to open image.")
        .decode()
        .unwrap()
}

fn build_cry(id: PokemonID) -> PokemonCry {
    let cry_bytes = std::fs::read(format!("./data/assets/{}/cry.wav", Into::<u32>::into(id)))
        .expect("Unable to locate cry resource.");
    PokemonCry::new(cry_bytes)
}

fn build_stats(pokemon_stats_dto: HashMap<StatID, PokemonStatsDTO>) -> PokemonStats {
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
        || pokemon_types_names.get(0).is_none()
    {
        return Err(DatabaseError::FetchError(
            "Pokemon has invalid number of types.".to_string(),
        ));
    }

    Ok(PokemonTypes::new(
        pokemon_types_names.get(0).unwrap().to_owned().into(),
        pokemon_types_names
            .get(1)
            .map_or(None, |t| Some(t.to_owned().into())),
    ))
}

fn build_description(description: PokemonDescriptionDTO) -> PokemonDescription {
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
        return Err(eyre::eyre!(
            "Failed to build attributes: no ability in first slot".to_string()
        ));
    }

    let abilities_vec: Vec<_> = vec![
        pokemon_abilities.get(&AbilitySlot(0)),
        pokemon_abilities.get(&AbilitySlot(1)),
    ]
    .iter()
    .filter_map(|a| a.and_then(|a| abilities.get(&a.id)))
    .map(|a| capitalize_words(&a.identifier.replace("-", " ")))
    .collect();

    if !abilities_vec.len() == 0 {
        return Err(eyre::eyre!(
            "Failed to build attributes: no abilities found".to_string()
        ));
    }

    let gender_rate = match pokemon_gender.rate {
        -1 => None,
        _ => {
            let female_rate = pokemon_gender.rate as f32 / 8.0;
            Some(PokemonGenderRates::new(1.0 - female_rate, female_rate)?)
        },
    };

    Ok(PokemonAttributes::new(
        (pokemon_size.height_dm as f32 / 10.0).to_string(),
        (pokemon_size.weight_hg as f32 / 10.0).to_string(),
        pokemon_species.genus,
        abilities_vec,
        gender_rate,
    ))
}
