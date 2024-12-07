use clap::Args;
use serde::Deserialize;

#[derive(Args, Deserialize, Debug, Clone)]
pub struct OutputArgs {
    /// The output model
    #[arg(required_unless_present = "config", default_value = "")]
    pub model: String,

    /// The output texture
    #[arg(required_unless_present = "config", default_value = "")]
    pub texture: String,

    /// The resource to point to in the model JSON. The output texture will be used if not specified
    #[arg(long)]
    pub texture_resource: Option<String>,
}