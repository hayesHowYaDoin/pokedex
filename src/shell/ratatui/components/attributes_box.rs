use ratatui::{
    prelude::Rect,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::TuiComponent;
use crate::core::ui::components::AttributesBox;

pub struct TuiAttributesBox<'a> {
    attributes_box: AttributesBox,
    block: Block<'a>,
}

impl<'a> TuiAttributesBox<'a> {
    pub fn new(attributes_box: AttributesBox, block: Block<'a>) -> Self {
        Self {
            attributes_box,
            block,
        }
    }
}

impl TuiComponent for TuiAttributesBox<'_> {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let widget: Paragraph = Paragraph::new(self.attributes_box.text())
            .wrap(Wrap { trim: false })
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
