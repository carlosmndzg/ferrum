use properties::PropertyRegistry;
use types::{StyledNode, Styles};

use crate::{
    css::types::{Declaration, Stylesheet},
    Node, NodeType,
};

pub(crate) mod properties;
pub(crate) mod types;
pub(crate) mod utils;
pub(crate) mod validations;

pub(crate) fn build_style_tree<'a>(
    root: &'a Node,
    author_stylesheet: &Stylesheet,
    user_agent_stylesheet: &Stylesheet,
) -> StyledNode<'a> {
    let property_registry = PropertyRegistry::new();

    let html_node = root
        .find_first_node(&|n| is_tag_node(n, "html"))
        .expect("No <html> node found in the DOM");

    let body_node = html_node
        .find_first_node(&|n| is_tag_node(n, "body"))
        .expect("No <body> node found in the DOM");

    let styles = find_styles(
        html_node,
        author_stylesheet,
        user_agent_stylesheet,
        None,
        &property_registry,
    );

    let children = vec![build_style_node(
        body_node,
        author_stylesheet,
        user_agent_stylesheet,
        Some(&styles),
        &property_registry,
    )];

    StyledNode {
        node: html_node,
        styles,
        children,
    }
}

fn build_style_node<'a>(
    node: &'a Node,
    author_stylesheet: &Stylesheet,
    user_agent_stylesheet: &Stylesheet,
    parent_styles: Option<&Styles>,
    property_registry: &PropertyRegistry,
) -> StyledNode<'a> {
    let styles = find_styles(
        node,
        author_stylesheet,
        user_agent_stylesheet,
        parent_styles,
        property_registry,
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
                property_registry,
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

fn find_styles(
    node: &Node,
    author_stylesheet: &Stylesheet,
    user_agent_stylesheet: &Stylesheet,
    parent_styles: Option<&Styles>,
    property_registry: &PropertyRegistry,
) -> Styles {
    let mut styles = Styles::default();

    // User agent rules
    let mut ua_rules = user_agent_stylesheet.matching_rules(node);
    ua_rules.sort_by_key(|rule| rule.specificity());

    for rule in ua_rules {
        styles.apply(&rule.declarations, property_registry);
    }

    // Author rules
    let mut author_rules = author_stylesheet.matching_rules(node);
    author_rules.sort_by_key(|rule| rule.specificity());

    for rule in author_rules {
        styles.apply(&rule.declarations, property_registry);
    }

    let style_attribute_declarations = find_style_attribute_declarations(node);

    styles.apply(&style_attribute_declarations, property_registry);

    // Defaulting values (Inheritance)
    if let Some(parent_styles) = parent_styles {
        for property_name in property_registry.inheritable_properties() {
            if !styles.has(property_name) && parent_styles.has(property_name) {
                styles.add(parent_styles.get(property_name).cloned().unwrap());
            }
        }
    }

    // Defaulting values (Initial values)
    for property_name in property_registry.available_properties() {
        if !styles.has(property_name) {
            let initial_values = property_registry.initial_value(property_name);

            for value in initial_values {
                styles.add(value);
            }
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
