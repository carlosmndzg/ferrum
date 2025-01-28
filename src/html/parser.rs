use scraper::Html;

use crate::dom::{Attribute, Node, NodeFactory};

pub(crate) fn parse(html: &str) -> Node {
    let dom = Html::parse_document(html);
    let parser = HtmlParser;

    parser.convert_dom(&dom.root_element())
}

struct HtmlParser;

impl HtmlParser {
    fn convert_dom(&self, node: &scraper::ElementRef) -> Node {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html5ever_basic_parse() {
        let html = r#"<html><head></head><body></body></html>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            NodeFactory::create_document(vec![NodeFactory::create_element(
                "html".to_string(),
                vec![],
                vec![
                    NodeFactory::create_element("head".to_string(), vec![], vec![]),
                    NodeFactory::create_element("body".to_string(), vec![], vec![]),
                ]
            )])
        );
    }

    #[test]
    fn test_html5ever_bad_html() {
        let html = r#"<body><h1 class="foo">Text &copy;</body>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            NodeFactory::create_document(vec![NodeFactory::create_element(
                "html".to_string(),
                vec![],
                vec![
                    NodeFactory::create_element("head".to_string(), vec![], vec![]),
                    NodeFactory::create_element(
                        "body".to_string(),
                        vec![],
                        vec![NodeFactory::create_element(
                            "h1".to_string(),
                            vec![Attribute {
                                name: "class".to_string(),
                                value: "foo".to_string()
                            }],
                            vec![NodeFactory::create_text("Text ©".to_string())]
                        )]
                    ),
                ]
            )])
        );
    }

    #[test]
    fn test_html5ever_parse_with_comments() {
        let html = r#"<html><!-- comment --><body></body></html>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            NodeFactory::create_document(vec![NodeFactory::create_element(
                "html".to_string(),
                vec![],
                vec![
                    NodeFactory::create_comment(" comment ".to_string()),
                    NodeFactory::create_element("head".to_string(), vec![], vec![]),
                    NodeFactory::create_element("body".to_string(), vec![], vec![]),
                ]
            )])
        );
    }

    #[test]
    fn test_html5ever_full_html() {
        let html = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>Document</title></head><body><h1 class="foo">Text &copy;</h1></body></html>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            NodeFactory::create_document(vec![NodeFactory::create_element(
                "html".to_string(),
                vec![Attribute {
                    name: "lang".to_string(),
                    value: "en".to_string()
                }],
                vec![
                    NodeFactory::create_element(
                        "head".to_string(),
                        vec![],
                        vec![
                            NodeFactory::create_element(
                                "meta".to_string(),
                                vec![Attribute {
                                    name: "charset".to_string(),
                                    value: "UTF-8".to_string()
                                }],
                                vec![]
                            ),
                            NodeFactory::create_element(
                                "title".to_string(),
                                vec![],
                                vec![NodeFactory::create_text("Document".to_string())]
                            ),
                        ]
                    ),
                    NodeFactory::create_element(
                        "body".to_string(),
                        vec![],
                        vec![NodeFactory::create_element(
                            "h1".to_string(),
                            vec![Attribute {
                                name: "class".to_string(),
                                value: "foo".to_string()
                            }],
                            vec![NodeFactory::create_text("Text ©".to_string())]
                        )]
                    ),
                ]
            )])
        );
    }
}
