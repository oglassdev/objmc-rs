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

mod cli;

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

    let args = cli::Cli::parse();

    match args.command {
        cli::Command::Convert(conv) => {
            let model_output = conv.output_model.clone();
            let texture_output = conv.output_texture.clone();

            let config: ConvertConfig = match conv.to_config() {
                Ok(conf) => conf,
                Err(err) => {
                    error!("{}", err);
                    return;
                }
            };

            match convert(config) {
                Ok((texture, model)) => {
                    match write_output(texture, texture_output, model, model_output) {
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
        cli::Command::Head {  } => todo!(),
        cli::Command::Join { output, models } => {
            let model_count = models.len();
            match join_models(models, &output) {
                Ok(()) => {
                    info!("Joined {} models successfully!", model_count);
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        },
    }
}

fn write_output(
    texture: RgbaImage,
    texture_output: String,
    model: Value,
    model_output: String
) -> Result<(), Box<dyn Error>> {
    let texture_file = File::create(texture_output)?;
    
    let mut texture_out = BufWriter::new(texture_file);
    
    texture.write_to(&mut texture_out, ImageFormat::Png)?;
    
    let model_file = File::create(model_output)?;
    
    let mut model_out = BufWriter::new(model_file);
    
    serde_json::to_writer(&mut model_out, &model)?;
    
    Ok(())
}