use parser::CssParser;
use types::{Declaration, Stylesheet};

pub(crate) mod parser;
pub(crate) mod types;

pub(crate) fn parse(css: &str) -> Stylesheet {
    let mut parser = CssParser::new(css);

    parser.parse()
}

pub(crate) fn parse_list_of_declarations(css: &str) -> Vec<Declaration> {
    let mut parser = CssParser::new(css);

    parser.parse_list_of_declarations()
}
