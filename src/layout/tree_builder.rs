use std::path::Path;

use crate::style::types::StyledNode;

use super::types::{BoxDimensions, EdgeSizes, LayoutNode, Rectangle};

pub(crate) struct LayoutTreeBuilder<'a> {
    dimensions: (usize, usize),
    file_path: &'a Path,
}

impl LayoutTreeBuilder<'_> {
    pub fn new(dimensions: (usize, usize), file_path: &Path) -> LayoutTreeBuilder {
        LayoutTreeBuilder {
            dimensions,
            file_path,
        }
    }

    pub fn build<'a>(&mut self, root: &'a StyledNode) -> LayoutNode<'a> {
        let mut icb = self.build_icb();

        icb.children.push(root.into());

        icb.box_dimensions.content.width = self.dimensions.0 as f32;

        let containing_block = &icb.box_dimensions;
        let child = icb.children.get_mut(0).unwrap();
        let child_desired_height =
            child.compute_desired_height(Option::Some(self.dimensions.1 as f32));

        child.compute_layout(containing_block, child_desired_height, self.file_path);

        icb.box_dimensions.content.height = self.dimensions.1 as f32;
        child.box_dimensions.content.height = self.dimensions.1 as f32;

        icb
    }

    fn build_icb<'a>(&self) -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions {
                content: Rectangle::default(),
                padding: EdgeSizes::default(),
                border: EdgeSizes::default(),
                margin: EdgeSizes::default(),
            },
            box_type: Default::default(),
            children: Vec::new(),
        }
    }
}
