#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
}

impl Pokemon {
    pub fn new(number: i32, name: String) -> Self {
        Pokemon { number, name }
    }
}