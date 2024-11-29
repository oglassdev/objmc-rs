use image::{Rgba, RgbaImage};
use crate::convert::texture::property::TextureProperty;
use crate::obj::frame::Frame;

pub struct VertexIndexes<'a> {
    pub frames: &'a Vec<Frame>,
    pub width: u32
}

impl TextureProperty for VertexIndexes<'_> {
    fn height(&self) -> u32 {
        ((self.frames.len() *
            self.frames[0]
                .faces.iter()
                .fold(0, |acc, face| acc + face.vertex_normals.len())
            * 2
        ) as f64 / self.width as f64).ceil() as u32
    }

    fn draw(&self, image: &mut RgbaImage, offset: u32) {
        let width = self.width;

        let mut idx = 0;

        for frame in self.frames.iter() {
            for face in frame.faces.iter() {
                for (texture, uv) in face.vertex_normals.iter() {
                    let [x, y] = to_rgb(*texture, *uv);

                    let pos_x = idx % width;
                    let pos_y = idx / width + offset;
                    image.put_pixel(pos_x, pos_y, x);

                    let pos_x = (idx + 1) % width;
                    let pos_y = (idx + 1) / width + offset;
                    image.put_pixel(pos_x, pos_y, y);
                    idx += 2;
                }
            }
        }
    }
}

fn to_rgb(texture_id: u32, uv_id: u32) -> [Rgba<u8>; 2] {
    [
        Rgba([
            ((texture_id >> 16) & 0xFF) as u8,
            ((texture_id >> 8) & 0xFF) as u8,
            (texture_id & 0xFF) as u8,
            255
        ]),
        Rgba([
            ((uv_id >> 16) & 0xFF) as u8,
            ((uv_id >> 8) & 0xFF) as u8,
            (uv_id & 0xFF) as u8,
            255
        ])
    ]
}