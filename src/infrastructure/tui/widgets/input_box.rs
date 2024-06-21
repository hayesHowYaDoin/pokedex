use ratatui::{
    prelude::{Buffer, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

pub struct InputBox<'a> {
    pub block: Block<'a>,
    pub text: String,
    pub style: Style,
}

impl<'a> InputBox<'a> {
    pub fn new(block: Block<'a>) -> Self {
        Self {
            block,
            text: String::default(),
            style: Style::default().fg(Color::White),
        }
    }
}

impl Widget for InputBox<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Input text here").render(area, buf);
    }
}

impl WidgetRef for InputBox<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Input text here").render(area, buf);
    }
}