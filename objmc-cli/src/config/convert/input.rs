use clap::Args;
use serde::Deserialize;

#[derive(Args, Deserialize, Debug, Clone)]
pub struct InputArgs {
    /// The input model OBJs
    #[arg(short, long = "obj", required_unless_present = "config")]
    pub objs: Vec<String>,

    /// The input model textures
    #[arg(short, long = "texture", required_unless_present = "config")]
    pub textures: Vec<String>
}