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
    NewCharacter(char),
    Delete,
    ScrollUp,
    ScrollDown,
    Noop,
}

pub struct ListPage<'a> {
    search_widget: Paragraph<'a>,
    search_widget_state: String,
    list_widget: Table<'a>,
    list_widget_state: TableState,
}

impl ListPage<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_event(&self, event: &Option<Event>) -> Action {
        match event {
            Some(Event::Key(key_event)) => {
                match key_event.code {
                    KeyCode::Up => Action::ScrollUp,
                    KeyCode::Down => Action::ScrollDown,
                    KeyCode::Char(c) => Action::NewCharacter(c),
                    KeyCode::Backspace => Action::Delete,
                    _ => Action::Noop,
                }
            },
            Some(_) => Action::Noop,
            None => Action::Noop,
        }
    }

    fn handle_action(&mut self, action: &Action) {
        match action {
            Action::NewCharacter(c) => {
                self.search_widget_state.push(*c);
                self.search_widget = Paragraph::new(self.search_widget_state.clone())
                    .block(Block::bordered().title("Search"));
            }
            Action::Delete => {
                self.search_widget_state.pop();
                self.search_widget = Paragraph::new(self.search_widget_state.clone())
                    .block(Block::bordered().title("Search"));
            }
            Action::ScrollUp => {
                if let Some(index) = self.list_widget_state.selected() {
                    if index > 0 {
                        self.list_widget_state.select(Some(index - 1));
                    }
                }
            }
            Action::ScrollDown => {
                if let Some(index) = self.list_widget_state.selected() {
                    // TODO: Determine max limit based on number of rows assigned to table
                    if index < 8 {
                        self.list_widget_state.select(Some(index + 1));
                    }
                }
            }
            Action::Noop => {},
        
        }
    }
}

impl Default for ListPage<'_> {
    fn default() -> Self {
        let search_widget_state = String::default();
        let search_widget = Paragraph::new(search_widget_state.clone())
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

        let list_widget_state = TableState::default().with_selected(0);

        Self {search_widget, search_widget_state, list_widget, list_widget_state}
    }
}

impl Page for ListPage<'_> {
    fn update(&mut self, event: &Option<Event>) {
        let action = self.handle_event(event);
        self.handle_action(&action);
    }

    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            frame.render_widget(&self.search_widget, layout[SEARCH_WIDGET_LAYOUT_IDX]);
            frame.render_stateful_widget(&self.list_widget, layout[LIST_WIDGET_LAYOUT_IDX], &mut self.list_widget_state);
            frame.render_widget(Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray), layout[FOOTER_WIDGET_LAYOUT_IDX]);
        })?;

        Ok(())
    }

}
