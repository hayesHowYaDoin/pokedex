use ratatui::{
    prelude::Rect,
    Frame,
    widgets::Block,
};
use ratatui_image::{
    picker::Picker,
    StatefulImage,
};

use crate::core::ui::components::ImageBox;
use super::TuiStatefulComponent;

pub struct TuiImageBox<'a> {
    image_box: ImageBox,
    block: Block<'a>,
    picker: Picker,
}

impl<'a> TuiImageBox<'a> {
    pub fn new(image_box: ImageBox, block: Block<'a>, picker: Picker) -> Self {
        Self { image_box, block, picker }
    }
}

impl TuiStatefulComponent for TuiImageBox<'_> {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect) {
        let mut image_protocol = self.picker.new_resize_protocol(self.image_box.image().to_owned());
        let widget = StatefulImage::default();

        frame.render_stateful_widget(widget, *layout, &mut image_protocol);
    }
}
