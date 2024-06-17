use ratatui::{
    prelude::{Backend, Terminal},
    widgets::Paragraph
};
use std::io::Result;

pub struct ListPage {
    // TODO: Add fields for page state
}

impl ListPage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)"),
                area,
            );
        })?;

        Ok(())
    }
}
