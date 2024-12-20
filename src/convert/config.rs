use std::error::Error;
use image::RgbaImage;
use tracing::warn;
use crate::obj::FramedObj;
use crate::obj::model::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub obj: FramedObj,

    pub textures: Vec<RgbaImage>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub autoplay: bool,

    pub duration: u32,

    pub fade_textures: bool,

    pub easing: Option<Easing>
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorBehavior(pub SingleColorBehavior, pub SingleColorBehavior, pub SingleColorBehavior);

impl ColorBehavior {
    pub fn to_u8(&self) -> u8 {
        (self.0 as u8) << 6 | (self.1 as u8) << 3 | self.2 as u8
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvertConfig {
    pub(crate) input: Input,

    pub(crate) texture_size: (u32, u32),

    pub(crate) texture_resource: String,

    pub(crate) offset: Position<f64>,

    pub(crate) scale: f64,

    pub(crate) animation: Animation,

    pub(crate) color_behavior: ColorBehavior,

    // Autorotate Values:
    //  Yaw -> 001
    //  Pitch -> 010
    //  Both -> 011
    pub(crate) autorotate_yaw: bool,

    pub(crate) autorotate_pitch: bool,

    pub(crate) compress: bool,

    pub(crate) no_shadow: bool,

    pub(crate) no_pow: bool,

    pub(crate) flip_uv: bool,

    pub(crate) visibility: Visibility,
}

impl ConvertConfig {
    pub fn new(
        input: Input,
        texture_resource: String,
        offset: Position<f64>,
        scale: f64,
        animation: Animation,
        color_behavior: ColorBehavior,
        autorotate_yaw: bool,
        autorotate_pitch: bool,
        compress: bool,
        no_shadow: bool,
        no_pow: bool,
        flip_uv: bool,
        visibility: Visibility,
    ) -> Result<Self, Box<dyn Error>> {
        let texture_size = match input.textures.first() {
            Some(tex) => tex.dimensions(),
            None => {
                return Err("No textures were provided".into())
            }
        };

        if input.textures.iter().any(|image| image.dimensions() != texture_size) {
            return Err("Texture dimensions do not match".into())
        }

        let compress = if input.obj.uvs.len() > 256 && compress {
            warn!("Model had more than 256 UV indexes, disabling compression");
            false
        } else { compress };

        Ok(
            Self {
                input,
                texture_size,
                texture_resource,
                offset,
                scale,
                animation,
                color_behavior,
                autorotate_yaw,
                autorotate_pitch,
                compress,
                no_shadow,
                no_pow,
                flip_uv,
                visibility,
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum Easing {
    Linear,
    InOutCubic,
    Bezier
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Visibility {
    pub gui: bool,
    pub first_person: bool,
    pub world: bool
}

impl Visibility {
    pub fn to_u8(&self) -> u8 {
        (if self.gui { 0b100 } else { 0 }) |
            (if self.first_person { 0b10 } else { 0 }) |
            (if self.world { 0b1 } else { 0 })
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum SingleColorBehavior {
    Pitch,
    Yaw,
    Roll,
    Time,
    Scale,
    Overlay,
    Hurt
}