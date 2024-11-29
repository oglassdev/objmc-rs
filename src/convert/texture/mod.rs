use image::RgbaImage;

pub mod settings;
pub mod face_id_header;
pub mod vertex_position_data;
pub mod uv_data;
pub mod vertex_indexes;

pub trait TextureProperty {
    fn height(&self) -> u32;

    fn draw(&self, image: &mut RgbaImage, offset: u32);
}
