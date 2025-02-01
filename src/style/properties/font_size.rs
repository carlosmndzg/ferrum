use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FontSize {
    Length(f32, Unit),
}

impl FontSize {
    pub(crate) fn maybe_new(value: &Value) -> Option<FontSize> {
        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(FontSize::Length(*length, Unit::Px));
            }
        }
        None
    }

    pub(crate) fn name(&self) -> &str {
        "font-size"
    }

    pub(crate) fn default() -> FontSize {
        FontSize::Length(16.0, Unit::Px)
    }
}
