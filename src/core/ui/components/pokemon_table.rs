use std::ops::{Add, Sub, AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RowIndex {
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

#[derive(Clone, Copy, Debug)]
pub struct PokemonTable {
    selected_row: RowIndex,
}

impl PokemonTable {
    pub fn new(selected_row: RowIndex) -> Self {
        PokemonTable {selected_row}
    }

    pub fn up(mut self) -> Self {
        self.selected_row -= 1;
        self
    }

    pub fn down(mut self) -> Self {
        self.selected_row += 1;
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
    fn test_index_new_valid() {
        let index = RowIndex::new(0, 5);
        assert_eq!(index.value(), 0);
    }

    #[test]
    fn test_index_new_too_large() {
        let index = RowIndex::new(5, 5);
        assert_eq!(index.value(), 4);
    }

    #[test]
    fn test_up() {
        let table = PokemonTable::new(RowIndex::new(1, 5)).up();
        assert_eq!(table.selected_index().value(), 0);
    }

    #[test]
    fn test_up_limit() {
        let table = PokemonTable::new(RowIndex::new(0, 5)).up();
        assert_eq!(table.selected_index().value(), 0);
    }

    #[test]
    fn test_down() {
        let table = PokemonTable::new(RowIndex::new(1, 5)).down();
        assert_eq!(table.selected_index().value(), 2);
    }

    #[test]
    fn test_down_limit() {
        let table = PokemonTable::new(RowIndex::new(4, 5)).down();
        assert_eq!(table.selected_index().value(), 4);
    }
}