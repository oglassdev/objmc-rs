use clap::{Args, ValueEnum};
use serde::Deserialize;
use objmc::convert::config::Easing;

fn default_false() -> bool { false }

fn default_animation_duration() -> u32 { 0 }

#[derive(Args, Deserialize, Debug, Clone)]
pub struct AnimationArgs {
    /// Always interpolate animation, colorbehavior of all 'time' overrides this
    #[arg(long)]
    #[serde(default = "default_false")]
    pub autoplay: bool,

    /// The duration of the animation
    #[arg(long, default_value_t = default_animation_duration())]
    #[serde(default = "default_animation_duration")]
    pub duration: u32,

    /// Interpolate texture frames
    #[arg(long, default_value_t = true)]
    #[serde(default = "default_false")]
    pub fade_textures: bool,

    #[arg(long)]
    pub easing: Option<EasingArg>,
}

#[derive(ValueEnum, Deserialize, Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EasingArg {
    Linear,
    InOutCubic,
    Bezier
}

impl EasingArg {
    pub(crate) fn to_easing(&self) -> Easing {
        match self {
            EasingArg::Linear => Easing::Linear,
            EasingArg::InOutCubic => Easing::InOutCubic,
            EasingArg::Bezier => Easing::Bezier
        }
    }
}
