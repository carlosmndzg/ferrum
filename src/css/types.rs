use std::cmp::Ordering;

use crate::{Node, NodeType};

#[derive(Debug, PartialEq)]
pub(crate) struct Stylesheet {
    pub(crate) rules: Vec<Rule>,
}

impl Stylesheet {
    pub(crate) fn matching_rules(&self, node: &Node) -> Vec<&Rule> {
        self.rules.iter().filter(|r| r.matches_node(node)).collect()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Rule {
    pub(crate) selector: Selector,
    pub(crate) declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq)]
pub(crate) struct SimpleSelector {
    pub(crate) tag_name: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) class: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) value: Vec<Value>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) enum Value {
    Rgb(Rgb),
    Dimension(f32, Unit),
    Percentage(f32),
    Keyword(String),
    NotDeclared,
    #[default]
    Temporal,
}

#[derive(Default, Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Rgb {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: f32,
}

impl Rgb {
    pub(crate) const fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Rgb { r, g, b, a }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) enum Unit {
    Px,
    None,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Specificity {
    pub(crate) a: u32,
    pub(crate) b: u32,
    pub(crate) c: u32,
}

impl Eq for Specificity {}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.a, self.b, self.c).cmp(&(other.a, other.b, other.c))
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Rule {
    pub(crate) fn specificity(&self) -> Specificity {
        self.selector.specificity()
    }
}

impl Selector {
    pub(crate) fn specificity(&self) -> Specificity {
        match self {
            Selector::Simple(s) => s.specificity(),
        }
    }
}

impl SimpleSelector {
    pub(crate) fn is_universal_selector(&self) -> bool {
        self.tag_name.is_none() && self.id.is_none() && self.class.is_empty()
    }

    pub(crate) fn specificity(&self) -> Specificity {
        let a = if self.id.is_some() { 1 } else { 0 };
        let b = self.class.len() as u32;
        let c = self.tag_name.as_ref().map_or(0, |_| 1);

        Specificity { a, b, c }
    }

    pub(crate) fn matches(&self, tag_name: &str, id: Option<&str>, classes: &[String]) -> bool {
        if self.is_universal_selector() {
            return true;
        }

        if let Some(ref t) = self.tag_name {
            if t != tag_name {
                return false;
            }
        }

        if let Some(ref i) = self.id {
            if i != id.unwrap_or("") {
                return false;
            }
        }

        if !self.class.is_empty() {
            let element_classes = classes.iter().collect::<Vec<_>>();
            if !self.class.iter().all(|c| element_classes.contains(&c)) {
                return false;
            }
        }

        true
    }
}

impl Rule {
    pub(crate) fn matches_node(&self, node: &Node) -> bool {
        let Node {
            node_type: NodeType::Element(element),
            ..
        } = node
        else {
            return false;
        };

        let tag_name = element.tag_name();
        let id = element.attributes().get("id").map(|s| s.as_str());
        let classes = element
            .attributes()
            .get("class")
            .map_or(vec![], |c| c.split(' ').map(String::from).collect());

        match &self.selector {
            Selector::Simple(s) => s.matches(tag_name.as_ref(), id, &classes),
        }
    }
}
