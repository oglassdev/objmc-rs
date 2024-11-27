use std::error::Error;
use image::{GenericImage, Rgba, RgbaImage};
use crate::cli::{Convert, Easing, Visibility};
use crate::obj::Obj;

pub struct SettingsHeader {
    // 0: Marker
    compress: bool,

    // 1: Texture size
    texture_size: [u16; 2],

    // 2: Vertices count
    vert_count: u32,

    // 3: Counts
    frame_count: u32,
    texture_count: u8,

    // 4: Animation
    duration: u32,
    easing: u8,
    autoplay: bool,
    fade_texture: bool,

    // 5: Data heights
    vp_height: u16,
    vt_height: u16,

    // 6: Extra settings (noshadow, autorotate, visibility, colorbehavior)
    no_shadow: bool,
    autorotate_pitch: bool,
    autorotate_yaw: bool,
    visibility: u8,
    color_behavior: u8
}

impl SettingsHeader {
    pub fn from_config(convert: &Convert, obj: &Obj, texture_size: [u16; 2]) -> Result<SettingsHeader, Box<dyn Error>> {
        if texture_size[0] < 8 || texture_size[1] < 8 {
            return Err("Invalid texture size".into());
        }

        let [tex_width, _] = texture_size;

        Ok(SettingsHeader {
            compress: convert.compress,

            texture_size,

            vert_count: 1848,

            duration: convert.duration.clamp(1, 65_536),
            easing: match &convert.easing {
                Some(Easing::Linear) => 1,
                Some(Easing::InOutCubic) => 2,
                Some(Easing::Bezier) => 3,
                None => 0,
            },
            autoplay: convert.autoplay,
            fade_texture: convert.fade_textures,

            // TODO: Multiple frames
            //  Current behavior is to keep compatibility with
            //  shader by setting frames and textures to 1
            frame_count: 1,
            texture_count: 1,

            vp_height: ((obj.vertices.len() * 3) as f64 / tex_width as f64).ceil() as u16,
            vt_height: ((obj.uvs.len() * 2) as f64 / tex_width as f64).ceil() as u16,

            no_shadow: convert.no_shadow,
            autorotate_pitch: convert.autorotate_pitch,
            autorotate_yaw: convert.autorotate_yaw,
            visibility: convert
                .visibility
                .iter()
                .map(|visibility| match visibility {
                    Visibility::Gui => 0b100,
                    Visibility::FirstPerson => 0b010,
                    Visibility::World => 0b001
                } as u8)
                .fold(0, |acc, x| acc | x),
            color_behavior: if !convert.colorbehavior.is_empty() {
                (*convert.colorbehavior.get(0).unwrap() as u8) << 6 |
                    (*convert.colorbehavior.get(1).unwrap() as u8) << 3 |
                    *convert.colorbehavior.get(2).unwrap() as u8
            } else { 0 }
        })
    }

    pub fn draw(&self, image_buffer: &mut RgbaImage) {
        image_buffer.put_pixel(0, 0, Rgba([12, 34, 56, if self.compress { 79 } else { 78 }]));

        let [size_x, size_y] = self.texture_size;

        image_buffer.put_pixel(1,0 , Rgba([
            (size_x / 256) as u8, (size_x % 256) as u8,
            (size_y / 256) as u8, (size_y % 256) as u8
        ]));

        let vert_count = self.vert_count;
        image_buffer.put_pixel(2,0 , Rgba([
            ((vert_count >> 24) & 0xFF) as u8,
            ((vert_count >> 16) & 0xFF) as u8,
            ((vert_count >> 8) & 0xFF) as u8,
            (vert_count & 0xFF) as u8
        ]));

        image_buffer.put_pixel(3, 0, Rgba([
            ((self.frame_count >> 16) & 0xFF) as u8,
            ((self.frame_count >> 8) & 0xFF) as u8,
            (self.frame_count & 0xFF) as u8,
            self.texture_count
        ]));

        let duration = self.duration;
        image_buffer.put_pixel(4, 0, Rgba([
            ((duration >> 16) & 0xFF) as u8,
            ((duration >> 8) & 0xFF) as u8,
            (duration & 0xFF) as u8,
            0x80 + ((self.autoplay as u8) << 6) + (self.easing << 4) + ((self.fade_texture as u8) << 2)
        ]));

        image_buffer.put_pixel(5, 0, Rgba([
            (self.vp_height >> 8) as u8,
            (self.vp_height & 0xFF) as u8,
            (self.vt_height >> 8) as u8,
            (self.vt_height & 0xFF) as u8
        ]));

        image_buffer.put_pixel(6, 0, Rgba([
            (self.no_shadow as u8) << 7 |
                ((self.autorotate_pitch as u8) << 6) |
                ((self.autorotate_yaw as u8) << 5) |
                self.visibility << 2,
            self.color_behavior, 255, 255
        ]));
    }
}