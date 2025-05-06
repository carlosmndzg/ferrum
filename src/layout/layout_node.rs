use std::mem;
use std::path::Path;

use crate::css::types::Rgb;
use crate::layout::box_types::{anonymous::Anonymous, block::Block, line::Line, word::Word};
use crate::layout::formatting_context::FormattingContext;
use crate::layout::{box_dimensions::BoxDimensions, box_types::BoxType};
use crate::style::types::StyledNode;

pub(crate) struct LayoutNodeFactory;

impl LayoutNodeFactory {
    pub(crate) fn line_box<'a>() -> LayoutNode<'a> {
        LayoutNode {
            box_type: BoxType::Line(Line),
            ..Default::default()
        }
    }

    pub(crate) fn word_box<'a>(
        text: String,
        font_size: f32,
        line_height: f32,
        font_weight: u32,
        color: Rgb,
    ) -> LayoutNode<'a> {
        LayoutNode {
            box_type: BoxType::Word(Word {
                text,
                font_size,
                line_height,
                font_weight,
                color,
            }),
            ..Default::default()
        }
    }

    pub(crate) fn anonymous_box<'a>() -> LayoutNode<'a> {
        LayoutNode {
            box_type: BoxType::Anonymous(Anonymous),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub(crate) struct LayoutNode<'a> {
    pub(crate) box_dimensions: BoxDimensions,
    pub(crate) box_type: BoxType<'a>,
    pub(crate) children: Vec<LayoutNode<'a>>,
}

impl<'a> LayoutNode<'a> {
    pub(crate) fn compute_layout(
        &mut self,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        let box_type = mem::replace(&mut self.box_type, BoxType::Temporal);
        box_type.compute_layout(self, containing_block, desired_height, file_path);
        self.box_type = box_type;
    }

    pub(crate) fn compute_desired_height(&self, parent_desired_height: Option<f32>) -> Option<f32> {
        if let BoxType::Block(Block { node, .. }, ..) = &self.box_type {
            let height = node.height();

            if height.is_auto() || (height.is_percentage() && parent_desired_height.is_none()) {
                None
            } else {
                Some(height.actual_value(parent_desired_height.unwrap_or(0.0)))
            }
        } else {
            None
        }
    }

    pub(crate) fn is_replaced_element(&self) -> bool {
        if let BoxType::Block(Block { node, .. }) = self.box_type {
            node.is_replaced_element()
        } else {
            false
        }
    }

    fn handle_block_level_child(&mut self, child: &'a StyledNode<'a>) {
        self.children.push(child.into());
    }

    fn handle_inline_level_child(
        &mut self,
        child: &'a StyledNode<'a>,
        formatting_context: &FormattingContext,
    ) {
        let is_inline_formatting_context = formatting_context == &FormattingContext::Inline;

        if is_inline_formatting_context {
            self.children.push(child.into());

            return;
        }

        let is_whitespace_child = child.node.is_only_whitespace();

        if is_whitespace_child {
            return;
        }

        let last_child = self.children.last_mut();

        if let Some(last_child) = last_child {
            let last_child_is_anonymous = matches!(&last_child.box_type, BoxType::Anonymous(_));

            if last_child_is_anonymous {
                last_child.children.push(child.into());
            }
        } else {
            let mut anonymous = LayoutNodeFactory::anonymous_box();

            anonymous.children.push(child.into());

            self.children.push(anonymous);
        }
    }
}

impl<'a> From<&'a StyledNode<'a>> for LayoutNode<'a> {
    fn from(node: &'a StyledNode) -> LayoutNode<'a> {
        let node_children = node.children_displayed();
        let has_children = !node_children.is_empty();
        let formatting_context = node.formatting_context();
        let has_block_formatting_context = formatting_context == FormattingContext::Block;

        if node.is_inline_level() && has_children && has_block_formatting_context {
            panic!("Inline-level node with block-level children is not supported!");
        }

        let mut ans = LayoutNode::default();

        for child in node_children {
            if child.is_block_level() {
                ans.handle_block_level_child(child);
            }

            if child.is_inline_level() {
                ans.handle_inline_level_child(child, &formatting_context);
            }
        }

        ans.box_type = node.box_type(formatting_context);

        ans
    }
}
