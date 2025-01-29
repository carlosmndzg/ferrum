use properties::PropertyFactory;
use types::{StyledNode, Styles};

use crate::{
    css::types::{Rule, Stylesheet},
    Element, Node, NodeType,
};

pub(crate) mod properties;
pub(crate) mod types;

pub(crate) fn build_style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    let html_node = root
        .find_first_node(&|n| is_tag_node(n, "html"))
        .expect("No <html> node found in the DOM");

    let body_node = html_node
        .find_first_node(&|n| is_tag_node(n, "body"))
        .expect("No <body> node found in the DOM");

    StyledNode {
        node: html_node,
        parent_node: None,
        styles: find_styles(html_node, stylesheet),
        children: vec![build_style_node(body_node, stylesheet, html_node)],
    }
}

fn build_style_node<'a>(
    node: &'a Node,
    stylesheet: &'a Stylesheet,
    parent_node: &'a Node,
) -> StyledNode<'a> {
    let styles = find_styles(node, stylesheet);

    let children = node
        .children
        .iter()
        .map(|child| build_style_node(child, stylesheet, node))
        .collect();

    StyledNode {
        node,
        parent_node: Some(parent_node),
        styles,
        children,
    }
}

fn is_tag_node(node: &Node, tag: &str) -> bool {
    matches!(&node.node_type, NodeType::Element(Element { tag_name, .. }) if tag_name == tag)
}

fn find_styles<'a>(node: &'a Node, stylesheet: &'a Stylesheet) -> Styles {
    let mut styles = Styles::default();

    let rules = find_matching_rules(node, stylesheet);
    let rules = sort_rules_by_specificity(rules);

    for rule in rules {
        for declaration in &rule.declarations {
            if let Some(property) = PropertyFactory::create_property(declaration) {
                styles.add_property(property);
            }
        }
    }

    styles
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

    // Sort by key but in reverse order

    rules.sort_by_key(|rule| rule.specificity());
    rules.reverse();

    rules
}
