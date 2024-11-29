use image::{GenericImageView, RgbaImage};
use crate::convert::texture::property::TextureProperty;

pub struct Texture {
    image: RgbaImage,
    flip: bool
}

impl TextureProperty for Texture {
    fn height(&self) -> u32 { self.image.height() }

    fn draw(&self, image: &mut RgbaImage, offset: u32) {
        if self.flip {
            let (width, height) = self.image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    unsafe {
                        image.put_pixel(x, height - y + offset, self.image.unsafe_get_pixel(x, y))
                    }
                }
            }
            return
        } else {
            let (width, height) = self.image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    unsafe {
                        image.put_pixel(x, y + offset, self.image.unsafe_get_pixel(x, y))
                    }
                }
            }
        }
    }
}