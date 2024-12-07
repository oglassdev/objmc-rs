use clap::Args;
use serde::Deserialize;

fn default_offset() -> Vec<f64> { vec![0.0, 0.0, 0.0] }

fn default_scale() -> f64 { 1.0 }

#[derive(Args, Deserialize, Debug, Clone)]
pub struct TransformArgs {
    /// Model offset
    #[arg(long, num_args = 3, default_values_t = default_offset())]
    #[serde(default = "default_offset")]
    pub offset: Vec<f64>,

    /// Model scale
    #[arg(long, default_value_t = default_scale())]
    #[serde(default = "default_scale")]
    pub scale: f64,
}