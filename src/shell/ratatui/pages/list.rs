use std::{default::Default, sync::LazyLock};

use color_eyre::eyre::Result;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal},
    style::{Color, Stylize},
    widgets::{Block, Paragraph},
};
use ratatui_image::picker::Picker;

use crate::core::ui::pages::ListPage;
use crate::shell::ratatui::components::{
    input_box::TuiInputBox, pokemon_table::TuiPokemonTable, TuiComponent, TuiStatefulComponent,
};

static LAYOUT: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
});

impl ListPage {
    pub fn render<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        _picker: &mut Picker,
    ) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.area());

            let search = TuiInputBox::new(
                self.search_widget.clone(),
                Block::bordered().title("Search"),
            );
            search.render(frame, &layout[0]);

            let mut list = TuiPokemonTable::new(self.list_widget.clone(), Block::bordered());
            list.render_mut(frame, &layout[1]);

            let widget =
                Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray);

            frame.render_widget(widget, layout[2]);
        })?;

        Ok(())
    }
}
