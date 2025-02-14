use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MarginRight {
    Auto,
    Length(f32, Unit),
    Percentage(f32),
}

impl MarginRight {
    pub(crate) fn maybe_new(value: &Value) -> Option<MarginRight> {
        if let Value::Keyword(keyword) = value {
            if keyword.as_str() == "auto" {
                return Some(MarginRight::Auto);
            }
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(MarginRight::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(MarginRight::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "margin-right"
    }

    pub(crate) fn default() -> MarginRight {
        MarginRight::Length(0.0, Unit::Px)
    }

    pub(crate) fn is_auto(&self) -> bool {
        matches!(self, MarginRight::Auto)
    }

    pub(crate) fn actual_value(&self, parent_width: f32) -> f32 {
        match self {
            MarginRight::Auto => 0.0,
            MarginRight::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
            MarginRight::Percentage(percentage) => parent_width * percentage / 100.0,
        }
    }
}
