use std::{
    default::Default,
    sync::LazyLock,
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
    image_box::TuiImageBox,
    attributes_box::TuiAttributesBox,
    sound::SoundPlayer,
    stat_chart::TuiPokemonStatChart,
    text_box::TuiTextBox,
    types_box::TuiTypesBox,
};

static OUTERMOST_VERTICAL: LazyLock<Layout> =
    LazyLock::new(|| Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(30),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Length(1),
        ]));

static INNER_FIRST_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Min(30),
        ]));

static INNER_SECOND_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(30),
        ]));

static INNER_THIRD_HORIZONTAL: LazyLock<Layout> = 
    LazyLock::new(|| Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ]));

static INNER_RIGHT_VERTICAL: LazyLock<Layout> =
LazyLock::new(|| Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(10),
        Constraint::Length(30),
    ]));

#[derive(Debug)]
pub struct TuiDetailPage {
    pub page: DetailPage,
    pub sound_thread: SoundPlayer,
}

impl TuiDetailPage {
    pub fn new(page: DetailPage) -> Result<Self> {
        let sound_thread = SoundPlayer::new(page.get_launch_sound().clone())?;
        Ok(Self{ page, sound_thread })
    }

    pub fn set_page(&mut self, page: DetailPage) {
        self.page = page;
    }

    pub fn on_enter(&mut self) {
        self.sound_thread.play();
    }

    pub fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>, picker: &mut Picker) -> Result<()> {
        terminal.draw(|frame: &mut ratatui::Frame<'_>| {
            let outer_vertical_layout = OUTERMOST_VERTICAL.split(frame.area());
            let inner_first_horizontal_layout = INNER_FIRST_HORIZONTAL.split(outer_vertical_layout[1]);
            let inner_second_horizontal_layout = INNER_SECOND_HORIZONTAL.split(outer_vertical_layout[2]);
            let inner_third_horizontal_layout = INNER_THIRD_HORIZONTAL.split(outer_vertical_layout[3]);
            let inner_right_vertical_layout = INNER_RIGHT_VERTICAL.split(inner_first_horizontal_layout[1]);

            // Title
            TuiTextBox::new(
                self.page.get_title_box().to_owned(),
                Block::default()
            ).render(frame, &outer_vertical_layout[0]);

            // Image
            TuiImageBox::new(
                self.page.get_image_box(),
                picker,
            ).render_mut(frame, &inner_first_horizontal_layout[0]);

            // Description
            TuiTextBox::new(
                self.page.get_description_box().to_owned(),
                Block::bordered().title("Description"),
            ).render(frame, &inner_right_vertical_layout[0]);

            // Attributes
            TuiAttributesBox::new(
                self.page.get_attributes_box().to_owned(),
                Block::bordered()
            ).render(frame, &inner_right_vertical_layout[1]);

            // Stats
            TuiPokemonStatChart::new(
                self.page.get_stat_chart().to_owned(),
                Block::bordered().title("Stats")
            ).render(frame, &inner_second_horizontal_layout[0]);

            // Types
            TuiTypesBox::new(
                self.page.get_types_box().to_owned(),
                Block::bordered().title("Types")
            ).render(frame, &inner_third_horizontal_layout[0]);

            // Weaknesses
            TuiTypesBox::new(
                self.page.get_weaknesses_box().to_owned(),
                Block::bordered().title("Weaknesses")
            ).render(frame, &inner_third_horizontal_layout[1]);

            // Footer
            frame.render_widget(
                Paragraph::new("Press 'backspace' to return, 'q' to quit").fg(Color::DarkGray), 
                outer_vertical_layout[4]
            );
        })?;

        Ok(())
    }
}
