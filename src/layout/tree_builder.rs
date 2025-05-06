use std::path::Path;

use crate::layout::layout_node::{LayoutNode, LayoutNodeFactory};
use crate::style::types::StyledNode;

pub(crate) struct LayoutTreeBuilder<'a> {
    dimensions: (usize, usize),
    file_path: &'a Path,
}

impl LayoutTreeBuilder<'_> {
    pub(crate) fn new(dimensions: (usize, usize), file_path: &Path) -> LayoutTreeBuilder {
        LayoutTreeBuilder {
            dimensions,
            file_path,
        }
    }

    pub(crate) fn build<'a>(&mut self, root: &'a StyledNode) -> LayoutNode<'a> {
        let mut icb = self.build_icb(root.into());

        self.compute_boxes(&mut icb);

        icb
    }

    fn build_icb<'a>(&self, root: LayoutNode<'a>) -> LayoutNode<'a> {
        let mut icb = LayoutNodeFactory::anonymous_box();

        icb.children.push(root);

        icb
    }

    fn compute_boxes(&self, icb: &mut LayoutNode) {
        let (window_width, window_height) = self.window_size();

        icb.box_dimensions.content.width = window_width;

        let containing_block = &icb.box_dimensions;
        let child = icb.children.get_mut(0).unwrap();
        let parent_desired_height = Some(window_height);
        let child_desired_height = child.compute_desired_height(parent_desired_height);

        child.compute_layout(containing_block, child_desired_height, self.file_path);

        child.box_dimensions.content.height = window_height;
        icb.box_dimensions.content.height = window_height;
    }

    fn window_size(&self) -> (f32, f32) {
        (self.dimensions.0 as f32, self.dimensions.1 as f32)
    }
}
