use std::error::Error;
use clap::Parser;
use tracing::{error, info};

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
    objmc::convert::convert()
    tracing_subscriber::fmt::init();

    let args = cli::Cli::parse();

    match args.command {
        cli::Command::Convert(conv) => {
            match convert(&conv) {
                Ok(()) => {
                    info!("Converted successfully!");
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
