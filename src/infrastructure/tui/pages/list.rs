use std::{
    cell::LazyCell,
    default::Default,
};

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal}, 
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::infrastructure::tui::{
    components::{InputBox, PokemonTable, Component, StatefulComponent},
    tui::Event
};
use super::page::{Page, Renderable};

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

enum Action {
    NewCharacter(char),
    Delete,
    ScrollUp,
    ScrollDown,
    Noop,
}

#[derive(Debug, Clone)]
pub struct ListPage {
    search_widget: InputBox,
    list_widget: PokemonTable,
}

impl ListPage {
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
                self.search_widget.push_char(*c);
            }
            Action::Delete => {
                self.search_widget.pop_char();
            }
            Action::ScrollUp => {
                self.list_widget.up();
            }
            Action::ScrollDown => {
                self.list_widget.down();
            }
            Action::Noop => {},
        
        }
    }
    
    fn next_page<B: Backend>(&self) -> Box<dyn Page<B>> {
        // TODO: Implement state transition logic
        Box::new(self.clone())
    }
}

impl Default for ListPage {
    fn default() -> Self {
        let search_widget = InputBox::new("");
        let list_widget = PokemonTable::default();

        Self {search_widget, list_widget}
    }
}

impl<B: Backend> Renderable<B> for ListPage {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            self.search_widget.render(frame, &layout[SEARCH_WIDGET_LAYOUT_IDX]);
            self.list_widget.render_mut(frame, &layout[LIST_WIDGET_LAYOUT_IDX]);

            frame.render_widget(
                Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray), 
                layout[FOOTER_WIDGET_LAYOUT_IDX]
            );
        })?;

        Ok(())
    }
}

impl<B: Backend> Page<B> for ListPage {
    fn update(&mut self, event: &Option<Event>) -> Box<dyn Page<B>> {
        let action = self.handle_event(event);
        self.handle_action(&action);

        self.next_page()
    }
}
