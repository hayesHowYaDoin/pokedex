use ratatui::{
    prelude::Rect,
    terminal::Frame,
    widgets::{Block, Paragraph},
};

use crate::core::ui::components::TextBox;
use super::TuiComponent;

impl TuiComponent for TextBox {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let widget = Paragraph::new(self.text())
            .block(Block::bordered());

        frame.render_widget(&widget, *layout);
    }
}