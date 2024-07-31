use color_eyre::Result;

use crate::core::{
    pokemon::{PokemonDescription, PokemonStats, PokemonTypes},
    ui::components::{PokemonStatChart, TextBox},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailPage {
    pub title: TextBox,
    pub stat_chart: PokemonStatChart,
    pub description: TextBox,
}

impl DetailPage {
    pub fn new(pokemon: &DetailPagePokemon) -> Result<Self> {
        let title = TextBox::new(&format!("#{} | {}", pokemon.number, pokemon.name));

        let (stats, labels) = get_stats_with_labels(&pokemon.stats);
        let max_chart = PokemonStatChart::new(&stats, &labels)?;

        let description = TextBox::new(&pokemon.description.text);

        Ok(DetailPage{ title, stat_chart: max_chart, description })
    }

    pub fn get_title_box(&self) -> &TextBox {
        &self.title
    }

    pub fn get_stat_chart(&self) -> &PokemonStatChart {
        &self.stat_chart
    }

    pub fn get_text_box(&self) -> &TextBox {
        &self.description
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


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetailPagePokemon {
    pub number: i32,
    pub name: String,
    pub types: PokemonTypes,
    pub description: PokemonDescription,
    pub stats: PokemonStats,
}

impl DetailPagePokemon {
    pub fn new(number: i32, name: String, types: PokemonTypes, description: PokemonDescription, stats: PokemonStats) -> Self {
        DetailPagePokemon{ number, name, types, description, stats }
    }
}
