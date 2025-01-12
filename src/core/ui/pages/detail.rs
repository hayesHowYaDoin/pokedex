use std::collections::HashSet;

use color_eyre::Result;
use image::DynamicImage;

use crate::core::{
    pokemon::{
        PokemonDescription, 
        PokemonMetadata, 
        PokemonStats, 
        PokemonTypes, 
        Type,
        type_defense_strengths, 
        type_defense_weaknesses,
    },
    ui::components::{
        ImageBox, 
        MetadataBox, 
        PokemonStatChart, 
        TextBox,
        TypesBox,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct DetailPage {
    pub title: TextBox,
    pub image: ImageBox,
    pub stat_chart: PokemonStatChart,
    pub description: TextBox,
    pub metadata: MetadataBox,
    pub types: TypesBox,
    pub weaknesses: TypesBox,
}

impl DetailPage {
    pub fn new(pokemon: &DetailPagePokemon) -> Result<Self> {
        let title = TextBox::new(&format!("#{} | {}", pokemon.number, pokemon.name));

        let image = ImageBox::new(pokemon.image.clone());

        let (stats, labels) = get_stats_with_labels(&pokemon.stats);
        let stat_chart = PokemonStatChart::new(&stats, &labels)?;

        let description = TextBox::new(&pokemon.description.text);

        let metadata = MetadataBox::new(&pokemon.metadata);

        let mut types_set = HashSet::from([pokemon.types.primary]);
        if let Some(secondary_type) = pokemon.types.secondary {
            types_set.insert(secondary_type);
        }
        let types = TypesBox::new(types_set);

        let weaknesses = TypesBox::new(pokemon.weaknesses.clone());

        Ok(DetailPage{ title, image, stat_chart, description, metadata, types, weaknesses })
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

    pub fn get_metadata_box(&self) -> &MetadataBox {
        &self.metadata
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
}

fn get_stats_with_labels(stats: &PokemonStats) -> ([i32; 6], [&str; 6]) {
    const STAT_LABELS: [&str; 6] = ["HP", "Attack", "Defense", "Sp. Atk", "Sp. Def", "Speed"];

    let raw_stats = [
        stats.hp as i32,
        stats.attack as i32,
        stats.defense as i32,
        stats.special_attack as i32,
        stats.special_defense as i32,
        stats.speed as i32,
    ];

    (raw_stats, STAT_LABELS)
}


#[derive(Clone, Debug, PartialEq)]
pub struct DetailPagePokemon {
    pub number: i32,
    pub name: String,
    pub image: DynamicImage,
    pub types: PokemonTypes,
    pub weaknesses: HashSet<Type>,
    pub description: PokemonDescription,
    pub metadata: PokemonMetadata,
    pub stats: PokemonStats,
}

impl DetailPagePokemon {
    pub fn new(
        number: i32,
        name: String,
        image: DynamicImage,
        types: PokemonTypes,
        description: PokemonDescription,
        metadata: PokemonMetadata,
        stats: PokemonStats
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

        DetailPagePokemon{ number, name, image, types, weaknesses, description, metadata, stats }
    }
}
