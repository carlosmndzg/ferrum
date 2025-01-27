use crate::css::parser::{CssParser, CssParserImpl};

pub fn create_parser() -> impl CssParser {
    CssParserImpl
}
