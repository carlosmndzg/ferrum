#[derive(Debug)]
pub enum Node {
    Document(Document),
    DocType(DocType),
    Comment(Comment),
    Text(Text),
    Element(Element),
}

#[derive(Debug)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug)]
pub struct DocType {
    pub name: String,
}

#[derive(Debug)]
pub struct Comment {
    pub text: String,
}

#[derive(Debug)]
pub struct Text {
    pub text: String,
}

#[derive(Debug)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

pub struct NodeFactory;

impl NodeFactory {
    pub fn create_document(children: Vec<Node>) -> Node {
        Node::Document(Document { children })
    }

    pub fn create_doctype(name: String) -> Node {
        Node::DocType(DocType { name })
    }

    pub fn create_comment(text: String) -> Node {
        Node::Comment(Comment { text })
    }

    pub fn create_text(text: String) -> Node {
        Node::Text(Text { text })
    }

    pub fn create_element(
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

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

// TODO Remove this function, only used for debugging
pub fn print_dom(node: Node) {
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
