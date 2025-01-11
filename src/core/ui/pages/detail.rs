use color_eyre::Result;
use image::DynamicImage;

use crate::core::{
    pokemon::{PokemonDescription, PokemonStats, PokemonTypes, PokemonMetadata},
    ui::components::{PokemonStatChart, TextBox, ImageBox, MetadataBox},
};

#[derive(Clone, Debug, PartialEq)]
pub struct DetailPage {
    pub title: TextBox,
    pub image: ImageBox,
    pub stat_chart: PokemonStatChart,
    pub description: TextBox,
    pub metadata: MetadataBox,
}

impl DetailPage {
    pub fn new(pokemon: &DetailPagePokemon) -> Result<Self> {
        let title = TextBox::new(&format!("#{} | {}", pokemon.number, pokemon.name));

        let image = ImageBox::new(pokemon.image.clone());

        let (stats, labels) = get_stats_with_labels(&pokemon.stats);
        let stat_chart = PokemonStatChart::new(&stats, &labels)?;

        let description = TextBox::new(&pokemon.description.text);

        let metadata = MetadataBox::new(&pokemon.metadata);

        Ok(DetailPage{ title, image, stat_chart, description, metadata })
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
        DetailPagePokemon{ number, name, image, types, description, metadata, stats }
    }
}
