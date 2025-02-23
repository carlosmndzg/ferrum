use crate::{css::types::Value, layout::types::LayoutNode};

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

    pub(crate) fn apply(&self, node: &mut LayoutNode) {
        for line in &mut node.children {
            let line_width = line.box_dimensions.content.width;
            let remaining_space = node.box_dimensions.content.width - line_width;

            for word in &mut line.children {
                match self {
                    TextAlign::Center => {
                        word.box_dimensions.content.x += remaining_space / 2.0;
                    }
                    TextAlign::Right => {
                        word.box_dimensions.content.x += remaining_space;
                    }
                    _ => {}
                }
            }
        }
    }
}
