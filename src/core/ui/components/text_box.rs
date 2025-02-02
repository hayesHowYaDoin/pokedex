#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextBox {
    text: String,
}

impl TextBox {
    pub fn new(text: String) -> Self {
        TextBox{ text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
