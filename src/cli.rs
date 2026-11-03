use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "tex-to-image",
    version,
    about = "Render LaTeX math from JSON to an image"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Convert a JSON object containing a latex property to PNG.
    Convert(ConvertArgs),
}

#[derive(Debug, Args)]
pub struct ConvertArgs {
    /// JSON file to read; use - or omit it to read standard input.
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Render font size in pixels per em.
    #[arg(long, default_value_t = 40.0, value_name = "PX")]
    pub font_size: f32,

    /// Padding around the rendered expression in pixels.
    #[arg(long, default_value_t = 10.0, value_name = "PX")]
    pub padding: f32,
}
