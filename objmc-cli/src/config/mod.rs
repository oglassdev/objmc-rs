use clap::{Parser, Subcommand};
use crate::config::convert::ConvertArgs;

pub mod convert;

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
    Convert(ConvertArgs),
    /// Create a skull texture containing objmc info
    #[command(about = "Create a skull texture containing objmc info")]
    Head {},
    #[command(about = "Join multiple models together")]
    Join {
        output: String,
        models: Vec<String>
    },
}
