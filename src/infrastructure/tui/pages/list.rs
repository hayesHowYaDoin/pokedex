use ratatui::{
    prelude::{Backend, Terminal},
    widgets::Paragraph
};
use std::io::Result;

pub struct ListPage {
    // TODO: Add fields for page state
}

impl<B: Backend> ListPage {
    pub fn new() -> Self {
        Self {}
    }

    fn render(&self, terminal: &mut Terminal<B>) -> Result<()>
        where B: Backend
    {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        Ok(())
    }
}
