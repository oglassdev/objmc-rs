use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use clap::Parser;
use image::{ImageFormat, RgbaImage};
use serde_json::Value;
use tracing::{error, info};
use objmc::convert::config::ConvertConfig;
use objmc::convert::convert;
use objmc::join::join_models;
use crate::config::{Cli, Command};
use crate::config::convert::ConvertResult;

mod config;

fn main() {

    /*
    Use this for our CLI & GUI implementations

    let mut objs: Vec<BufReader<File>> = Vec::new();

    for path in config.input. {
        objs.push(BufReader::new(File::open(path)?));
    }

    let framed_obj = FramedObj::read(objs)?;

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
     */
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    match args.command {
        Command::Convert(conv) => {
            let result: ConvertResult = match conv.to_config() {
                Ok(conf) => conf,
                Err(err) => {
                    error!("{}", err);
                    return;
                }
            };

            match convert(result.config) {
                Ok((texture, model)) => {
                    match write_output(texture, result.output_texture, model, result.output_model) {
                        Ok(()) => {
                            info!("Converted successfully!");
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                }
                Err(err) => {
                    error!("{}", err);
                }
            };
        },
        Command::Head {  } => todo!(),
        Command::Join { output, models } => {
            let model_count = models.len();
            match join_models(models, &output) {
                Ok(()) => {
                    info!("Joined {} models successfully!", model_count);
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
    }
}

fn write_output(
    texture: RgbaImage,
    texture_output: File,
    model: Value,
    model_output: File
) -> Result<(), Box<dyn Error>> {
    texture.write_to(&mut BufWriter::new(texture_output), ImageFormat::Png)?;
    
    serde_json::to_writer(&mut BufWriter::new(model_output), &model)?;
    
    Ok(())
}