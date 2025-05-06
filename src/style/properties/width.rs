use crate::{css::types::Value, style::validations::Validations};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Width {
    value: Value,
}

impl Width {
    pub(super) fn new() -> Self {
        Width {
            value: Value::default(),
        }
    }

    pub(crate) fn is_auto(&self) -> bool {
        matches!(&self.value, Value::Keyword(k) if k == "auto")
    }

    pub(crate) fn actual_value(&self, containing_block_width: f32) -> f32 {
        match &self.value {
            Value::Dimension(value, _) => *value,
            Value::Percentage(value) => containing_block_width * value / 100.,
            _ => 0.,
        }
    }
}

impl CssProperty for Width {
    fn name(&self) -> &'static str {
        "width"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::Width(Width {
            value: Value::Keyword("auto".to_string()),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::length(value)
            || Validations::percentage(value)
            || Validations::keyword(value, &["auto"])
        {
            return vec![Property::Width(Width {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
