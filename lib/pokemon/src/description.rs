#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonDescription {
    pub text: String,
}

impl PokemonDescription {
    pub fn new(text: String) -> Self {
        PokemonDescription { text }
    }
}
