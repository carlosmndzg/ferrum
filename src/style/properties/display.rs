use crate::css::types::Value;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Display {
    Block,
    Inline,
    None,
}

impl Display {
    pub(crate) fn maybe_new(value: &Value) -> Option<Display> {
        if let Value::Keyword(keyword) = value {
            match keyword.as_str() {
                "block" => return Some(Display::Block),
                "inline" => return Some(Display::Inline),
                "none" => return Some(Display::None),
                _ => {}
            }
        }

        None
    }

    pub(crate) fn name(&self) -> &str {
        "display"
    }

    pub(crate) fn default() -> Display {
        Display::Inline
    }
}
