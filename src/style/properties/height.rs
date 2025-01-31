use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Height {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl Height {
    pub(crate) fn maybe_new(value: &Value) -> Option<Height> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(Height::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(Height::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(Height::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "height"
    }

    pub(crate) fn default() -> Height {
        Height::Auto
    }
}
