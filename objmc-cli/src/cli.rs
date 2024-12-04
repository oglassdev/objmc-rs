use clap::{Args, Parser, Subcommand, ValueEnum};
use clap::builder::PossibleValue;
use objmc::convert::config::{ColorBehavior, ConvertConfig, Easing, Visibility};

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
    pub easing: Option<Easing>,

    /// Item color overlay behavior
    #[arg(long, num_args = 3, default_values = ["pitch", "yaw", "roll"])]
    pub colorbehavior: Vec<ColorBehavior>,

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
    pub visibility: Vec<Visibility>,
}

impl Into<ConvertConfig> for Convert {
    fn into(self) -> ConvertConfig {
        todo!()
    }
}