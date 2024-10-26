use clap::{Parser, Subcommand};
use micro_util::gen_clap_handler;
use tracing::Level;
use tracing_unwrap::ResultExt;
mod problems {
    automod::dir!(pub "src/problems");
}

mod utils;

gen_clap_handler!("src/problems");

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Commands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run { problem: String },
}

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_level(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect_or_log("Init tracing failed");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run { problem }) => {
            handle(&problem);
        }
        None => {
            tracing::error!("No command specified, use --help for more information");
        }
    }
}
