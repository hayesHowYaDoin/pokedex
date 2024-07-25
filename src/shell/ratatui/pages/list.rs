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

use crate::core::ui::pages::ListPage;
use crate::shell::ratatui::components::{TuiComponent, TuiStatefulComponent};
use super::TuiPage;

const SEARCH_WIDGET_LAYOUT_IDX: usize = 0;
const LIST_WIDGET_LAYOUT_IDX: usize = 1;
const FOOTER_WIDGET_LAYOUT_IDX: usize = 2;

static LAYOUT: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ]));

impl<B: Backend> TuiPage<B> for ListPage {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            self.search_widget.render(frame, &layout[SEARCH_WIDGET_LAYOUT_IDX]);
            self.list_widget.render_mut(frame, &layout[LIST_WIDGET_LAYOUT_IDX]);

            frame.render_widget(
                Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray), 
                layout[FOOTER_WIDGET_LAYOUT_IDX]
            );
        })?;

        Ok(())
    }
}
