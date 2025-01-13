use ratatui::{
    prelude::Rect,
    Frame,
    widgets::{Block, Paragraph},
};

use crate::core::ui::components::InputBox;
use super::TuiComponent;

pub struct TuiInputBox {
    input_box: InputBox,
    block: Block<'static>,
}

impl TuiInputBox {
    pub fn new(input_box: InputBox, block: Block<'static>) -> Self {
        Self { input_box, block }
    }
}

impl TuiComponent for TuiInputBox {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let widget = Paragraph::new(self.input_box.text())
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}