use std::path::Path;

use crate::layout::layout_node::LayoutNode;
use crate::layout::tree_builder::LayoutTreeBuilder;
use crate::style::types::StyledNode;

pub(crate) mod box_dimensions;
pub(crate) mod box_types;
pub(crate) mod formatting_context;
pub(crate) mod layout_node;
pub(crate) mod tree_builder;

pub fn build_layout_tree<'a>(
    style_tree: &'a StyledNode,
    file_path: &Path,
    dimensions: (usize, usize),
) -> LayoutNode<'a> {
    let mut layout_tree_builder = LayoutTreeBuilder::new(dimensions, file_path);

    let root = layout_tree_builder.build(style_tree);

    root
}
