mod cli;
mod commands;
mod config;
mod core;
mod features;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    commands::run(cli.command)
}
