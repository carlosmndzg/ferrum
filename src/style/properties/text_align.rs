use crate::css::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TextAlign {
    Left,
    Right,
    Center,
}

impl TextAlign {
    pub(crate) fn maybe_new(value: &Value) -> Option<TextAlign> {
        if let Value::Keyword(keyword) = value {
            match keyword.as_str() {
                "left" => return Some(TextAlign::Left),
                "right" => return Some(TextAlign::Right),
                "center" => return Some(TextAlign::Center),
                _ => {}
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "text-align"
    }

    pub(crate) fn default() -> TextAlign {
        TextAlign::Left
    }

    pub(crate) fn value(&self) -> TextAlign {
        self.clone()
    }
}
