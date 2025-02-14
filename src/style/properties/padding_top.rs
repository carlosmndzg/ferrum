use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PaddingTop {
    Length(f32, Unit),
    Percentage(f32),
}

impl PaddingTop {
    pub(crate) fn maybe_new(value: &Value) -> Option<PaddingTop> {
        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(PaddingTop::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(PaddingTop::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "padding-top"
    }

    pub(crate) fn default() -> PaddingTop {
        PaddingTop::Length(0.0, Unit::Px)
    }

    pub(crate) fn actual_value(&self, parent_width: f32) -> f32 {
        match self {
            PaddingTop::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
            PaddingTop::Percentage(percentage) => parent_width * percentage / 100.0,
        }
    }
}
