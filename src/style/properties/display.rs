use crate::{css::types::Value, style::validations::Validations};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Display {
    value: Value,
}

impl Display {
    pub(super) fn new() -> Self {
        Display {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self) -> &Value {
        &self.value
    }
}

impl CssProperty for Display {
    fn name(&self) -> &'static str {
        "display"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::Display(Display {
            value: Value::Keyword("inline".to_string()),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::keyword(value, &["inline", "block", "none"]) {
            return vec![Property::Display(Display {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
