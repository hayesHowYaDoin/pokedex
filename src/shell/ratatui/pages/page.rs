use color_eyre::eyre::Result;
use ratatui::prelude::{Backend, Terminal};
use ratatui_image::picker::Picker;

pub trait TuiPage<B: Backend> {
    fn render(&mut self, terminal: &mut Terminal<B>, picker: &mut Picker) -> Result<()>;
}
