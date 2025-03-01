use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct MarginTop {
    value: Value,
}

impl MarginTop {
    pub(crate) fn new() -> Self {
        MarginTop {
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

impl CssProperty for MarginTop {
    fn name(&self) -> &'static str {
        "margin-top"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::MarginTop(MarginTop {
            value: Value::Dimension(0., Unit::Px),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::wide_keyword(value) || Validations::margin_width(value) {
            return vec![Property::MarginTop(MarginTop {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
