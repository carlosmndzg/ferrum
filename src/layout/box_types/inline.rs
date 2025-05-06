use crate::style::types::StyledNode;

#[derive(PartialEq)]
pub(crate) struct Inline<'a> {
    pub(crate) node: &'a StyledNode<'a>,
}
