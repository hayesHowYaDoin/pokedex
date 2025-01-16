use color_eyre::eyre::Result;

use crate::shell::ratatui::tui::Tui;
use crate::core::ui::{
    Event,
    PageState,
    next_state,
    pages::ListPage,
    repository::{ListPagePokemonRepository, DetailPagePokemonRepository},
};

pub struct App<R: ListPagePokemonRepository + DetailPagePokemonRepository> {
    repository: Box<R>,
    current_state: PageState,
}

impl<R: ListPagePokemonRepository + DetailPagePokemonRepository> App<R> {
    pub fn new(repository: Box<R>) -> Result<Self> {
        let pokemon = repository.fetch_all()?;
        let current_state = PageState::List(ListPage::new(pokemon, "".to_string()));

        Ok(App {repository, current_state})
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        loop {
            let event = Event::from(tui.next().await);
            if should_quit(&event) {
                break;
            }

            self.transition(&event, &mut tui)?;
        };

        tui.exit()?;
        Ok(())
    }

    fn transition(&mut self, event: &Event, tui: &mut Tui) -> Result<&Self>{
        let mut next_state = next_state(&self.current_state, event, self.repository.as_ref())?;

        match next_state {
            PageState::List(ref mut page) => {
                page.render(&mut tui.terminal, &mut tui.picker)?;
            }
            PageState::Detail(ref mut page) => {
                page.render(&mut tui.terminal, &mut tui.picker)?;

                if let PageState::List(_) = self.current_state {
                    page.on_enter();
                }
            }
        };

        self.current_state = next_state;

        Ok(self)
    }
}

fn should_quit(event: &Event) -> bool{
    event == &Event::NewCharacter('q')
}
