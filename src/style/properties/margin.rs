use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Margin {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl Margin {
    pub(crate) fn maybe_new(value: &Value) -> Option<Margin> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(Margin::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(Margin::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(Margin::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "margin"
    }

    pub(crate) fn default() -> Margin {
        Margin::Length(0.0, Unit::Px)
    }
}
