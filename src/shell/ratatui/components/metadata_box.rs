use ratatui::{
    prelude::Rect, widgets::{Block, Paragraph, Wrap}, Frame
};

use crate::core::ui::components::MetadataBox;
use super::TuiComponent;

pub struct TuiMetadataBox<'a> {
    metadata_box: MetadataBox,
    block: Block<'a>,
}

impl<'a> TuiMetadataBox<'a> {
    pub fn new(metadata_box: MetadataBox, block: Block<'a>) -> Self {
        Self { metadata_box, block }
    }
}

impl TuiComponent for TuiMetadataBox<'_> {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let widget: Paragraph = Paragraph::new(self.metadata_box.text())
            .wrap(Wrap { trim: false })
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
