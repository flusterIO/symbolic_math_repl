use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use anyhow::{Context, Result};

use super::model::RenderRequest;

pub fn read_request(input_path: Option<&Path>) -> Result<RenderRequest> {
    let mut json = String::new();
    match input_path {
        Some(path) if path != Path::new("-") => {
            File::open(path)
                .with_context(|| format!("could not open input file {}", path.display()))?
                .read_to_string(&mut json)
                .with_context(|| format!("could not read input file {}", path.display()))?;
        }
        _ => {
            io::stdin()
                .read_to_string(&mut json)
                .context("could not read JSON from standard input")?;
        }
    }

    let request: RenderRequest = serde_json::from_str(&json)
        .context("input must be a JSON object containing a string property named 'latex'")?;
    anyhow::ensure!(
        !request.latex.trim().is_empty(),
        "the latex property must not be empty"
    );
    Ok(request)
}
