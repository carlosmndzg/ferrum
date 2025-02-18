use crate::{css::types::Value, style::types::Rgb};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BorderColor {
    Declaration { value: Rgb },
    NotDeclared,
}

impl BorderColor {
    pub(crate) fn maybe_new(value: &Value) -> Option<BorderColor> {
        if let Value::Keyword(keyword) = value {
            if let Some(color) = Rgb::convert_keyword_to_rgb(keyword) {
                return Some(BorderColor::Declaration { value: color });
            }
        }

        if let Value::Color(color) = value {
            if let Some(color) = color.into() {
                return Some(BorderColor::Declaration { value: color });
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "border-color"
    }

    pub(crate) fn default() -> BorderColor {
        BorderColor::NotDeclared
    }

    pub(crate) fn actual_value<'a>(&'a self, value_color_property: &'a Rgb) -> &'a Rgb {
        match self {
            BorderColor::Declaration { value } => value,
            BorderColor::NotDeclared => value_color_property,
        }
    }
}
