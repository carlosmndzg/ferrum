use crate::style::types::StyledNode;

use super::types::{BoxDimensions, EdgeSizes, LayoutNode, Rectangle};

pub(crate) struct LayoutTreeBuilder {
    dimensions: (f32, f32),
}

impl LayoutTreeBuilder {
    pub fn new(dimensions: (f32, f32)) -> Self {
        LayoutTreeBuilder { dimensions }
    }

    pub fn build(&mut self, _style_node: &StyledNode) -> LayoutNode {
        println!("Building layout tree...");

        LayoutNode {
            box_dimensions: BoxDimensions {
                content: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: self.dimensions.0,
                    height: self.dimensions.1,
                },
                padding: EdgeSizes {
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                },
                border: EdgeSizes {
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                },
                margin: EdgeSizes {
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0,
                },
            },
            children: vec![],
        }
    }
}
