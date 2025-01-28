use parser::CssParser;
use types::Stylesheet;

pub(crate) mod parser;
pub(crate) mod types;

pub(crate) fn parse(css: &str) -> Stylesheet {
    let mut parser = CssParser::new(css);

    parser.parse()
}
