use crate::core::pokemon::Pokemon;
use super::{
    pages::ListPage,
    Event,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageState {
    List(ListPage),
    Detail,
    Exit,
}

pub struct PageStateMachine {
    page: PageState,
}

impl PageStateMachine {
    pub fn new(pokemon: &[Pokemon]) -> Self {
        PageStateMachine {
            page: PageState::List(ListPage::new(pokemon)),
        }
    }

    pub fn next(&mut self, event: &Event) -> PageState {
        let next_page = match &self.page {
            PageState::List(list_page) => next_list(list_page, event),
            PageState::Detail => PageState::Detail,
            _ => PageState::Exit,
        };

        self.page = next_page;
        self.page.clone()
    }
}

fn next_list(page: &ListPage, event: &Event) -> PageState {
    match event {
        Event::NewCharacter(c) => {
            if *c == 'q' {
                return PageState::Exit;
            }

            let mut next_page = page.clone();
            next_page.search_widget.push_char(*c);
            PageState::List(next_page)
        }
        Event::DeleteCharacter => {
            let mut next_page = page.clone();
            next_page.search_widget.pop_char();
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

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use cascade::cascade;

    use super::*;

    static POKEMON: LazyLock<Vec<Pokemon>> = LazyLock::new(|| vec![
        Pokemon {number: 1, name: "Bulbasaur".to_string()},
        Pokemon {number: 2, name: "Ivysaur".to_string()},
        Pokemon {number: 3, name: "Venusaur".to_string()},
        Pokemon {number: 4, name: "Charmander".to_string()},
        Pokemon {number: 5, name: "Charmeleon".to_string()},
    ]);

    #[test]
    fn test_next_list_new_character_q() {
        let mut state_machine = PageStateMachine::new(&[]);
        assert_eq!(state_machine.next(&Event::NewCharacter('q')), PageState::Exit);
    }

    #[test]
    fn test_next_list_new_character_other() {
        let mut list_page = ListPage::new(&[]);
        list_page.search_widget.push_char('a');

        let next_list_page = PageStateMachine::new(&[]).next(&Event::NewCharacter('a'));
        assert_eq!(next_list_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_delete_character() {
        let mut state_machine = PageStateMachine::new(&[]);
        let event = Event::DeleteCharacter;
        let next_state = state_machine.next(&event);
        assert_eq!(next_state, PageState::List(ListPage::new(&[])));
    }

    #[test]
    fn test_next_list_down() {
        let mut state_machine = PageStateMachine::new(&POKEMON);
        let next_state = state_machine.next(&Event::Down);

        let mut expected_next_page = ListPage::new(&POKEMON);
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

        let mut expected_next_page = ListPage::new(&POKEMON);
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
        let event = Event::Select;
        let next_state = state_machine.next(&event);
        assert_eq!(next_state, PageState::Detail);
    }

    #[test]
    fn test_next_list_noop() {
        let mut state_machine = PageStateMachine::new(&[]);
        let previous_state = state_machine.next(&Event::Up);
        let next_state = state_machine.next(&Event::Noop);
        assert_eq!(next_state, previous_state);
    }
}