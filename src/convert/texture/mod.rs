use image::RgbaImage;
use crate::convert::config::{ConvertConfig, Easing};
use crate::convert::texture::property::{FaceIdProperty, PositionData, SettingsProperty, Texture, TextureProperty, UVData, VertexIndexes};

mod property;

pub fn create_texture(config: &ConvertConfig) -> RgbaImage {
    let (width, height) = config.texture_size;

    let face_count = config.input.obj.frames[0].faces.len() as u32;

    let settings_property = SettingsProperty {
        compress: config.compress,

        texture_size: [width as u16, height as u16],

        vert_count: face_count * 4,

        duration: config.animation_config.duration.clamp(1, 65_536),
        easing: match &config.animation_config.easing {
            Some(Easing::Linear) => 1,
            Some(Easing::InOutCubic) => 2,
            Some(Easing::Bezier) => 3,
            None => 0,
        },
        autoplay: config.animation_config.autoplay,
        fade_texture: config.animation_config.fade_textures,

        frame_count: config.input.obj.frames.len() as u32,
        texture_count: config.input.textures.len() as u8,

        vp_height: ((config.input.obj.vertices.len() * 3) as f64 / width as f64).ceil() as u16,
        vt_height: ((config.input.obj.uvs.len() * 2) as f64 / width as f64).ceil() as u16,

        no_shadow: config.no_shadow,
        autorotate_pitch: config.autorotate_pitch,
        autorotate_yaw: config.autorotate_yaw,
        visibility: config
            .visibility
            .to_u8(),
        color_behavior: config.color_behavior.to_u8()
    };

    let face_id_property = FaceIdProperty {
        length: face_count,
        width,
    };

    let texture_properties: Vec<Texture> = config.input.textures.iter().map(|image| {
        Texture {
            image,
            flip: config.flip_uv
        }
    }).collect();

    let position_data_property = PositionData {
        vertices: &config.input.obj.vertices,
        width,
        scale: config.scale,
        offset: config.offset
    };

    let uv_data_property = UVData {
        uvs: &config.input.obj.uvs,
        width
    };

    let vertex_indexes_property = VertexIndexes {
        frames: &config.input.obj.frames,
        width
    };

    let base_height = settings_property.height() +
        face_id_property.height() +
        texture_properties.len() as u32 * height +
        position_data_property.height() +
        uv_data_property.height() +
        vertex_indexes_property.height();

    let mut buf = RgbaImage::new(
        width,
        if config.no_pow { base_height } else { base_height.next_power_of_two() }
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