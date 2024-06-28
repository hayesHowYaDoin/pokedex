use std::{
    cell::LazyCell,
    default::Default,
};

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Style, Terminal}, 
    style::{Color, Stylize}, 
    widgets::{Block, Paragraph, Row, TableState, Table}
};

use crate::infrastructure::tui::tui::Event;
use super::page::Page;

const SEARCH_WIDGET_LAYOUT_IDX: usize = 0;
const LIST_WIDGET_LAYOUT_IDX: usize = 1;
const FOOTER_WIDGET_LAYOUT_IDX: usize = 2;

const LAYOUT: LazyCell<Layout> = 
    LazyCell::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ]));

const WIDTHS: [Constraint; 4] = [
    Constraint::Length(5),
    Constraint::Length(15),
    Constraint::Length(10),
    Constraint::Length(10),
];

enum Action {
    Quit,
    Noop,
}

pub struct ListPage<'a> {
    search_widget: Paragraph<'a>,
    list_widget: Table<'a>,
}

impl ListPage<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_event(&self, event: &Option<Event>) -> Action {
        match event {
            Some(Event::Key(key_event)) => {
                match key_event.code {
                    KeyCode::Char('q') => Action::Quit,
                    _ => Action::Noop,
                }
            },
            Some(_) => Action::Noop,
            None => Action::Noop,
        }
    }

    fn handle_action(&mut self, _action: &Action) {
        // TODO: Implement function
    }
}

impl Default for ListPage<'_> {
    fn default() -> Self {
        let search_widget = Paragraph::new("Enter search query here!")
            .block(Block::bordered().title("Search"));

        let rows = [
            Row::new(vec!["001", "Bulbasaur", "Grass", "Poison"]).green(),
            Row::new(vec!["002", "Ivysaur", "Grass", "Poison"]).green(),
            Row::new(vec!["003", "Venusaur", "Grass", "Poison"]).green(),
            Row::new(vec!["004", "Charmander", "Fire", ""]).red(),
            Row::new(vec!["005", "Charmeleon", "Fire", ""]).red(),
            Row::new(vec!["006", "Charizard", "Fire", "Flying"]).red(),
            Row::new(vec!["007", "Squirtle", "Water", ""]).blue(),
            Row::new(vec!["008", "Wartortle", "Water", ""]).blue(),
            Row::new(vec!["009", "Blastoise", "Water", ""]).blue(),
        ];

        let list_widget = Table::new(rows, WIDTHS)
        .column_spacing(1)
        .header(
            Row::new(vec!["#", "Name", "Type 1", "Type 2"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .block(Block::bordered())
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

        Self {search_widget, list_widget}
    }
}

impl Page for ListPage<'_> {
    fn update(&mut self, event: &Option<Event>) {
        let action = self.handle_event(event);
        self.handle_action(&action);
    }

    fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        let mut table_state = TableState::default();
        table_state.select(Some(0));

        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            frame.render_widget(&self.search_widget, layout[SEARCH_WIDGET_LAYOUT_IDX]);
            frame.render_stateful_widget(&self.list_widget, layout[LIST_WIDGET_LAYOUT_IDX], &mut table_state);
            frame.render_widget(Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray), layout[FOOTER_WIDGET_LAYOUT_IDX]);
        })?;

        Ok(())
    }

}
