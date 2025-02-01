use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Padding {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl Padding {
    pub(crate) fn maybe_new(value: &Value) -> Option<Padding> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(Padding::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(Padding::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(Padding::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "padding"
    }

    pub(crate) fn default() -> Padding {
        Padding::Length(0.0, Unit::Px)
    }
}
