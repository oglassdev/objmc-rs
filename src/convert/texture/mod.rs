use image::RgbaImage;
use crate::cli::{Easing, Visibility};
use crate::convert::state::ConvertState;
use crate::convert::texture::property::{FaceIdProperty, PositionData, SettingsProperty, Texture, TextureProperty, UVData, VertexIndexes};

mod property;

pub fn create_texture(state: &ConvertState) -> RgbaImage {
    let (width, height) = state.texture_size;

    let face_count = state.framed_obj.frames[0].faces.len() as u32;

    let settings_property = SettingsProperty {
        compress: state.compress,

        texture_size: [width as u16, height as u16],

        vert_count: face_count * 4,

        duration: state.args.duration.clamp(1, 65_536),
        easing: match &state.args.easing {
            Some(Easing::Linear) => 1,
            Some(Easing::InOutCubic) => 2,
            Some(Easing::Bezier) => 3,
            None => 0,
        },
        autoplay: state.args.autoplay,
        fade_texture: state.args.fade_textures,

        frame_count: state.framed_obj.frames.len() as u32,
        texture_count: state.textures.len() as u8,

        vp_height: ((state.framed_obj.vertices.len() * 3) as f64 / width as f64).ceil() as u16,
        vt_height: ((state.framed_obj.uvs.len() * 2) as f64 / width as f64).ceil() as u16,

        no_shadow: state.args.no_shadow,
        autorotate_pitch: state.args.autorotate_pitch,
        autorotate_yaw: state.args.autorotate_yaw,
        visibility: state.args
            .visibility
            .iter()
            .map(|visibility| match visibility {
                Visibility::Gui => 0b100,
                Visibility::FirstPerson => 0b010,
                Visibility::World => 0b001
            } as u8)
            .fold(0, |acc, x| acc | x),
        color_behavior: if !state.args.colorbehavior.is_empty() {
            (*state.args.colorbehavior.get(0).unwrap() as u8) << 6 |
                (*state.args.colorbehavior.get(1).unwrap() as u8) << 3 |
                *state.args.colorbehavior.get(2).unwrap() as u8
        } else { 0 }
    };

    let face_id_property = FaceIdProperty {
        length: face_count,
        width,
    };

    let texture_properties: Vec<Texture> = state.textures.iter().map(|image| {
        Texture {
            image,
            flip: state.args.flip_uv
        }
    }).collect();

    let position_data_property = PositionData {
        vertices: &state.framed_obj.vertices,
        width
    };

    let uv_data_property = UVData {
        uvs: &state.framed_obj.uvs,
        width
    };

    let vertex_indexes_property = VertexIndexes {
        frames: &state.framed_obj.frames,
        width
    };

    let mut buf = RgbaImage::new(
        width,
        settings_property.height() +
            face_id_property.height() +
            texture_properties.len() as u32 * height +
            position_data_property.height() +
            uv_data_property.height() +
            vertex_indexes_property.height()
    );

    let mut offset: u32 = 0;

    settings_property.draw(&mut buf, offset);
    offset += settings_property.height();

    face_id_property.draw(&mut buf, offset);
    offset += face_id_property.height();

    for texture in texture_properties.iter() {
        texture.draw(&mut buf, offset);
        offset += texture.height();
    }

    position_data_property.draw(&mut buf, offset);
    offset += position_data_property.height();

    uv_data_property.draw(&mut buf, offset);
    offset += uv_data_property.height();

    vertex_indexes_property.draw(&mut buf, offset);

    buf
}