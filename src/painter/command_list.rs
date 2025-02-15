use crate::layout::types::{BoxType, LayoutNode};

use super::commands::{draw_rectangle::DrawRectangle, draw_text::DrawText, Command};

pub(crate) struct CommandList {
    pub(crate) commands: Vec<Box<dyn Command>>,
}

impl CommandList {
    pub(crate) fn new(root: &LayoutNode) -> Self {
        let mut instance = Self {
            commands: Vec::new(),
        };

        instance.build_commands(root);

        instance
    }

    pub(crate) fn build_commands(&mut self, node: &LayoutNode) {
        self.build_commands_for_background(node);

        if let BoxType::Word { .. } = node.box_type {
            self.build_commands_for_text(node);
        } else {
            for child in &node.children {
                self.build_commands(child);
            }
        }
    }

    pub(crate) fn build_commands_for_background(&mut self, node: &LayoutNode) {
        if let BoxType::Block(styled_node, ..) | BoxType::Inline(styled_node) = node.box_type {
            let background_color = styled_node.background_color().value();
            let padding_box = node.box_dimensions.padding_box();

            self.commands.push(Box::new(DrawRectangle::new(
                padding_box.x,
                padding_box.y,
                padding_box.width,
                padding_box.height,
                background_color.clone(),
            )));
        }
    }

    pub(crate) fn build_commands_for_text(&mut self, node: &LayoutNode) {
        let BoxType::Word {
            text,
            font_size,
            color,
            ..
        } = &node.box_type
        else {
            return;
        };

        self.commands.push(Box::new(DrawText::new(
            node.box_dimensions.content.x,
            node.box_dimensions.content.y,
            text.clone(),
            *font_size,
            color.clone(),
        )));
    }
}
