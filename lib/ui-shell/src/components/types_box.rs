use ratatui::{
    prelude::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::TuiComponent;
use crate::palette::{type_color_dark, type_color_light};
use ui_core::components::TypesBox;

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
        let text = self
            .types_box
            .types()
            .iter()
            .flat_map(|t| {
                vec![
                    Span::styled(
                        format!(" {t} "),
                        Style::default()
                            .fg(type_color_dark(t))
                            .bg(type_color_light(t)),
                    ),
                    Span::raw(" "),
                ]
            })
            .take(self.types_box.types().len() * 2 - 1)
            .collect::<Vec<Span>>();

        let widget: Paragraph = Paragraph::new(Line::from(text))
            .wrap(Wrap { trim: false })
            .block(self.block.clone())
            .alignment(ratatui::layout::Alignment::Center);

        frame.render_widget(&widget, *layout);
    }
}
