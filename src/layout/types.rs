use core::{fmt, panic};

use crate::{style::types::StyledNode, NodeType};

#[derive(Default)]
#[allow(unused)]
pub(crate) struct LayoutNode<'a> {
    pub(crate) box_dimensions: BoxDimensions,
    pub(crate) box_type: BoxType<'a>,
    pub(crate) children: Vec<LayoutNode<'a>>,
}

impl fmt::Debug for LayoutNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayoutNode")
            .field("box_type", &self.box_type.box_representation())
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct BoxDimensions {
    pub(crate) content: Rectangle,
    pub(crate) padding: EdgeSizes,
    pub(crate) border: EdgeSizes,
    pub(crate) margin: EdgeSizes,
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Debug, Default)]
#[allow(unused)]
pub(crate) struct EdgeSizes {
    pub(crate) top: f32,
    pub(crate) left: f32,
    pub(crate) bottom: f32,
    pub(crate) right: f32,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) enum BoxType<'a> {
    Block(&'a StyledNode<'a>),
    Inline(&'a StyledNode<'a>),
    #[default]
    Anonymous,
}

impl BoxType<'_> {
    pub(crate) fn box_representation(&self) -> String {
        match self {
            BoxType::Block(node) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Block | {}", e.tag_name);
                }

                panic!("Node type not supported");
            }
            BoxType::Inline(node) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Inline element | {}", e.tag_name);
                }

                if let NodeType::Text(t) = node_type {
                    return format!("Inline text | {}", t.text);
                }

                panic!("Node type not supported");
            }
            BoxType::Anonymous => "Anonymous".to_string(),
        }
    }
}
