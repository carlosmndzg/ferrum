use crate::{css::types::Value, style::types::Unit};

use super::border_style::BorderStyle;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BorderWidth {
    Length(f32, Unit),
}

impl BorderWidth {
    pub(crate) fn maybe_new(value: &Value) -> Option<BorderWidth> {
        if let Value::Keyword(keyword) = value {
            return BorderWidth::border_width_from_keyword(keyword);
        }

        if let Value::Dimension(length, unit) = value {
            if unit == "px" {
                return Some(BorderWidth::Length(*length, Unit::Px));
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "border-width"
    }

    pub(crate) fn default() -> BorderWidth {
        BorderWidth::border_width_from_keyword("medium").unwrap()
    }

    pub(crate) fn actual_value(&self, border_style: &BorderStyle) -> f32 {
        if let BorderStyle::None | BorderStyle::Hidden = border_style {
            return 0.;
        }

        match self {
            BorderWidth::Length(length, unit) => match unit {
                Unit::Px => *length,
            },
        }
    }

    fn border_width_from_keyword(keyword: &str) -> Option<BorderWidth> {
        match keyword {
            "thin" => Some(BorderWidth::Length(1., Unit::Px)),
            "medium" => Some(BorderWidth::Length(3., Unit::Px)),
            "thick" => Some(BorderWidth::Length(5., Unit::Px)),
            _ => None,
        }
    }
}
