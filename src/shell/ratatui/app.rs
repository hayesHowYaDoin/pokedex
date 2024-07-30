use color_eyre::eyre::Result;

use crate::core::ui::repository::ListPagePokemonRepository;
use crate::shell::ratatui::{pages::TuiPage, tui::Tui};
use crate::core::ui::{
    Event,
    PageState,
    PageStateMachine,
};

pub struct App {
    should_quit: bool,
    state: PageStateMachine,
}

impl App {
    pub fn new(repository: &impl ListPagePokemonRepository) -> Result<Self> {
        Ok(App { should_quit: false, state: PageStateMachine::new(repository)? })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        loop {
            let event = Event::from(tui.next().await);

            self.update(&event);

            match self.state.next(&event).clone() {
                PageState::List(mut page) => {
                    page.render(&mut tui.terminal)?;
                }
                PageState::Detail(mut _page) => {},
                PageState::Exit => break,
            }
        };

        tui.exit()?;
        Ok(())
    }

    fn update(&mut self, event: &Event) {
        if event == &Event::NewCharacter('q') {
            self.should_quit = true;
        }
    }
}
