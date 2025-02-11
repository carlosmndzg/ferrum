use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PaddingRight {
    Length(f32, Unit),
    Percentage(f32),
}

impl PaddingRight {
    pub(crate) fn maybe_new(value: &Value) -> Option<PaddingRight> {
        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(PaddingRight::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(PaddingRight::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "padding-right"
    }

    pub(crate) fn default() -> PaddingRight {
        PaddingRight::Length(0.0, Unit::Px)
    }
}
