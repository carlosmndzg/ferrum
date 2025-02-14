use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MarginTop {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl MarginTop {
    pub(crate) fn maybe_new(value: &Value) -> Option<MarginTop> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(MarginTop::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(MarginTop::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(MarginTop::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "margin-top"
    }

    pub(crate) fn default() -> MarginTop {
        MarginTop::Length(0.0, Unit::Px)
    }

    pub(crate) fn actual_value(&self, parent_width: f32) -> f32 {
        match self {
            MarginTop::Auto => 0.0,
            MarginTop::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
            MarginTop::Percentage(percentage) => parent_width * percentage / 100.0,
        }
    }
}
