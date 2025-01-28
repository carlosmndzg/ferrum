use crate::dom::Node;
use parser::HtmlParser;
use scraper::Html;

mod parser;

pub(crate) fn parse(html: &str) -> Node {
    let dom = Html::parse_document(html);
    let parser = HtmlParser;

    parser.convert_dom(&dom.root_element())
}

#[cfg(test)]
mod tests {
    use crate::dom::{Attribute, NodeFactory};

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
