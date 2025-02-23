use crate::layout::{
    box_types::{block::Block, inline::Inline, word::Word},
    types::{BoxType, LayoutNode},
};

use super::{
    commands::{
        draw_border::DrawBorder, draw_rectangle::DrawRectangle, draw_text::DrawText, Command,
    },
    fonts_context::FontsContext,
};

pub(crate) struct CommandList {
    pub(crate) commands: Vec<Box<dyn Command>>,
}

impl CommandList {
    pub(crate) fn new(root: &LayoutNode, fonts_ctx: &mut FontsContext) -> Self {
        let mut instance = Self {
            commands: Vec::new(),
        };

        instance.build_commands(root, fonts_ctx);

        instance
    }

    pub(crate) fn build_commands(&mut self, node: &LayoutNode, fonts_ctx: &mut FontsContext) {
        self.build_commands_for_background(node);
        self.build_commands_for_border(node);

        if let BoxType::Word { .. } = node.box_type {
            self.build_commands_for_text(node, fonts_ctx);
        } else {
            for child in &node.children {
                self.build_commands(child, fonts_ctx);
            }
        }
    }

    pub(crate) fn build_commands_for_border(&mut self, node: &LayoutNode) {
        if let BoxType::Block(
            Block {
                node: styled_node, ..
            },
            ..,
        )
        | BoxType::Inline(Inline { node: styled_node }) = node.box_type
        {
            let color = &styled_node.color().value();
            let border_color = styled_node.border_color().actual_value(color);
            let border_box = node.box_dimensions.border_box();
            let border_width = styled_node
                .border_width()
                .actual_value(&styled_node.border_style().value());

            self.commands.push(Box::new(DrawBorder::new(
                border_box.x,
                border_box.y,
                border_box.width,
                border_box.height,
                border_width,
                border_color.clone(),
            )));
        }
    }

    pub(crate) fn build_commands_for_background(&mut self, node: &LayoutNode) {
        if let BoxType::Block(
            Block {
                node: styled_node, ..
            },
            ..,
        )
        | BoxType::Inline(Inline { node: styled_node }) = node.box_type
        {
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

    pub(crate) fn build_commands_for_text(
        &mut self,
        node: &LayoutNode,
        fonts_ctx: &mut FontsContext,
    ) {
        let BoxType::Word(Word {
            text,
            font_size,
            font_weight,
            color,
            ..
        }) = &node.box_type
        else {
            return;
        };

        fonts_ctx.add_font_if_not_exists(*font_weight);

        self.commands.push(Box::new(DrawText::new(
            node.box_dimensions.content.x,
            node.box_dimensions.content.y,
            text.clone(),
            *font_size,
            *font_weight,
            color.clone(),
        )));
    }
}
