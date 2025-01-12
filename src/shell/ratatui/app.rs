use color_eyre::eyre::Result;

use crate::shell::ratatui::tui::Tui;
use crate::core::ui::{
    Event,
    PageState,
    PageStateMachine,
    repository::ListPagePokemonRepository
};

pub struct App {
    state: PageStateMachine,
}

impl App {
    pub fn new(repository: Box<dyn ListPagePokemonRepository>) -> Result<Self> {
        Ok(App { state: PageStateMachine::new(repository)? })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        loop {
            let event = Event::from(tui.next().await);
            if self.should_quit(&event) {
                break;
            }

            let next_state = self.state.next(&event)?;
            self.transition(next_state, &mut tui)?;
        };

        tui.exit()?;
        Ok(())
    }

    fn should_quit(&mut self, event: &Event) -> bool{
        event == &Event::NewCharacter('q')
    }

    fn transition(&mut self, mut next_state: PageState, tui: &mut Tui) -> Result<()>{
        match next_state {
            PageState::List(ref mut page) => {
                page.render(&mut tui.terminal, &mut tui.picker)?;
            }
            PageState::Detail(ref mut page) => {
                if let PageState::List(_) = self.state.page {
                    page.on_enter();
                }
                page.render(&mut tui.terminal, &mut tui.picker)?;
            },
        }

        self.state.page = next_state;
        Ok(())
    }
}
