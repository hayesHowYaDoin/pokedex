use crate::core::ui::components::{InputBox, PokemonTable, PokemonTableEntry};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListPage {
    pub search_widget: InputBox,
    pub list_widget: PokemonTable,
    pokemon: Vec<PokemonTableEntry>,
}

impl ListPage {
    pub fn new(pokemon: &[PokemonTableEntry], query: &str) -> Self {
        ListPage {
            search_widget: InputBox::new(query),
            list_widget: PokemonTable::new(&filter_pokemon(pokemon, query), 0),
            pokemon: pokemon.to_vec(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.search_widget.push_char(c);
        self.list_widget.set_pokemon(&filter_pokemon(&self.pokemon, self.search_widget.text()));
    }

    pub fn pop_char(&mut self) {
        self.search_widget.pop_char();
        self.list_widget.set_pokemon(&filter_pokemon(&self.pokemon, self.search_widget.text()));
    }

    pub fn clear(&mut self) {
        self.search_widget.clear();
        self.list_widget.set_pokemon(&self.pokemon);
    }
}

fn filter_pokemon(pokemon: &[PokemonTableEntry], query: &str) -> Vec<PokemonTableEntry> {
    if query.is_empty() {
        return pokemon.to_vec();
    }

    let query = query.to_lowercase();

    let filtered_by_name: Vec<PokemonTableEntry> = pokemon.iter()
        .filter(|p| filter_pokemon_by_name(p, &query))
        .cloned()
        .collect();

    let filtered_by_primary_type: Vec<PokemonTableEntry> = pokemon.iter()
        .filter(|p| filter_pokemon_by_primary_type(p, &query))
        .cloned()
        .collect();

    let filtered_by_secondary_type: Vec<PokemonTableEntry> = pokemon.iter()
        .filter(|p| filter_pokemon_by_secondary_type(p, &query))
        .cloned()
        .collect();

    let mut result: Vec<PokemonTableEntry> = Vec::new();
    result.extend(filtered_by_name);
    result.extend(filtered_by_primary_type);
    result.extend(filtered_by_secondary_type);

    result.clone().into_iter()
        .filter(|p| result.iter().filter(|&x| x == p).count() == 1)
        .collect()
}

fn filter_pokemon_by_name(pokemon: &PokemonTableEntry, query: &str) -> bool {
    let query = query.to_lowercase();
    let name = pokemon.name.to_lowercase();

    name.contains(&query)
}

fn filter_pokemon_by_primary_type(pokemon: &PokemonTableEntry, query: &str) -> bool {
    let query = query.to_lowercase();
    let primary = pokemon.primary_type.to_string().to_lowercase();

    primary.contains(&query)
}

fn filter_pokemon_by_secondary_type(pokemon: &PokemonTableEntry, query: &str) -> bool {
    let query = query.to_lowercase();
    let secondary = pokemon.secondary_type.as_ref().map(|t| t.to_string().to_lowercase());

    secondary.map_or(false, |t| t.contains(&query))
}

#[cfg(test)]
mod tests {
    use crate::core::pokemon::Type;
    use super::*;
    
    #[test]
    fn test_new() {
        let list_page = ListPage::new(&[], "");
        assert_eq!(list_page.search_widget.text(), "");
        assert_eq!(list_page.list_widget.get_selected(), None);
    }

    #[test]
    fn test_empty_query() {
        let pokemon = vec![
            PokemonTableEntry::new(1, "Bulbasaur".to_string(), Type::Grass, Some(Type::Poison)),
            PokemonTableEntry::new(2, "Ivysaur".to_string(), Type::Grass, Some(Type::Poison)),
            PokemonTableEntry::new(3, "Venusaur".to_string(), Type::Grass, Some(Type::Poison)),
        ];

        let list_page = ListPage::new(&pokemon, "");
        assert_eq!(list_page.list_widget.get_pokemon(), &pokemon);
    }
}
