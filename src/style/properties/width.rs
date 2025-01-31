use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Width {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl Width {
    pub(crate) fn maybe_new(value: &Value) -> Option<Width> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(Width::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(Width::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(Width::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "width"
    }

    pub(crate) fn default() -> Width {
        Width::Auto
    }
}
