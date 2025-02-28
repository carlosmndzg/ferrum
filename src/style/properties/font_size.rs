use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct FontSize {
    value: Value,
}

impl FontSize {
    pub(super) fn new() -> Self {
        FontSize {
            value: Value::default(),
        }
    }

    pub(crate) fn value(&self) -> f32 {
        match &self.value {
            Value::Dimension(value, Unit::Px) => *value,
            _ => panic!("Invalid font-size value"),
        }
    }
}

impl CssProperty for FontSize {
    fn name(&self) -> &'static str {
        "font-size"
    }

    fn is_inheritable(&self) -> bool {
        true
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::FontSize(FontSize {
            value: Value::Dimension(16., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::length(value) {
            return vec![Property::FontSize(FontSize {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
