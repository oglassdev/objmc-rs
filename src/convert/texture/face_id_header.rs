use image::{Rgba, RgbaImage};

pub fn create_face_uv_id_header(buf: &mut RgbaImage, length: u32) {
    let width = buf.width();

    for i in 0..length {
        let x = i % width;
        let y = i / width + 1;

        buf.put_pixel(x, y, Rgba([
            ((x >> 8) & 0xFF) as u8,
            (x & 0xFF) as u8,
            ((y >> 8) & 0xFF) as u8,
            (y & 0xFF) as u8
        ]));
    }
}