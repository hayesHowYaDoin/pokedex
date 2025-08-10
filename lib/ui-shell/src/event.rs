use crossterm::event::KeyCode;

use crate::tui::TuiEvent;
use ui_core::Event;

pub trait FromTuiEvent {
    fn from_tui_event(tui_event: Option<TuiEvent>) -> Self;
}

impl FromTuiEvent for Event {
    fn from_tui_event(tui_event: Option<TuiEvent>) -> Self {
        match tui_event {
            Some(TuiEvent::Key(key_event)) => match key_event.code {
                KeyCode::Up => Event::Up,
                KeyCode::Down => Event::Down,
                KeyCode::Char(c) => Event::NewCharacter(c),
                KeyCode::Backspace => Event::DeleteCharacter,
                KeyCode::Enter => Event::Select,
                KeyCode::Esc => Event::Quit,
                _ => Event::Noop,
            },
            _ => Event::Noop,
        }
    }
}
