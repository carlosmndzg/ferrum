use crate::{
    css::types::{Rgb, Value},
    style::{utils::keyword_to_rgb, validations::Validations},
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct BackgroundColor {
    value: Value,
}

impl BackgroundColor {
    pub(super) fn new() -> Self {
        BackgroundColor {
            value: Value::default(),
        }
    }

    pub(crate) fn value(&self) -> Rgb {
        match &self.value {
            Value::Rgb(rgb) => rgb.clone(),
            Value::Keyword(keyword) => keyword_to_rgb(keyword),
            _ => panic!("Invalid background-color value"),
        }
    }
}

impl CssProperty for BackgroundColor {
    fn name(&self) -> &'static str {
        "background-color"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::BackgroundColor(BackgroundColor {
            value: Value::Rgb(Rgb::new(0, 0, 0, 0.)),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::wide_keyword(value) || Validations::color(value) {
            return vec![Property::BackgroundColor(BackgroundColor {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
