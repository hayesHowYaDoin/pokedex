use ansi_to_tui::IntoText;

use ratatui::{
    prelude::Rect,
    Frame,
    widgets::{Block, Paragraph},
};

use crate::core::ui::components::TextImage;
use super::TuiComponent;

impl TuiComponent for TextImage {
    fn render(&self, frame: &mut Frame, layout: &Rect, block: &Block) {
        let widget: Paragraph = Paragraph::new(self.text())
            .block(block.clone());

        frame.render_widget(&widget, *layout);
    }
}
