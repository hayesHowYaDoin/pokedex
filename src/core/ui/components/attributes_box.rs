use crate::core::pokemon::{PokemonGenders, PokemonAttributes};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesBox {
    text: String,
}

impl AttributesBox {
    pub fn new(attributes: PokemonAttributes) -> Self {
        let abilities = attributes.abilities.join(", ");
        let genders = attributes.genders.iter()
            .map(|gender| match gender {
                PokemonGenders::Male => "♂",
                PokemonGenders::Female => "♀",
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let text = format!(
            "Height: {}\nWeight: {}\nCategory: {}\nAbilities: {}\nGender: {}",
            attributes.height,
            attributes.weight,
            attributes.category,
            abilities,
            genders,
        ).to_string();

        AttributesBox{ text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
