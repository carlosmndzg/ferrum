use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MarginBottom {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl MarginBottom {
    pub(crate) fn maybe_new(value: &Value) -> Option<MarginBottom> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(MarginBottom::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(MarginBottom::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(MarginBottom::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "margin-bottom"
    }

    pub(crate) fn default() -> MarginBottom {
        MarginBottom::Length(0.0, Unit::Px)
    }

    pub(crate) fn actual_value(&self, parent_width: f32) -> f32 {
        match self {
            MarginBottom::Auto => 0.0,
            MarginBottom::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
            MarginBottom::Percentage(percentage) => parent_width * percentage / 100.0,
        }
    }
}
