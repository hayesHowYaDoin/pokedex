use ratatui::{
    prelude::Rect,
    terminal::Frame,
    widgets::{Block, Paragraph},
};

use crate::core::ui::components::InputBox;
use super::TuiComponent;

impl TuiComponent for InputBox {
    fn render(&mut self, frame: &mut Frame, layout: &Rect) {
        let widget = Paragraph::new(self.text().clone())
            .block(Block::bordered().title("Search"));

        frame.render_widget(&widget, *layout);
    }
}