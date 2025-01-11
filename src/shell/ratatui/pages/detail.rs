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

use crate::core::ui::pages::DetailPage;
use crate::shell::ratatui::components::TuiComponent;
use super::TuiPage;

static OUTERMOST_VERTICAL: LazyLock<Layout> =
    LazyLock::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(30),
            Constraint::Length(5),
            Constraint::Length(2),
        ]));

static INNER_FIRST_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(65),
            Constraint::Percentage(35),
        ]));

static INNER_SECOND_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]));

static INNER_RIGHT_VERTICAL: LazyLock<Layout> =
LazyLock::new(|| Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(10),
        Constraint::Min(10),
    ]));

impl<B: Backend> TuiPage<B> for DetailPage {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let title_block = Block::default();
        let description_block = Block::bordered().title("Description");
        let stats_block = Block::bordered().title("Stats");

        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            let outer_vertical_layout = OUTERMOST_VERTICAL.split(frame.area());
            let inner_first_horizontal_layout = INNER_FIRST_HORIZONTAL.split(outer_vertical_layout[1]);
            let inner_second_horizontal_layout = INNER_SECOND_HORIZONTAL.split(outer_vertical_layout[2]);
            let inner_right_vertical_layout = INNER_RIGHT_VERTICAL.split(inner_first_horizontal_layout[1]);

            self.get_title_box().render(frame, &outer_vertical_layout[0], &title_block);

            self.get_text_box().render(frame, &inner_right_vertical_layout[0], &description_block);
            self.get_stat_chart().render(frame, &inner_second_horizontal_layout[0], &stats_block);

            self.get_image_box().render(frame, &inner_first_horizontal_layout[0], &Block::default());

            frame.render_widget(
                Paragraph::new("Press 'backspace' to return, 'q' to quit").fg(Color::DarkGray), 
                outer_vertical_layout[3]
            );
        })?;

        Ok(())
    }
}
    