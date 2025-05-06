use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PaddingLeft {
    value: Value,
}

impl PaddingLeft {
    pub(crate) fn new() -> Self {
        PaddingLeft {
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

impl CssProperty for PaddingLeft {
    fn name(&self) -> &'static str {
        "padding-left"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::PaddingLeft(PaddingLeft {
            value: Value::Dimension(0., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::padding_width(value) {
            return vec![Property::PaddingLeft(PaddingLeft {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
