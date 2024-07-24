use std::sync::LazyLock;

use ratatui::{
    prelude::{Constraint, Rect, Style},
    style::{Color, Stylize},
    terminal::Frame,
    widgets::{Block, Row, Table, TableState},
};

use crate::core::ui::components::{PokemonTable, PokemonTableEntry};
use super::TuiStatefulComponent;

static ROWS: LazyLock<Vec<Row>> = LazyLock::new(|| vec![
    Row::new(vec!["1", "Bulbasaur", "Grass", "Poison"]).green(),
    Row::new(vec!["2", "Ivysaur", "Grass", "Poison"]).green(),
    Row::new(vec!["3", "Venusaur", "Grass", "Poison"]).green(),
    Row::new(vec!["4", "Charmander", "Fire", ""]).red(),
    Row::new(vec!["5", "Charmeleon", "Fire", ""]).red(),
    Row::new(vec!["6", "Charizard", "Fire", "Flying"]).red(),
    Row::new(vec!["7", "Squirtle", "Water", ""]).blue(),
    Row::new(vec!["8", "Wartortle", "Water", ""]).blue(),
    Row::new(vec!["9", "Blastoise", "Water", ""]).blue(),
]);

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(5),
    Constraint::Length(15),
    Constraint::Length(10),
    Constraint::Length(10),
];

impl TuiStatefulComponent for PokemonTable {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect) {
        let table = Table::new(self.get_pokemon().to_owned(), WIDTHS)
            .column_spacing(1)
            .header(
                Row::new(vec!["#", "Name", "Type 1", "Type 2"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(Block::bordered())
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let mut table_state = TableState::default().with_selected(self.get_selected_index());
        frame.render_stateful_widget(table, *layout, &mut table_state);
    }
}

fn row_color(_entry: &PokemonTableEntry) -> Color {
    Color::Green
}

impl From<PokemonTableEntry> for Row<'_> {
    fn from(entry: PokemonTableEntry) -> Self {
        let color = row_color(&entry);
        Row::new(vec![
            entry.number.to_string(),
            entry.name,
            entry.type1,
            entry.type2.unwrap_or_else(|| "".to_string()),
        ]).fg(color)
    }
}