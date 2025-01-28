use crate::dom::{Attribute, Node, NodeFactory};

pub(crate) struct HtmlParser;

impl HtmlParser {
    pub(crate) fn convert_dom(&self, node: &scraper::ElementRef) -> Node {
        let mut nodes = vec![];

        for child in node.children() {
            if let Some(node) = HtmlParser::traverse_and_convert(child) {
                nodes.push(node);
            }
        }

        let value = node.value();
        let wrapper: VectorWrapper<Attribute> = value.attrs().into();
        let attributes: Vec<Attribute> = wrapper.into();

        NodeFactory::create_document(vec![NodeFactory::create_element(
            value.name().to_string(),
            attributes,
            nodes,
        )])
    }

    fn traverse_and_convert(node: ego_tree::NodeRef<'_, scraper::Node>) -> Option<Node> {
        match node.value() {
            scraper::Node::Element(element) => {
                let mut nodes = vec![];

                for child in node.children() {
                    if let Some(node) = HtmlParser::traverse_and_convert(child) {
                        nodes.push(node);
                    }
                }

                let wrapper: VectorWrapper<Attribute> = element.attrs().into();
                let attributes: Vec<Attribute> = wrapper.into();

                Some(NodeFactory::create_element(
                    element.name().to_string(),
                    attributes,
                    nodes,
                ))
            }
            scraper::Node::Text(text) => Some(NodeFactory::create_text(text.to_string())),
            scraper::Node::Comment(comment) => {
                Some(NodeFactory::create_comment(comment.to_string()))
            }
            scraper::Node::Doctype(doctype) => {
                Some(NodeFactory::create_doctype(doctype.name().to_string()))
            }
            _ => None,
        }
    }
}

struct VectorWrapper<T>(Vec<T>);

impl From<scraper::node::Attrs<'_>> for VectorWrapper<Attribute> {
    fn from(attrs: scraper::node::Attrs<'_>) -> Self {
        VectorWrapper(
            attrs
                .map(|attr| Attribute {
                    name: attr.0.to_string(),
                    value: attr.1.to_string(),
                })
                .collect(),
        )
    }
}

impl From<VectorWrapper<Attribute>> for Vec<Attribute> {
    fn from(wrapper: VectorWrapper<Attribute>) -> Vec<Attribute> {
        wrapper.0
    }
}
