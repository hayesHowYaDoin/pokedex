use ratatui::{
    prelude::{Rect, Style},
    style::Stylize,
    Frame,
    widgets::{BarChart, Block},
};

use crate::core::ui::components::PokemonStatChart;
use super::TuiComponent;

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
        let data: Vec<(&str, u64)> = self.stat_chart.get_fields().iter().map(|field| {
            (field.get_label(), field.get_value() as u64)
        }).collect();
        
        let chart = BarChart::default()
            .block(self.block.clone())
            .bar_width(8)
            .bar_gap(2)
            .group_gap(3)
            .bar_style(Style::new().blue())
            .value_style(Style::new().white())
            .label_style(Style::new().white().italic())
            .data(&data)
            .max(self.stat_chart.get_max_value() as u64);
    
        frame.render_widget(chart, *layout);
    }
}