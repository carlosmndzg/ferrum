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

fn is_style_node(node: &Node) -> bool {
    matches!(&node.node_type, NodeType::Element(Element { tag_name, .. }) if tag_name == "style")
}

pub(crate) fn find_first_style_node(node: &Node) -> Option<&Node> {
    if is_style_node(node) {
        return Some(node);
    }

    for child in &node.children {
        if let Some(n) = find_first_style_node(child) {
            return Some(n);
        }
    }

    None
}
