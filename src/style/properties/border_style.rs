use crate::{css::types::Value, style::validations::Validations};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct BorderStyle {
    value: Value,
}

impl BorderStyle {
    pub(super) fn new() -> Self {
        BorderStyle {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self) -> &Value {
        &self.value
    }
}

impl CssProperty for BorderStyle {
    fn name(&self) -> &'static str {
        "border-style"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::BorderStyle(BorderStyle {
            value: Value::Keyword("none".to_string()),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::border_style(value) {
            return vec![Property::BorderStyle(BorderStyle {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
