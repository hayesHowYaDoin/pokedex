use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Rect, Style},
    style::Stylize,
    widgets::{BarChart, Block, Paragraph},
    Frame,
};

use super::TuiComponent;
use ui_core::components::PokemonStatChart;

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

    area
}

pub struct TuiPokemonStatChart<'a> {
    stat_chart: PokemonStatChart,
    block: Block<'a>,
}

impl<'a> TuiPokemonStatChart<'a> {
    pub fn new(stat_chart: PokemonStatChart, block: Block<'a>) -> Self {
        Self { stat_chart, block }
    }
}

impl TuiComponent for TuiPokemonStatChart<'_> {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let data: Vec<(&str, u64)> = self
            .stat_chart
            .get_fields()
            .iter()
            .map(|field| (field.get_label(), field.get_value() as u64))
            .collect();

        let chart = BarChart::default()
            .bar_width(8)
            .bar_gap(2)
            .group_gap(3)
            .bar_style(Style::new().blue())
            .value_style(Style::new().white())
            .label_style(Style::new().white().italic())
            .data(&data)
            .max(self.stat_chart.get_max_value() as u64);

        let area = center(
            *layout,
            Constraint::Length(5 * 8 + 4 * 2),
            Constraint::Length(layout.height / 2),
        );

        let border = Paragraph::new("").block(self.block.clone());

        frame.render_widget(border, *layout);
        frame.render_widget(chart, area);
    }
}
