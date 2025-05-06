use crate::{
    css::types::Value,
    layout::types::{BoxType, LayoutNode},
    style::validations::Validations,
};

use super::{CssProperty, Property};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextAlign {
    value: Value,
}

impl TextAlign {
    pub(super) fn new() -> Self {
        TextAlign {
            value: Value::default(),
        }
    }

    pub(crate) fn apply(&self, node: &mut LayoutNode) {
        let Value::Keyword(keyword) = &self.value else {
            return;
        };

        match keyword.as_str() {
            "center" => self.apply_text_align_center(node),
            "right" => self.apply_text_align_right(node),
            "justify" => self.apply_text_align_justify(node),
            _ => {}
        }
    }

    fn apply_text_align_center(&self, node: &mut LayoutNode) {
        for line in &mut node.children {
            let line_width = line.box_dimensions.content.width;
            let remaining_space = node.box_dimensions.content.width - line_width;

            for word in &mut line.children {
                word.box_dimensions.content.x += remaining_space / 2.0;
            }
        }
    }

    fn apply_text_align_right(&self, node: &mut LayoutNode) {
        for line in &mut node.children {
            let line_width = line.box_dimensions.content.width;
            let remaining_space = node.box_dimensions.content.width - line_width;

            for word in &mut line.children {
                word.box_dimensions.content.x += remaining_space;
            }
        }
    }

    fn apply_text_align_justify(&self, node: &mut LayoutNode) {
        let length = node.children.len();
        let lines_except_last = &mut node.children[..length - 1];

        for line in lines_except_last {
            let line_width = line.box_dimensions.content.width;
            let remaining_space =
                node.box_dimensions.content.width + self.count_whitespace_length(line) - line_width;

            line.children.retain(|word| {
                if let BoxType::Word(word) = &word.box_type {
                    word.text != " "
                } else {
                    false
                }
            });

            if line.children.len() <= 1 {
                continue;
            }

            let space = remaining_space / (line.children.len() - 1) as f32;
            let mut acc_x = 0.;

            for word in &mut line.children {
                word.box_dimensions.content.x = line.box_dimensions.content.x + acc_x;
                acc_x += word.box_dimensions.content.width + space;
            }
        }
    }

    fn count_whitespace_length(&self, line: &LayoutNode) -> f32 {
        let mut count = 0.0;

        for word in &line.children {
            if let BoxType::Word(word_box) = &word.box_type {
                if word_box.text == " " {
                    count += word.box_dimensions.content.width;
                }
            }
        }

        count
    }
}

impl CssProperty for TextAlign {
    fn name(&self) -> &'static str {
        "text-align"
    }

    fn is_inheritable(&self) -> bool {
        true
    }

    fn is_shorthand(&self) -> bool {
        false
    }

    fn initial_value(&self) -> Vec<Property> {
        vec![Property::TextAlign(TextAlign {
            value: Value::Keyword("left".to_string()),
        })]
    }

    fn maybe_new(&self, value: &[Value]) -> Vec<Property> {
        if value.len() != 1 {
            return Vec::new();
        }

        let value = value.first().unwrap();

        if Validations::keyword(value, &["left", "center", "right", "justify"]) {
            return vec![Property::TextAlign(TextAlign {
                value: value.clone(),
            })];
        }

        Vec::new()
    }
}
