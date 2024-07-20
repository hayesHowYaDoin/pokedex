use std::sync::LazyLock;

use ratatui::{
    prelude::{Constraint, Rect, Style},
    style::Stylize,
    terminal::Frame,
    widgets::{Block, Row, Table, TableState},
};

use crate::core::ui::components::{PokemonTable, RowIndex};
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

impl Into<usize> for RowIndex {
    fn into(self) -> usize {
        let RowIndex(index) = self;
        index as usize
    }
}

impl TuiStatefulComponent for PokemonTable {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect) {
        let table = Table::new(ROWS.to_owned(), WIDTHS)
            .column_spacing(1)
            .header(
                Row::new(vec!["#", "Name", "Type 1", "Type 2"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(Block::bordered())
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let mut table_state = TableState::default().with_selected(Some(self.selected_index().into()));
        frame.render_stateful_widget(table, *layout, &mut table_state);
    }
}