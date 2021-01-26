use super::Texture;
use crate::vec::{Point3, Vec3};

// For now the image only support RGB
#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,

    data: Vec<u8>,
}

impl Image {
    #[inline]
    #[must_use]
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        assert!(data.len() == width * height * 3);

        Self {
            width,
            height,
            data,
        }
    }

    #[must_use]
    pub fn load(filename: &str) -> Self {
        // TODO: Handle error
        let image_buffer = image::open(filename).unwrap().into_rgb8();
        let width = image_buffer.width();
        let height = image_buffer.height();

        Self::new(width as usize, height as usize, image_buffer.into_raw())
    }
}

impl Texture for Image {
    fn value(&self, texture_coordinates: (f32, f32), _p: &Point3) -> Vec3 {
        debug_assert!(0. <= texture_coordinates.0 && texture_coordinates.0 <= 1.);
        debug_assert!(0. <= texture_coordinates.1 && texture_coordinates.1 <= 1.);

        let mut i = (texture_coordinates.0 * self.width as f32) as usize;
        let mut j = (texture_coordinates.1 * self.height as f32) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        j = self.height - 1 - j;

        let color_scale = 1. / 255.;
        let pixel = i * 3 + j * 3 * self.width;
        Vec3::new(
            color_scale * self.data[pixel] as f32,
            color_scale * self.data[pixel + 1] as f32,
            color_scale * self.data[pixel + 2] as f32,
        )
    }
}
