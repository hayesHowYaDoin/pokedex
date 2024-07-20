use crate::core::ui::{
    components::{InputBox, PokemonTable, RowIndex},
    Event,
};
use super::Page;

#[derive(Clone, Debug)]
pub struct ListPage {
    pub search_widget: InputBox,
    pub list_widget: PokemonTable,
}

impl ListPage {
    pub fn new(number_of_rows: u32) -> Self {
        // TODO: Pass in collection of all Pokemon
        let search_widget = InputBox::default();
        let list_widget = PokemonTable::new(RowIndex(0), number_of_rows);

        ListPage {search_widget, list_widget}
    }
}

impl Page for ListPage {
    fn update(&mut self, event: &Event) {
        match event {
            Event::NewCharacter(c) => {
                self.search_widget.push_char(*c);
            }
            Event::DeleteCharacter => {
                self.search_widget.pop_char();
            }
            Event::Up => {
                self.list_widget.up();
            }
            Event::Down => {
                self.list_widget.down();
            }
            Event::Noop => {},
        }
    }
    
    fn next(&self) -> Box<dyn Page> {
        // TODO: Implement state transition logic
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    
}
