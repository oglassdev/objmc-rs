mod texture;
mod model;
mod state;

use image::{ImageReader, RgbaImage};

use crate::cli::Convert;
use std::{fs::File, io::BufWriter};
use std::error::Error;
use std::io::BufReader;
use tracing::info;
use crate::convert::model::create_model_output;
use crate::convert::state::ConvertState;
use crate::convert::texture::create_texture;
use crate::obj::FramedObj;

pub fn convert(convert: &Convert) -> Result<(), Box<dyn Error>> {
    info!("Starting conversion...");

    let mut objs: Vec<BufReader<File>> = Vec::new();

    for path in convert.obj.iter() {
        objs.push(BufReader::new(File::open(path)?));
    }

    let framed_obj = FramedObj::read(objs);

    let mut textures: Vec<RgbaImage> = Vec::new();

    for path in convert.texture.iter() {
        let image = ImageReader::open(path)?.decode()?.to_rgba8();
        if !textures.is_empty() && image.dimensions() != textures[0].dimensions() {
            return Err("Textures were of different size".into())
        }
        textures.push(image);
    }

    if textures.is_empty() {
        return Err("Please provide a texture".into())
    }

    let state = ConvertState {
        args: convert,
        compress: framed_obj.uvs.len() <= 256 && convert.compress,
        texture_size: textures[0].dimensions(),
        framed_obj,
        textures
    };

    let new_file = File::create(&state.args.output_texture)?;

    let mut writer = BufWriter::new(new_file);

    let tex = create_texture(&state);

    tex.write_to(&mut writer, image::ImageFormat::Png)?;

    let new_file = File::create(&state.args.output_model)?;

    let writer = BufWriter::new(new_file);

    let resource = state.args.texture_resource.as_ref().unwrap_or(&state.args.output_texture);

    serde_json::to_writer(writer, &create_model_output(&state, resource, tex.height()))?;

    Ok(())
}