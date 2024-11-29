use image::RgbaImage;
use crate::cli::Convert;
use crate::obj::FramedObj;

pub(super) struct ConvertState<'a> {
    pub args: &'a Convert,

    pub compress: bool,
    
    pub texture_size: (u32, u32),

    pub framed_obj: FramedObj,

    pub textures: Vec<RgbaImage>
}