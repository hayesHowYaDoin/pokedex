use crossterm::event::KeyCode;

use crate::core::ui::Event;
use crate::shell::ratatui::tui::TuiEvent;

impl From<Option<TuiEvent>> for Event {
    fn from(tui_event: Option<TuiEvent>) -> Self {
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
