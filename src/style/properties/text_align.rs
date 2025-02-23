use crate::{
    css::types::Value,
    layout::types::{BoxType, LayoutNode},
};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

impl TextAlign {
    pub(crate) fn maybe_new(value: &Value) -> Option<TextAlign> {
        if let Value::Keyword(keyword) = value {
            match keyword.as_str() {
                "left" => return Some(TextAlign::Left),
                "right" => return Some(TextAlign::Right),
                "center" => return Some(TextAlign::Center),
                "justify" => return Some(TextAlign::Justify),
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
        match self {
            TextAlign::Left => {}
            TextAlign::Center => self.apply_text_align_center(node),
            TextAlign::Right => self.apply_text_align_right(node),
            TextAlign::Justify => self.apply_text_align_justify(node),
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
