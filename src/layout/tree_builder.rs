use crate::style::types::StyledNode;

use super::types::{BoxDimensions, BoxType, EdgeSizes, LayoutNode, Rectangle};

pub(crate) struct LayoutTreeBuilder {
    dimensions: (f32, f32),
}

impl LayoutTreeBuilder {
    pub fn new(dimensions: (f32, f32)) -> Self {
        LayoutTreeBuilder { dimensions }
    }

    pub fn build<'a>(&mut self, root: &'a StyledNode) -> LayoutNode<'a> {
        let mut icb = self.build_icb();

        icb.children.push(self.build_layout_node(root));

        icb
    }

    fn build_layout_node<'a>(&self, node: &'a StyledNode) -> LayoutNode<'a> {
        let node_children = &self.get_children_that_are_displayed(node);
        let formatting_context = self.determine_formatting_context(node_children);

        if node.is_inline_level() && formatting_context == FormattingContext::Block {
            panic!("Inline-level node with block-level children is not supported!");
        }

        let mut ans = LayoutNode::default();

        if node.is_inline_level() {
            ans.box_type = BoxType::Inline(node);
        } else {
            ans.box_type = BoxType::Block(node);
        }

        for child in node_children {
            if child.is_block_level() {
                ans.children.push(self.build_layout_node(child));
            }

            if child.is_inline_level() {
                if formatting_context == FormattingContext::Inline {
                    ans.children.push(self.build_layout_node(child));
                } else {
                    // Remove formatting text nodes
                    if child.node.is_only_whitespace() {
                        continue;
                    }

                    if !ans.children.is_empty()
                        && ans.children.last().unwrap().box_type == BoxType::Anonymous
                    {
                        ans.children
                            .last_mut()
                            .unwrap()
                            .children
                            .push(self.build_layout_node(child));
                    } else {
                        let mut anonymous = LayoutNode {
                            box_type: BoxType::Anonymous,
                            ..Default::default()
                        };

                        anonymous.children.push(self.build_layout_node(child));
                        ans.children.push(anonymous);
                    }
                }
            }
        }

        ans
    }

    fn get_children_that_are_displayed<'a>(
        &self,
        node: &'a StyledNode<'a>,
    ) -> Vec<&'a StyledNode<'a>> {
        node.children
            .iter()
            .filter(|child| !child.has_display_none())
            .collect()
    }

    fn determine_formatting_context(&self, children: &[&StyledNode]) -> FormattingContext {
        if children.iter().any(|child| child.is_block_level()) {
            FormattingContext::Block
        } else {
            FormattingContext::Inline
        }
    }

    fn build_icb<'a>(&self) -> LayoutNode<'a> {
        LayoutNode {
            box_dimensions: BoxDimensions {
                content: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: self.dimensions.0,
                    height: self.dimensions.1,
                },
                padding: EdgeSizes::default(),
                border: EdgeSizes::default(),
                margin: EdgeSizes::default(),
            },
            box_type: Default::default(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum FormattingContext {
    Block,
    Inline,
}
