use crate::{
    css::types::{Unit, Value},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FontWeight {
    value: Value,
}

impl FontWeight {
    pub(super) fn new() -> Self {
        FontWeight {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self) -> u32 {
        match &self.value {
            Value::Dimension(value, Unit::None) => *value as u32,
            Value::Keyword(keyword) => match keyword.as_str() {
                "normal" => 400,
                "bold" => 700,
                _ => panic!("Invalid font-weight value"),
            },
            _ => panic!("Invalid font-weight value"),
        }
    }
}

impl CssProperty for FontWeight {
    fn name(&self) -> &'static str {
        "font-weight"
    }

    fn is_inheritable(&self) -> bool {
        true
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::FontWeight(FontWeight {
            value: Value::Dimension(400., Unit::None),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::font_weight(value) {
            return vec![Property::FontWeight(FontWeight {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
