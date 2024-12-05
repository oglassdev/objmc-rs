use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use clap::{Args, Parser, Subcommand, ValueEnum};
use image::ImageReader;
use objmc::convert::config::{AnimationConfig, ColorBehavior, ColorBehaviorConfig, ConvertConfig, Easing, Input, Visibility};
use objmc::obj::FramedObj;
use objmc::obj::model::Position;

/// A tool to bypass Minecraft Java Edition model limits by baking vertex data into texture.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
#[command(version, about, long_about = None)]
pub enum Command {
    /// Convert an OBJ and texture to a model JSON and texture containing shader data.
    #[command(about)]
    Convert(Convert),
    /// Create a skull texture containing objmc info
    #[command(about = "Create a skull texture containing objmc info")]
    Head {},
    #[command(about = "Join multiple models together")]
    Join {
        output: String,
        models: Vec<String>
    },
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Convert {
    /// The output model JSON
    pub output_model: String,

    /// The output model texture
    pub output_texture: String,

    /// The resource to point to in the model JSON. The output texture will be used if not specified
    #[arg(long)]
    pub texture_resource: Option<String>,

    /// The input model obj
    #[arg(short, long, required = true)]
    pub obj: Vec<String>,

    /// The input model texture
    #[arg(short, long, required = true)]
    pub texture: Vec<String>,
    
    /// Model offset
    #[arg(long, num_args = 3, default_values_t = [0.0, 0.0, 0.0])]
    pub offset: Vec<f64>,

    /// Model scale
    #[arg(long, default_value_t = 1.0)]
    pub scale: f64,

    /// The duration of the animation
    #[arg(long, default_value_t = 0)]
    pub duration: u32,

    /// Interpolate texture frames
    #[arg(long, default_value_t = true)]
    pub fade_textures: bool,

    #[arg(long)]
    pub easing: Option<EasingArg>,

    /// Item color overlay behavior
    #[arg(long, num_args = 3, default_values = ["pitch", "yaw", "roll"])]
    pub colorbehavior: Vec<ColorBehaviorArg>,

    // yaw -> 001
    // pitch -> 010
    // both -> 011

    /// Attempt to estimate yaw rotation with normals
    #[arg(long)]
    pub autorotate_yaw: bool,

    /// Attempt to estimate pitch rotation with normals
    #[arg(long)]
    pub autorotate_pitch: bool,

    #[arg(short, long)]
    pub compress: bool,

    /// Disable shadows from face normals
    #[arg(long)]
    pub no_shadow: bool,

    /// Disable power of two textures
    #[arg(long)]
    pub no_pow: bool,

    /// Invert the texture to compensate for flipped UV
    #[arg(long)]
    pub flip_uv: bool,

    /// Always interpolate animation, colorbehavior of all 'time' overrides this
    #[arg(long)]
    pub autoplay: bool,

    /// Determines where model is visible
    #[arg(long, default_values = ["gui", "first-person", "world"])]
    pub visibility: Vec<VisibilityArg>,
}

impl Convert {
    pub fn to_config(self) -> Result<ConvertConfig, Box<dyn Error>> {
        let mut objs = Vec::with_capacity(self.obj.len());

        for path in self.obj {
            let file = File::open(path)?;
            objs.push(BufReader::new(file));
        }

        let obj = FramedObj::read(objs)?;

        let mut textures = Vec::with_capacity(self.texture.len());

        for path in self.texture {
            let file = File::open(path)?;
            
            let image = ImageReader::new(BufReader::new(file)).decode()?;
            textures.push(image.into_rgba8());
        }

        ConvertConfig::new(
            Input {
                obj,
                textures
            },
            self.texture_resource.unwrap_or(self.output_texture),
            Position::new(self.offset[0], self.offset[1], self.offset[2]),
            0.0,
            AnimationConfig {
                autoplay: self.autoplay,
                duration: self.duration,
                fade_textures: self.fade_textures,
                easing: self.easing.map(|easing| easing.to_easing()),
            },
            ColorBehaviorConfig(
                self.colorbehavior[0].to_color_behavior(),
                self.colorbehavior[1].to_color_behavior(),
                self.colorbehavior[2].to_color_behavior()
            ),
            self.autorotate_yaw,
            self.autorotate_pitch,
            self.compress,
            self.no_shadow,
            self.no_pow,
            self.flip_uv,
            Visibility {
                gui: self.visibility.iter().any(|arg| *arg == VisibilityArg::Gui),
                first_person: self.visibility.iter().any(|arg| *arg == VisibilityArg::FirstPerson),
                world: self.visibility.iter().any(|arg| *arg == VisibilityArg::World),
            }
        )
    }
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum EasingArg {
    Linear,
    InOutCubic,
    Bezier
}

impl EasingArg {
    fn to_easing(&self) -> Easing {
        match self {
            EasingArg::Linear => Easing::Linear,
            EasingArg::InOutCubic => Easing::InOutCubic,
            EasingArg::Bezier => Easing::Bezier
        }
    }
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum VisibilityArg {
    Gui,
    FirstPerson,
    World
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum ColorBehaviorArg {
    Pitch,
    Yaw,
    Roll,
    Time,
    Scale,
    Overlay,
    Hurt
}

impl ColorBehaviorArg {
    fn to_color_behavior(&self) -> ColorBehavior {
        match self {
            ColorBehaviorArg::Pitch => ColorBehavior::Pitch,
            ColorBehaviorArg::Yaw => ColorBehavior::Yaw,
            ColorBehaviorArg::Roll => ColorBehavior::Roll,
            ColorBehaviorArg::Time => ColorBehavior::Time,
            ColorBehaviorArg::Scale => ColorBehavior::Scale,
            ColorBehaviorArg::Overlay => ColorBehavior::Overlay,
            ColorBehaviorArg::Hurt => ColorBehavior::Hurt
        }
    }
}