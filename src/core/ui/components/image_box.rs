use image::DynamicImage;


#[derive(Clone, Debug, PartialEq)]
pub struct ImageBox {
    image: DynamicImage,
}

impl ImageBox {
    pub fn new(image: DynamicImage) -> Self {
        Self {image: image}
    }

    pub fn image(&self) -> &DynamicImage {
        &self.image
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Add tests
}
