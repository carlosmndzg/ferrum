use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PaddingRight {
    value: Value,
}

impl PaddingRight {
    pub(crate) fn new() -> Self {
        PaddingRight {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self, containing_block_width: f32) -> f32 {
        match &self.value {
            Value::Dimension(value, Unit::Px) => *value,
            Value::Percentage(value) => containing_block_width * value / 100.,
            _ => 0.,
        }
    }
}

impl CssProperty for PaddingRight {
    fn name(&self) -> &'static str {
        "padding-right"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::PaddingRight(PaddingRight {
            value: Value::Dimension(0., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::padding_width(value) {
            return vec![Property::PaddingRight(PaddingRight {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
