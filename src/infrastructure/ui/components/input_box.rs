use std::default::Default;

use ratatui::{
    prelude::Rect,
    terminal::Frame,
    widgets::{Block, Paragraph},
};

use super::Component;

#[derive(Clone, Debug)]
pub struct InputBox {
    text: String,
}

impl InputBox {
    pub fn new(text: &str) -> Self {
        Self {text: text.to_string()}
    }

    pub fn push_char(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn pop_char(&mut self) {
        self.text.pop();
    }

    pub fn clear(&mut self) {
        self.text.clear();
    
    }
}

impl Default for InputBox {
    fn default() -> Self {
        Self::new("")
    }

}

impl Component for InputBox {
    fn render(&mut self, frame: &mut Frame, layout: &Rect) {
        let widget = Paragraph::new(self.text.clone())
            .block(Block::bordered().title("Search"));

        frame.render_widget(&widget, *layout);
    }
}