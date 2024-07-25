use ratatui::{
    prelude::Rect,
    terminal::Frame,
};

pub trait TuiComponent {
    fn render(&mut self, frame: &mut Frame, layout: &Rect);
}

pub trait TuiStatefulComponent {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect);
}
