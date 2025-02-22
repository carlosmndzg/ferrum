use properties::{PropertyFactory, AVAILABLE_PROPERTIES, INHERITABLE_PROPERTIES};
use types::{StyledNode, Styles};

use crate::{
    css::types::{Declaration, Rule, Stylesheet},
    Element, Node, NodeType,
};

pub(crate) mod properties;
pub(crate) mod types;

pub(crate) fn build_style_tree<'a>(
    root: &'a Node,
    stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
) -> StyledNode<'a> {
    let html_node = root
        .find_first_node(&|n| is_tag_node(n, "html"))
        .expect("No <html> node found in the DOM");

    let body_node = html_node
        .find_first_node(&|n| is_tag_node(n, "body"))
        .expect("No <body> node found in the DOM");

    let styles = find_styles(html_node, stylesheet, user_agent_stylesheet, None);
    let children = vec![build_style_node(
        body_node,
        stylesheet,
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
    stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
    parent_styles: Option<&Styles>,
) -> StyledNode<'a> {
    let styles = find_styles(node, stylesheet, user_agent_stylesheet, parent_styles);

    let children = node
        .children
        .iter()
        .map(|child| build_style_node(child, stylesheet, user_agent_stylesheet, Some(&styles)))
        .collect();

    StyledNode {
        node,
        styles,
        children,
    }
}

fn is_tag_node(node: &Node, tag: &str) -> bool {
    matches!(&node.node_type, NodeType::Element(Element { tag_name, .. }) if tag_name == tag)
}

fn find_styles<'a>(
    node: &'a Node,
    stylesheet: &'a Stylesheet,
    user_agent_stylesheet: &'a Stylesheet,
    parent_styles: Option<&'a Styles>,
) -> Styles {
    let mut styles = Styles::default();

    let style_attribute_declarations = find_style_attribute_declarations(node);

    for declaration in style_attribute_declarations {
        if let Some(property) = PropertyFactory::create_property(&declaration) {
            styles.add_property(property);
        }
    }

    // Declared values
    let rules = find_matching_rules(node, stylesheet);
    let rules = sort_rules_by_specificity(rules);

    // Cascade values
    for rule in rules {
        for declaration in &rule.declarations {
            if let Some(property) = PropertyFactory::create_property(declaration) {
                styles.add_property(property);
            }
        }
    }

    // User agent values
    let ua_rules = find_matching_rules(node, user_agent_stylesheet);
    let ua_rules = sort_rules_by_specificity(ua_rules);

    for rule in ua_rules {
        for declaration in &rule.declarations {
            if let Some(property) = PropertyFactory::create_property(declaration) {
                styles.add_property(property);
            }
        }
    }

    // Defaulting values (Inheritance)
    if let Some(parent_styles) = parent_styles {
        for property_name in INHERITABLE_PROPERTIES {
            if !styles.has_property(property_name) && parent_styles.has_property(property_name) {
                styles.add_property(parent_styles.get_property_clone(property_name).unwrap());
            }
        }
    }

    // Defaulting values (Initial values)
    for property_name in AVAILABLE_PROPERTIES {
        if !styles.has_property(property_name) {
            styles.add_property(PropertyFactory::create_initial_property(property_name));
        }
    }

    styles
}

fn find_style_attribute_declarations(node: &Node) -> Vec<Declaration> {
    if let NodeType::Element(element) = &node.node_type {
        if let Some(style) = element.get_attribute("style") {
            let mut declarations = crate::css::parse_list_of_declarations(style);

            // We reverse it because we want to apply the last declaration first
            declarations.reverse();

            return declarations;
        }
    }

    Vec::new()
}

fn find_matching_rules<'a>(node: &'a Node, stylesheet: &'a Stylesheet) -> Vec<&'a Rule> {
    stylesheet
        .rules
        .iter()
        .filter(|rule| rule.matches_node(node))
        .collect()
}

fn sort_rules_by_specificity(rules: Vec<&Rule>) -> Vec<&Rule> {
    let mut rules = rules;

    rules.sort_by_key(|rule| rule.specificity());
    rules.reverse();

    rules
}

#[cfg(test)]
mod tests {
    use crate::{
        css::types::{Declaration, Selector, SimpleSelector, Value},
        dom::Attribute,
        style::{properties::color::Color, types::Rgb},
        Text,
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

        let node = Node {
            node_type: NodeType::Element(Element {
                tag_name: "div".to_string(),
                attributes: vec![],
            }),
            children: vec![Node {
                node_type: NodeType::Element(Element {
                    tag_name: "p".to_string(),
                    attributes: vec![
                        Attribute {
                            name: "class".to_string(),
                            value: "foo".to_string(),
                        },
                        Attribute {
                            name: "id".to_string(),
                            value: "bar".to_string(),
                        },
                    ],
                }),
                children: vec![Node {
                    node_type: NodeType::Text(Text {
                        text: "Hello, world!".to_string(),
                    }),
                    children: vec![],
                }],
            }],
        };

        let ua_stylesheet = Stylesheet { rules: vec![] };

        let styled_node = build_style_node(&node, &stylesheet, &ua_stylesheet, None);

        let color = styled_node
            .styles
            .get_property_clone("color")
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
            .get_property_clone("color")
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
            .get_property_clone("color")
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
