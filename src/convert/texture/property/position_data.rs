use image::{Rgba, RgbaImage};
use crate::convert::texture::property::TextureProperty;
use crate::obj::model::Position;

pub struct PositionData<'a> {
    pub vertices: &'a Vec<Position<f64>>,
    pub width: u32
}

impl TextureProperty for PositionData<'_> {
    fn height(&self) -> u32 {
        ((self.vertices.len() * 3) as f64 / self.width as f64).ceil() as u32
    }

    fn draw(&self, image: &mut RgbaImage, offset: u32) {
        let width = self.width;

        for (idx, vertex) in self.vertices.iter().enumerate() {
            let idx = idx as u32 * 3;

            let [x, y, z] = pos_to_rgb(vertex);

            let pos_x = idx % width;
            let pos_y = idx / width + offset;
            image.put_pixel(pos_x, pos_y, x);

            let pos_x = (idx + 1) % width;
            let pos_y = (idx + 1) / width + offset;
            image.put_pixel(pos_x, pos_y, y);

            let pos_x = (idx + 2) % width;
            let pos_y = (idx + 2) / width + offset;
            image.put_pixel(pos_x, pos_y, z);
        }
    }
}

fn pos_to_rgb(vertex: &Position<f64>) -> [Rgba<u8>; 3] {
    // TODO: Scale & offset
    //  ex: let x = (8388608.0 + vertex.x * 65536.0 * scale + offset.x * 65536.0).floor() as u32
    let x = (8388608.0 + vertex.x * 65536.0).floor() as u32;
    let y = (8388608.0 + vertex.y * 65536.0).floor() as u32;
    let z = (8388608.0 + vertex.z * 65536.0).floor() as u32;

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
        ]),
        Rgba([
            ((z >> 16) & 0xFF) as u8,
            ((z >> 8) & 0xFF) as u8,
            (z & 0xFF) as u8,
            255
        ]),
    ]
}