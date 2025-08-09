use color_eyre::{eyre, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PokemonGenderRates {
    pub male: f32,
    pub female: f32,
}

impl PokemonGenderRates {
    pub fn new(male: f32, female: f32) -> Result<Self> {
        if male + female != 1.0 {
            return Err(eyre::eyre!("Male and female rates must sum to 1.0"));
        }

        Ok(PokemonGenderRates { male, female })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PokemonAttributes {
    pub height_m: String,
    pub weight_kg: String,
    pub category: String,
    pub abilities: Vec<String>,
    pub genders: Option<PokemonGenderRates>,
}

impl PokemonAttributes {
    pub fn new(
        height: String,
        weight: String,
        category: String,
        abilities: Vec<String>,
        genders: Option<PokemonGenderRates>,
    ) -> Self {
        PokemonAttributes {
            height_m: height,
            weight_kg: weight,
            category,
            abilities,
            genders,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expected() {
        PokemonGenderRates::new(0.875, 0.125).unwrap();
        PokemonGenderRates::new(0.5, 0.5).unwrap();
        PokemonGenderRates::new(0.0, 1.0).unwrap();
        PokemonGenderRates::new(1.0, 0.0).unwrap();
    }

    #[test]
    fn less_than_1() {
        assert!(PokemonGenderRates::new(0.5, 0.4).is_err());
        assert!(PokemonGenderRates::new(0.0, 0.0).is_err());
        assert!(PokemonGenderRates::new(0.1, 0.2).is_err());
    }

    #[test]
    fn greater_than_1() {
        assert!(PokemonGenderRates::new(0.6, 0.5).is_err());
        assert!(PokemonGenderRates::new(1.0, 0.1).is_err());
        assert!(PokemonGenderRates::new(0.9, 0.2).is_err());
    }
}