use crate::{css::types::Value, style::validations::Validations};

use super::{
    border_color::BorderColor, border_style::BorderStyle, border_width::BorderWidth, CssProperty,
    Property,
};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Border;

impl Border {
    pub(crate) fn new() -> Self {
        Border
    }
}

impl CssProperty for Border {
    fn name(&self) -> &'static str {
        "border"
    }

    fn is_inheritable(&self) -> bool {
        false
    }

    fn is_shorthand(&self) -> bool {
        true
    }

    fn initial_value(&self) -> Vec<Property> {
        let mut ans = Vec::new();

        ans.extend(BorderWidth::new().initial_value());
        ans.extend(BorderStyle::new().initial_value());
        ans.extend(BorderColor::new().initial_value());

        ans
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.is_empty() || value.len() > 3 {
            return Vec::new();
        }

        let mut ans = Vec::new();
        let (mut border_width_found, mut border_style_found, mut border_color_found) =
            (false, false, false);

        for v in value {
            if Validations::border_width(v) {
                if border_width_found {
                    return Vec::new();
                }

                border_width_found = true;
                ans.extend(BorderWidth::new().maybe_new(&[v.clone()]));
            } else if Validations::border_style(v) {
                if border_style_found {
                    return Vec::new();
                }

                border_style_found = true;
                ans.extend(BorderStyle::new().maybe_new(&[v.clone()]));
            } else if Validations::color(v) {
                if border_color_found {
                    return Vec::new();
                }

                border_color_found = true;
                ans.extend(BorderColor::new().maybe_new(&[v.clone()]));
            } else {
                return Vec::new();
            }
        }

        if !border_width_found {
            ans.extend(BorderWidth::new().initial_value());
        }
        if !border_style_found {
            ans.extend(BorderStyle::new().initial_value());
        }
        if !border_color_found {
            ans.extend(BorderColor::new().initial_value());
        }

        ans
    }
}
