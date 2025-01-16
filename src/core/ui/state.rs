use color_eyre::Result;

use super::{
    pages::{DetailPage, ListPage, ListPagePokemon},
    repository::{DetailPagePokemonRepository, ListPagePokemonRepository},
    Event,
};
use crate::core::ui::components::PokemonTableEntry;

#[derive(Clone, Debug, PartialEq)]
pub enum PageState {
    List(ListPage),
    Detail(DetailPage),
}

pub fn next_state<R>(state: &PageState, event: &Event, repository: &R) -> Result<PageState>
where
    R: ListPagePokemonRepository + DetailPagePokemonRepository,
{
    let next_page = match state {
        PageState::List(page) => next_list(page, event, repository)?,
        PageState::Detail(page) => next_detail(page, event, repository)?,
    };

    Ok(next_page)
}

fn next_list(
    page: &ListPage,
    event: &Event,
    repository: &dyn DetailPagePokemonRepository,
) -> Result<PageState> {
    match event {
        Event::NewCharacter(c) => {
            let mut next_page = page.clone();
            next_page.push_char(*c);

            Ok(PageState::List(next_page))
        }
        Event::DeleteCharacter => {
            let mut next_page = page.clone();
            next_page.pop_char();

            Ok(PageState::List(next_page))
        }
        Event::Up => {
            let mut next_page = page.clone();
            next_page.list_widget.up();

            Ok(PageState::List(next_page))
        }
        Event::Down => {
            let mut next_page = page.clone();
            next_page.list_widget.down();

            Ok(PageState::List(next_page))
        }
        Event::Select => {
            let id = page.list_widget.get_selected().map_or(0, |p| p.number);
            let pokemon = repository.fetch(id).expect("Failed to create DetailPage");
            Ok(PageState::Detail(DetailPage::new(&pokemon)?))
        }
        Event::Noop => Ok(PageState::List(page.clone())),
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
            primary_type: pokemon.types.primary,
            secondary_type: pokemon.types.secondary,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        sync::LazyLock,
    };

    use cascade::cascade;
    use image::DynamicImage;

    use super::*;
    use crate::core::{
        pokemon::{PokemonAttributes, PokemonDescription, PokemonStats, PokemonTypes, Type},
        ui::pages::{DetailPagePokemon, ListPagePokemon},
    };

    static LIST_POKEMON: LazyLock<Vec<ListPagePokemon>> = LazyLock::new(|| {
        vec![
            ListPagePokemon::new(1, "".to_string(), PokemonTypes::new(Type::Normal, None)),
            ListPagePokemon::new(2, "".to_string(), PokemonTypes::new(Type::Normal, None)),
            ListPagePokemon::new(3, "".to_string(), PokemonTypes::new(Type::Normal, None)),
            ListPagePokemon::new(4, "".to_string(), PokemonTypes::new(Type::Normal, None)),
            ListPagePokemon::new(5, "".to_string(), PokemonTypes::new(Type::Normal, None)),
        ]
    });

    static DETAIL_POKEMON: LazyLock<DetailPagePokemon> = LazyLock::new(|| {
        DetailPagePokemon::new(
            1,
            "".to_string(),
            DynamicImage::default(),
            PokemonTypes::new(Type::Normal, None),
            PokemonDescription::new("".to_string()),
            PokemonAttributes::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
                vec!["".to_string()],
                HashSet::new(),
            ),
            PokemonStats::new(0, 0, 0, 0, 0, 0),
        )
    });

    struct TestRepository();

    impl ListPagePokemonRepository for TestRepository {
        fn fetch_all(&self) -> Result<Vec<ListPagePokemon>> {
            Ok(LIST_POKEMON.clone())
        }
    }

    impl DetailPagePokemonRepository for TestRepository {
        fn fetch(&self, _id: u32) -> Result<DetailPagePokemon> {
            Ok(DETAIL_POKEMON.clone())
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
        )
        .unwrap();

        let next_state = next_state(&previous_state, &Event::Up, &repository).unwrap();

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
