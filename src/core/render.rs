use anyhow::Result;
use ratex_layout::{LayoutOptions, layout, to_display_list};
use ratex_parser::parse;
use ratex_render::{RenderOptions, render_to_png};

pub fn render_png(latex: &str, font_size: f32, padding: f32) -> Result<Vec<u8>> {
    anyhow::ensure!(
        font_size.is_finite() && font_size > 0.0,
        "font size must be a positive finite number"
    );
    anyhow::ensure!(
        padding.is_finite() && padding >= 0.0,
        "padding must be a non-negative finite number"
    );

    let nodes = parse(latex).map_err(|error| anyhow::anyhow!("invalid LaTeX: {error}"))?;
    let layout_box = layout(&nodes, &LayoutOptions::default());
    let display_list = to_display_list(&layout_box);
    let options = RenderOptions {
        font_size,
        padding,
        ..RenderOptions::default()
    };

    render_to_png(&display_list, &options)
        .map_err(|error| anyhow::anyhow!("RaTeX could not render the expression to PNG: {error}"))
}
