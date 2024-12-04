use std::error::Error;
use clap::Parser;
use tracing::{error, info};

mod cli;

fn main() {
    objmc::convert::convert()
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
        cli::Command::Join { output, models } => {
            let model_count = models.len();
            match join_models(models, &output) {
                Ok(()) => {
                    info!("Joined {} models successfully!", model_count);
                }
                Err(err) => {
                    error!("{}", err);
                }
            }
        },
    }
}
