use crate::{css::types::Value, style::types::Unit};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PaddingBottom {
    Length(f32, Unit),
    Percentage(f32),
}

impl PaddingBottom {
    pub(crate) fn maybe_new(value: &Value) -> Option<PaddingBottom> {
        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(PaddingBottom::Length(*length, Unit::Px));
            }
        }

        if let Value::Percentage(percentage) = value {
            return Some(PaddingBottom::Percentage(*percentage));
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "padding-bottom"
    }

    pub(crate) fn default() -> PaddingBottom {
        PaddingBottom::Length(0.0, Unit::Px)
    }
}
