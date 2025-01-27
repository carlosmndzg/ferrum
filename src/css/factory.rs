use crate::css::parser::{CssParser, CssParserImpl};

pub(crate) fn create_parser() -> impl CssParser {
    CssParserImpl
}
