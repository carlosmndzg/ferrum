use std::path::Path;

use crate::{
    layout::{
        formatting_context::FormattingContext,
        types::{BoxDimensions, LayoutNode},
    },
    style::types::StyledNode,
    NodeType,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Block<'a> {
    pub(crate) node: &'a StyledNode<'a>,
    pub(crate) formatting_context: FormattingContext,
}

impl Block<'_> {
    pub(crate) fn compute_layout(
        &self,
        node: &mut LayoutNode,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        self.compute_width(node, containing_block, desired_height, file_path);
        self.compute_position(node, containing_block);
        self.compute_height(node, desired_height, file_path);
    }

    fn compute_width(
        &self,
        node: &mut LayoutNode,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        let mut is_width_auto = self.node.width().is_auto();
        let is_margin_left_auto = self.node.margin_left().is_auto();
        let is_margin_right_auto = self.node.margin_right().is_auto();

        let padding_left = self
            .node
            .padding_left()
            .actual_value(containing_block.content.width);
        let padding_right = self
            .node
            .padding_right()
            .actual_value(containing_block.content.width);
        let border_left = self
            .node
            .border_width()
            .actual_value(self.node.border_style());
        let border_right = self
            .node
            .border_width()
            .actual_value(self.node.border_style());
        let mut width = self
            .node
            .width()
            .actual_value(containing_block.content.width);
        let mut margin_left = self
            .node
            .margin_left()
            .actual_value(containing_block.content.width);
        let mut margin_right = self
            .node
            .margin_right()
            .actual_value(containing_block.content.width);

        if self.node.is_replaced_element() {
            is_width_auto = false;

            width = self.compute_width_replace_element(containing_block, desired_height, file_path);
        }

        let border_box_size = width + padding_left + padding_right + border_left + border_right;

        match (is_width_auto, is_margin_left_auto, is_margin_right_auto) {
            (false, true, true) | (false, true, false) | (false, false, true)
                if border_box_size > containing_block.content.width => {}
            (false, false, false) => {
                margin_right = containing_block.content.width
                    - width
                    - margin_left
                    - padding_left
                    - padding_right
                    - border_left
                    - border_right;
            }
            (true, _, _) => {
                width = containing_block.content.width
                    - margin_left
                    - margin_right
                    - padding_left
                    - padding_right
                    - border_left
                    - border_right;
            }
            (false, true, false) => {
                margin_left = containing_block.content.width
                    - width
                    - margin_right
                    - padding_left
                    - padding_right
                    - border_left
                    - border_right;
            }
            (false, false, true) => {
                margin_right = containing_block.content.width
                    - width
                    - margin_left
                    - padding_left
                    - padding_right
                    - border_left
                    - border_right;
            }
            (false, true, true) => {
                margin_left = (containing_block.content.width - border_box_size) / 2.0;
                margin_right = margin_left;
            }
        }

        node.box_dimensions.content.width = width;
        node.box_dimensions.padding.left = padding_left;
        node.box_dimensions.padding.right = padding_right;
        node.box_dimensions.border.left = border_left;
        node.box_dimensions.border.right = border_right;
        node.box_dimensions.margin.left = margin_left;
        node.box_dimensions.margin.right = margin_right;
    }

    fn compute_width_replace_element(
        &self,
        containing_block: &BoxDimensions,
        desired_height: Option<f32>,
        file_path: &Path,
    ) -> f32 {
        let is_width_auto = self.node.width().is_auto();
        let is_height_auto = self.node.height().is_auto();
        let declared_width = self
            .node
            .width()
            .actual_value(containing_block.content.width);

        let (intrinsic_width, intrinsic_height) =
            self.intrinsic_image_dimensions(self.node, file_path);
        let intrinsic_ratio = intrinsic_width / intrinsic_height;

        if is_width_auto && is_height_auto {
            intrinsic_width
        } else if is_width_auto {
            desired_height.unwrap_or(0.) * intrinsic_ratio
        } else {
            declared_width
        }
    }

    fn intrinsic_image_dimensions(&self, node: &StyledNode, document_path: &Path) -> (f32, f32) {
        let NodeType::Element(element) = &node.node.node_type else {
            panic!("Node is not an element");
        };

        if element.tag_name != "img" {
            panic!("Node is not an image");
        }

        let Some(src) = element.get_attribute("src") else {
            return (0.0, 0.0);
        };

        let Some(folder) = document_path.parent() else {
            return (0.0, 0.0);
        };

        let path = folder.join(src);
        let image = image::open(path).unwrap();

        (image.width() as f32, image.height() as f32)
    }

    fn compute_position(&self, node: &mut LayoutNode, containing_block: &BoxDimensions) {
        let margin_top = self
            .node
            .margin_top()
            .actual_value(containing_block.content.width);
        let margin_bottom = self
            .node
            .margin_bottom()
            .actual_value(containing_block.content.width);
        let padding_top = self
            .node
            .padding_top()
            .actual_value(containing_block.content.width);
        let padding_bottom = self
            .node
            .padding_bottom()
            .actual_value(containing_block.content.width);
        let border_top = self
            .node
            .border_width()
            .actual_value(self.node.border_style());
        let border_bottom = self
            .node
            .border_width()
            .actual_value(self.node.border_style());

        node.box_dimensions.margin.top = margin_top;
        node.box_dimensions.margin.bottom = margin_bottom;
        node.box_dimensions.padding.top = padding_top;
        node.box_dimensions.padding.bottom = padding_bottom;
        node.box_dimensions.border.top = border_top;
        node.box_dimensions.border.bottom = border_bottom;

        node.box_dimensions.content.x = containing_block.content.x
            + node.box_dimensions.margin.left
            + node.box_dimensions.padding.left
            + node.box_dimensions.border.left;

        node.box_dimensions.content.y = containing_block.content.y
            + containing_block.content.height
            + node.box_dimensions.margin.top
            + node.box_dimensions.padding.top
            + node.box_dimensions.border.top;
    }

    fn compute_height(&self, node: &mut LayoutNode, desired_height: Option<f32>, file_path: &Path) {
        if self.node.is_replaced_element() {
            self.compute_height_replaced_element(node, desired_height, file_path);
        } else {
            self.formatting_context.handle(
                node,
                &self.node.text_align().value(),
                desired_height,
                file_path,
            );
        }
    }

    fn compute_height_replaced_element(
        &self,
        node: &mut LayoutNode,
        desired_height: Option<f32>,
        file_path: &Path,
    ) {
        let styled_node = self.node;

        if let Some(desired_height) = desired_height {
            node.box_dimensions.content.height = desired_height;
        } else {
            let (intrinsic_width, intrinsic_height) =
                self.intrinsic_image_dimensions(styled_node, file_path);
            let intrinsic_ratio = intrinsic_width / intrinsic_height;

            node.box_dimensions.content.height =
                node.box_dimensions.content.width / intrinsic_ratio;
        }
    }
}
