use crate::css::{
    preprocessor,
    tokenizer::{Token, Tokenizer},
};

pub trait CssParser {
    fn parse(&self, css: &str);
}

pub struct CssParserImpl;

impl CssParser for CssParserImpl {
    fn parse(&self, css: &str) {
        let input = preprocessor::preprocess(css);
        let mut tokenizer = Tokenizer::new(&input);

        let mut token = tokenizer.next_token();

        loop {
            if token == Token::Eof {
                break;
            }

            println!("{:?}", token);

            token = tokenizer.next_token();
        }
    }
}
