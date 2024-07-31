use ratatui::{
    prelude::{Constraint, Rect, Style},
    style::Stylize,
    terminal::Frame,
    widgets::{Block, Row, Table, TableState},
};

use crate::core::ui::components::{PokemonTable, PokemonTableEntry};
use super::{
    TuiStatefulComponent,
    super::palette::type_color,
};

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(5),
    Constraint::Length(15),
    Constraint::Length(10),
    Constraint::Length(10),
];

impl TuiStatefulComponent for PokemonTable {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect, block: &Block) {
        let table = Table::new(self.get_pokemon().to_owned(), WIDTHS)
            .column_spacing(1)
            .header(
                Row::new(vec!["#", "Name", "Type 1", "Type 2"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(block.clone())
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let mut table_state = TableState::default().with_selected(self.get_selected_index());
        frame.render_stateful_widget(table, *layout, &mut table_state);
    }
}

impl From<PokemonTableEntry> for Row<'_> {
    fn from(entry: PokemonTableEntry) -> Self {
        let color = type_color(&entry.primary_type);
        Row::new(vec![
            entry.number.to_string(),
            entry.name,
            entry.primary_type.to_string(),
            entry.secondary_type.map_or("".to_string(), |t| t.to_string()),
        ]).fg(color)
    }
}