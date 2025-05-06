use crate::{css::types::Value, style::validations::Validations};

use super::{
    margin_bottom::MarginBottom, margin_left::MarginLeft, margin_right::MarginRight,
    margin_top::MarginTop, CssProperty, Property,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Margin;

impl Margin {
    pub(crate) fn new() -> Self {
        Margin
    }

    fn create_margin_properties_with_values(
        &self,
        values: (Value, Value, Value, Value),
    ) -> Vec<Property> {
        let mut ans = Vec::new();

        ans.extend(MarginTop::new().maybe_new(&[values.0]));
        ans.extend(MarginRight::new().maybe_new(&[values.1]));
        ans.extend(MarginBottom::new().maybe_new(&[values.2]));
        ans.extend(MarginLeft::new().maybe_new(&[values.3]));

        ans
    }
}

impl CssProperty for Margin {
    fn name(&self) -> &'static str {
        "margin"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        true
    }

    fn initial_value(&self) -> Vec<Property> {
        let mut ans = Vec::new();

        ans.extend(MarginTop::new().initial_value());
        ans.extend(MarginRight::new().initial_value());
        ans.extend(MarginBottom::new().initial_value());
        ans.extend(MarginLeft::new().initial_value());

        ans
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() == 1 {
            let value = value.first().unwrap();

            if Validations::wide_keyword(value) {
                return self.create_margin_properties_with_values((
                    value.clone(),
                    value.clone(),
                    value.clone(),
                    value.clone(),
                ));
            }

            if Validations::margin_width(value) {
                return self.create_margin_properties_with_values((
                    value.clone(),
                    value.clone(),
                    value.clone(),
                    value.clone(),
                ));
            }

            return Vec::new();
        } else if value.len() >= 2 && value.len() <= 4 {
            for val in value {
                if !Validations::margin_width(val) {
                    return Vec::new();
                }
            }

            let values = match value.len() {
                2 => (
                    value[0].clone(),
                    value[1].clone(),
                    value[0].clone(),
                    value[1].clone(),
                ),
                3 => (
                    value[0].clone(),
                    value[1].clone(),
                    value[2].clone(),
                    value[1].clone(),
                ),
                4 => (
                    value[0].clone(),
                    value[1].clone(),
                    value[2].clone(),
                    value[3].clone(),
                ),
                _ => return Vec::new(),
            };

            return self.create_margin_properties_with_values(values);
        }

        Vec::new()
    }
}
