use tree_builder::LayoutTreeBuilder;
use types::LayoutNode;

use crate::style::types::StyledNode;

mod tree_builder;
mod types;

pub fn build_layout_tree(style_tree: &StyledNode, dimensions: (f32, f32)) -> LayoutNode {
    let mut layout_tree_builder = LayoutTreeBuilder::new(dimensions);

    layout_tree_builder.build(style_tree)
}
