#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonCry {
    pub cry: Vec<u8>,
}

impl PokemonCry {
    pub fn new(cry: Vec<u8>) -> Self {
        PokemonCry { cry }
    }
}
