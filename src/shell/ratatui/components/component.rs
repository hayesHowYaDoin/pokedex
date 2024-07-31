use ratatui::{
    prelude::Rect,
    terminal::Frame,
    widgets::Block,
};

pub trait TuiComponent {
    fn render(&self, frame: &mut Frame, layout: &Rect, block: &Block);
}

pub trait TuiStatefulComponent {
    fn render_mut(&mut self, frame: &mut Frame, layout: &Rect, block: &Block);
}
