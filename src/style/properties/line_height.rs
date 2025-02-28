use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct LineHeight {
    value: Value,
}

impl LineHeight {
    pub(super) fn new() -> Self {
        LineHeight {
            value: Value::default(),
        }
    }

    pub(crate) fn value(&self) -> f32 {
        match &self.value {
            Value::Dimension(value, Unit::None) => *value,
            _ => panic!("Invalid line-height value"),
        }
    }
}

impl CssProperty for LineHeight {
    fn name(&self) -> &'static str {
        "line-height"
    }

    fn is_inheritable(&self) -> bool {
        true
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::LineHeight(LineHeight {
            value: Value::Dimension(1.2, Unit::None),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::number(value) {
            return vec![Property::LineHeight(LineHeight {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
