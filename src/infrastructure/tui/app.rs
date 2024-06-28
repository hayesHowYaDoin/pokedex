use std::default::Default;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;

use crate::infrastructure::tui::{
    tui::{Event, Tui, TuiBackend},
    pages::{Page, ListPage},
};

enum Action {
    Quit,
    Noop,
}

pub struct App {
    should_quit: bool,
    page: Box<dyn Page<TuiBackend>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        loop {
            let event = tui.next().await;

            self.page = self.page.update(&event);
            self.page.render(&mut tui.terminal)?;

            let action = self.handle_events(&event);
            self.update(&action);

            if self.should_quit { break }
        };

        tui.exit()?;
        Ok(())
    }

    fn update(&mut self, action: &Action) {
        // TODO: Implement application update funtion
        match action {
            Action::Quit => self.should_quit = true,
            Action::Noop => {},
        }
    }

    fn handle_events(&mut self, event: &Option<Event>) -> Action {
        // TODO: Implement application update function
        match event {
            Some(Event::Key(key_event)) => {
                match key_event.code {
                    KeyCode::Char('q') => Action::Quit,
                    _ => Action::Noop,
                }
            },
            _ => Action::Noop,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let page = Box::new(ListPage::new());
        Self {should_quit: false, page}
    }
}
