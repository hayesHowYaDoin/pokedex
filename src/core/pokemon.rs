use color_eyre::{Result, eyre};

use std::collections::HashSet;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

pub fn type_defense_weaknesses(type_: &Type) -> HashSet<Type> {
    match type_ {
        Type::Normal => HashSet::from([Type::Fighting]),
        Type::Fire => HashSet::from([Type::Water, Type::Rock, Type::Ground]),
        Type::Water => HashSet::from([Type::Electric, Type::Grass]),
        Type::Electric => HashSet::from([Type::Ground]),
        Type::Grass => HashSet::from([Type::Fire, Type::Ice, Type::Poison, Type::Flying, Type::Bug]),
        Type::Ice => HashSet::from([Type::Fire, Type::Fighting, Type::Rock, Type::Steel]),
        Type::Fighting => HashSet::from([Type::Flying, Type::Psychic, Type::Fairy]),
        Type::Poison => HashSet::from([Type::Ground, Type::Psychic]),
        Type::Ground => HashSet::from([Type::Water, Type::Grass, Type::Ice]),
        Type::Flying => HashSet::from([Type::Electric, Type::Ice, Type::Rock]),
        Type::Psychic => HashSet::from([Type::Bug, Type::Ghost, Type::Dark]),
        Type::Bug => HashSet::from([Type::Fire, Type::Flying, Type::Rock]),
        Type::Rock => HashSet::from([Type::Water, Type::Grass, Type::Fighting, Type::Ground, Type::Steel]),
        Type::Ghost => HashSet::from([Type::Ghost, Type::Dark]),
        Type::Dragon => HashSet::from([Type::Ice, Type::Dragon, Type::Fairy]),
        Type::Dark => HashSet::from([Type::Fighting, Type::Bug, Type::Fairy]),
        Type::Steel => HashSet::from([Type::Fire, Type::Fighting, Type::Ground]),
        Type::Fairy => HashSet::from([Type::Poison, Type::Steel]),
    }
}

pub fn type_defense_strengths(type_: &Type) -> HashSet<Type> {
    match type_ {
        Type::Normal => HashSet::from([]),
        Type::Fire => HashSet::from([Type::Bug, Type::Fire, Type::Grass, Type::Steel, Type::Ice, Type::Fairy]),
        Type::Water => HashSet::from([Type::Fire, Type::Water, Type::Ice, Type::Steel]),
        Type::Electric => HashSet::from([Type::Steel, Type::Electric, Type::Flying]),
        Type::Grass => HashSet::from([Type::Water, Type::Ground, Type::Grass, Type::Electric]),
        Type::Ice => HashSet::from([Type::Ice]),
        Type::Fighting => HashSet::from([Type::Rock, Type::Bug, Type::Dark]),
        Type::Poison => HashSet::from([Type::Fighting, Type::Poison, Type::Grass, Type::Bug, Type::Fairy]),
        Type::Ground => HashSet::from([Type::Poison, Type::Rock]),
        Type::Flying => HashSet::from([Type::Grass, Type::Fighting, Type::Bug]),
        Type::Psychic => HashSet::from([Type::Fighting, Type::Psychic]),
        Type::Bug => HashSet::from([Type::Grass, Type::Fighting, Type::Ground]),
        Type::Rock => HashSet::from([Type::Normal, Type::Fire, Type::Poison, Type::Flying]),
        Type::Ghost => HashSet::from([Type::Poison, Type::Bug]),
        Type::Dragon => HashSet::from([Type::Fire, Type::Water, Type::Electric, Type::Grass]),
        Type::Dark => HashSet::from([Type::Ghost, Type::Dark]),
        Type::Steel => HashSet::from([Type::Normal, Type::Grass, Type::Ice, Type::Flying, Type::Psychic, Type::Bug, Type::Rock, Type::Dragon, Type::Steel, Type::Fairy]),
        Type::Fairy => HashSet::from([Type::Fighting, Type::Bug, Type::Dark]),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PokemonGenderRates {
    pub male: f32,
    pub female: f32,
}

impl PokemonGenderRates {
    pub fn new(male: f32, female: f32) -> Result<Self> {
        if male + female != 1.0 {
            return Err(eyre::eyre!("Male and female rates must sum to 1.0"));
        }

        Ok(PokemonGenderRates{male, female})
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PokemonAttributes {
    pub height_m: String,
    pub weight_kg: String,
    pub category: String,
    pub abilities: Vec<String>,
    pub genders: Option<PokemonGenderRates>,
}

impl PokemonAttributes {
    pub fn new(
        height: String,
        weight: String,
        category: String,
        abilities: Vec<String>,
        genders: Option<PokemonGenderRates>,
    ) -> Self {
        PokemonAttributes {
            height_m: height,
            weight_kg: weight,
            category,
            abilities,
            genders,
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonImage {
    pub image: String,
}

impl PokemonImage {
    pub fn new(image: String) -> Self {
        PokemonImage {image}
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PokemonCry {
    pub cry: Vec<u8>,
}

impl PokemonCry {
    pub fn new(cry: Vec<u8>) -> Self {
        PokemonCry {cry}
    }
}
