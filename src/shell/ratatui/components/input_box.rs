use ratatui::{
    prelude::Rect,
    terminal::Frame,
    widgets::{Block, Paragraph},
};

use crate::core::ui::components::InputBox;
use super::TuiComponent;

impl TuiComponent for InputBox {
    fn render(&self, frame: &mut Frame, layout: &Rect, block: &Block) {
        let widget = Paragraph::new(self.text())
            .block(block.clone());

        frame.render_widget(&widget, *layout);
    }
}