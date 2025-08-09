use pokemon::Type;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypesBox {
    types: Vec<Type>,
}

impl TypesBox {
    pub fn new<T: IntoIterator<Item = Type>>(types: T) -> Self {
        TypesBox {
            types: Vec::<Type>::from_iter(types),
        }
    }

    pub fn types(&self) -> &Vec<Type> {
        &self.types
    }
}
