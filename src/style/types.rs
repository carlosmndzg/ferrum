use core::fmt;
use std::collections::HashMap;

use crate::{style::properties::Property, Element, Node, NodeType};

pub(crate) struct StyledNode<'a> {
    pub(crate) node: &'a Node,
    pub(crate) styles: Styles,
    pub(crate) children: Vec<StyledNode<'a>>,
}

impl fmt::Debug for StyledNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StyledNode")
            .field("node", &self.node_representation())
            .field("styles", &self.styles)
            .field("children", &self.children)
            .finish()
    }
}

impl StyledNode<'_> {
    fn node_representation(&self) -> String {
        self.node_type_summary(&self.node.node_type)
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
    properties: HashMap<String, Property>,
}

impl Styles {
    pub(crate) fn add_property(&mut self, property: Property) {
        let name = property.name().to_string();

        if self.properties.contains_key(&name) {
            return;
        }

        self.properties.insert(name, property);
    }

    pub(crate) fn get_property_clone(&self, name: &str) -> Option<Property> {
        if !self.properties.contains_key(name) {
            return None;
        }

        Some(self.properties.get(name).cloned().unwrap())
    }

    pub(crate) fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Unit {
    Px,
}
