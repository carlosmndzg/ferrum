use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MarginLeft {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl MarginLeft {
    pub(crate) fn maybe_new(value: &Value) -> Option<MarginLeft> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(MarginLeft::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(MarginLeft::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(MarginLeft::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "margin-left"
    }

    pub(crate) fn default() -> MarginLeft {
        MarginLeft::Length(0.0, Unit::Px)
    }
}
