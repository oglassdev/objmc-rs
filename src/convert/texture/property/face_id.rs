use image::{Rgba, RgbaImage};
use crate::convert::texture::property::TextureProperty;

pub struct FaceIdProperty {
    pub length: u32,
    pub width: u32,
}

impl TextureProperty for FaceIdProperty {
    fn height(&self) -> u32 {
        let result = self.length / self.width;
        if self.length % self.width == 0 { result } else { result + 1 }
    }

    fn draw(&self, buf: &mut RgbaImage, offset: u32) {
        let width = buf.width();

        for i in 0..self.length {
            let x = i % width;
            let y = (i / width) + offset;

            buf.put_pixel(x, y, Rgba([
                ((x >> 8) & 0xFF) as u8,
                (x & 0xFF) as u8,
                ((y >> 8) & 0xFF) as u8,
                (y & 0xFF) as u8
            ]));
        }
    }
}