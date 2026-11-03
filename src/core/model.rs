use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RenderRequest {
    pub latex: String,
    #[serde(default)]
    pub transparent: bool,
    #[serde(default)]
    pub dark_mode: bool,
}
