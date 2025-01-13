use crate::core::pokemon::{PokemonGenders, PokemonMetadata};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MetadataBox {
    text: String,
}

impl MetadataBox {
    pub fn new(metadata: &PokemonMetadata) -> Self {
        let abilities = metadata.abilities.join(", ");
        let genders = metadata.genders.iter()
            .map(|gender| match gender {
                PokemonGenders::Male => "♂",
                PokemonGenders::Female => "♀",
            })
            .collect::<Vec<&str>>()
            .join(" ");

        let text = format!(
            "Height: {}\nWeight: {}\nCategory: {}\nAbilities: {}\nGender: {}",
            metadata.height,
            metadata.weight,
            metadata.category,
            abilities,
            genders,
        ).to_string();

        MetadataBox{ text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
