#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextBox {
    text: String,
}

impl TextBox {
    pub fn new(text: &str) -> Self {
        TextBox{ text: text.to_string() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
