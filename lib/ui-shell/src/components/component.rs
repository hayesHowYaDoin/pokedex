use ratatui::{prelude::Rect, Frame};

pub trait TuiComponent {
    fn render(&self, frame: &mut Frame, layout: &Rect);
}

pub trait TuiStatefulComponent {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect);
}
