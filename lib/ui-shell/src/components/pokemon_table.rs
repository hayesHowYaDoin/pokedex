use ratatui::{
    prelude::{Constraint, Rect, Style},
    style::Stylize,
    widgets::{Block, Row, Table, TableState},
    Frame,
};

use ui_core::components::{PokemonTable, PokemonTableEntry};
use crate::palette::type_color_medium;
use super::TuiStatefulComponent;

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(5),
    Constraint::Length(15),
    Constraint::Length(10),
    Constraint::Length(10),
];

pub struct TuiPokemonTable<'a> {
    pokemon_table: PokemonTable,
    block: Block<'a>,
}

impl<'a> TuiPokemonTable<'a> {
    pub fn new(pokemon_table: PokemonTable, block: Block<'a>) -> Self {
        Self {
            pokemon_table,
            block,
        }
    }
}

impl TuiStatefulComponent for TuiPokemonTable<'_> {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect) {
        let rows = self.pokemon_table.get_pokemon().into_iter().map(|p| into_row(p.to_owned()));
        let table = Table::new(rows, WIDTHS)
            .column_spacing(1)
            .header(
                Row::new(vec!["#", "Name", "Type 1", "Type 2"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(self.block.clone())
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let mut table_state =
            TableState::default().with_selected(self.pokemon_table.get_selected_index());
        frame.render_stateful_widget(table, *layout, &mut table_state);
    }
}

fn into_row<'a>(entry: PokemonTableEntry) -> Row<'a> {
    let color = type_color_medium(&entry.primary_type);
    Row::new(vec![
        entry.number.to_string(),
        entry.name,
        entry.primary_type.to_string(),
        entry
            .secondary_type
            .map_or("".to_string(), |t| t.to_string()),
    ])
    .fg(color)
}
