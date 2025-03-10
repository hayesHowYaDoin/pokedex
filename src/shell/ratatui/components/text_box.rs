use ratatui::{
    prelude::Rect,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::TuiComponent;
use crate::core::ui::components::TextBox;

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
            .wrap(Wrap { trim: false })
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
