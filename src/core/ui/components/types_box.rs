use crate::core::pokemon::PokemonTypes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypesBox {
    types: PokemonTypes,
}

impl TypesBox {
    pub fn new(types: &PokemonTypes) -> Self {
        TypesBox{ types: types.clone() }
    }

    pub fn types(&self) -> &PokemonTypes {
        &self.types
    }
}
