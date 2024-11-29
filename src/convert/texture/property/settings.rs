use image::{GenericImage, Rgba, RgbaImage};
use crate::convert::texture::property::TextureProperty;

pub struct SettingsProperty {
    // 0: Marker
    pub compress: bool,

    // 1: Texture size
    pub texture_size: [u16; 2],

    // 2: Vertices count
    pub vert_count: u32,

    // 3: Counts
    pub frame_count: u32,
    pub texture_count: u8,

    // 4: Animation
    pub duration: u32,
    pub easing: u8,
    pub autoplay: bool,
    pub fade_texture: bool,

    // 5: Data heights
    pub vp_height: u16,
    pub vt_height: u16,

    // 6: Extra settings (noshadow, autorotate, visibility, colorbehavior)
    pub no_shadow: bool,
    pub autorotate_pitch: bool,
    pub autorotate_yaw: bool,
    pub visibility: u8,
    pub color_behavior: u8
}

impl TextureProperty for SettingsProperty {
    fn height(&self) -> u32 { 1 }
    
    fn draw(&self, image_buffer: &mut RgbaImage, offset: u32) {
        image_buffer.put_pixel(0, offset, Rgba([12, 34, 56, if self.compress { 79 } else { 78 }]));

        let [size_x, size_y] = self.texture_size;

        image_buffer.put_pixel(1, offset, Rgba([
            (size_x / 256) as u8, (size_x % 256) as u8,
            (size_y / 256) as u8, (size_y % 256) as u8
        ]));

        let vert_count = self.vert_count;
        image_buffer.put_pixel(2, offset, Rgba([
            ((vert_count >> 24) & 0xFF) as u8,
            ((vert_count >> 16) & 0xFF) as u8,
            ((vert_count >> 8) & 0xFF) as u8,
            (vert_count & 0xFF) as u8
        ]));

        image_buffer.put_pixel(3, offset, Rgba([
            ((self.frame_count >> 16) & 0xFF) as u8,
            ((self.frame_count >> 8) & 0xFF) as u8,
            (self.frame_count & 0xFF) as u8,
            self.texture_count
        ]));

        let duration = self.duration;
        image_buffer.put_pixel(4, offset, Rgba([
            ((duration >> 16) & 0xFF) as u8,
            ((duration >> 8) & 0xFF) as u8,
            (duration & 0xFF) as u8,
            0x80 + ((self.autoplay as u8) << 6) + (self.easing << 4) + ((self.fade_texture as u8) << 2)
        ]));

        image_buffer.put_pixel(5, offset, Rgba([
            (self.vp_height >> 8) as u8,
            (self.vp_height & 0xFF) as u8,
            (self.vt_height >> 8) as u8,
            (self.vt_height & 0xFF) as u8
        ]));

        image_buffer.put_pixel(6, offset, Rgba([
            (self.no_shadow as u8) << 7 |
                ((self.autorotate_pitch as u8) << 6) |
                ((self.autorotate_yaw as u8) << 5) |
                self.visibility << 2,
            self.color_behavior, 255, 255
        ]));
    }
}