use crate::css::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FontWeight {
    UnitLess(u32),
}

impl FontWeight {
    pub(crate) fn maybe_new(value: &Value) -> Option<FontWeight> {
        if let Value::Dimension(length, unit) = value {
            if unit.is_empty() && *length >= 1. && *length <= 1000. {
                return Some(FontWeight::UnitLess((*length).round() as u32));
            }
        }

        if let Value::Keyword(keyword) = value {
            if keyword == "normal" {
                return Some(FontWeight::UnitLess(400));
            }

            if keyword == "bold" {
                return Some(FontWeight::UnitLess(700));
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "font-weight"
    }

    pub(crate) fn default() -> FontWeight {
        FontWeight::UnitLess(400)
    }

    pub(crate) fn value(&self) -> u32 {
        match self {
            FontWeight::UnitLess(value) => *value,
        }
    }
}
