pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize("world"), "World");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("rUST"), "RUST");
    }

    #[test]
    fn test_capitalize_words() {
        assert_eq!(capitalize_words("hello world"), "Hello World");
        assert_eq!(
            capitalize_words("rust programming language"),
            "Rust Programming Language"
        );
        assert_eq!(capitalize_words(""), "");
        assert_eq!(capitalize_words("rUST is fUN"), "RUST Is FUN");
    }
}
