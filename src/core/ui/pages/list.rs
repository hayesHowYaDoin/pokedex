use crate::core::{
    pokemon::Pokemon, 
    ui::components::{InputBox, PokemonTable},
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ListPage {
    pub search_widget: InputBox,
    pub list_widget: PokemonTable,
}

impl ListPage {
    pub fn new(pokemon: &[Pokemon]) -> Self {
        let search_widget = InputBox::default();
        let list_widget = PokemonTable::new(pokemon, 0);

        ListPage {search_widget, list_widget}
    }

    pub fn get_selected(&self) -> Pokemon {
        self.list_widget.get_selected()
    }
}

// #[cfg(test)]
// mod tests {
//     use cascade::cascade;

//     use super::*;
    
//     #[test]
//     fn test_update_new_character() {
//         let mut page = ListPage::new(10);
//         let event = Event::NewCharacter('A');
        
//         page.update(&event);
        
//         assert_eq!(page.search_widget.text(), "A");
//     }
    
//     #[test]
//     fn test_update_delete_character() {
//         let page = cascade! {
//             ListPage::new(10);
//             ..update(&Event::NewCharacter('A'));
//             ..update(&Event::DeleteCharacter);
//         };
        
//         assert_eq!(page.search_widget.text(), "");
//     }
    
//     #[test]
//     fn test_update_up() {
//         let page = cascade! {
//             ListPage::new(10);
//             ..update(&Event::Down);
//             ..update(&Event::Down);
//             ..update(&Event::Up);
//         };
        
//         assert_eq!(page.list_widget.get_selected_index().value(), 1);
//     }
    
//     #[test]
//     fn test_update_down() {
//         let page = cascade! {
//             ListPage::new(10);
//             ..update(&Event::Down);
//         };
        
//         assert_eq!(page.list_widget.get_selected_index().value(), 1);
//     }
    
//     #[test]
//     fn test_update_noop() {
//         let mut page = ListPage::new(10);
//         let event = Event::Noop;
        
//         let previous_text = page.search_widget.text().to_string();
//         let previous_index = page.list_widget.get_selected_index().value();

//         page.update(&event);
        
//         assert_eq!(page.search_widget.text(), previous_text);
//         assert_eq!(page.list_widget.get_selected_index().value(), previous_index);
//     }
// }
