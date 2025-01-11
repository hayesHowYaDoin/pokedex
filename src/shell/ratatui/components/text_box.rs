use ratatui::{
    prelude::Rect,
    Frame,
    widgets::{Block, Paragraph}
};

use crate::core::ui::components::TextBox;
use super::TuiComponent;

pub struct TuiTextBox<'a> {
    text_box: TextBox,
    block: Block<'a>,
}

impl<'a> TuiTextBox<'a> {
    pub fn new(text_box: TextBox, block: Block<'a>) -> Self {
        Self { text_box, block }
    }
}

impl TuiComponent for TuiTextBox<'_> {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let widget: Paragraph = Paragraph::new(self.text_box.text())
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
