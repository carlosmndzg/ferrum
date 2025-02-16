use crate::{css::types::Value, style::types::Rgb};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Color {
    pub(crate) value: Rgb,
}

impl Color {
    pub(crate) fn maybe_new(value: &Value) -> Option<Color> {
        if let Value::Keyword(keyword) = value {
            if let Some(color) = Rgb::convert_keyword_to_rgb(keyword) {
                return Some(Color { value: color });
            }
        }

        if let Value::Color(color) = value {
            if let Some(color) = color.into() {
                return Some(Color { value: color });
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "color"
    }

    pub(crate) fn default() -> Color {
        Color {
            value: Rgb::wrap_color(0, 0, 0, 1.0).unwrap(),
        }
    }

    pub(crate) fn value(&self) -> Rgb {
        self.value.clone()
    }
}
