use std::default::Default;
use image::DynamicImage;


#[derive(Clone, Debug, PartialEq)]
pub struct ImageBox {
    image: DynamicImage,
}

impl ImageBox {
    pub fn new() -> Self {
        Self {image: image::ImageReader::open("./test_images/Pokémon_Bulbasaur_art.png")
        .expect("Unable to open image.").decode().unwrap()}
    }

    pub fn image(&self) -> &DynamicImage {
        &self.image
    }
}

impl Default for ImageBox {
    fn default() -> Self {
        Self::new()
    }

}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Add tests
}
