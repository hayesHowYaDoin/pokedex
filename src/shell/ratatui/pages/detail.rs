use std::{
    sync::LazyLock,
    default::Default,
};

use color_eyre::eyre::Result;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal}, 
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::core::ui::pages::DetailPage;
use crate::shell::ratatui::components::TuiComponent;
use super::TuiPage;

const DESCRIPTION_LAYOUT_IDX: usize = 0;
const STATS_LAYOUT_IDX: usize = 1;
const FOOTER_LAYOUT_IDX: usize = 2;

static LAYOUT: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Min(5),
            Constraint::Length(2),
        ]));

impl<B: Backend> TuiPage<B> for DetailPage {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            self.get_text_box().render(frame, &layout[DESCRIPTION_LAYOUT_IDX]);
            // self.get_max_chart().render(frame, layout[STATS_LAYOUT_IDX]);

            frame.render_widget(
                Paragraph::new("Press 'backspace' to return, 'q' to quit").fg(Color::DarkGray), 
                layout[FOOTER_LAYOUT_IDX]
            );
        })?;

        Ok(())
    }
}
    