use crate::style::types::StyledNode;

use super::types::{BoxDimensions, EdgeSizes, LayoutNode, Rectangle};

pub(crate) struct LayoutTreeBuilder {
    dimensions: (f32, f32),
}

impl LayoutTreeBuilder {
    pub fn new(dimensions: (f32, f32)) -> Self {
        LayoutTreeBuilder { dimensions }
    }

    pub fn build<'a>(&mut self, root: &'a StyledNode) -> LayoutNode<'a> {
        let mut icb = self.build_icb();

        icb.children.push(root.into());

        icb.box_dimensions.content.width = self.dimensions.0;

        let containing_block = &icb.box_dimensions;
        let child = icb.children.get_mut(0).unwrap();
        let child_desired_height = child.compute_desired_height(Option::Some(self.dimensions.1));

        child.compute_layout(containing_block, child_desired_height);

        icb.box_dimensions.content.height = self.dimensions.1;
        child.box_dimensions.content.height = self.dimensions.1;

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
