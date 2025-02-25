use properties::{PropertyFactory, AVAILABLE_PROPERTIES, INHERITABLE_PROPERTIES};
use types::{StyledNode, Styles};

use crate::{
    css::types::{Declaration, Stylesheet},
    Node, NodeType,
};

pub(crate) mod properties;
pub(crate) mod types;

pub(crate) fn build_style_tree<'a>(
    root: &'a Node,
    author_stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
) -> StyledNode<'a> {
    let html_node = root
        .find_first_node(&|n| is_tag_node(n, "html"))
        .expect("No <html> node found in the DOM");

    let body_node = html_node
        .find_first_node(&|n| is_tag_node(n, "body"))
        .expect("No <body> node found in the DOM");

    let styles = find_styles(html_node, author_stylesheet, user_agent_stylesheet, None);

    let children = vec![build_style_node(
        body_node,
        author_stylesheet,
        user_agent_stylesheet,
        Some(&styles),
    )];

    StyledNode {
        node: html_node,
        styles,
        children,
    }
}

fn build_style_node<'a>(
    node: &'a Node,
    author_stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
    parent_styles: Option<&Styles>,
) -> StyledNode<'a> {
    let styles = find_styles(
        node,
        author_stylesheet,
        user_agent_stylesheet,
        parent_styles,
    );

    let children = node
        .children
        .iter()
        .map(|child| {
            build_style_node(
                child,
                author_stylesheet,
                user_agent_stylesheet,
                Some(&styles),
            )
        })
        .collect();

    StyledNode {
        node,
        styles,
        children,
    }
}

fn is_tag_node(node: &Node, tag: &str) -> bool {
    matches!(&node.node_type, NodeType::Element(element) if tag == element.tag_name())
}

fn find_styles<'a>(
    node: &'a Node,
    author_stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
    parent_styles: Option<&'a Styles>,
) -> Styles {
    let mut styles = Styles::default();

    // User agent rules
    let mut ua_rules = user_agent_stylesheet.matching_rules(node);
    ua_rules.sort_by_key(|rule| rule.specificity());

    for rule in ua_rules {
        styles.apply(&rule.declarations);
    }

    // Author rules
    let mut author_rules = author_stylesheet.matching_rules(node);
    author_rules.sort_by_key(|rule| rule.specificity());

    for rule in author_rules {
        styles.apply(&rule.declarations);
    }

    let style_attribute_declarations = find_style_attribute_declarations(node);

    styles.apply(&style_attribute_declarations);

    // Defaulting values (Inheritance)
    if let Some(parent_styles) = parent_styles {
        for property_name in INHERITABLE_PROPERTIES {
            if !styles.has(property_name) && parent_styles.has(property_name) {
                styles.add(parent_styles.get(property_name).cloned().unwrap());
            }
        }
    }

    // Defaulting values (Initial values)
    for property_name in AVAILABLE_PROPERTIES {
        if !styles.has(property_name) {
            styles.add(PropertyFactory::create_initial_property(property_name));
        }
    }

    styles
}

fn find_style_attribute_declarations(node: &Node) -> Vec<Declaration> {
    if let NodeType::Element(element) = &node.node_type {
        if let Some(style) = element.attributes().get("style") {
            let declarations = crate::css::parse_list_of_declarations(style);

            return declarations;
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::{
        css::types::{Declaration, Rule, Selector, SimpleSelector, Value},
        dom::Attributes,
        style::{properties::color::Color, types::Rgb},
        Element, Text,
    };

    use super::{properties::Property, *};

    #[test]
    fn test_cascade_inheritance_initial() {
        let stylesheet = Stylesheet {
            rules: vec![
                Rule {
                    selector: Selector::Simple(SimpleSelector {
                        tag_name: None,
                        id: Some("bar".to_string()),
                        class: vec![],
                    }),
                    declarations: vec![Declaration {
                        name: "color".to_string(),
                        value: Value::Keyword("aquamarine".to_string()),
                    }],
                },
                Rule {
                    selector: Selector::Simple(SimpleSelector {
                        tag_name: Some("p".to_string()),
                        id: None,
                        class: vec!["foo".to_string()],
                    }),
                    declarations: vec![Declaration {
                        name: "color".to_string(),
                        value: Value::Keyword("rebeccapurple".to_string()),
                    }],
                },
            ],
        };

        let node = Node::new(
            NodeType::Element(Element::new("div", Attributes::from_iter(vec![]))),
            vec![Node::new(
                NodeType::Element(Element::new(
                    "p",
                    Attributes::from_iter(vec![("class", "foo"), ("id", "bar")]),
                )),
                vec![Node::new(
                    NodeType::Text(Text::new("Hello, world!".to_string())),
                    vec![],
                )],
            )],
        );

        let ua_stylesheet = Stylesheet { rules: vec![] };

        let styled_node = build_style_node(&node, &stylesheet, &ua_stylesheet, None);

        let color = styled_node
            .styles
            .get("color")
            .cloned()
            .expect("Expected a color property");

        let Property::Color(color) = color else {
            panic!("Expected a color property");
        };

        // Div node color
        assert_eq!(
            color,
            Color {
                value: Rgb {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 1.0
                }
            }
        );

        let p_node = styled_node.children.first().expect("Expected child p node");

        let Property::Color(color) = p_node
            .styles
            .get("color")
            .cloned()
            .expect("Expected a color property")
        else {
            panic!("Expected a color property");
        };

        // P node color
        assert_eq!(
            color,
            Color {
                value: Rgb {
                    r: 127,
                    g: 255,
                    b: 212,
                    a: 1.0
                }
            }
        );

        let text_node = p_node
            .children
            .first()
            .expect("Expected child text node for p node");

        let Property::Color(color) = text_node
            .styles
            .get("color")
            .cloned()
            .expect("Expected a color property")
        else {
            panic!("Expected a color property");
        };

        // Text node color
        assert_eq!(
            color,
            Color {
                value: Rgb {
                    r: 127,
                    g: 255,
                    b: 212,
                    a: 1.0
                }
            }
        );
    }
}
