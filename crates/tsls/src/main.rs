// use serde_json::json;
mod cli;
mod config;

use clap::Parser;
use flexi_logger::{FileSpec, Logger, WriteMode};
use log::warn;
use server::server_mode;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// mode
    #[clap(short, long, default_value = "server")]
    mode: String,

    #[clap(short, long, default_value = "")]
    setup: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initial the logger
    // flexi_logger::Logger::try_with_env_or_str("debug")?.start()?;
    let _logger = Logger::try_with_str("debug")?
        .log_to_file(FileSpec::default())
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;
    // .start()
    // .unwrap();
    warn!("Logger initialized");

    // Note that  we must have our logging only write out to stderr.
    let args = Args::parse();

    if !args.setup[0].is_empty() {
        cli::setup(args.setup);
    }

    match args.mode.as_str() {
        "server" => {
            server_mode().unwrap();
            Ok(())
        }
        "headless" => unimplemented!("headless mode"),
        _ => unimplemented!("unknown mode"),
    }
}
