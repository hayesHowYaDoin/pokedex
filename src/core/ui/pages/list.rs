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
        let list_widget = PokemonTable::new(RowIndex::new(0, number_of_rows));

        ListPage {search_widget, list_widget}
    }
}

impl Page for ListPage {
    fn update(&mut self, event: &Event) -> &mut dyn Page {
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

        self
    }
    
    fn next(&self) -> Box<dyn Page> {
        // TODO: Implement state transition logic
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_update_new_character() {
        let mut page = ListPage::new(10);
        let event = Event::NewCharacter('A');
        
        page.update(&event);
        
        assert_eq!(page.search_widget.text(), "A");
    }
    
    #[test]
    fn test_update_delete_character() {
        let mut page = ListPage::new(10);
        
        page.update(&Event::NewCharacter('A'));
        page.update(&Event::DeleteCharacter);
        
        assert_eq!(page.search_widget.text(), "");
    }
    
    #[test]
    fn test_update_up() {
        let mut page = ListPage::new(10);
        
        page.update(&Event::Down)
            .update(&Event::Down)
            .update(&Event::Up);
        
        assert_eq!(page.list_widget.selected_index().value(), 1);
    }
    
    #[test]
    fn test_update_down() {
        let mut page = ListPage::new(10);
        
        page.update(&Event::Down);
        
        assert_eq!(page.list_widget.selected_index().value(), 1);
    }
    
    #[test]
    fn test_update_noop() {
        let mut page = ListPage::new(10);
        let event = Event::Noop;
        
        let previous_text = page.search_widget.text().to_string();
        let previous_index = page.list_widget.selected_index().value();

        page.update(&event);
        
        assert_eq!(page.search_widget.text(), previous_text);
        assert_eq!(page.list_widget.selected_index().value(), previous_index);
    }
}
