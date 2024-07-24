use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RowIndex {
    index: i32,
    number_of_rows: u32,
}

impl RowIndex {
    pub fn new(index: i32, number_of_rows: u32) -> Self {
        let index: i32 = match index {
            i if i >= number_of_rows as i32 => number_of_rows as i32 - 1,
            i if i < 1 => 0,
            _ => index,
        };

        RowIndex {index, number_of_rows}
    }

    pub fn value(&self) -> u32 {
        self.index as u32
    }
}

impl Add<u32> for RowIndex {
    type Output = RowIndex;

    fn add(self, rhs: u32) -> Self::Output {
        RowIndex::new(self.index + rhs as i32, self.number_of_rows)
    }
}

impl AddAssign<u32> for RowIndex {
    fn add_assign(&mut self, rhs: u32) {
        *self = *self + rhs;
    }
}

impl Sub<u32> for RowIndex {
    type Output = RowIndex;

    fn sub(self, rhs: u32) -> Self::Output {
        RowIndex::new(self.index - rhs as i32, self.number_of_rows)
    }
}

impl SubAssign<u32> for RowIndex {
    fn sub_assign(&mut self, rhs: u32) {
        *self = *self - rhs;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PokemonTableEntry {
    pub number: i32,
    pub name: String,
    pub type1: String,
    pub type2: Option<String>,
}

impl PokemonTableEntry {
    pub fn new(number: i32, name: String, type1: String, type2: Option<String>) -> Self {
        PokemonTableEntry {number, name, type1, type2}
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PokemonTable {
    pokemon: Vec<PokemonTableEntry>,
    selected_row: RowIndex,
}

impl PokemonTable {
    pub fn new(pokemon: &[PokemonTableEntry], selected_row: usize) -> Self {
        PokemonTable {
            pokemon: pokemon.to_vec(),
            selected_row: RowIndex::new(selected_row as i32, pokemon.len() as u32)
        }
    }

    pub fn up(&mut self) {
        self.selected_row -= 1;
    }

    pub fn down(&mut self) {
        self.selected_row += 1;
    }

    pub fn get_selected(&self) -> Option<&PokemonTableEntry> {
        if let Some(row) = self.get_selected_index() {
            return self.pokemon.get(row);
        }
        
        None
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        if self.pokemon.is_empty() {
            return None;
        }

        Some(self.selected_row.value() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cascade::cascade;

    use std::sync::LazyLock;

    
    static POKEMON: LazyLock<Vec<PokemonTableEntry>> = LazyLock::new(|| vec![
        PokemonTableEntry {number: 1, name: "Bulbasaur".to_string(), type1: "Grass".to_string(), type2: Some("Poison".to_string())},
        PokemonTableEntry {number: 2, name: "Ivysaur".to_string(), type1: "Grass".to_string(), type2: Some("Poison".to_string())},
        PokemonTableEntry {number: 3, name: "Venusaur".to_string(), type1: "Grass".to_string(), type2: Some("Poison".to_string())},
        PokemonTableEntry {number: 4, name: "Charmander".to_string(), type1: "Fire".to_string(), type2: None},
        PokemonTableEntry {number: 5, name: "Charmeleon".to_string(), type1: "Fire".to_string(), type2: None},
    ]);

    #[test]
    fn test_index_new_valid() {
        let table = PokemonTable::new(&POKEMON.as_slice(), POKEMON.len() - 1);
        assert_eq!(table.get_selected_index(), Some(POKEMON.len() - 1));

        
        let table = PokemonTable::new(&POKEMON.as_slice(), 0);
        assert_eq!(table.get_selected_index(), Some(0));
    }

    #[test]
    fn test_index_out_of_bounds() {
        let table = PokemonTable::new(&POKEMON.as_slice(), POKEMON.len());
        assert_eq!(table.get_selected_index(), Some(POKEMON.len() - 1));
    }

    #[test]
    fn test_up() {
        let table = cascade! {
            PokemonTable::new(&POKEMON.as_slice(), 1);
            ..up();
        };
        assert_eq!(table.get_selected_index(), Some(0));
    }

    #[test]
    fn test_up_limit() {
        let table = cascade! {
            PokemonTable::new(&POKEMON.as_slice(), 0);
            ..up();
        };
        assert_eq!(table.get_selected_index(), Some(0));
    }

    #[test]
    fn test_down() {
        let table = cascade! {
            PokemonTable::new(&POKEMON.as_slice(), POKEMON.len() - 2);
            ..down();
        };
        assert_eq!(table.get_selected_index(), Some(POKEMON.len() - 1));
    }

    #[test]
    fn test_down_limit() {
        let table = cascade! {
            PokemonTable::new(&POKEMON.as_slice(), POKEMON.len() - 1);
            ..down();
        };
        assert_eq!(table.get_selected_index(), Some(POKEMON.len() - 1));
    }

    #[test]
    fn test_get_selected() {
        let table = cascade! {
            PokemonTable::new(&POKEMON.as_slice(), 1);
            ..down();
        };
        assert_eq!(table.get_selected(), Some(&POKEMON[2]));
    }

    #[test]
    fn test_get_selected_empty() {
        let table = PokemonTable::new(&[], 1);
        assert_eq!(table.get_selected(), None);

        let down_table = cascade! { table.clone();..down(); };
        assert_eq!(down_table.get_selected(), None);

        let up_table = cascade! { table.clone();..up(); };
        assert_eq!(up_table.get_selected(), None);
    }
}