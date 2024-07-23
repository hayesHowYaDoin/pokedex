use color_eyre::eyre::{Result, WrapErr};

use crate::core::pokemon::Pokemon;
use super::{
    pages::ListPage,
    Event,
};

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
            page: PageState::List(ListPage::new(&pokemon)),
        }
    }

    pub fn next(&mut self, event: &Event) -> PageState {
        match &self.page {
            PageState::List(list_page) => self._next_list(&list_page, event),
            PageState::Detail => PageState::Detail,
            _ => PageState::Exit,
        }
    }

    fn _next_list(&self, page: &ListPage, event: &Event) -> PageState {
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
            Event::Select => PageState::Detail,
            Event::Noop => PageState::List(page.to_owned()),
        }
    }
}

