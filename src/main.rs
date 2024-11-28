use std::error::Error;
use clap::Parser;
use tracing::{error, info};
use crate::convert::convert;

mod cli;
mod convert;
pub mod obj;

fn main() {
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
        cli::Command::Join { input } => todo!(),
    }
}
