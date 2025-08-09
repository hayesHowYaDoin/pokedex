#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonImage {
    pub image: String,
}

impl PokemonImage {
    pub fn new(image: String) -> Self {
        PokemonImage { image }
    }
}
