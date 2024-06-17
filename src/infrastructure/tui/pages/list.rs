use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal},
    widgets::{Block, Paragraph},
};
use std::io::Result;
use std::cell::LazyCell;

use crate::infrastructure::tui::widgets::InputBox;

const SEARCH_WIDGET_LAYOUT_IDX: usize = 0;
const LIST_WIDGET_LAYOUT_IDX: usize = 1;

pub struct ListPage<'a> {
    search_widget: InputBox<'a>,
    list_widget: Paragraph<'a>,
}

impl ListPage<'_> {
    pub fn new() -> Self {
        let search_widget = InputBox::new(Block::bordered()
            .title("Search"));
        let list_widget = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .block(Block::bordered().title("All the Pokemon I et:"));

        Self {search_widget, list_widget}
    }

    pub fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        let layout: LazyCell<Layout> = LazyCell::new(|| Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]));

        terminal.draw(|frame| {
            let layout = layout.split(frame.size());

            frame.render_widget(&self.search_widget, layout[SEARCH_WIDGET_LAYOUT_IDX]);
            frame.render_widget(&self.list_widget, layout[LIST_WIDGET_LAYOUT_IDX]);
        })?;

        Ok(())
    }
}
