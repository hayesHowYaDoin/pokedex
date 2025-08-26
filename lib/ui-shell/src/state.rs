use crate::pages::TuiPage;
use ui_core::PageState;

pub trait TuiState {
    fn as_tui_page_mut(&mut self) -> &mut dyn TuiPage;
}

impl TuiState for PageState {
    fn as_tui_page_mut(&mut self) -> &mut dyn TuiPage {
        match self {
            PageState::List(page) => page,
            PageState::Detail(page) => page,
        }
    }
}
