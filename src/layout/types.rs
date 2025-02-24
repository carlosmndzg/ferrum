use core::{fmt, panic};
use std::{mem, path::Path};

use crate::{
    style::{
        properties::height::Height,
        types::{Rgb, StyledNode},
    },
    NodeType,
};

use super::{
    box_types::{
        anonymous::Anonymous, block::Block, inline::Inline, line::Line, word::Word as WordBox,
    },
    formatting_context::FormattingContext,
};

#[derive(Default)]
pub(crate) struct LayoutNode<'a> {
    pub(crate) box_dimensions: BoxDimensions,
    pub(crate) box_type: BoxType<'a>,
    pub(crate) children: Vec<LayoutNode<'a>>,
}

impl fmt::Debug for LayoutNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LayoutNode")
            .field("box_type", &self.box_type.box_representation())
            .field("box_dimensions", &self.box_dimensions)
            .field("children", &self.children)
            .finish()
    }
}

impl<'a> LayoutNode<'a> {
    pub(crate) fn create_line_box() -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions::default(),
            box_type: BoxType::Line(Line),
            children: Vec::new(),
        }
    }

    pub(crate) fn create_word_box(
        text: String,
        font_size: f32,
        line_height: f32,
        font_weight: u32,
        color: Rgb,
    ) -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions::default(),
            box_type: BoxType::Word(WordBox {
                text,
                font_size,
                line_height,
                font_weight,
                color,
            }),
            children: Vec::new(),
        }
    }

    pub(crate) fn compute_layout(
        &mut self,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        let box_type = mem::replace(&mut self.box_type, BoxType::Temporal);
        box_type.compute_layout(self, containing_block, desired_height, file_path);
        self.box_type = box_type;
    }

    pub(crate) fn compute_desired_height(&self, parent_desired_height: Option<f32>) -> Option<f32> {
        if let BoxType::Block(Block { node, .. }, ..) = &self.box_type {
            let height = node.height();

            if height.is_auto() {
                None
            } else if let (Height::Percentage(_), None) = (height, parent_desired_height) {
                None
            } else {
                Some(height.actual_value(parent_desired_height.unwrap_or(0.0)))
            }
        } else {
            None
        }
    }

    pub(crate) fn is_replaced_element(&self) -> bool {
        if let BoxType::Temporal = self.box_type {
            panic!("This function can only be invoked after the layout has been computed!");
        }

        if let BoxType::Block(Block { node, .. }) = self.box_type {
            node.is_replaced_element()
        } else {
            false
        }
    }
}

impl<'a> From<&'a StyledNode<'a>> for LayoutNode<'a> {
    fn from(node: &'a StyledNode) -> LayoutNode<'a> {
        let node_children = node.children_displayed();
        let formatting_context = node.formatting_context();

        if node.is_inline_in_block_context(&formatting_context) {
            panic!("Inline-level node with block-level children is not supported!");
        }

        let mut ans = LayoutNode::default();

        for child in node_children {
            if child.is_block_level() {
                ans.children.push(child.into());
            }

            if child.is_inline_level() {
                if formatting_context == FormattingContext::Inline {
                    ans.children.push(child.into());
                } else {
                    if child.node.is_only_whitespace() {
                        continue;
                    }

                    if !ans.children.is_empty()
                        && matches!(ans.children.last().unwrap().box_type, BoxType::Anonymous(_))
                    {
                        ans.children.last_mut().unwrap().children.push(child.into());
                    } else {
                        let mut anonymous = LayoutNode {
                            box_type: BoxType::Anonymous(Anonymous),
                            ..Default::default()
                        };

                        anonymous.children.push(child.into());
                        ans.children.push(anonymous);
                    }
                }
            }
        }

        ans.box_type = node.box_type(formatting_context);

        ans
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

impl BoxDimensions {
    pub(crate) fn border_box(&self) -> Rectangle {
        let mut border_box = self.content.clone();

        border_box.x -= self.padding.left + self.border.left;
        border_box.y -= self.padding.top + self.border.top;
        border_box.width +=
            self.padding.left + self.padding.right + self.border.left + self.border.right;
        border_box.height +=
            self.padding.top + self.padding.bottom + self.border.top + self.border.bottom;

        border_box
    }

    pub(crate) fn padding_box(&self) -> Rectangle {
        let mut padding_box = self.content.clone();

        padding_box.x -= self.padding.left;
        padding_box.y -= self.padding.top;
        padding_box.width += self.padding.left + self.padding.right;
        padding_box.height += self.padding.top + self.padding.bottom;

        padding_box
    }
}

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, PartialEq)]
pub(crate) enum BoxType<'a> {
    Block(Block<'a>),
    Inline(Inline<'a>),
    Anonymous(Anonymous),
    Line(Line),
    Word(WordBox),
    Temporal,
}

impl BoxType<'_> {
    pub(crate) fn compute_layout(
        &self,
        layout_node: &mut LayoutNode,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        match self {
            BoxType::Block(block) => {
                block.compute_layout(layout_node, containing_block, desired_height, file_path)
            }
            BoxType::Anonymous(anonymous) => {
                anonymous.compute_layout(layout_node, containing_block, file_path)
            }
            _ => {}
        }
    }
}

impl Default for BoxType<'_> {
    fn default() -> Self {
        BoxType::Anonymous(Anonymous)
    }
}

impl BoxType<'_> {
    pub(crate) fn box_representation(&self) -> String {
        match self {
            BoxType::Block(Block { node, .. }) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Block | {}", e.tag_name());
                }

                panic!("Node type not supported");
            }
            BoxType::Inline(Inline { node }) => {
                let node_type = &node.node.node_type;

                if let NodeType::Element(e) = node_type {
                    return format!("Inline element | {}", e.tag_name());
                }

                if let NodeType::Text(t) = node_type {
                    return format!("Inline text | {}", t.get());
                }

                panic!("Node type not supported");
            }
            BoxType::Anonymous(_) => "Anonymous".to_string(),
            BoxType::Line(_) => "Line".to_string(),
            BoxType::Word(WordBox { text, .. }) => format!("Word | {}", text),
            _ => panic!("Temporal should not appear!!!"),
        }
    }
}
