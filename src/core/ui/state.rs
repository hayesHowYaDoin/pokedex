use color_eyre::Result;

use crate::core::ui::components::PokemonTableEntry;
use super::{
    pages::{DetailPage, DetailPagePokemon, ListPage, ListPagePokemon},
    repository::ListPagePokemonRepository,
    Event,
};

// TODO: Remove when no longer needed for testing
use crate::core::pokemon::{PokemonDescription, PokemonStats, PokemonTypes, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum PageState {
    List(ListPage),
    Detail(DetailPage),
    Exit,
}

pub struct PageStateMachine {
    page: PageState,
}

impl PageStateMachine {
    pub fn new(_pokemon_repository: &impl ListPagePokemonRepository) -> Result<Self> {
        // Ok(PageStateMachine {
        //     page: PageState::List(ListPage::new(&ListPagePokemonRepository::fetch_all(pokemon_repository)?, ""))
        // })

        let test_pokemon = DetailPagePokemon::new(
            1,
            "Bulbasaur".to_string(),
            PokemonTypes::new(Type::Grass, Some(Type::Poison)),
            PokemonDescription::new("A strange seed was planted on its back at birth. The plant sprouts and grows with this POKéMON.".to_string()),
            PokemonStats::new(45, 49, 49, 65, 65, 45),
        );

        Ok(PageStateMachine {
            page: PageState::Detail(DetailPage::new(&test_pokemon)?)
        })
    }

    pub fn next(&mut self, event: &Event) -> PageState {
        let next_page = match &self.page {
            PageState::List(page) => next_list(page, event),
            PageState::Detail(page) => next_detail(page, event),
            _ => PageState::Exit,
        };

        self.page = next_page;
        self.page.clone()
    }
}

fn next_list(page: &ListPage, event: &Event) -> PageState {
    match event {
        Event::NewCharacter(c) => {
            if *c == 'q' { return PageState::Exit; }

            let mut next_page = page.clone();
            next_page.push_char(*c);

            PageState::List(next_page)
        }
        Event::DeleteCharacter => {
            let mut next_page = page.clone();
            next_page.pop_char();

            PageState::List(next_page)
        }
        Event::Up => {
            let mut next_page = page.clone();
            next_page.list_widget.up();

            PageState::List(next_page)
        }
        Event::Down => {
            let mut next_page = page.clone();
            next_page.list_widget.down();

            PageState::List(next_page)
        }
        Event::Select | Event::Noop => PageState::List(page.clone()),
    }
}

fn next_detail(page: &DetailPage, event: &Event) -> PageState {
    match event {
        Event::NewCharacter(c) if *c == 'q' => return PageState::Exit,
        _ => PageState::Detail(page.clone())
    }
    
}

impl<'a> From<&'a ListPagePokemon> for PokemonTableEntry {
    fn from(pokemon: &'a ListPagePokemon) -> Self {
        PokemonTableEntry {
            number: pokemon.number,
            name: pokemon.name.clone(),
            primary_type: pokemon.primary_type,
            secondary_type: pokemon.secondary_type,
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use cascade::cascade;

    use crate::core::{
        pokemon::Type,
        ui::{
            components::PokemonTableEntry, pages::ListPagePokemon, repository
        }
    };
    use super::*;

    static LIST_POKEMON: LazyLock<Vec<ListPagePokemon>> = LazyLock::new(|| vec![
        ListPagePokemon::new(1, "".to_string(), Type::Normal, None),
        ListPagePokemon::new(2, "".to_string(), Type::Normal, None),
        ListPagePokemon::new(3, "".to_string(), Type::Normal, None),
        ListPagePokemon::new(4, "".to_string(), Type::Normal, None),
        ListPagePokemon::new(5, "".to_string(), Type::Normal, None),
    ]);

    struct TestRepository();
    
    impl ListPagePokemonRepository for TestRepository {
        fn fetch_all(&self) -> Result<Vec<ListPagePokemon>> {
            Ok(LIST_POKEMON.clone())
        }
    }

    #[test]
    fn test_next_list_new_character_q() {
        let repository = TestRepository();
        let mut state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");
        
        assert_eq!(state_machine.next(&Event::NewCharacter('q')), PageState::Exit);
    }

    #[test]
    fn test_next_list_new_character_other() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");

        let mut list_page = ListPage::new(&pokemon, "");
        list_page.search_widget.push_char('a');

        let next_list_page = PageStateMachine::new(&repository).unwrap().next(&Event::NewCharacter('a'));
        assert_eq!(next_list_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_delete_character() {
        let repository = TestRepository();

        let mut state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");

        let initial_state = state_machine.page.clone();

        match state_machine.page {
            PageState::List(ref mut page) => {
                page.search_widget.push_char('a');
            }
            _ => panic!("Expected state machine to be initialize with PageState::List"),
        }

        let next_state = state_machine.next(&Event::DeleteCharacter);

        assert_eq!(next_state, initial_state);
    }

    #[test]
    fn test_next_list_down() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");

        let mut state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");

        let next_state = state_machine.next(&Event::Down);

        let mut expected_next_page = ListPage::new(&pokemon, "");
        expected_next_page.list_widget.down();

        assert_eq!(next_state, PageState::List(expected_next_page));
    }

    #[test]
    fn test_next_list_up() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");

        let state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");

        let next_state: PageStateMachine = cascade! {
            state_machine;
            ..next(&Event::Down);
            ..next(&Event::Down);
            ..next(&Event::Up);
        };

        let mut expected_next_page = ListPage::new(&pokemon, "");
        expected_next_page.list_widget = cascade! {
            expected_next_page.list_widget;
            ..down();
            ..down();
            ..up();
        };

        assert_eq!(next_state.page, PageState::List(expected_next_page));
    }

    #[test]
    fn test_next_list_select() {
        let repository = TestRepository();
        let mut state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");
    
        let previous_state = state_machine.page.clone();
        let next_state = state_machine.next(&Event::Select);
        assert_eq!(next_state, previous_state);
    }

    #[test]
    fn test_next_list_noop() {
        let repository = TestRepository();
        let mut state_machine = PageStateMachine::new(&repository)
            .expect("Failed to create PageStateMachine");

        let previous_state = state_machine.next(&Event::Up);
        let next_state = state_machine.next(&Event::Noop);
        assert_eq!(next_state, previous_state);
    }
}