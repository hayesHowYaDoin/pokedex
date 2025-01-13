use std::collections::HashSet;

use cascade::cascade;

use crate::core::{
    pokemon::Type,
    ui::components::{InputBox, PokemonTable, PokemonTableEntry},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListPage {
    pub search_widget: InputBox,
    pub list_widget: PokemonTable,
    pokemon: Vec<ListPagePokemon>,
}

impl ListPage {
    pub fn new(pokemon: &[ListPagePokemon], query: &str) -> Self {
        let filtered_pokemon: Vec<PokemonTableEntry> = filter_pokemon(pokemon, query)
            .into_iter()
            .map(|p| p.into())
            .collect();

        ListPage {
            search_widget: InputBox::new(query),
            list_widget: PokemonTable::new(&filtered_pokemon, 0),
            pokemon: pokemon.to_vec(),
        }
    }

    pub fn push_char(&mut self, c: char) {
        self.search_widget.push_char(c);
        self.update_table_pokemon();
    }

    pub fn pop_char(&mut self) {
        self.search_widget.pop_char();
        self.update_table_pokemon();
    }

    pub fn clear(&mut self) {
        self.search_widget.clear();
        self.update_table_pokemon();
    }

    fn update_table_pokemon(&mut self) {
        let filtered_pokemon: Vec<PokemonTableEntry> = filter_pokemon(&self.pokemon, self.search_widget.text())
            .into_iter()
            .map(|p| p.into())
            .collect();

        self.list_widget.set_pokemon(&filtered_pokemon);
    }
}

fn filter_pokemon(pokemon: &[ListPagePokemon], query: &str) -> Vec<ListPagePokemon> {
    let pokemon = cascade! {
        pokemon.to_owned();
        ..sort_by_key(|p| p.number);
    };

    if query.is_empty() {
        return pokemon;
    }

    let query = query.to_lowercase();

    let filtered_by_name: Vec<_> = pokemon.iter()
        .filter(|p| filter_pokemon_by_name(p, &query))
        .cloned()
        .collect();

    let filtered_by_primary_type: Vec<_> = pokemon.iter()
        .filter(|p| filter_pokemon_by_primary_type(p, &query))
        .cloned()
        .collect();

    let filtered_by_secondary_type: Vec<_> = pokemon.iter()
        .filter(|p| filter_pokemon_by_secondary_type(p, &query))
        .cloned()
        .collect();

    let mut result: Vec<ListPagePokemon> = Vec::new();
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

fn filter_pokemon_by_name(pokemon: &ListPagePokemon, query: &str) -> bool {
    let query = query.to_lowercase();
    let name = pokemon.name.to_lowercase();

    name.contains(&query)
}

fn filter_pokemon_by_primary_type(pokemon: &ListPagePokemon, query: &str) -> bool {
    let query = query.to_lowercase();
    let primary = pokemon.primary_type.to_string().to_lowercase();

    primary.contains(&query)
}

fn filter_pokemon_by_secondary_type(pokemon: &ListPagePokemon, query: &str) -> bool {
    let query = query.to_lowercase();
    let secondary = pokemon.secondary_type.as_ref().map(|t| t.to_string().to_lowercase());

    secondary.is_some_and(|t| t.contains(&query))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ListPagePokemon {
    pub number: i32,
    pub name: String,
    pub primary_type: Type,
    pub secondary_type: Option<Type>,
}

impl ListPagePokemon {
    pub fn new(number: i32, name: String, primary_type: Type, secondary_type: Option<Type>) -> Self {
        ListPagePokemon {
            number,
            name,
            primary_type,
            secondary_type,
        }
    }
}

impl From<ListPagePokemon> for PokemonTableEntry {
    fn from(pokemon: ListPagePokemon) -> Self {
        PokemonTableEntry {
            number: pokemon.number,
            name: pokemon.name,
            primary_type: pokemon.primary_type,
            secondary_type: pokemon.secondary_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::core::pokemon::Type;
    use super::*;

    static POKEMON: LazyLock<Vec<ListPagePokemon>> = LazyLock::new(|| vec![
        ListPagePokemon {number: 1, name: "Bulbasaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        ListPagePokemon {number: 2, name: "Ivysaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        ListPagePokemon {number: 3, name: "Venusaur".to_string(), primary_type: Type::Grass, secondary_type: Some(Type::Poison)},
        ListPagePokemon {number: 4, name: "Charmander".to_string(), primary_type: Type::Fire, secondary_type: None},
        ListPagePokemon {number: 5, name: "Charmeleon".to_string(), primary_type: Type::Fire, secondary_type: None},
        ListPagePokemon {number: 6, name: "Charizard".to_string(), primary_type: Type::Fire, secondary_type: Some(Type::Flying)},
        ListPagePokemon {number: 16, name: "Pidgey".to_string(), primary_type: Type::Normal, secondary_type: Some(Type::Flying)},
        ListPagePokemon {number: 22, name: "Fearow".to_string(), primary_type: Type::Normal, secondary_type: Some(Type::Flying)},
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
        assert_eq!(list_page.list_widget.get_pokemon(), &POKEMON.clone().into_iter().map(|p| p.into()).collect::<Vec<_>>());
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
