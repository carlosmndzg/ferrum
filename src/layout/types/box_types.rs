use std::path::Path;

use crate::layout::box_types::anonymous::Anonymous;
use crate::layout::box_types::block::Block;
use crate::layout::box_types::inline::Inline;
use crate::layout::box_types::line::Line;
use crate::layout::box_types::word::Word;

use super::{BoxDimensions, LayoutNode};

#[derive(Debug, PartialEq)]
pub(crate) enum BoxType<'a> {
    Block(Block<'a>),
    Inline(Inline<'a>),
    Anonymous(Anonymous),
    Line(Line),
    Word(Word),
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
