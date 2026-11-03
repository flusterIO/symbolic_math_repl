use anyhow::Result;
use std::path::Path;

use crate::{
    cli::ConvertArgs,
    config::OUTPUT_DIR,
    core::{input, output, render},
    features::image_manipulation,
};

pub fn run(args: ConvertArgs) -> Result<()> {
    let request = input::read_request(args.input.as_deref())?;
    let mut png = render::render_png(&request.latex, args.font_size, args.padding)?;
    if request.transparent || request.dark_mode {
        png = image_manipulation::apply_options(&png, request.transparent, request.dark_mode)?;
    }
    let path = output::save_png(Path::new(OUTPUT_DIR), &png)?;

    // stdout is intentionally machine-readable: exactly one path per run.
    println!("{}", path.display());
    Ok(())
}
