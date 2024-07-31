use ratatui::{
    prelude::{Rect, Style},
    style::Stylize,
    terminal::Frame,
    widgets::{BarChart, Block},
};

use crate::core::ui::components::PokemonStatChart;
use super::TuiComponent;

impl TuiComponent for PokemonStatChart {
    fn render(&self, frame: &mut Frame, layout: &Rect, block: &Block) {
        let data: Vec<(&str, u64)> = self.get_fields().iter().map(|field| {
            (field.get_label(), field.get_value() as u64)
        }).collect();
        
        let chart = BarChart::default()
            .block(block.clone())
            .bar_width(8)
            .bar_gap(2)
            .group_gap(3)
            .bar_style(Style::new().blue())
            .value_style(Style::new().white())
            .label_style(Style::new().white().italic())
            .data(&data)
            .max(self.get_max_value() as u64);
    
        frame.render_widget(chart, *layout);
    }
}