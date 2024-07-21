use std::default::Default;


#[derive(Clone, Debug)]
pub struct InputBox {
    text: String,
}

impl InputBox {
    pub fn new(text: &str) -> Self {
        Self {text: text.to_string()}
    }

    pub fn push_char(&mut self, c: char) -> &Self {
        self.text.push(c);
        self
    }

    pub fn pop_char(&mut self) -> &Self {
        self.text.pop();
        self
    }

    pub fn clear(&mut self) -> &Self {
        self.text.clear();
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Default for InputBox {
    fn default() -> Self {
        Self::new("")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input_box = InputBox::new("Hello");
        assert_eq!(input_box.text(), "Hello");
    }

    #[test]
    fn test_push_char() {
        let mut input_box = InputBox::new("Hello");
        input_box.push_char('!');

        assert_eq!(input_box.text(), "Hello!");
    }

    #[test]
    fn test_pop_char() {
        let mut input_box = InputBox::new("Hello");
        input_box.pop_char();

        assert_eq!(input_box.text(), "Hell");
    }

    #[test]
    fn test_clear() {
        let mut input_box = InputBox::new("Hello");
        input_box.clear();

        assert_eq!(input_box.text(), "");
    }
}
