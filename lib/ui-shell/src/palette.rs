use colors_transform::{Color as TransformColor, Rgb};
use ratatui::style::Color as TuiColor;

use pokemon::Type;

fn rgb_to_color(rgb: Rgb) -> TuiColor {
    TuiColor::Rgb(
        rgb.get_red() as u8,
        rgb.get_green() as u8,
        rgb.get_blue() as u8,
    )
}

fn type_color(t: &Type) -> Rgb {
    match t {
        Type::Normal => Rgb::from_hex_str("#aab09f").unwrap(),
        Type::Fire => Rgb::from_hex_str("#ea7a3c").unwrap(),
        Type::Water => Rgb::from_hex_str("#539ae2").unwrap(),
        Type::Electric => Rgb::from_hex_str("#e5c531").unwrap(),
        Type::Grass => Rgb::from_hex_str("#71c558").unwrap(),
        Type::Ice => Rgb::from_hex_str("#70cbd4").unwrap(),
        Type::Fighting => Rgb::from_hex_str("#cb5f48").unwrap(),
        Type::Poison => Rgb::from_hex_str("#b468b7").unwrap(),
        Type::Ground => Rgb::from_hex_str("#cc9f4f").unwrap(),
        Type::Flying => Rgb::from_hex_str("#7da6de").unwrap(),
        Type::Psychic => Rgb::from_hex_str("#e5709b").unwrap(),
        Type::Bug => Rgb::from_hex_str("#94bc4a").unwrap(),
        Type::Rock => Rgb::from_hex_str("#b2a061").unwrap(),
        Type::Ghost => Rgb::from_hex_str("#846ab6").unwrap(),
        Type::Dragon => Rgb::from_hex_str("#6a7baf").unwrap(),
        Type::Dark => Rgb::from_hex_str("#736c75").unwrap(),
        Type::Steel => Rgb::from_hex_str("#89a1b0").unwrap(),
        Type::Fairy => Rgb::from_hex_str("#e397d1").unwrap(),
    }
}

pub fn type_color_light(t: &Type) -> TuiColor {
    let default = 20.0;

    match t {
        Type::Normal => rgb_to_color(type_color(&Type::Normal).lighten(default)),
        Type::Fire => rgb_to_color(type_color(&Type::Fire).lighten(default)),
        Type::Water => rgb_to_color(type_color(&Type::Water).lighten(default)),
        Type::Electric => rgb_to_color(type_color(&Type::Electric).lighten(default)),
        Type::Grass => rgb_to_color(type_color(&Type::Grass).lighten(default)),
        Type::Ice => rgb_to_color(type_color(&Type::Ice).lighten(default)),
        Type::Fighting => rgb_to_color(type_color(&Type::Fighting).lighten(default)),
        Type::Poison => rgb_to_color(type_color(&Type::Poison).lighten(default)),
        Type::Ground => rgb_to_color(type_color(&Type::Ground).lighten(default)),
        Type::Flying => rgb_to_color(type_color(&Type::Flying).lighten(default)),
        Type::Psychic => rgb_to_color(type_color(&Type::Psychic).lighten(default)),
        Type::Bug => rgb_to_color(type_color(&Type::Bug).lighten(default)),
        Type::Rock => rgb_to_color(type_color(&Type::Rock).lighten(default)),
        Type::Ghost => rgb_to_color(type_color(&Type::Ghost).lighten(default)),
        Type::Dragon => rgb_to_color(type_color(&Type::Dragon).lighten(default)),
        Type::Dark => rgb_to_color(type_color(&Type::Dark).lighten(default)),
        Type::Steel => rgb_to_color(type_color(&Type::Steel).lighten(default)),
        Type::Fairy => rgb_to_color(type_color(&Type::Fairy).lighten(default)),
    }
}

pub fn type_color_medium(t: &Type) -> TuiColor {
    match t {
        Type::Normal => rgb_to_color(type_color(&Type::Normal)),
        Type::Fire => rgb_to_color(type_color(&Type::Fire)),
        Type::Water => rgb_to_color(type_color(&Type::Water)),
        Type::Electric => rgb_to_color(type_color(&Type::Electric)),
        Type::Grass => rgb_to_color(type_color(&Type::Grass)),
        Type::Ice => rgb_to_color(type_color(&Type::Ice)),
        Type::Fighting => rgb_to_color(type_color(&Type::Fighting)),
        Type::Poison => rgb_to_color(type_color(&Type::Poison)),
        Type::Ground => rgb_to_color(type_color(&Type::Ground)),
        Type::Flying => rgb_to_color(type_color(&Type::Flying)),
        Type::Psychic => rgb_to_color(type_color(&Type::Psychic)),
        Type::Bug => rgb_to_color(type_color(&Type::Bug)),
        Type::Rock => rgb_to_color(type_color(&Type::Rock)),
        Type::Ghost => rgb_to_color(type_color(&Type::Ghost)),
        Type::Dragon => rgb_to_color(type_color(&Type::Dragon)),
        Type::Dark => rgb_to_color(type_color(&Type::Dark)),
        Type::Steel => rgb_to_color(type_color(&Type::Steel)),
        Type::Fairy => rgb_to_color(type_color(&Type::Fairy)),
    }
}

pub fn type_color_dark(t: &Type) -> TuiColor {
    let default = -40.0;

    match t {
        Type::Normal => rgb_to_color(type_color(&Type::Normal).lighten(default)),
        Type::Fire => rgb_to_color(type_color(&Type::Fire).lighten(default)),
        Type::Water => rgb_to_color(type_color(&Type::Water).lighten(default)),
        Type::Electric => rgb_to_color(type_color(&Type::Electric).lighten(default)),
        Type::Grass => rgb_to_color(type_color(&Type::Grass).lighten(default)),
        Type::Ice => rgb_to_color(type_color(&Type::Ice).lighten(default)),
        Type::Fighting => rgb_to_color(type_color(&Type::Fighting).lighten(default)),
        Type::Poison => rgb_to_color(type_color(&Type::Poison).lighten(default)),
        Type::Ground => rgb_to_color(type_color(&Type::Ground).lighten(default)),
        Type::Flying => rgb_to_color(type_color(&Type::Flying).lighten(default)),
        Type::Psychic => rgb_to_color(type_color(&Type::Psychic).lighten(default)),
        Type::Bug => rgb_to_color(type_color(&Type::Bug).lighten(default)),
        Type::Rock => rgb_to_color(type_color(&Type::Rock).lighten(default)),
        Type::Ghost => rgb_to_color(type_color(&Type::Ghost).lighten(default)),
        Type::Dragon => rgb_to_color(type_color(&Type::Dragon).lighten(default)),
        Type::Dark => rgb_to_color(type_color(&Type::Dark).lighten(default)),
        Type::Steel => rgb_to_color(type_color(&Type::Steel).lighten(default)),
        Type::Fairy => rgb_to_color(type_color(&Type::Fairy).lighten(default)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_color_normal() {
        let _ = type_color(&Type::Normal);
    }

    #[test]
    fn test_type_color_fire() {
        let _ = type_color(&Type::Fire);
    }

    #[test]
    fn test_type_color_water() {
        let _ = type_color(&Type::Water);
    }

    #[test]
    fn test_type_color_electric() {
        let _ = type_color(&Type::Electric);
    }

    #[test]
    fn test_type_color_grass() {
        let _ = type_color(&Type::Grass);
    }

    #[test]
    fn test_type_color_ice() {
        let _ = type_color(&Type::Ice);
    }

    #[test]
    fn test_type_color_fighting() {
        let _ = type_color(&Type::Fighting);
    }

    #[test]
    fn test_type_color_poison() {
        let _ = type_color(&Type::Poison);
    }

    #[test]
    fn test_type_color_ground() {
        let _ = type_color(&Type::Ground);
    }

    #[test]
    fn test_type_color_flying() {
        let _ = type_color(&Type::Flying);
    }

    #[test]
    fn test_type_color_psychic() {
        let _ = type_color(&Type::Psychic);
    }

    #[test]
    fn test_type_color_bug() {
        let _ = type_color(&Type::Bug);
    }

    #[test]
    fn test_type_color_rock() {
        let _ = type_color(&Type::Rock);
    }

    #[test]
    fn test_type_color_ghost() {
        let _ = type_color(&Type::Ghost);
    }

    #[test]
    fn test_type_color_dragon() {
        let _ = type_color(&Type::Dragon);
    }

    #[test]
    fn test_type_color_dark() {
        let _ = type_color(&Type::Dark);
    }

    #[test]
    fn test_type_color_steel() {
        let _ = type_color(&Type::Steel);
    }

    #[test]
    fn test_type_color_fairy() {
        let _ = type_color(&Type::Fairy);
    }
}
