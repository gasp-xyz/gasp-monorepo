use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(short, long)]
    pub config: PathBuf,
}

impl CliArgs {
    pub fn build() -> Self {
        CliArgs::parse()
    }
}
