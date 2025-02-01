use crate::{css::types::Value, style::types::Rgb};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BackgroundColor {
    pub(crate) value: Rgb,
}

impl BackgroundColor {
    pub(crate) fn maybe_new(value: &Value) -> Option<BackgroundColor> {
        if let Value::Keyword(keyword) = value {
            if let Some(color) = Rgb::convert_keyword_to_rgb(keyword) {
                return Some(BackgroundColor { value: color });
            }
        }

        if let Value::Color(color) = value {
            if let Some(color) = color.into() {
                return Some(BackgroundColor { value: color });
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "background-color"
    }

    pub(crate) fn default() -> BackgroundColor {
        BackgroundColor {
            value: Rgb::wrap_color(255, 255, 255).unwrap(),
        }
    }
}
