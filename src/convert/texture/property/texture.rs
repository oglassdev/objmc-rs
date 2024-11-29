use image::{GenericImageView, RgbaImage};
use crate::convert::texture::property::TextureProperty;

pub struct Texture<'a> {
    pub image: &'a RgbaImage,
    pub flip: bool
}

impl TextureProperty for Texture<'_> {
    fn height(&self) -> u32 { self.image.height() }

    fn draw(&self, image: &mut RgbaImage, offset: u32) {
        let (width, height) = self.image.dimensions();
        
        if self.flip {
            for x in 0..width {
                for y in 0..height {
                    unsafe {
                        image.put_pixel(x, y + offset, self.image.unsafe_get_pixel(x, y))
                    }
                }
            }
        } else {
            for x in 0..width {
                for y in 0..height {
                    unsafe {
                        image.put_pixel(x, height - y + offset - 1, self.image.unsafe_get_pixel(x, y))
                    }
                }
            }
        }
    }
}