use color_eyre::eyre::Result;
use ratatui::prelude::{Backend, Terminal};

use crate::core::ui::pages::Page;

pub trait TuiPage<B: Backend>: Page {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()>;
}
