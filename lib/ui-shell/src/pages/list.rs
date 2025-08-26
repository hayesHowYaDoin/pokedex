use std::{default::Default, sync::LazyLock};

use color_eyre::eyre::Result;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Stylize},
    widgets::{Block, Paragraph},
};
use ratatui_image::picker::Picker;

use super::page::TuiPage;
use crate::{
    components::{
        input_box::TuiInputBox, pokemon_table::TuiPokemonTable, TuiComponent, TuiStatefulComponent,
    },
    tui::Terminal,
};
use ui_core::pages::ListPage;

static LAYOUT: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
});

impl TuiPage for ListPage {
    fn on_enter(&mut self) {
        log::trace!("Entering ListPage");
    }

    fn on_exit(&mut self) {
        log::trace!("Entering ListPage");
    }

    fn render(&mut self, terminal: &mut Terminal, _picker: &mut Picker) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.area());

            let search = TuiInputBox::new(
                self.search_widget.clone(),
                Block::bordered().title("Search"),
            );
            search.render(frame, &layout[0]);

            let mut list = TuiPokemonTable::new(self.list_widget.clone(), Block::bordered());
            list.render_mut(frame, &layout[1]);

            let widget = Paragraph::new("Press 'enter' for detailed view, 'esc' to quit")
                .fg(Color::DarkGray);

            frame.render_widget(widget, layout[2]);
        })?;

        Ok(())
    }
}
