#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pokemon {
    pub number: i32,
    pub name: String,
}

impl Pokemon {
    pub fn new(number: i32, name: String) -> Self {
        Pokemon {number, name}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Type::Normal => "Normal",
            Type::Fire => "Fire",
            Type::Water => "Water",
            Type::Electric => "Electric",
            Type::Grass => "Grass",
            Type::Ice => "Ice",
            Type::Fighting => "Fighting",
            Type::Poison => "Poison",
            Type::Ground => "Ground",
            Type::Flying => "Flying",
            Type::Psychic => "Psychic",
            Type::Bug => "Bug",
            Type::Rock => "Rock",
            Type::Ghost => "Ghost",
            Type::Dragon => "Dragon",
            Type::Dark => "Dark",
            Type::Steel => "Steel",
            Type::Fairy => "Fairy",
        };
        write!(f, "{}", s)
    }

}

impl From<String> for Type {
    fn from(s: String) -> Self {
        match s {
            s if s == Type::Normal.to_string() => Type::Normal,
            s if s == Type::Fire.to_string() => Type::Fire,
            s if s == Type::Water.to_string() => Type::Water,
            s if s == Type::Electric.to_string() => Type::Electric,
            s if s == Type::Grass.to_string() => Type::Grass,
            s if s == Type::Ice.to_string() => Type::Ice,
            s if s == Type::Fighting.to_string() => Type::Fighting,
            s if s == Type::Poison.to_string() => Type::Poison,
            s if s == Type::Ground.to_string() => Type::Ground,
            s if s == Type::Flying.to_string() => Type::Flying,
            s if s == Type::Psychic.to_string() => Type::Psychic,
            s if s == Type::Bug.to_string() => Type::Bug,
            s if s == Type::Rock.to_string() => Type::Rock,
            s if s == Type::Ghost.to_string() => Type::Ghost,
            s if s == Type::Dragon.to_string() => Type::Dragon,
            s if s == Type::Dark.to_string() => Type::Dark,
            s if s == Type::Steel.to_string() => Type::Steel,
            s if s == Type::Fairy.to_string() => Type::Fairy,
            _ => Type::Normal,
        
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonTypes {
    pub primary: Type,
    pub secondary: Option<Type>,
}

impl PokemonTypes {
    pub fn new(primary: Type, secondary: Option<Type>) -> Self {
        PokemonTypes {
            primary,
            secondary,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonDescription {
    pub text: String,
}

impl PokemonDescription {
    pub fn new(text: String) -> Self {
        PokemonDescription {text}
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub special_attack: u32,
    pub special_defense: u32,
    pub speed: u32,
}

impl PokemonStats {
    pub fn new(hp: u32, attack: u32, defense: u32, special_attack: u32, special_defense: u32, speed: u32) -> Self {
        PokemonStats {
            hp,
            attack,
            defense,
            special_attack,
            special_defense,
            speed,
        }
    }
}
