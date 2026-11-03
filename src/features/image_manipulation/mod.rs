use std::io::Cursor;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};

/// Applies requested background and text-color transformations to a RaTeX PNG.
pub fn apply_options(png: &[u8], transparent: bool, dark_mode: bool) -> Result<Vec<u8>> {
    let mut rgba = image::load_from_memory_with_format(png, ImageFormat::Png)
        .context("could not decode renderer PNG for image manipulation")?
        .into_rgba8();

    for pixel in rgba.pixels_mut() {
        let [red, green, blue, source_alpha] = pixel.0;

        if transparent {
            // RaTeX renders dark text on a white background. Preserve its
            // antialiasing as glyph alpha while selecting the requested text color.
            let luminance =
                (0.2126 * f32::from(red) + 0.7152 * f32::from(green) + 0.0722 * f32::from(blue))
                    .round() as u8;
            let alpha = ((u16::from(255 - luminance) * u16::from(source_alpha)) / 255) as u8;
            let text_color = if dark_mode { 255 } else { 0 };
            pixel.0 = [text_color, text_color, text_color, alpha];
        } else if dark_mode {
            pixel.0 = [255 - red, 255 - green, 255 - blue, source_alpha];
        }
    }

    let mut encoded = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(rgba)
        .write_to(&mut encoded, ImageFormat::Png)
        .context("could not encode manipulated PNG")?;
    Ok(encoded.into_inner())
}
