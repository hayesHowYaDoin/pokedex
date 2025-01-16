use std::default::Default;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InputBox {
    text: String,
}

impl InputBox {
    pub fn new(text: String) -> Self {
        Self {text: text}
    }

    pub fn push_char(&mut self, c: impl Into<char>) {
        self.text.push(c.into());
    }

    pub fn pop_char(&mut self) {
        self.text.pop();
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Default for InputBox {
    fn default() -> Self {
        Self::new("".to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use cascade::cascade;

    #[test]
    fn test_push_char() {
        let input_box = cascade! {
            InputBox::new("Hello".to_string());
            ..push_char('!');
        };
        assert_eq!(input_box.text(), "Hello!");
    }

    #[test]
    fn test_pop_char() {
        let input_box = cascade! {
            InputBox::new("Hello".to_string());
            ..pop_char();
        };
        assert_eq!(input_box.text(), "Hell");
    }

    #[test]
    fn test_clear() {
        let input_box = cascade! {
            InputBox::new("Hello".to_string());
            ..clear();
        };
        assert_eq!(input_box.text(), "");
    }

    #[test]
    fn test_text() {
        let input_box = InputBox::new("Hello".to_string());
        assert_eq!(input_box.text(), "Hello");
    }
}
