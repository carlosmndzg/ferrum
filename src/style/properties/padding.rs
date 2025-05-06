use crate::{css::types::Value, style::validations::Validations};

use super::{
    padding_bottom::PaddingBottom, padding_left::PaddingLeft, padding_right::PaddingRight,
    padding_top::PaddingTop, CssProperty, Property,
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Padding;

impl Padding {
    pub(crate) fn new() -> Self {
        Padding
    }

    fn create_padding_properties_with_values(
        &self,
        values: (Value, Value, Value, Value),
    ) -> Vec<Property> {
        let mut ans = Vec::new();

        ans.extend(PaddingTop::new().maybe_new(&[values.0]));
        ans.extend(PaddingRight::new().maybe_new(&[values.1]));
        ans.extend(PaddingBottom::new().maybe_new(&[values.2]));
        ans.extend(PaddingLeft::new().maybe_new(&[values.3]));

        ans
    }
}

impl CssProperty for Padding {
    fn name(&self) -> &'static str {
        "padding"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        true
    }

    fn initial_value(&self) -> Vec<Property> {
        let mut ans = Vec::new();

        ans.extend(PaddingTop::new().initial_value());
        ans.extend(PaddingRight::new().initial_value());
        ans.extend(PaddingBottom::new().initial_value());
        ans.extend(PaddingLeft::new().initial_value());

        ans
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() == 1 {
            let value = value.first().unwrap();

            if Validations::wide_keyword(value) {
                return self.create_padding_properties_with_values((
                    value.clone(),
                    value.clone(),
                    value.clone(),
                    value.clone(),
                ));
            }

            if Validations::padding_width(value) {
                return self.create_padding_properties_with_values((
                    value.clone(),
                    value.clone(),
                    value.clone(),
                    value.clone(),
                ));
            }

            return Vec::new();
        } else if value.len() >= 2 && value.len() <= 4 {
            for val in value {
                if !Validations::padding_width(val) {
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

            return self.create_padding_properties_with_values(values);
        }

        Vec::new()
    }
}
