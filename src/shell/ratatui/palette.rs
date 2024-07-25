use colors_transform::{Rgb, Color as TransformColor};
use ratatui::style::Color as TuiColor;

use crate::core::pokemon::Type;

fn rgb_to_color(rgb: Rgb) -> TuiColor {
    TuiColor::Rgb(rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8)
}

pub fn type_color(t: &Type) -> TuiColor {
    match t {
        Type::Normal => rgb_to_color(Rgb::from_hex_str("#aab09f").unwrap()),
        Type::Fire => rgb_to_color(Rgb::from_hex_str("#ea7a3c").unwrap()),
        Type::Water => rgb_to_color(Rgb::from_hex_str("#539ae2").unwrap()),
        Type::Electric => rgb_to_color(Rgb::from_hex_str("#e5c531").unwrap()),
        Type::Grass => rgb_to_color(Rgb::from_hex_str("#71c558").unwrap()),
        Type::Ice => rgb_to_color(Rgb::from_hex_str("#70cbd4").unwrap()),
        Type::Fighting => rgb_to_color(Rgb::from_hex_str("#cb5f48").unwrap()),
        Type::Poison => rgb_to_color(Rgb::from_hex_str("#b468b7").unwrap()),
        Type::Ground => rgb_to_color(Rgb::from_hex_str("#cc9f4f").unwrap()),
        Type::Flying => rgb_to_color(Rgb::from_hex_str("#7da6de").unwrap()),
        Type::Psychic => rgb_to_color(Rgb::from_hex_str("#e5709b").unwrap()),
        Type::Bug => rgb_to_color(Rgb::from_hex_str("#94bc4a").unwrap()),
        Type::Rock => rgb_to_color(Rgb::from_hex_str("#b2a061").unwrap()),
        Type::Ghost => rgb_to_color(Rgb::from_hex_str("#846ab6").unwrap()),
        Type::Dragon => rgb_to_color(Rgb::from_hex_str("#6a7baf").unwrap()),
        Type::Dark => rgb_to_color(Rgb::from_hex_str("#736c75").unwrap()),
        Type::Steel => rgb_to_color(Rgb::from_hex_str("#89a1b0").unwrap()),
        Type::Fairy => rgb_to_color(Rgb::from_hex_str("#e397d1").unwrap()),
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
