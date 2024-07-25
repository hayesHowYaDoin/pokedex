use std::collections::HashSet;

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

    result.into_iter()
        .fold((Vec::new(), HashSet::new()), |(mut result, mut seen), item| {
            if seen.insert(item.clone()) {
                result.push(item);
            }
            (result, seen)
        }).0
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
    use std::sync::LazyLock;

    use crate::core::pokemon::Type;
    use super::*;

    static POKEMON: LazyLock<Vec<PokemonTableEntry>> = LazyLock::new(|| vec![
        PokemonTableEntry {number: 1, name: "Bulbasaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        PokemonTableEntry {number: 2, name: "Ivysaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        PokemonTableEntry {number: 3, name: "Venusaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        PokemonTableEntry {number: 4, name: "Charmander".to_string(), primary_type: Type::Fire, secondary_type: None},
        PokemonTableEntry {number: 5, name: "Charmeleon".to_string(), primary_type: Type::Fire, secondary_type: None},
        PokemonTableEntry {number: 6, name: "Charizard".to_string(), primary_type: Type::Fire, secondary_type: Some(Type::Flying)},
        PokemonTableEntry {number: 16, name: "Pidgey".to_string(), primary_type: Type::Normal, secondary_type: Some(Type::Flying)},
        PokemonTableEntry {number: 22, name: "Fearow".to_string(), primary_type: Type::Normal, secondary_type: Some(Type::Flying)},
    ]);
    
    #[test]
    fn test_new() {
        let list_page = ListPage::new(&[], "");
        assert_eq!(list_page.search_widget.text(), "");
        assert_eq!(list_page.list_widget.get_selected(), None);
    }

    #[test]
    fn test_empty_query() {
        let list_page = ListPage::new(&POKEMON, "");
        assert_eq!(list_page.list_widget.get_pokemon(), POKEMON.as_slice());
    }

    #[test]
    fn test_query_name() {
        let list_page = ListPage::new(&POKEMON, "saur");
        let filtered_pokemon = list_page.list_widget.get_pokemon();
        assert_eq!(filtered_pokemon.len(), 3);
        assert_eq!(filtered_pokemon[0].name, "Bulbasaur");
        assert_eq!(filtered_pokemon[1].name, "Ivysaur");
        assert_eq!(filtered_pokemon[2].name, "Venusaur");
    }

    #[test]
    fn test_query_name_primary_secondary() {
        let list_page = ListPage::new(&POKEMON, "F");
        let filtered_pokemon = list_page.list_widget.get_pokemon();
        assert_eq!(filtered_pokemon.len(), 5);
        assert_eq!(filtered_pokemon[0].name, "Fearow");
        assert_eq!(filtered_pokemon[1].name, "Charmander");
        assert_eq!(filtered_pokemon[2].name, "Charmeleon");
        assert_eq!(filtered_pokemon[3].name, "Charizard");
        assert_eq!(filtered_pokemon[4].name, "Pidgey");
    }
}
