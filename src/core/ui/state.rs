use crate::core::{
    pokemon::Pokemon,
    ui::components::PokemonTableEntry,
};
use super::{
    pages::{DetailPage, ListPage},
    Event,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageState {
    List(ListPage),
    Detail(DetailPage),
    Exit,
}

pub struct PageStateMachine {
    page: PageState,
}

impl PageStateMachine {
    pub fn new(pokemon: &[Pokemon]) -> Self {
        let pokemon_table_entries: Vec<PokemonTableEntry> = pokemon
            .iter()
            .map(|p| p.into())
            .collect();

        PageStateMachine {
            page: PageState::List(ListPage::new(&pokemon_table_entries, ""))
        }

        // PageStateMachine {
        //     page: PageState::Detail(DetailPage::new()),
        // }
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

fn next_detail(page: &DetailPage, _event: &Event) -> PageState {
    PageState::Detail(page.clone())
}

impl<'a> From<&'a Pokemon> for PokemonTableEntry {
    fn from(pokemon: &'a Pokemon) -> Self {
        PokemonTableEntry {
            number: pokemon.number,
            name: pokemon.name.clone(),
            primary_type: pokemon.types.primary.clone(),
            secondary_type: pokemon.types.secondary.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use cascade::cascade;

    use crate::core::{
        pokemon::Type,
        ui::components::PokemonTableEntry,
    };
    use super::*;

    static POKEMON: LazyLock<Vec<Pokemon>> = LazyLock::new(|| vec![
        Pokemon::new(1, "".to_string(), Type::Normal, None),
        Pokemon::new(2, "".to_string(), Type::Normal, None),
        Pokemon::new(3, "".to_string(), Type::Normal, None),
        Pokemon::new(4, "".to_string(), Type::Normal, None),
        Pokemon::new(5, "".to_string(), Type::Normal, None),
    ]);

    #[test]
    fn test_next_list_new_character_q() {
        let mut state_machine = PageStateMachine::new(&[]);
        assert_eq!(state_machine.next(&Event::NewCharacter('q')), PageState::Exit);
    }

    #[test]
    fn test_next_list_new_character_other() {
        let mut list_page = ListPage::new(&[], "");
        list_page.search_widget.push_char('a');

        let next_list_page = PageStateMachine::new(&[]).next(&Event::NewCharacter('a'));
        assert_eq!(next_list_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_delete_character() {
        let mut state_machine = PageStateMachine::new(&POKEMON);
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
        let mut state_machine = PageStateMachine::new(&POKEMON);
        let next_state = state_machine.next(&Event::Down);

        let pokemon_table_entries: Vec<PokemonTableEntry> = POKEMON
            .iter()
            .map(|p| p.into())
            .collect();

        let mut expected_next_page = ListPage::new(&pokemon_table_entries, "");
        expected_next_page.list_widget.down();

        assert_eq!(next_state, PageState::List(expected_next_page));
    }

    #[test]
    fn test_next_list_up() {
        let state_machine = PageStateMachine::new(&POKEMON);
        let next_state: PageStateMachine = cascade! {
            state_machine;
            ..next(&Event::Down);
            ..next(&Event::Down);
            ..next(&Event::Up);
        };
        
        let pokemon_table_entries: Vec<PokemonTableEntry> = POKEMON
        .iter()
        .map(|p| p.into())
        .collect();

        let mut expected_next_page = ListPage::new(&pokemon_table_entries, "");
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
        let mut state_machine = PageStateMachine::new(&[]);
        let previous_state = state_machine.page.clone();
        let next_state = state_machine.next(&Event::Select);
        assert_eq!(next_state, previous_state);
    }

    #[test]
    fn test_next_list_noop() {
        let mut state_machine = PageStateMachine::new(&[]);
        let previous_state = state_machine.next(&Event::Up);
        let next_state = state_machine.next(&Event::Noop);
        assert_eq!(next_state, previous_state);
    }
}