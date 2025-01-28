#[derive(Debug, PartialEq)]
pub(crate) enum Node {
    Document(Document),
    DocType(DocType),
    Comment(Comment),
    Text(Text),
    Element(Element),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Document {
    pub(crate) children: Vec<Node>,
}

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
    pub(crate) children: Vec<Node>,
}

pub(crate) struct NodeFactory;

impl NodeFactory {
    pub(crate) fn create_document(children: Vec<Node>) -> Node {
        Node::Document(Document { children })
    }

    pub(crate) fn create_doctype(name: String) -> Node {
        Node::DocType(DocType { name })
    }

    pub(crate) fn create_comment(text: String) -> Node {
        Node::Comment(Comment { text })
    }

    pub(crate) fn create_text(text: String) -> Node {
        Node::Text(Text { text })
    }

    pub(crate) fn create_element(
        tag_name: String,
        attributes: Vec<Attribute>,
        children: Vec<Node>,
    ) -> Node {
        Node::Element(Element {
            tag_name,
            attributes,
            children,
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) value: String,
}

pub(crate) fn find_first_style_node(node: &Node) -> Option<&Node> {
    match node {
        Node::Document(document) => {
            for child in &document.children {
                let result = find_first_style_node(child);

                if let Some(Node::Element(_)) = result {
                    return result;
                }
            }
        }
        Node::Element(element) => {
            if element.tag_name == "style" {
                return Some(node);
            }

            for child in &element.children {
                let result = find_first_style_node(child);

                if let Some(Node::Element(_)) = result {
                    return result;
                }
            }
        }
        _ => {}
    }

    None
}
