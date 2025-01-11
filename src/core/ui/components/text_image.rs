use std::default::Default;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextImage {
    text: String,
}

impl TextImage {
    pub fn new(text: &str) -> Self {
        Self {text: text.to_string()}
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Default for TextImage {
    fn default() -> Self {
        Self::new("")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let input_box = TextImage::new("Hello");
        assert_eq!(input_box.text(), "Hello");
    }
}
