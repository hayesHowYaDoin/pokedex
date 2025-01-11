use ratatui::{
    prelude::Rect,
    Frame,
    widgets::{Block, Paragraph},
};
use ratatui_image::{
    picker::Picker,
    StatefulImage,
    Resize,
};

use crate::core::ui::components::ImageBox;
use super::TuiComponent;

impl TuiComponent for ImageBox {
    fn render(&self, frame: &mut Frame, layout: &Rect, _block: &Block) {
        // TODO: Currently fails due to query after reading terminal events.
        // Likely need to move outside of this function and pass into the application.
        // let mut picker = Picker::from_query_stdio().expect("Unable to font size.");
        let mut picker = Picker::from_fontsize((8, 12));
        picker.set_background_color([0, 0, 0, 0]);

        let mut image_protocol = picker.new_resize_protocol(self.image().to_owned());
        let widget = StatefulImage::default();

        frame.render_stateful_widget(widget, *layout, &mut image_protocol);
    }
}
