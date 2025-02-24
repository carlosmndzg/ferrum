use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) node_type: NodeType,
    pub(crate) children: Vec<Node>,
}

impl Node {
    pub(crate) fn new(node_type: NodeType, children: Vec<Node>) -> Self {
        Self {
            node_type,
            children,
        }
    }

    pub(crate) fn find_first_node(&self, predicate: &dyn Fn(&Node) -> bool) -> Option<&Node> {
        if predicate(self) {
            return Some(self);
        }

        for child in &self.children {
            if let Some(node) = child.find_first_node(predicate) {
                return Some(node);
            }
        }

        None
    }

    pub(crate) fn is_only_whitespace(&self) -> bool {
        if let NodeType::Text(t) = &self.node_type {
            return t.get().trim().is_empty();
        }

        false
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum NodeType {
    Document(Document),
    DocType(DocType),
    Comment(Comment),
    Text(Text),
    Element(Element),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Document;

impl Document {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct DocType {
    name: String,
}

impl DocType {
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Comment {
    text: String,
}

impl Comment {
    pub(crate) fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Text {
    text: String,
}

impl Text {
    pub(crate) fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub(crate) fn get(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Element {
    tag_name: String,
    attributes: Attributes,
}

impl Element {
    pub(crate) fn new(tag_name: impl Into<String>, attributes: Attributes) -> Self {
        Self {
            tag_name: tag_name.into(),
            attributes,
        }
    }

    pub(crate) fn tag_name(&self) -> &str {
        &self.tag_name
    }

    pub(crate) fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Attributes {
    attrs: HashMap<String, String>,
}

impl Attributes {
    pub(crate) fn get(&self, name: &str) -> Option<&String> {
        self.attrs.get(name)
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for Attributes {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let mut attrs = HashMap::new();

        for (name, value) in iter {
            attrs.insert(name.to_string(), value.to_string());
        }

        Self { attrs }
    }
}
