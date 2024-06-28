use color_eyre::eyre::Result;
use ratatui::prelude::{Backend, Terminal};

use crate::infrastructure::tui::tui::Event;

pub trait Page {
    fn update(&mut self, event: &Option<Event>);

    fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>;
}
