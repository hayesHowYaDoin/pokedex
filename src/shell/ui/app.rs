use color_eyre::eyre::Result;

use crate::core::pokemon::{self, Pokemon, PokemonNameRepository, PokemonNumberRepository, PokemonTypesRepository};
use crate::shell::ui::{pages::TuiPage, tui::Tui};
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
    pub fn new<R>(repository: R) -> Result<Self> 
    where R: PokemonNumberRepository + PokemonNameRepository + PokemonTypesRepository {
        let pokemon = repository.fetch_all_numbers()?
            .into_iter()
            .map(|number| {
                let name = repository.fetch_name(number)?;
                let primary_type = repository.fetch_primary_type(number)?;
                let secondary_type = repository.fetch_secondary_type(number)?;
                Ok(Pokemon::new(number, name, primary_type, secondary_type))
                })
            .collect::<Result<Vec<Pokemon>>>()?;

        Ok(App { should_quit: false, state: PageStateMachine::new(&pokemon) })
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
