use color_eyre::Result;

use super::{
    pages::{DetailPage, DetailPagePokemon, ListPage, ListPagePokemon},
    repository::ListPagePokemonRepository,
    Event,
};
use crate::core::{pokemon::PokemonGenders, ui::components::PokemonTableEntry};

// TODO: Remove when no longer needed for testing
use crate::core::pokemon::{PokemonDescription, PokemonAttributes, PokemonStats, PokemonTypes, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum PageState {
    List(ListPage),
    Detail(DetailPage),
}

pub fn next_state(
    state: &PageState,
    event: &Event,
    repository: &dyn ListPagePokemonRepository,
) -> Result<PageState> {
    let next_page = match state {
        PageState::List(page) => next_list(page, event),
        PageState::Detail(page) => next_detail(page, event, repository)?,
    };

    Ok(next_page)
}

fn next_list(page: &ListPage, event: &Event) -> PageState {
    match event {
        Event::NewCharacter(c) => {
            let mut next_page = page.clone();
            next_page.push_char(*c);

            PageState::List(next_page)
        }
        Event::DeleteCharacter => {
            let mut next_page = page.clone();
            next_page.pop_char();

            PageState::List(next_page)
        }
        Event::Up => {
            let mut next_page = page.clone();
            next_page.list_widget.up();

            PageState::List(next_page)
        }
        Event::Down => {
            let mut next_page = page.clone();
            next_page.list_widget.down();

            PageState::List(next_page)
        }
        Event::Select => {
            // TODO: Replace hard-coded values with repository fetch
            let number = 1;
            let name = "Bulbasaur".to_string();
            let image = image::ImageReader::open("./test_assets/1.png")
                .expect("Unable to open image.")
                .decode()
                .unwrap()
                .resize(3000, 3000, image::imageops::FilterType::Nearest);
            let types = PokemonTypes::new(Type::Grass, Some(Type::Poison));
            let description = PokemonDescription::new(
                "A strange seed was planted on its back at birth. The plant sprouts and grows with this POKéMON.".to_string()
            );
            let attributes = PokemonAttributes::new(
                "2' 04\"".to_string(),
                "15.2 lbs".to_string(),
                "Seed".to_string(),
                vec!["Overgrow".to_string()],
                [PokemonGenders::Male, PokemonGenders::Female]
                    .into_iter()
                    .collect(),
            );
            let stats = PokemonStats::new(45, 49, 49, 65, 65, 45);

            let test_pokemon =
                DetailPagePokemon::new(number, name, image, types, description, attributes, stats);

            PageState::Detail(DetailPage::new(&test_pokemon).expect("Failed to create DetailPage"))
        }
        Event::Noop => PageState::List(page.clone()),
    }
}

fn next_detail(
    page: &DetailPage,
    event: &Event,
    repository: &dyn ListPagePokemonRepository,
) -> Result<PageState> {
    match event {
        Event::DeleteCharacter => {
            let pokemon = repository.fetch_all()?;
            Ok(PageState::List(ListPage::new(&pokemon, "")))
        }
        _ => Ok(PageState::Detail(page.clone())),
    }
}

impl From<&ListPagePokemon> for PokemonTableEntry {
    fn from(pokemon: &ListPagePokemon) -> Self {
        PokemonTableEntry {
            number: pokemon.number,
            name: pokemon.name.clone(),
            primary_type: pokemon.primary_type,
            secondary_type: pokemon.secondary_type,
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use cascade::cascade;

    use super::*;
    use crate::core::{pokemon::Type, ui::pages::ListPagePokemon};

    static LIST_POKEMON: LazyLock<Vec<ListPagePokemon>> = LazyLock::new(|| {
        vec![
            ListPagePokemon::new(1, "".to_string(), Type::Normal, None),
            ListPagePokemon::new(2, "".to_string(), Type::Normal, None),
            ListPagePokemon::new(3, "".to_string(), Type::Normal, None),
            ListPagePokemon::new(4, "".to_string(), Type::Normal, None),
            ListPagePokemon::new(5, "".to_string(), Type::Normal, None),
        ]
    });

    struct TestRepository();

    impl ListPagePokemonRepository for TestRepository {
        fn fetch_all(&self) -> Result<Vec<ListPagePokemon>> {
            Ok(LIST_POKEMON.clone())
        }
    }

    #[test]
    fn test_next_list_new_character_other() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");
        let mut list_page = ListPage::new(&pokemon, "");

        let next_list_page: PageState = next_state(
            &PageState::List(list_page.clone()),
            &Event::NewCharacter('a'),
            &repository,
        )
        .unwrap();

        list_page.search_widget.push_char('a');

        assert_eq!(next_list_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_delete_character() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");
        let mut list_page = ListPage::new(&pokemon, "a");

        let next_page = next_state(
            &PageState::List(list_page.clone()),
            &Event::DeleteCharacter,
            &repository,
        )
        .unwrap();

        list_page.search_widget.pop_char();

        assert_eq!(next_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_down() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");
        let mut list_page = ListPage::new(&pokemon, "");

        let next_page = next_state(
            &PageState::List(list_page.clone()),
            &Event::Down,
            &repository,
        )
        .unwrap();

        list_page.list_widget.down();

        assert_eq!(next_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_up() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");
        let mut list_page = ListPage::new(&pokemon, "");

        let mut next_page = next_state(
            &PageState::List(list_page.clone()),
            &Event::Down,
            &repository,
        )
        .unwrap();
        next_page = next_state(&next_page, &Event::Down, &repository).unwrap();
        next_page = next_state(&next_page, &Event::Up, &repository).unwrap();

        list_page.list_widget = cascade! {
            list_page.list_widget;
            ..down();
            ..down();
            ..up();
        };

        assert_eq!(next_page, PageState::List(list_page));
    }

    #[test]
    fn test_next_list_noop() {
        let repository = TestRepository();
        let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");

        let previous_state = next_state(
            &PageState::List(ListPage::new(&pokemon, "")),
            &Event::Up,
            &repository,
        ).unwrap();
        
        let next_state = next_state(
            &previous_state,
            &Event::Up,
            &repository,
        ).unwrap();

        assert_eq!(next_state, previous_state);
    }

    // TODO: Implement tests for detail page logic.
    // #[test]
    // fn test_next_list_select() {
        // let repository = TestRepository();
        // let pokemon = repository.fetch_all().expect("Failed to fetch all pokemon");
        // let list_page = ListPage::new(&pokemon, "");

        // let next_page = next_state(
        //     &PageState::List(list_page),
        //     &Event::Select,
        //     &repository
        // ).unwrap();

        // let expected_next_page = PageState::Detail(DetailPage::new(&DetailPagePokemon::new());

        // assert_eq!(next_page, list_page);
    // }
}
