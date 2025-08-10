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
        attributes_box::TuiAttributesBox, image_box::TuiImageBox, sound::play_sound,
        stat_chart::TuiPokemonStatChart, text_box::TuiTextBox, types_box::TuiTypesBox,
        TuiComponent, TuiStatefulComponent,
    },
    tui::Terminal,
};
use ui_core::pages::DetailPage;

static OUTERMOST_VERTICAL: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(30),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
});

static INNER_FIRST_HORIZONTAL: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(30)])
});

static INNER_SECOND_HORIZONTAL: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30)])
});

static INNER_THIRD_HORIZONTAL: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
});

static INNER_RIGHT_VERTICAL: LazyLock<Layout> = LazyLock::new(|| {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Length(30)])
});

impl TuiPage for DetailPage {
    fn on_enter(&mut self) {
        log::trace!(
            "Entering DetailPage for Pokémon: {}",
            self.get_title_box().text()
        );

        let sound = self.get_launch_sound().clone();
        tokio::spawn(async move {
            if let Err(err) = play_sound(sound) {
                log::warn!("Failed to play sound effect: {}", err);
            }
        });
    }

    fn on_exit(&mut self) {
        log::trace!(
            "Exiting DetailPage for Pokémon: {}",
            self.get_title_box().text()
        );
    }

    fn render(&mut self, terminal: &mut Terminal, picker: &mut Picker) -> Result<()> {
        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            let outer_vertical_layout = OUTERMOST_VERTICAL.split(frame.area());
            let inner_first_horizontal_layout =
                INNER_FIRST_HORIZONTAL.split(outer_vertical_layout[1]);
            let inner_second_horizontal_layout =
                INNER_SECOND_HORIZONTAL.split(outer_vertical_layout[2]);
            let inner_third_horizontal_layout =
                INNER_THIRD_HORIZONTAL.split(outer_vertical_layout[3]);
            let inner_right_vertical_layout =
                INNER_RIGHT_VERTICAL.split(inner_first_horizontal_layout[1]);

            // Title
            TuiTextBox::new(self.get_title_box().to_owned(), Block::default())
                .render(frame, &outer_vertical_layout[0]);

            // Image
            TuiImageBox::new(self.get_image_box(), picker)
                .render_mut(frame, &inner_first_horizontal_layout[0]);

            // Description
            TuiTextBox::new(
                self.get_description_box().to_owned(),
                Block::bordered().title("Description"),
            )
            .render(frame, &inner_right_vertical_layout[0]);

            // Attributes
            TuiAttributesBox::new(self.get_attributes_box().to_owned(), Block::bordered())
                .render(frame, &inner_right_vertical_layout[1]);

            // Stats
            TuiPokemonStatChart::new(
                self.get_stat_chart().to_owned(),
                Block::bordered().title("Stats"),
            )
            .render(frame, &inner_second_horizontal_layout[0]);

            // Types
            TuiTypesBox::new(
                self.get_types_box().to_owned(),
                Block::bordered().title("Types"),
            )
            .render(frame, &inner_third_horizontal_layout[0]);

            // Weaknesses
            TuiTypesBox::new(
                self.get_weaknesses_box().to_owned(),
                Block::bordered().title("Weaknesses"),
            )
            .render(frame, &inner_third_horizontal_layout[1]);

            // Footer
            frame.render_widget(
                Paragraph::new("Press 'backspace' to return, 'esc' to quit").fg(Color::DarkGray),
                outer_vertical_layout[4],
            );
        })?;

        Ok(())
    }
}
