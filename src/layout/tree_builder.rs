use crate::style::types::StyledNode;

use super::types::{BoxDimensions, BoxType, EdgeSizes, FormattingContext, LayoutNode, Rectangle};

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

        icb.box_dimensions.content.width = self.dimensions.0;

        let containing_block = &icb.box_dimensions;
        let child = icb.children.get_mut(0).unwrap();

        self.compute_layout(child, containing_block);

        icb.box_dimensions.content.height = self.dimensions.1;

        icb
    }

    fn build_layout_node<'a>(&self, node: &'a StyledNode) -> LayoutNode<'a> {
        let node_children = &self.get_children_that_are_displayed(node);
        let formatting_context = self.determine_formatting_context(node_children);

        if node.is_inline_level() && formatting_context == FormattingContext::Block {
            panic!("Inline-level node with block-level children is not supported!");
        }

        let mut ans = LayoutNode::default();

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

        if node.is_inline_level() {
            ans.box_type = BoxType::Inline(node);
        } else {
            ans.box_type = BoxType::Block(node, formatting_context);
        }

        ans
    }

    fn compute_layout(&self, node: &mut LayoutNode, containing_block: &BoxDimensions) {
        if let BoxType::Block(styled_node, ..) = &node.box_type {
            self.compute_block_layout(node, styled_node, containing_block);
        } else {
            panic!("Inline layout is not supported yet!");
        }
    }

    fn compute_block_layout(
        &self,
        node: &mut LayoutNode,
        styled_node: &StyledNode,
        containing_block: &BoxDimensions,
    ) {
        self.compute_width_block_layout(node, styled_node, containing_block);
        self.compute_position_block_layout(node, styled_node, containing_block);
        self.compute_height_block_layout(node, styled_node);
    }

    fn compute_width_block_layout(
        &self,
        node: &mut LayoutNode,
        styled_node: &StyledNode,
        containing_block: &BoxDimensions,
    ) {
        let is_width_auto = styled_node.width().is_auto();
        let is_margin_left_auto = styled_node.margin_left().is_auto();
        let is_margin_right_auto = styled_node.margin_right().is_auto();

        let padding_left = styled_node
            .padding_left()
            .actual_value(containing_block.content.width);
        let padding_right = styled_node
            .padding_right()
            .actual_value(containing_block.content.width);
        let mut width = styled_node
            .width()
            .actual_value(containing_block.content.width);
        let mut margin_left = styled_node
            .margin_left()
            .actual_value(containing_block.content.width);
        let mut margin_right = styled_node
            .margin_right()
            .actual_value(containing_block.content.width);

        let border_box_size = width + padding_left + padding_right;

        match (is_width_auto, is_margin_left_auto, is_margin_right_auto) {
            (false, true, true) | (false, true, false) | (false, false, true)
                if border_box_size > containing_block.content.width => {}
            (false, false, false) => {
                margin_right = containing_block.content.width
                    - width
                    - margin_left
                    - padding_left
                    - padding_right;
            }
            (true, _, _) => {
                width = containing_block.content.width
                    - margin_left
                    - margin_right
                    - padding_left
                    - padding_right;
            }
            (false, true, false) => {
                margin_left = containing_block.content.width
                    - width
                    - margin_right
                    - padding_left
                    - padding_right;
            }
            (false, false, true) => {
                margin_right = containing_block.content.width
                    - width
                    - margin_left
                    - padding_left
                    - padding_right;
            }
            (false, true, true) => {
                margin_left = (containing_block.content.width - border_box_size) / 2.0;
                margin_right = margin_left;
            }
        }

        node.box_dimensions.content.width = width;
        node.box_dimensions.padding.left = padding_left;
        node.box_dimensions.padding.right = padding_right;
        node.box_dimensions.margin.left = margin_left;
        node.box_dimensions.margin.right = margin_right;
    }

    fn compute_position_block_layout(
        &self,
        node: &mut LayoutNode,
        styled_node: &StyledNode,
        containing_block: &BoxDimensions,
    ) {
        let margin_top = styled_node
            .margin_top()
            .actual_value(containing_block.content.width);
        let margin_bottom = styled_node
            .margin_bottom()
            .actual_value(containing_block.content.width);
        let padding_top = styled_node
            .padding_top()
            .actual_value(containing_block.content.width);
        let padding_bottom = styled_node
            .padding_bottom()
            .actual_value(containing_block.content.width);

        node.box_dimensions.margin.top = margin_top;
        node.box_dimensions.margin.bottom = margin_bottom;
        node.box_dimensions.padding.top = padding_top;
        node.box_dimensions.padding.bottom = padding_bottom;

        node.box_dimensions.content.x = containing_block.content.x
            + node.box_dimensions.margin.left
            + node.box_dimensions.padding.left;

        node.box_dimensions.content.y = containing_block.content.y
            + containing_block.content.height
            + node.box_dimensions.margin.top
            + node.box_dimensions.padding.top;
    }

    fn compute_height_block_layout(&self, node: &mut LayoutNode, styled_node: &StyledNode) {
        let BoxType::Block(_, formatting_context) = &node.box_type else {
            panic!("Expected block box type");
        };

        if formatting_context == &FormattingContext::Inline {
            panic!("Inline formatting context is not supported yet!");
        }

        for child in &mut node.children {
            self.compute_layout(child, &node.box_dimensions);

            node.box_dimensions.content.height += child.box_dimensions.margin.top
                + child.box_dimensions.padding.top
                + child.box_dimensions.content.height
                + child.box_dimensions.margin.bottom
                + child.box_dimensions.padding.bottom
        }

        let height = styled_node.height();

        if !height.is_auto() {
            node.box_dimensions.content.height =
                height.actual_value(node.box_dimensions.content.width);
        }
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
        if children.is_empty() {
            return FormattingContext::Block;
        }

        if children.iter().any(|child| child.is_block_level()) {
            FormattingContext::Block
        } else {
            FormattingContext::Inline
        }
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
