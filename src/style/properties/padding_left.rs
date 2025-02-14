use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PaddingLeft {
    Length(f32, Unit),
    Percentage(f32),
}

impl PaddingLeft {
    pub(crate) fn maybe_new(value: &Value) -> Option<PaddingLeft> {
        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(PaddingLeft::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(PaddingLeft::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "padding-left"
    }

    pub(crate) fn default() -> PaddingLeft {
        PaddingLeft::Length(0.0, Unit::Px)
    }

    pub(crate) fn actual_value(&self, parent_width: f32) -> f32 {
        match self {
            PaddingLeft::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
            PaddingLeft::Percentage(percentage) => parent_width * percentage / 100.0,
        }
    }
}
