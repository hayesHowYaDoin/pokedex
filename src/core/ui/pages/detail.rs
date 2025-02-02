use std::collections::HashSet;

use color_eyre::Result;
use image::DynamicImage;

use crate::core::{
    pokemon::{
        type_defense_strengths, type_defense_weaknesses, PokemonAttributes, PokemonCry,
        PokemonDescription, PokemonStats, PokemonTypes, Type,
    },
    ui::components::{AttributesBox, ImageBox, PokemonStatChart, Sound, TextBox, TypesBox},
};

#[derive(Clone, Debug, PartialEq)]
pub struct DetailPage {
    pub title: TextBox,
    pub image: ImageBox,
    pub stat_chart: PokemonStatChart,
    pub description: TextBox,
    pub attributes: AttributesBox,
    pub types: TypesBox,
    pub weaknesses: TypesBox,
    pub launch_sound: Sound,
}

impl DetailPage {
    pub fn new(pokemon: DetailPagePokemon) -> Result<Self> {
        let title = TextBox::new(format!("#{} | {}", pokemon.number, pokemon.name));

        let image = ImageBox::new(pokemon.image);

        let (stats, labels) = get_stats_with_labels(&pokemon.stats);
        let stat_chart = PokemonStatChart::new(&stats, &labels)?;

        let description = TextBox::new(pokemon.description.text);

        let attributes = AttributesBox::new(pokemon.attributes);

        let mut types_vec = vec![pokemon.types.primary];
        if let Some(secondary_type) = pokemon.types.secondary {
            types_vec.push(secondary_type);
        }
        let types = TypesBox::new(types_vec);

        let weaknesses = TypesBox::new(pokemon.weaknesses);

        let launch_sound = Sound::new(pokemon.cry.cry);

        Ok(DetailPage {
            title,
            image,
            stat_chart,
            description,
            attributes,
            types,
            weaknesses,
            launch_sound,
        })
    }

    pub fn get_title_box(&self) -> &TextBox {
        &self.title
    }

    pub fn get_image_box(&self) -> &ImageBox {
        &self.image
    }

    pub fn get_description_box(&self) -> &TextBox {
        &self.description
    }

    pub fn get_attributes_box(&self) -> &AttributesBox {
        &self.attributes
    }

    pub fn get_stat_chart(&self) -> &PokemonStatChart {
        &self.stat_chart
    }

    pub fn get_types_box(&self) -> &TypesBox {
        &self.types
    }

    pub fn get_weaknesses_box(&self) -> &TypesBox {
        &self.weaknesses
    }

    pub fn get_launch_sound(&self) -> &Sound {
        &self.launch_sound
    }
}

fn get_stats_with_labels(stats: &PokemonStats) -> ([u32; 6], [&str; 6]) {
    const STAT_LABELS: [&str; 6] = ["HP", "Attack", "Defense", "Sp. Atk", "Sp. Def", "Speed"];

    let raw_stats = [
        stats.hp,
        stats.attack,
        stats.defense,
        stats.special_attack,
        stats.special_defense,
        stats.speed,
    ];

    (raw_stats, STAT_LABELS)
}

#[derive(Clone, Debug, PartialEq)]
pub struct DetailPagePokemon {
    pub number: u32,
    pub name: String,
    pub image: DynamicImage,
    pub types: PokemonTypes,
    pub weaknesses: HashSet<Type>,
    pub description: PokemonDescription,
    pub attributes: PokemonAttributes,
    pub stats: PokemonStats,
    pub cry: PokemonCry,
}

impl DetailPagePokemon {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        number: u32,
        name: String,
        image: DynamicImage,
        types: PokemonTypes,
        description: PokemonDescription,
        attributes: PokemonAttributes,
        stats: PokemonStats,
        cry: PokemonCry,
    ) -> Self {
        let mut weaknesses = type_defense_weaknesses(&types.primary);
        if types.secondary.is_some() {
            weaknesses.extend(type_defense_weaknesses(&types.secondary.unwrap()));
        }
        let mut strengths: HashSet<Type> = type_defense_strengths(&types.primary);
        if types.secondary.is_some() {
            strengths.extend(type_defense_strengths(&types.secondary.unwrap()));
        }

        weaknesses.retain(|weakness| !strengths.contains(weakness));

        DetailPagePokemon {
            number,
            name,
            image,
            types,
            weaknesses,
            description,
            attributes,
            stats,
            cry,
        }
    }
}
