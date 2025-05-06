use crate::{
    css::types::{Rgb, Value},
    style::{utils::keyword_to_rgb, validations::Validations},
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Color {
    value: Value,
}

impl Color {
    pub(super) fn new() -> Self {
        Color {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self) -> Rgb {
        match &self.value {
            Value::Rgb(rgb) => rgb.clone(),
            Value::Keyword(keyword) => keyword_to_rgb(keyword),
            _ => panic!("Invalid color value"),
        }
    }
}

impl CssProperty for Color {
    fn name(&self) -> &'static str {
        "color"
    }

    fn is_inheritable(&self) -> bool {
        true
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::Color(Color {
            value: Value::Rgb(Rgb::new(0, 0, 0, 1.)),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::wide_keyword(value) || Validations::color(value) {
            return vec![Property::Color(Color {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
