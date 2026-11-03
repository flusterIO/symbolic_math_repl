use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub fn save_png(output_root: &Path, png: &[u8]) -> Result<PathBuf> {
    fs::create_dir_all(output_root).with_context(|| {
        format!(
            "could not create output directory {}",
            output_root.display()
        )
    })?;
    let output_root = output_root.canonicalize().with_context(|| {
        format!(
            "could not resolve output directory {}",
            output_root.display()
        )
    })?;
    let output_path = output_root.join("render.png");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .with_context(|| format!("could not create PNG file {}", output_path.display()))?;
    file.write_all(png)
        .with_context(|| format!("could not write PNG file {}", output_path.display()))?;
    file.sync_all()
        .with_context(|| format!("could not flush PNG file {}", output_path.display()))?;

    Ok(output_path)
}
