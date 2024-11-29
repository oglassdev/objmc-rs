mod settings;
mod face_id;
mod texture;
mod position_data;
mod uv_data;
mod vertex_indexes;

pub use settings::SettingsProperty;
pub use face_id::FaceIdProperty;
pub use texture::Texture;
pub use position_data::PositionData;
pub use uv_data::UVData;
pub use vertex_indexes::VertexIndexes;

use image::RgbaImage;

pub trait TextureProperty {
    fn height(&self) -> u32;

    fn draw(&self, image: &mut RgbaImage, offset: u32);
}