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

// TODO Remove this function, only used for debugging
pub(crate) fn print_dom(node: Node) {
    match node {
        Node::Document(document) => {
            println!("Document");

            for child in document.children {
                print_dom(child);
            }
        }
        Node::DocType(doc_type) => println!("DocType {}", doc_type.name),
        Node::Comment(comment) => println!("Comment: \"{}\"", comment.text),
        Node::Text(text) => println!(
            "Text \"{}\"",
            if text.text.trim().is_empty() {
                "\\n"
            } else {
                &text.text
            }
        ),
        Node::Element(element) => {
            println!(
                "Element \"{}\" | Attributes: \"{:?}\"",
                element.tag_name, element.attributes
            );
            for child in element.children {
                print_dom(child);
            }
        }
    }
}
