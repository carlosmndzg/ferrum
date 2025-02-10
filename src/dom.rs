#[derive(Debug, PartialEq)]
pub(crate) struct Node {
    pub(crate) children: Vec<Node>,
    pub(crate) node_type: NodeType,
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

#[derive(Debug, PartialEq)]
pub(crate) struct DocType {
    pub(crate) name: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Comment {
    pub(crate) text: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Text {
    pub(crate) text: String,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Element {
    pub(crate) tag_name: String,
    pub(crate) attributes: Vec<Attribute>,
}

pub(crate) struct NodeFactory;

impl NodeFactory {
    pub(crate) fn create_document(children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Document(Document {}),
        }
    }

    pub(crate) fn create_doctype(name: String) -> Node {
        Node {
            children: vec![],
            node_type: NodeType::DocType(DocType { name }),
        }
    }

    pub(crate) fn create_comment(text: String) -> Node {
        Node {
            children: vec![],
            node_type: NodeType::Comment(Comment { text }),
        }
    }

    pub(crate) fn create_text(text: String) -> Node {
        Node {
            children: vec![],
            node_type: NodeType::Text(Text { text }),
        }
    }

    pub(crate) fn create_element(
        tag_name: String,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
    ) -> Node {
        Node {
            children,
            node_type: NodeType::Element(Element {
                tag_name,
                attributes,
            }),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl Node {
    pub(crate) fn find_first_node<'a>(&'a self, f: &impl Fn(&'a Node) -> bool) -> Option<&'a Node> {
        if f(self) {
            return Some(self);
        }

        for child in &self.children {
            if let Some(n) = child.find_first_node(f) {
                return Some(n);
            }
        }

        None
    }

    pub(crate) fn is_only_whitespace(&self) -> bool {
        match &self.node_type {
            NodeType::Text(t) => t.text.trim().is_empty(),
            _ => false,
        }
    }
}

impl Element {
    pub(crate) fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|a| a.name == name)
            .map(|a| a.value.as_str())
    }
}
