use ratatui::{
    Frame,
    prelude::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap}
};

use crate::core::ui::components::TypesBox;
use crate::shell::ratatui::palette::type_color;

use super::TuiComponent;

pub struct TuiTypesBox<'a> {
    types_box: TypesBox,
    block: Block<'a>,
}

impl<'a> TuiTypesBox<'a> {
    pub fn new(types_box: TypesBox, block: Block<'a>) -> Self {
        Self { types_box, block }
    }
}

impl TuiComponent for TuiTypesBox<'_> {
    fn render(&self, frame: &mut Frame, layout: &Rect) {
        let text = self.types_box.types()
            .iter()
            .map(|t| Span::styled(
                t.to_string(), 
                Style::default().fg(type_color(t))
            ))
            .collect::<Vec<Span>>();

        let widget: Paragraph = Paragraph::new(Line::from(text))
            .wrap(Wrap { trim: false })
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
