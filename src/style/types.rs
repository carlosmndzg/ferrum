use core::fmt;
use std::collections::HashSet;

use crate::{style::properties::Property, Element, Node, NodeType};

pub(crate) struct StyledNode<'a> {
    pub(crate) node: &'a Node,
    pub(crate) parent_node: Option<&'a Node>,
    pub(crate) styles: Styles,
    pub(crate) children: Vec<StyledNode<'a>>,
}

impl fmt::Debug for StyledNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledNode")
            .field("node", &self.node_representation())
            .field("parent_node", &self.parent_representation())
            .field("styles", &self.styles)
            .field("children", &self.children)
            .finish()
    }
}

impl StyledNode<'_> {
    fn node_representation(&self) -> String {
        self.node_type_summary(&self.node.node_type)
    }

    fn parent_representation(&self) -> String {
        if let Some(parent) = self.parent_node {
            self.node_type_summary(&parent.node_type)
        } else {
            "null".to_string()
        }
    }

    fn node_type_summary(&self, node_type: &NodeType) -> String {
        match node_type {
            NodeType::Element(Element { tag_name, .. }) => tag_name.clone(),
            NodeType::Text(text) => text.text.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Styles {
    properties: Vec<Box<dyn Property>>,
    styles_added: HashSet<String>,
}

impl Styles {
    pub(crate) fn add_property(&mut self, property: Box<dyn Property>) {
        let name = property.name().to_string();

        if self.styles_added.contains(&name) {
            return;
        }

        self.properties.push(property);
        self.styles_added.insert(name);
    }
}
