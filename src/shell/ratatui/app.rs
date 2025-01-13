use color_eyre::eyre::Result;

use crate::shell::ratatui::{
    pages::{TuiDetailPage, TuiListPage},
    tui::Tui,
};
use crate::core::ui::{
    Event,
    PageState,
    next_state,
    pages::ListPage,
    repository::ListPagePokemonRepository,
};

#[derive(Debug)]
enum TuiPage {
    List(TuiListPage),
    Detail(TuiDetailPage),
}

pub struct App {
    repository: Box<dyn ListPagePokemonRepository>,
    current_state: TuiPage,
}

impl App {
    pub fn new(repository: Box<dyn ListPagePokemonRepository>) -> Result<Self> {
        let pokemon = repository.fetch_all()?;
        let current_state = TuiPage::List(TuiListPage::new(ListPage::new(&pokemon, "")));

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
        let current_state: PageState = match self.current_state {
            TuiPage::List(ref mut page) => PageState::List(page.page.clone()),
            TuiPage::Detail(ref mut page) => PageState::Detail(page.page.clone()),
        };
        let next_state = next_state(&current_state, &event, self.repository.as_ref())?;

        match next_state {
            PageState::List(page) => {
                let mut tui_page = TuiListPage::new(page);
                tui_page.render(&mut tui.terminal, &mut tui.picker)?;

                self.current_state = TuiPage::List(tui_page);
            }
            PageState::Detail(page) => {
                if let TuiPage::Detail( detail_page) = &mut self.current_state {
                    detail_page.set_page(page);
                }
                else if let TuiPage::List(_) = &self.current_state {
                    let mut tui_page = TuiDetailPage::new(page)?;
                    tui_page.on_enter();
                    tui_page.render(&mut tui.terminal, &mut tui.picker)?;
                    self.current_state = TuiPage::Detail(tui_page);
                }
            },
        };

        Ok(self)
    }
}

fn should_quit(event: &Event) -> bool{
    event == &Event::NewCharacter('q')
}
