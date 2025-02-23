use std::path::Path;

use tree_builder::LayoutTreeBuilder;
use types::LayoutNode;

use crate::style::types::StyledNode;

mod tree_builder;
pub(crate) mod types;

pub(crate) mod box_types {
    pub(crate) mod anonymous;
    pub(crate) mod block;
    pub(crate) mod inline;
    pub(crate) mod line;
    pub(crate) mod word;
}

pub(crate) mod formatting_context;

pub fn build_layout_tree<'a>(
    style_tree: &'a StyledNode,
    file_path: &Path,
    dimensions: (f32, f32),
) -> LayoutNode<'a> {
    let mut layout_tree_builder = LayoutTreeBuilder::new(dimensions, file_path);

    let root = layout_tree_builder.build(style_tree);

    root
}
