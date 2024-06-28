use std::{
    sync::LazyLock,
    default::Default,
};

use ratatui::{
    prelude::{Constraint, Rect, Style},
    style::Stylize,
    terminal::Frame,
    widgets::{Block, Row, Table, TableState},
};

use super::StatefulComponent;

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

#[derive(Clone, Debug)]
pub struct PokemonTable {
    state: TableState,
}

impl PokemonTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn up(&mut self) {
        if let Some(index) = self.state.selected() {
            if index > 0 {
                self.state.select(Some(index - 1));
            }
        }
    }

    pub fn down(&mut self) {
        if let Some(index) = self.state.selected() {
            if index < ROWS.len() - 1 {
                self.state.select(Some(index + 1));
            }
        }
    }
}

impl Default for PokemonTable {
    fn default() -> Self {
        let state = TableState::default().with_selected(0);
        Self {state}
    }
}

impl StatefulComponent for PokemonTable {
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

        frame.render_stateful_widget(table, *layout, &mut self.state);
    }
}