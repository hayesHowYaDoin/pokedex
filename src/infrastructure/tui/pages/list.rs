use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal},
    widgets::{Block, Paragraph},
};
use std::io::Result;
use std::cell::LazyCell;

const SEARCH_WIDGET_LAYOUT_IDX: usize = 0;
const LIST_WIDGET_LAYOUT_IDX: usize = 1;

const LAYOUT: LazyCell<Layout> = 
    LazyCell::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
        ]));

pub struct ListPage<'a> {
    search_widget: Paragraph<'a>,
    list_widget: Paragraph<'a>,
}

impl ListPage<'_> {
    pub fn new() -> Self {
        let search_widget = Paragraph::new("Enter search query here!")
            .block(Block::bordered().title("Search"));
        let list_widget = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .block(Block::bordered());

        Self {search_widget, list_widget}
    }

    pub fn render<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<()> {
        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            frame.render_widget(&self.search_widget, layout[SEARCH_WIDGET_LAYOUT_IDX]);
            frame.render_widget(&self.list_widget, layout[LIST_WIDGET_LAYOUT_IDX]);
        })?;

        Ok(())
    }
}
