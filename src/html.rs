use crate::{
    dom::{Attributes, Comment, DocType, Document, Node},
    Element, NodeType, Text,
};
use ego_tree::{iter::Children, NodeRef};
use scraper::{node::Attrs, ElementRef, Html};

pub(crate) fn parse(html: &str) -> Node {
    Html::parse_document(html).into()
}

impl From<Html> for Node {
    fn from(html: Html) -> Self {
        html.root_element().into()
    }
}

impl From<ElementRef<'_>> for Node {
    fn from(element: ElementRef) -> Self {
        let (element, children) = (element.value(), element.children());
        let tag_name = element.name().to_string();
        let attributes = element.attrs().into();
        let children = children.map(Node::try_from).filter_map(Result::ok);
        let node_type = NodeType::Element(Element::new(tag_name, attributes));
        let html_element = Node::new(node_type, children.collect());

        Node::new(NodeType::Document(Document::new()), vec![html_element])
    }
}

impl From<Children<'_, scraper::Node>> for Node {
    fn from(children: Children<'_, scraper::Node>) -> Self {
        let children = children
            .map(Node::try_from)
            .filter_map(Result::ok)
            .collect();

        Node::new(NodeType::Document(Document::new()), children)
    }
}

impl From<Attrs<'_>> for Attributes {
    fn from(attrs: Attrs) -> Self {
        attrs.collect()
    }
}

impl TryFrom<NodeRef<'_, scraper::Node>> for Node {
    type Error = ();

    fn try_from(node: NodeRef<'_, scraper::Node>) -> Result<Self, Self::Error> {
        match node.value() {
            scraper::Node::Element(element) => {
                let tag_name = element.name().to_string();
                let attributes = element.attrs().into();
                let children = node.children().map(Node::try_from).filter_map(Result::ok);

                Ok(Node::new(
                    NodeType::Element(Element::new(tag_name, attributes)),
                    children.collect(),
                ))
            }
            scraper::Node::Text(text) => Ok(Node::new(
                NodeType::Text(Text::new(text.to_string())),
                vec![],
            )),
            scraper::Node::Comment(comment) => Ok(Node::new(
                NodeType::Comment(Comment::new(comment.to_string())),
                vec![],
            )),
            scraper::Node::Doctype(doctype) => Ok(Node::new(
                NodeType::DocType(DocType::new(doctype.name().to_string())),
                vec![],
            )),
            _ => Err(()),
        }
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
            Node::new(
                NodeType::Document(Document::new()),
                vec![Node::new(
                    NodeType::Element(Element::new(
                        "html".to_string(),
                        Attributes::from_iter(vec![])
                    )),
                    vec![
                        Node::new(
                            NodeType::Element(Element::new(
                                "head".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![]
                        ),
                        Node::new(
                            NodeType::Element(Element::new(
                                "body".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![]
                        ),
                    ]
                )]
            )
        );
    }

    #[test]
    fn test_html5ever_bad_html() {
        let html = r#"<body><h1 class="foo">Text &copy;</body>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            Node::new(
                NodeType::Document(Document::new()),
                vec![Node::new(
                    NodeType::Element(Element::new(
                        "html".to_string(),
                        Attributes::from_iter(vec![])
                    )),
                    vec![
                        Node::new(
                            NodeType::Element(Element::new(
                                "head".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![]
                        ),
                        Node::new(
                            NodeType::Element(Element::new(
                                "body".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![Node::new(
                                NodeType::Element(Element::new(
                                    "h1".to_string(),
                                    Attributes::from_iter(vec![("class", "foo")])
                                )),
                                vec![Node::new(
                                    NodeType::Text(Text::new("Text ©".to_string())),
                                    vec![]
                                )]
                            )]
                        )
                    ]
                )]
            )
        );
    }

    #[test]
    fn test_html5ever_parse_with_comments() {
        let html = r#"<html><!-- comment --><body></body></html>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            Node::new(
                NodeType::Document(Document::new()),
                vec![Node::new(
                    NodeType::Element(Element::new(
                        "html".to_string(),
                        Attributes::from_iter(vec![])
                    )),
                    vec![
                        Node::new(
                            NodeType::Comment(Comment::new(" comment ".to_string())),
                            vec![]
                        ),
                        Node::new(
                            NodeType::Element(Element::new(
                                "head".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![]
                        ),
                        Node::new(
                            NodeType::Element(Element::new(
                                "body".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![]
                        ),
                    ]
                )]
            )
        );
    }

    #[test]
    fn test_html5ever_full_html() {
        let html = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><title>Document</title></head><body><h1 class="foo">Text &copy;</h1></body></html>"#;
        let dom = parse(html);

        assert_eq!(
            dom,
            Node::new(
                NodeType::Document(Document::new()),
                vec![Node::new(
                    NodeType::Element(Element::new(
                        "html".to_string(),
                        Attributes::from_iter(vec![("lang", "en")])
                    )),
                    vec![
                        Node::new(
                            NodeType::Element(Element::new(
                                "head".to_string(),
                                Attributes::from_iter(vec![])
                            ),),
                            vec![
                                Node::new(
                                    NodeType::Element(Element::new(
                                        "meta".to_string(),
                                        Attributes::from_iter(vec![("charset", "UTF-8")])
                                    )),
                                    vec![]
                                ),
                                Node::new(
                                    NodeType::Element(Element::new(
                                        "title".to_string(),
                                        Attributes::from_iter(vec![])
                                    )),
                                    vec![Node::new(
                                        NodeType::Text(Text::new("Document".to_string())),
                                        vec![]
                                    )]
                                ),
                            ]
                        ),
                        Node::new(
                            NodeType::Element(Element::new(
                                "body".to_string(),
                                Attributes::from_iter(vec![])
                            )),
                            vec![Node::new(
                                NodeType::Element(Element::new(
                                    "h1".to_string(),
                                    Attributes::from_iter(vec![("class", "foo")])
                                )),
                                vec![Node::new(
                                    NodeType::Text(Text::new("Text ©".to_string())),
                                    vec![]
                                )]
                            )]
                        ),
                    ]
                )]
            )
        );
    }
}
