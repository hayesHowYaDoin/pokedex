use std::{
    sync::LazyLock,
    default::Default,
};

use color_eyre::eyre::Result;
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Terminal}, 
    style::{Color, Stylize},
    widgets::{Block, Paragraph},
};

use crate::core::ui::pages::ListPage;
use crate::shell::ratatui::{
    components::{TuiComponent, TuiStatefulComponent},
    pages::TuiPage,
};

const SEARCH_LAYOUT_IDX: usize = 0;
const LIST_LAYOUT_IDX: usize = 1;
const FOOTER_LAYOUT_IDX: usize = 2;

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
        let search_block = Block::bordered().title("Search");
        let list_block = Block::bordered();

        terminal.draw(|frame| {
            let layout = LAYOUT.split(frame.size());

            self.search_widget.render(frame, &layout[SEARCH_LAYOUT_IDX], &search_block);
            self.list_widget.render_mut(frame, &layout[LIST_LAYOUT_IDX], &list_block);

            frame.render_widget(
                Paragraph::new("Press 'enter' for detailed view, 'q' to quit").fg(Color::DarkGray), 
                layout[FOOTER_LAYOUT_IDX]
            );
        })?;

        Ok(())
    }
}
