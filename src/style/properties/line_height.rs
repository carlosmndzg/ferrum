use crate::css::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LineHeight {
    UnitLess(f32),
}

impl LineHeight {
    pub(crate) fn maybe_new(value: &Value) -> Option<LineHeight> {
        if let Value::Dimension(length, unit) = value {
            if unit.is_empty() {
                return Some(LineHeight::UnitLess(*length));
            }
        }
        None
    }

    pub(crate) fn name(&self) -> &str {
        "line-height"
    }

    pub(crate) fn default() -> LineHeight {
        LineHeight::UnitLess(1.2)
    }
}
