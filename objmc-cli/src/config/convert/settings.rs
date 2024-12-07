use clap::{Args, ValueEnum};
use serde::Deserialize;
use objmc::convert::config::SingleColorBehavior;

fn default_false() -> bool { false }

fn default_offset() -> Vec<f64> { vec![0.0, 0.0, 0.0] }

fn default_scale() -> f64 { 1.0 }

fn default_colorbehavior() -> Vec<ColorBehaviorArg> { vec![ColorBehaviorArg::Pitch, ColorBehaviorArg::Yaw, ColorBehaviorArg::Roll] }

fn default_visibility() -> Vec<VisibilityArg> { vec![VisibilityArg::Gui, VisibilityArg::FirstPerson, VisibilityArg::World] }

#[derive(Args, Deserialize, Debug, Clone)]
pub struct SettingsArgs {
    /// Item color overlay behavior
    #[arg(long, num_args = 3, default_values = ["pitch", "yaw", "roll"])]
    #[serde(default = "default_colorbehavior")]
    pub colorbehavior: Vec<ColorBehaviorArg>,

    // yaw -> 001
    // pitch -> 010
    // both -> 011

    /// Attempt to estimate yaw rotation with normals
    #[arg(long)]
    #[serde(default = "default_false")]
    pub autorotate_yaw: bool,

    /// Attempt to estimate pitch rotation with normals
    #[arg(long)]
    #[serde(default = "default_false")]
    pub autorotate_pitch: bool,

    #[arg(short, long)]
    #[serde(default = "default_false")]
    pub compress: bool,

    /// Disable shadows from face normals
    #[arg(long)]
    #[serde(default = "default_false")]
    pub no_shadow: bool,

    /// Disable power of two textures
    #[arg(long)]
    #[serde(default = "default_false")]
    pub no_pow: bool,

    /// Invert the texture to compensate for flipped UV
    #[arg(long)]
    #[serde(default = "default_false")]
    pub flip_uv: bool,

    /// Determines where model is visible
    #[arg(long, default_values = ["gui", "first-person", "world"])]
    #[serde(default = "default_visibility")]
    pub visibility: Vec<VisibilityArg>,
}


#[derive(ValueEnum, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VisibilityArg {
    Gui,
    FirstPerson,
    World
}

#[derive(ValueEnum, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[serde(rename_all = "lowercase")]
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
    pub(crate) fn to_color_behavior(&self) -> SingleColorBehavior {
        match self {
            ColorBehaviorArg::Pitch => SingleColorBehavior::Pitch,
            ColorBehaviorArg::Yaw => SingleColorBehavior::Yaw,
            ColorBehaviorArg::Roll => SingleColorBehavior::Roll,
            ColorBehaviorArg::Time => SingleColorBehavior::Time,
            ColorBehaviorArg::Scale => SingleColorBehavior::Scale,
            ColorBehaviorArg::Overlay => SingleColorBehavior::Overlay,
            ColorBehaviorArg::Hurt => SingleColorBehavior::Hurt
        }
    }
}