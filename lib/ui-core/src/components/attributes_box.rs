use pokemon::PokemonAttributes;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AttributesBox {
    text: String,
}

impl AttributesBox {
    pub fn new(attributes: PokemonAttributes) -> Self {
        let abilities = attributes.abilities.join(", ");
        let gender_rates = match attributes.genders {
            Some(rates) => {
                format!(
                    "♀: {:.0}% ♂: {:.0}%",
                    rates.female * 100.0,
                    rates.male * 100.0
                )
            }
            None => "N/A".to_string(),
        };
        let text = format!(
            "Height: {}m\nWeight: {}kg\nCategory: {}\nAbilities: {}\nGender: {}",
            attributes.height_m, attributes.weight_kg, attributes.category, abilities, gender_rates,
        )
        .to_string();

        AttributesBox { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
