use color_eyre::eyre::Result;
use ratatui::prelude::{Backend, Terminal};

pub trait TuiPage<B: Backend> {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()>;
}
