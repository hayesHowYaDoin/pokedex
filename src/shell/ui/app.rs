use std::default::Default;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode;

use crate::shell::ui::{
    tui::{Tui, TuiBackend, TuiEvent},
    pages::TuiPage,
};
use crate::core::ui::{
    Event,
    pages::ListPage
};

enum Action {
    Quit,
    Noop,
}

pub struct App {
    should_quit: bool,
    page: Box<dyn TuiPage<TuiBackend>>,
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
            let event = Event::from(tui.next().await);

            self.update(&event);

            self.page.update(&event);
            self.page.render(&mut tui.terminal)?;

            if self.should_quit { break }
        };

        tui.exit()?;
        Ok(())
    }

    fn update(&mut self, event: &Event) {
        if event == &Event::NewCharacter('q') {
            self.should_quit = true;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let page = Box::new(ListPage::new(151));
        Self {should_quit: false, page}
    }
}
