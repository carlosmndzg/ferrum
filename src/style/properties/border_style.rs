use crate::css::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BorderStyle {
    None,
    Hidden,
    Solid,
}

impl BorderStyle {
    pub(crate) fn maybe_new(value: &Value) -> Option<BorderStyle> {
        if let Value::Keyword(keyword) = value {
            match keyword.as_str() {
                "none" => return Some(BorderStyle::None),
                "hidden" => return Some(BorderStyle::Hidden),
                "solid" => return Some(BorderStyle::Solid),
                _ => {}
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "border-style"
    }

    pub(crate) fn default() -> BorderStyle {
        BorderStyle::None
    }

    pub(crate) fn value(&self) -> BorderStyle {
        self.clone()
    }
}
