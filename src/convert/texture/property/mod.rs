mod face_id;
mod settings;

pub use face_id::FaceIdProperty;
pub use settings::SettingsProperty;

use image::RgbaImage;

pub trait TextureProperty {
    fn height(&self) -> u32;

    fn draw(&self, image: &mut RgbaImage, offset: u32);
}