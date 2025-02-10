use tree_builder::LayoutTreeBuilder;
use types::LayoutNode;

use crate::style::types::StyledNode;

mod tree_builder;
pub(crate) mod types;

pub fn build_layout_tree<'a>(style_tree: &'a StyledNode, dimensions: (f32, f32)) -> LayoutNode<'a> {
    let mut layout_tree_builder = LayoutTreeBuilder::new(dimensions);

    let root = layout_tree_builder.build(style_tree);

    root
}
