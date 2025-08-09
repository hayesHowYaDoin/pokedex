use color_eyre::Result;
use ratatui_image::picker::Picker;

use crate::tui::Terminal;

pub trait TuiPage {
    fn on_enter(&mut self);

    fn on_exit(&mut self);

    fn render(&mut self, terminal: &mut Terminal, picker: &mut Picker) -> Result<()>;
}
