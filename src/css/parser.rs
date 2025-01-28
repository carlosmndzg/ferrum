use crate::css::{
    preprocessor,
    tokenizer::{Token, Tokenizer},
};

pub(crate) fn parse(css: &str) {
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
