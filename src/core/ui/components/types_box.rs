use crate::core::pokemon::Type;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypesBox {
    types: HashSet<Type>,
}

impl TypesBox {
    pub fn new<T: IntoIterator<Item = Type>>(types: T) -> Self {
        TypesBox { types: HashSet::from_iter(types) }
    }

    pub fn types(&self) -> &HashSet<Type> {
        &self.types
    }
}
