use std::{fs, path::Path};

use parser::CssParser;
use types::{Declaration, Rule, Stylesheet};

use crate::{Node, NodeType};

pub(crate) mod parser;
pub(crate) mod types;

pub(crate) fn parse_author(root: &Node, file_path: &Path) -> Stylesheet {
    let mut rules = Vec::new();

    parse_node(root, &mut rules, file_path);

    Stylesheet { rules }
}

pub(crate) fn parse_ua(input: &str) -> Stylesheet {
    let mut parser = CssParser::new(input);

    parser.parse()
}

pub(crate) fn parse_list_of_declarations(input: &str) -> Vec<Declaration> {
    let mut parser = CssParser::new(input);

    parser.parse_list_of_declarations()
}

fn parse_node(node: &Node, rules: &mut Vec<Rule>, file_path: &Path) {
    let node_type = &node.node_type;

    if let NodeType::Element(element) = node_type {
        let tag_name = element.tag_name();

        if tag_name == "style" {
            handle_style_node(node, rules);
        } else if tag_name == "link" {
            handle_link_node(node, rules, file_path);
        }
    }

    for child in &node.children {
        parse_node(child, rules, file_path);
    }
}

fn handle_style_node(node: &Node, rules: &mut Vec<Rule>) {
    if let Some(Node {
        node_type: NodeType::Text(text),
        ..
    }) = &node.children.first()
    {
        let text = text.get();
        let mut parser = CssParser::new(text);

        let stylesheet = parser.parse();

        rules.extend(stylesheet.rules);
    }
}

fn handle_link_node(node: &Node, rules: &mut Vec<Rule>, file_path: &Path) {
    if let NodeType::Element(element) = &node.node_type {
        let Some(href) = element.attributes().get("href") else {
            return;
        };

        let Some(folder) = file_path.parent() else {
            return;
        };

        let path = folder.join(href);

        if let Ok(input) = fs::read_to_string(path) {
            let mut parser = CssParser::new(&input);

            let stylesheet = parser.parse();

            rules.extend(stylesheet.rules);
        }
    }
}
