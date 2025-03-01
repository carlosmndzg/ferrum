use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Height {
    value: Value,
}

impl Height {
    pub(super) fn new() -> Self {
        Height {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self, parent_height: f32) -> f32 {
        match &self.value {
            Value::Keyword(k) if k == "auto" => 0.0,
            Value::Percentage(p) => (p / 100.0) * parent_height,
            Value::Dimension(value, Unit::Px) => *value,
            _ => 0.0,
        }
    }

    pub(crate) fn is_auto(&self) -> bool {
        matches!(&self.value, Value::Keyword(k) if k == "auto")
    }

    pub(crate) fn is_percentage(&self) -> bool {
        matches!(&self.value, Value::Percentage(_))
    }
}

impl CssProperty for Height {
    fn name(&self) -> &'static str {
        "height"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::Height(Height {
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
            return vec![Property::Height(Height {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
