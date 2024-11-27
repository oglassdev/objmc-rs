use clap::Parser;
use tracing::info;

mod cli;
mod convert;
pub mod obj;

fn main() {
    tracing_subscriber::fmt::init();

    let args = cli::Cli::parse();

    info!("{:?}", args);

    match args.command {
        cli::Command::Convert(conv) => {
        },
        cli::Command::Head {  } => todo!(),
        cli::Command::Join { input } => todo!(),
    }
}
