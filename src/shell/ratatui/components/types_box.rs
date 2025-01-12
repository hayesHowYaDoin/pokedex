use ratatui::{
    Frame,
    prelude::Rect,
    style::{Color, Modifier, Style},
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
        let primary = self.types_box.types().primary;
        let secondary = self.types_box.types().secondary;

        let mut text = vec![
                Span::styled(primary.to_string(), Style::default()
                    .fg(type_color(&primary))
                    .add_modifier(Modifier::BOLD)
                ),
        ];
        if secondary.is_some() {
            text.push(" ".into());
            text.push(
                Span::styled(
                    secondary.map_or_else(|| "".to_string(), |t| t.to_string()), 
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                ),
            );
        }

        let widget: Paragraph = Paragraph::new(Line::from(text))
            .wrap(Wrap { trim: false })
            .block(self.block.clone());

        frame.render_widget(&widget, *layout);
    }
}
