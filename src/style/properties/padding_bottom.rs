use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct PaddingBottom {
    value: Value,
}

impl PaddingBottom {
    pub(crate) fn new() -> Self {
        PaddingBottom {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self, containing_block_height: f32) -> f32 {
        match &self.value {
            Value::Dimension(value, _) => *value,
            Value::Percentage(value) => containing_block_height * value / 100.,
            _ => 0.,
        }
    }
}

impl CssProperty for PaddingBottom {
    fn name(&self) -> &'static str {
        "padding-bottom"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::PaddingBottom(PaddingBottom {
            value: Value::Dimension(0., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::padding_width(value) {
            return vec![Property::PaddingBottom(PaddingBottom {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
