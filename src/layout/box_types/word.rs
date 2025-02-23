use crate::style::types::Rgb;

#[derive(Debug, PartialEq)]
pub(crate) struct Word {
    pub(crate) text: String,
    pub(crate) font_size: f32,
    pub(crate) line_height: f32,
    pub(crate) font_weight: u32,
    pub(crate) color: Rgb,
}
