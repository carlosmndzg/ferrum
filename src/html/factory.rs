use crate::html::parser::{Html5everParser, HtmlParser};

pub(crate) fn create_parser() -> impl HtmlParser {
    Html5everParser
}
