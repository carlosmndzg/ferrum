#[derive(Debug, PartialEq)]
pub(crate) struct Stylesheet {
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Rule {
    pub(crate) selectors: Vec<Selector>,
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

#[derive(Debug, PartialEq)]
pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) value: Value,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Keyword(String),
    ColorValue(Color),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}
