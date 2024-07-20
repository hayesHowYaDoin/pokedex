use std::ops::{Add, Sub};

use derive_more::{Add, AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Add, AddAssign, SubAssign)]
pub struct RowIndex(pub u32);

impl Add<u32> for RowIndex {
    type Output = RowIndex;

    fn add(self, rhs: u32) -> Self::Output {
        RowIndex(self.0 + rhs)
    }
}

impl Sub<u32> for RowIndex {
    type Output = RowIndex;

    fn sub(self, rhs: u32) -> Self::Output {
        RowIndex(self.0 - rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PokemonTable {
    selected_row: RowIndex,
    number_of_rows: u32,
}

impl PokemonTable {
    pub fn new(selected_row: RowIndex, number_of_rows: u32) -> Self {
        let selected_row = match selected_row {
            RowIndex(row) if row >= number_of_rows => RowIndex(number_of_rows - 1),
            _ => selected_row,
        };

        PokemonTable {selected_row, number_of_rows}
    }

    pub fn up(&mut self) -> &Self {
        self.selected_row -= RowIndex(1);
        self
    }

    pub fn down(&mut self) -> &Self {
        self.selected_row = self.selected_row + 1;
        self
    }

    pub fn selected_index(&self) -> RowIndex {
        self.selected_row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let table = PokemonTable::new(RowIndex(0), 5);
        assert_eq!(table.selected_index(), RowIndex(0));
    }

    #[test]
    fn test_new_invalid_selected_index() {
        let table = PokemonTable::new(RowIndex(5), 5);
        assert_eq!(table.selected_index(), RowIndex(4));
    }

    #[test]
    fn test_up() {
        let mut table = PokemonTable::new(RowIndex(0), 5);
        table.up();

        assert_eq!(table.selected_index(), RowIndex(0));
    }

    #[test]
    fn test_up_limit() {
        let mut table = PokemonTable::new(RowIndex(4), 5);
        table.up();

        assert_eq!(table.selected_index(), RowIndex(4));
    }

    #[test]
    fn test_down() {
        let mut table = PokemonTable::new(RowIndex(3), 5);
        table.down();
        
        assert_eq!(table.selected_index(), RowIndex(2));
    }

    #[test]
    fn test_down_limit() {
        let mut table = PokemonTable::new(RowIndex(0), 5);
        table.down();
        
        assert_eq!(table.selected_index(), RowIndex(0));
    }
}