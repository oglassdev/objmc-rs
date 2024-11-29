use image::{GenericImage, Rgba, RgbaImage};
use crate::convert::texture::property::TextureProperty;
use crate::obj::Normal;

pub struct UVData<'a> {
    pub uvs: &'a Vec<Normal>,
    pub width: u32
}

impl TextureProperty for UVData<'_> {
    fn height(&self) -> u32 {
        ((self.uvs.len() * 2) as f64 / self.width as f64).ceil() as u32
    }

    fn draw(&self, image: &mut RgbaImage, offset: u32) {
        let width = self.width;

        for (idx, normal) in self.uvs.iter().enumerate() {
            let idx = idx as u32 * 2;

            let [x, y] = normal_to_gb(normal);

            let pos_x = idx % width;
            let pos_y = idx / width + offset;
            image.put_pixel(pos_x, pos_y, x);

            let pos_x = (idx + 1) % width;
            let pos_y = (idx + 1) / width + offset;
            image.put_pixel(pos_x, pos_y, y);
        }
    }
}

fn normal_to_gb(normal: &Normal) -> [Rgba<u8>; 2] {
    // TODO: Scale & offset
    //  ex: let x = 8388608 + (vertex.x * 65536) * scale + offset[0] * 65536
    let x = (normal.x * 65536.0).floor() as u32;
    let y = (normal.y * 65536.0).floor() as u32;

    [
        Rgba([
            ((x >> 16) & 0xFF) as u8,
            ((x >> 8) & 0xFF) as u8,
            (x & 0xFF) as u8,
            255
        ]),
        Rgba([
            ((y >> 16) & 0xFF) as u8,
            ((y >> 8) & 0xFF) as u8,
            (y & 0xFF) as u8,
            255
        ])
    ]
}