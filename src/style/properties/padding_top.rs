use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PaddingTop {
    value: Value,
}

impl PaddingTop {
    pub(crate) fn new() -> Self {
        PaddingTop {
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

impl CssProperty for PaddingTop {
    fn name(&self) -> &'static str {
        "padding-top"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::PaddingTop(PaddingTop {
            value: Value::Dimension(0., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::padding_width(value) {
            return vec![Property::PaddingTop(PaddingTop {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
