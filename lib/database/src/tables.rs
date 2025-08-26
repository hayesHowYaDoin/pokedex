// Members are necessary to define tables, but may be unused outside of macro.
#![allow(dead_code)]

use derive_crud::Read;

#[derive(Debug, Read)]
#[crud_table("pokemon")]
pub struct PokemonDTO {
    #[crud_id]
    pub id: i64,
    pub species_id: i64,
    pub identifier: String,
}

#[derive(Debug, Read)]
#[crud_table("types")]
pub struct TypeDTO {
    #[crud_id]
    pub id: i64,
    pub identifier: String,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_types")]
pub struct PokemonTypeDTO {
    #[crud_id]
    pub pokemon_id: i64,
    pub type_id: i64,
    pub slot: i64,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_sizes")]
pub struct PokemonSizeDTO {
    #[crud_id]
    pub id: i64,
    pub height_dm: i64,
    pub weight_dg: i64,
}

#[derive(Debug, Read)]
#[crud_table("stat_names")]
pub struct StatNamesDTO {
    #[crud_id]
    pub stat_id: i64,
    pub local_language_id: i64,
    pub name: String,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_stats")]
pub struct PokemonStatsDTO {
    #[crud_id]
    pub pokemon_id: i64,
    pub stat_id: i64,
    pub base_stat: i64,
    pub effort: i64,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_descriptions")]
pub struct PokemonDescriptionDTO {
    #[crud_id]
    pub species_id: i64,
    pub version_id: i64,
    pub language_id: i64,
    pub flavor_text: String,
}

#[derive(Debug, Read)]
#[crud_table("abilities")]
pub struct AbilityDTO {
    #[crud_id]
    pub id: i64,
    pub identifier: String,
    pub generation_id: i64,
    pub is_main_series: i64,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_abilities")]
pub struct PokemonAbilitiesDTO {
    #[crud_id]
    pub pokemon_id: i64,
    pub ability_id: i64,
    pub is_hidden: i64,
    pub slot: i64,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_species_names")]
pub struct PokemonSpeciesNamesDTO {
    #[crud_id]
    pub pokemon_species_id: i64,
    pub local_language_id: i64,
    pub name: String,
    pub genus: String,
}

#[derive(Debug, Read)]
#[crud_table("pokemon_genders")]
pub struct PokemonGenderDTO {
    #[crud_id]
    pub id: i64,
    pub gender_rate: i64,
}
