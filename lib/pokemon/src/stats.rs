#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub special_attack: u32,
    pub special_defense: u32,
    pub speed: u32,
}

impl PokemonStats {
    pub fn new(
        hp: u32,
        attack: u32,
        defense: u32,
        special_attack: u32,
        special_defense: u32,
        speed: u32,
    ) -> Self {
        PokemonStats {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        }
    }
}