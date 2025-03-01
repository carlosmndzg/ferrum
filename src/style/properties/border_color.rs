use crate::{
    css::types::{Rgb, Value},
    style::{utils::keyword_to_rgb, validations::Validations},
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct BorderColor {
    value: Value,
}

impl BorderColor {
    pub(super) fn new() -> Self {
        BorderColor {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self, color: &Rgb) -> Rgb {
        match &self.value {
            Value::Rgb(rgb) => rgb.clone(),
            Value::Keyword(keyword) => keyword_to_rgb(keyword),
            Value::NotDeclared => color.clone(),
            _ => panic!("Unexpected value for border-color"),
        }
    }
}

impl CssProperty for BorderColor {
    fn name(&self) -> &'static str {
        "border-color"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::BorderColor(BorderColor {
            value: Value::NotDeclared,
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::color(value) {
            return vec![Property::BorderColor(BorderColor {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
