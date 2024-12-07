mod input;
mod output;
mod transform;
mod animation;
mod settings;

use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use clap::Args;
use image::{ImageFormat, ImageReader};
use serde::Deserialize;
use tracing::info;
use input::InputArgs;
use objmc::convert::config::{Animation, ColorBehavior, ConvertConfig, Input, Visibility};
use objmc::obj::FramedObj;
use objmc::obj::model::Position;
use output::OutputArgs;
use crate::config::convert::animation::AnimationArgs;
use crate::config::convert::settings::{SettingsArgs, VisibilityArg};
use crate::config::convert::transform::TransformArgs;

pub struct ConvertResult {
    pub config: ConvertConfig,
    pub output_model: File,
    pub output_texture: File
}

#[derive(Args, Deserialize, Debug, Clone)]
pub struct ConvertArgs {
    #[arg(long)]
    pub config: Option<String>,
    
    #[group(flatten)]
    pub input: InputArgs,

    #[command(flatten)]
    pub output: OutputArgs,

    #[command(flatten)]
    pub animation: AnimationArgs,

    #[command(flatten)]
    pub transform: TransformArgs,

    #[command(flatten)]
    pub settings: SettingsArgs,
}

impl ConvertArgs {
    pub fn to_config(self) -> Result<ConvertResult, Box<dyn Error>> {
        let dir: PathBuf;
        let args = if let Some(path) = self.config {
            let path = Path::new(&path).canonicalize()?;

            dir = path.parent().unwrap().to_path_buf();

            info!("Using config file: {:?}", &path);

            let file = File::open(path)?;

            serde_json::from_reader::<_, ConvertArgs>(
                BufReader::new(file)
            )?
        } else {
            dir = current_dir()?;
            self
        };

        let mut objs = Vec::with_capacity(args.input.objs.len());

        for path in args.input.objs {
            let path = PathBuf::from(path);

            info!("Loading obj file: {:?}", &path);

            let path = if path.is_relative() { (&dir).join(&path) } else { path };

            let file = File::open(path)?;
            objs.push(BufReader::new(file));
        }

        let obj = FramedObj::read(objs)?;

        let mut textures = Vec::with_capacity(args.input.textures.len());

        for path in args.input.textures {
            let path = PathBuf::from(path);

            let path = if path.is_relative() { (&dir).join(&path) } else { path };

            info!("Loading texture file: {:?}", &path);

            let file = File::open(path)?;

            let image = ImageReader::with_format(BufReader::new(file), ImageFormat::Png).decode()?;
            textures.push(image.into_rgba8());
        }

        let output_model = PathBuf::from(args.output.model);
        let output_model = File::create(
            if output_model.is_relative() { (&dir).join(&output_model) } else { output_model }
        )?;

        let output_texture = PathBuf::from(&args.output.texture);
        let output_texture = File::create(
            if output_texture.is_relative() { (&dir).join(&output_texture) } else { output_texture }
        )?;

        let offset = &args.transform.offset;

        if offset.len() < 3 {
            return Err("Offset should have three values".into())
        }

        ConvertConfig::new(
            Input {
                obj,
                textures
            },
            args.output.texture_resource.unwrap_or(args.output.texture),
            Position::new(offset[0], offset[1], offset[2]),
            args.transform.scale,
            Animation {
                autoplay: args.animation.autoplay,
                duration: args.animation.duration,
                fade_textures: args.animation.fade_textures,
                easing: args.animation.easing.map(|easing| easing.to_easing())
            },
            ColorBehavior(
                args.settings.colorbehavior[0].to_color_behavior(),
                args.settings.colorbehavior[1].to_color_behavior(),
                args.settings.colorbehavior[2].to_color_behavior()
            ),
            args.settings.autorotate_yaw,
            args.settings.autorotate_pitch,
            args.settings.compress,
            args.settings.no_shadow,
            args.settings.no_pow,
            args.settings.flip_uv,
            Visibility {
                gui: args.settings.visibility.iter().any(|arg| *arg == VisibilityArg::Gui),
                first_person: args.settings.visibility.iter().any(|arg| *arg == VisibilityArg::FirstPerson),
                world: args.settings.visibility.iter().any(|arg| *arg == VisibilityArg::World),
            }
        ).map(|config| ConvertResult {
            config,
            output_model,
            output_texture
        })
    }
}