use std::path::Path;

use crate::{
    layout::{
        formatting_context::FormattingContext,
        types::{BoxDimensions, BoxType, LayoutNode},
    },
    style::properties::text_align::TextAlign,
};

use super::inline::Inline;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Anonymous;

impl Anonymous {
    pub(crate) fn compute_layout(
        &self,
        node: &mut LayoutNode,
        containing_block: &BoxDimensions,
        file_path: &Path,
    ) {
        self.compute_width(node, containing_block);
        self.compute_position(node, containing_block);
        self.compute_height(node, file_path);
    }

    pub(crate) fn compute_width(&self, node: &mut LayoutNode, containing_block: &BoxDimensions) {
        node.box_dimensions.content.width = containing_block.content.width;
    }

    pub(crate) fn compute_position(&self, node: &mut LayoutNode, containing_block: &BoxDimensions) {
        node.box_dimensions.content.x = containing_block.content.x;
        node.box_dimensions.content.y =
            containing_block.content.y + containing_block.content.height;
    }

    pub(crate) fn compute_height(&self, node: &mut LayoutNode, file_path: &Path) {
        let text_alignment = &self.text_alignment(node);

        FormattingContext::Inline.handle(node, text_alignment, None, file_path);
    }

    pub(crate) fn text_alignment(&self, node: &LayoutNode) -> TextAlign {
        let child = node
            .children
            .first()
            .expect("Anonymous box must have a child");

        match &child.box_type {
            BoxType::Inline(Inline { node }) => node.text_align().clone(),
            _ => panic!("Anonymous box must have an inline child"),
        }
    }
}
