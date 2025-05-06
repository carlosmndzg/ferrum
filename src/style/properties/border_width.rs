use crate::{css::types::Value, style::validations::Validations};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BorderWidth {
    value: Value,
}

impl BorderWidth {
    pub(super) fn new() -> Self {
        BorderWidth {
            value: Value::default(),
        }
    }

    pub(crate) fn actual_value(&self, border_style: &Value) -> f32 {
        if matches!(border_style, Value::Keyword(value) if value == "none" || value == "hidden") {
            return 0.;
        }

        match &self.value {
            Value::Keyword(value) => match value.as_str() {
                "thin" => 1.,
                "medium" => 3.,
                "thick" => 5.,
                _ => panic!("Invalid border-width value"),
            },
            Value::Dimension(value, _) => *value,
            _ => panic!("Invalid border-width value"),
        }
    }
}

impl CssProperty for BorderWidth {
    fn name(&self) -> &'static str {
        "border-width"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::BorderWidth(BorderWidth {
            value: Value::Keyword("medium".to_string()),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::border_width(value) {
            return vec![Property::BorderWidth(BorderWidth {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
