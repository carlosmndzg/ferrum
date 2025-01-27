use crate::html::parser::{Html5everParser, HtmlParser};

pub fn create_parser() -> impl HtmlParser {
    Html5everParser
}
