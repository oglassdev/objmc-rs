mod texture;
mod model;
pub mod config;

use std::error::Error;
use image::RgbaImage;
use serde_json::Value;
use tracing::info;

use crate::convert::config::ConvertConfig;
use crate::convert::model::create_model_output;
use crate::convert::texture::create_texture;

pub fn convert(config: ConvertConfig) -> Result<(RgbaImage, Value), Box<dyn Error>> {
    info!("Starting conversion...");

    let tex = create_texture(&config);

    let model = create_model_output(&config, tex.height());

    Ok((tex, model))
}