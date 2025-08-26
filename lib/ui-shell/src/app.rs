use color_eyre::eyre::Result;

use crate::{event::FromTuiEvent, state::TuiState, tui::Tui};
use ui_core::{
    next_state,
    pages::ListPage,
    repository::{DetailPagePokemonRepository, ListPagePokemonRepository},
    Event, PageState,
};

pub struct App<R: ListPagePokemonRepository + DetailPagePokemonRepository> {
    repository: Box<R>,
    current_state: PageState,
}

impl<R: ListPagePokemonRepository + DetailPagePokemonRepository> App<R> {
    pub async fn new(repository: Box<R>) -> Result<Self> {
        let pokemon = repository.fetch_all().await?;
        let current_state = PageState::List(ListPage::new(pokemon, "".to_string()));

        Ok(App {
            repository,
            current_state,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        let result = loop {
            let event = Event::from_tui_event(tui.next().await);
            if should_quit(&event) {
                break Ok(());
            }

            if let Err(err) = self.transition(&event, &mut tui).await {
                break Err(err);
            }
        };

        tui.exit()?;
        result
    }

    async fn transition(&mut self, event: &Event, tui: &mut Tui) -> Result<&Self> {
        let mut next_state =
            next_state(&self.current_state, event, self.repository.as_ref()).await?;
        let state_changed =
            std::mem::discriminant(&self.current_state) != std::mem::discriminant(&next_state);

        let next_page = next_state.as_tui_page_mut();

        next_page.render(&mut tui.terminal, &mut tui.picker)?;
        if state_changed {
            self.current_state.as_tui_page_mut().on_exit();
            next_page.on_enter();
        }

        self.current_state = next_state;
        Ok(self)
    }
}

fn should_quit(event: &Event) -> bool {
    event == &Event::Quit
}
