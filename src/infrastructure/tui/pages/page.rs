use color_eyre::eyre::Result;
use ratatui::prelude::{Backend, Terminal};

use crate::infrastructure::tui::tui::Event;

pub trait Renderable<B: Backend> {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()>;
}

pub trait Page<B: Backend>: Renderable<B> {
    fn update(&mut self, event: &Option<Event>) -> Box<dyn Page<B>>;
}
