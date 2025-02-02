use ratatui::{prelude::Rect, Frame};
use ratatui_image::{picker::Picker, StatefulImage};

use super::TuiStatefulComponent;
use crate::core::ui::components::ImageBox;

pub struct TuiImageBox<'a> {
    image_box: &'a ImageBox,
    picker: &'a Picker,
}

impl<'a> TuiImageBox<'a> {
    pub fn new(image_box: &'a ImageBox, picker: &'a Picker) -> Self {
        Self { image_box, picker }
    }
}

impl TuiStatefulComponent for TuiImageBox<'_> {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect) {
        let mut image_protocol = self
            .picker
            .new_resize_protocol(self.image_box.image().clone());
        let widget = StatefulImage::default();

        frame.render_stateful_widget(widget, *layout, &mut image_protocol);
    }
}
