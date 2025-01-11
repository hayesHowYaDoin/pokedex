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
use ratatui_image::picker::Picker;

use crate::core::ui::pages::DetailPage;
use crate::shell::ratatui::components::{
    TuiComponent,
    TuiStatefulComponent,
    text_box::TuiTextBox,
    stat_chart::TuiPokemonStatChart,
    image_box::TuiImageBox,
};
use super::TuiPage;

static OUTERMOST_VERTICAL: LazyLock<Layout> =
    LazyLock::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(30),
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(1),
        ]));

static INNER_FIRST_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Length(30),
        ]));

static INNER_SECOND_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(60),
        ]));

static INNER_THIRD_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(30),
                Constraint::Length(30),
            ]));

static INNER_RIGHT_VERTICAL: LazyLock<Layout> =
LazyLock::new(|| Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(15),
        Constraint::Length(15),
    ]));

impl<B: Backend> TuiPage<B> for DetailPage {
    fn render(&mut self, terminal: &mut Terminal<B>, picker: &mut Picker) -> Result<()> {
        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            let outer_vertical_layout = OUTERMOST_VERTICAL.split(frame.area());
            let inner_first_horizontal_layout = INNER_FIRST_HORIZONTAL.split(outer_vertical_layout[1]);
            let inner_second_horizontal_layout = INNER_SECOND_HORIZONTAL.split(outer_vertical_layout[2]);
            let _inner_third_horizontal_layout = INNER_THIRD_HORIZONTAL.split(outer_vertical_layout[3]);
            let inner_right_vertical_layout = INNER_RIGHT_VERTICAL.split(inner_first_horizontal_layout[1]);

            // Title
            TuiTextBox::new(
                self.get_title_box().to_owned(),
                Block::default()
            ).render(frame, &outer_vertical_layout[0]);

            // Image
            TuiImageBox::new(
                self.get_image_box().to_owned(),
                picker,
            ).render_mut(frame, &inner_first_horizontal_layout[0]);

            // Description
            TuiTextBox::new(
                self.get_description().to_owned(),
                Block::bordered().title("Description"),
            ).render(frame, &inner_right_vertical_layout[0]);

            // Other
            TuiTextBox::new(
                self.get_other().to_owned(),
                Block::bordered().title("Other")
            ).render(frame, &inner_right_vertical_layout[1]);

            // Stats
            TuiPokemonStatChart::new(
                self.get_stat_chart().to_owned(),
                Block::bordered().title("Stats")
            ).render(frame, &inner_second_horizontal_layout[0]);

            // Footer
            frame.render_widget(
                Paragraph::new("Press 'backspace' to return, 'q' to quit").fg(Color::DarkGray), 
                outer_vertical_layout[4]
            );
        })?;

        Ok(())
    }
}
    