use crate::core::ui::Event;

pub trait Page {
    fn update(&mut self, action: &Event) -> &mut dyn Page;
    fn next(&self) -> Box<dyn Page>;
}
