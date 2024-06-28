use ratatui::{
    prelude::Rect,
    terminal::Frame,
};

pub trait Component {
    fn render(&mut self, frame: &mut Frame, layout: &Rect);
}

pub trait StatefulComponent {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect);
}
