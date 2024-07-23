use color_eyre::eyre::Result;

use crate::{
    core::pokemon::Pokemon, 
    shell::ui::{
        pages::TuiPage,
        tui::Tui,
    }
};
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
    pub fn new(pokemon: &[Pokemon]) -> Self {
        App { should_quit: false, state: PageStateMachine::new(pokemon) }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        tui.enter()?;
        tui.start();

        loop {
            let event = Event::from(tui.next().await);

            self.update(&event);

            match self.state.next(&event).clone() {
                PageState::List(mut list_page) => {
                    list_page.render(&mut tui.terminal)?;
                }
                PageState::Detail => {},
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
