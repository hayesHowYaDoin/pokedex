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
        ListPage {
            search_widget: InputBox::default(),
            list_widget: PokemonTable::new(pokemon, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new() {
        let list_page = ListPage::new(&[]);
        assert_eq!(list_page.search_widget.text(), "");
        assert_eq!(list_page.list_widget.get_selected(), None);
    }
}
