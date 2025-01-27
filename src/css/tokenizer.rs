// Tokenization follows the CSS Syntax Module Level 3 specification:
// https://www.w3.org/TR/css-syntax-3/#tokenization

use std::char;

pub struct Tokenizer {
    input: Vec<char>,
    next_position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            next_position: 0,
        }
    }

    fn next_input_code_point(&mut self) -> Option<char> {
        self.input.get(self.next_position).copied()
    }

    fn current_input_code_point(&self) -> Option<char> {
        self.input.get(self.next_position - 1).copied()
    }

    fn reconsume_current_input_code_point(&mut self) {
        self.next_position -= 1;
    }

    fn consume_next_input_code_point(&mut self) -> Option<char> {
        let character = self.next_input_code_point();

        self.next_position += 1;

        character
    }

    fn peek(&self, position: usize) -> Option<char> {
        self.input.get(position + self.next_position).copied()
    }

    fn is_digit(&self, character: char) -> bool {
        character.is_ascii_digit()
    }

    fn is_hex_digit(&self, character: char) -> bool {
        character.is_ascii_hexdigit()
    }

    fn is_letter(&self, character: char) -> bool {
        character.is_ascii_alphabetic()
    }

    fn is_non_ascii(&self, character: char) -> bool {
        !character.is_ascii()
    }

    fn is_ident_start(&self, character: char) -> bool {
        self.is_letter(character) || self.is_non_ascii(character) || character == '_'
    }

    fn is_ident(&self, character: char) -> bool {
        self.is_ident_start(character) || self.is_digit(character) || character == '-'
    }

    fn is_non_printable(&self, character: char) -> bool {
        character.is_ascii_control() || character == '\u{0007}'
    }

    fn is_newline(&self, character: char) -> bool {
        character == '\n'
    }

    fn is_whitespace(&self, character: char) -> bool {
        self.is_newline(character) || character == '\t' || character == ' '
    }

    fn is_valid_escape(&mut self) -> bool {
        if self.current_input_code_point() != Some('\\') {
            return false;
        }

        if self.next_input_code_point() == Some('\n') {
            return false;
        }

        true
    }

    fn check_if_next_codepoints_are_and_consume(&mut self, code_points: &str) -> bool {
        if self.next_position + code_points.len() > self.input.len() {
            return false;
        }

        let next_code_points = self.input
            [self.next_position..self.next_position + code_points.len()]
            .iter()
            .collect::<String>();

        if code_points == next_code_points {
            self.next_position += code_points.len();
            return true;
        }

        false
    }

    fn check_if_would_start_ident_sequence(&mut self) -> bool {
        let first = self.current_input_code_point();

        match first {
            Some('-') => {
                let second = self.next_input_code_point();

                match second {
                    Some(c) if c == '-' || self.is_ident_start(c) => true,
                    Some(_) => {
                        self.next_input_code_point();

                        let is_valid = self.is_valid_escape();

                        self.reconsume_current_input_code_point();

                        is_valid
                    }
                    _ => {
                        self.reconsume_current_input_code_point();
                        false
                    }
                }
            }
            Some(c) if self.is_ident_start(c) => true,
            Some('\\') if self.next_input_code_point() != Some('\n') => true,
            _ => false,
        }
    }

    fn check_if_would_start_number(&mut self) -> bool {
        let first = self.current_input_code_point();

        match first {
            Some('+') | Some('-') => {
                let second = self.next_input_code_point();

                match second {
                    Some(c) if self.is_digit(c) => true,
                    Some('.') => {
                        let third = self.peek(1);

                        matches!(third, Some(c) if self.is_digit(c))
                    }
                    _ => false,
                }
            }
            Some('.') => {
                matches!(self.next_input_code_point(), Some(c) if self.is_digit(c))
            }
            Some(c) if self.is_digit(c) => true,
            _ => false,
        }
    }

    fn consume_whitespace(&mut self) -> Token {
        loop {
            let character = self.next_input_code_point();

            match character {
                Some(c) if self.is_whitespace(c) => {
                    self.consume_next_input_code_point();
                    continue;
                }
                _ => break,
            }
        }

        Token::Whitespace
    }

    fn consume_escaped_code_point(&mut self) -> char {
        let character = self.consume_next_input_code_point();

        match character {
            Some(c) if self.is_hex_digit(c) => {
                let mut code = String::new();
                code.push(c);

                for _ in 0..5 {
                    let next_character = self.next_input_code_point();

                    match next_character {
                        Some(c) if self.is_hex_digit(c) => {
                            code.push(c);
                            self.consume_next_input_code_point();
                        }
                        _ => break,
                    }
                }

                if let Some(c) = self.next_input_code_point() {
                    if self.is_whitespace(c) {
                        self.consume_next_input_code_point();
                    }
                }

                let code_point = u32::from_str_radix(&code, 16).unwrap();

                if code_point == 0 || code_point > 0x10FFFF {
                    '\u{FFFD}'
                } else {
                    char::from_u32(code_point).unwrap()
                }
            }
            None => '\u{FFFD}',
            _ => self.current_input_code_point().unwrap(),
        }
    }

    fn consume_string(&mut self) -> Token {
        let mut string = String::new();
        let ending_code_point = self.current_input_code_point().unwrap();

        loop {
            let character = self.consume_next_input_code_point();

            match character {
                Some(c) if c == ending_code_point => break,
                Some('\n') => {
                    self.reconsume_current_input_code_point();
                    return Token::BadString;
                }
                Some('\\') => {
                    if self.next_input_code_point().is_some() {
                        if self.next_input_code_point() == Some('\n') {
                            self.consume_next_input_code_point();
                        } else {
                            string.push(self.consume_escaped_code_point());
                        }
                    }
                }
                None => break,
                _ => string.push(self.current_input_code_point().unwrap()),
            }
        }

        Token::String(string)
    }

    fn convert_repr(&self, repr: String) -> f64 {
        let repr = repr.chars().collect::<Vec<char>>();

        let mut index = 0;

        let s = if let Some('-') = repr.get(index) {
            index += 1;
            -1
        } else {
            if let Some('+') = repr.get(index) {
                index += 1;
            }

            1
        };

        let mut i = 0;

        loop {
            let character = repr.get(index);

            match character {
                Some(c) if self.is_digit(*c) => {
                    i = i * 10 + c.to_digit(10).unwrap();
                    index += 1;
                }
                _ => break,
            }
        }

        if let Some('.') = repr.get(index) {
            index += 1;
        }

        let mut f = 0;
        let mut d = 0;

        loop {
            let character = repr.get(index);

            match character {
                Some(c) if self.is_digit(*c) => {
                    f = f * 10 + c.to_digit(10).unwrap();
                    d += 1;
                    index += 1;
                }
                _ => break,
            }
        }

        if matches!(repr.get(index), Some('E') | Some('e')) {
            index += 1;
        }

        let t = if let Some('-') = repr.get(index) {
            index += 1;

            -1
        } else {
            if let Some('+') = repr.get(index) {
                index += 1;
            }

            1
        };

        let mut e = 0;

        loop {
            let character = repr.get(index);

            match character {
                Some(c) if self.is_digit(*c) => {
                    e = e * 10 + c.to_digit(10).unwrap();
                    index += 1;
                }
                _ => break,
            }
        }

        (s as f64) * (i as f64 + f as f64 / 10_f64.powi(d)) * 10_f64.powi(t * e as i32)
    }

    fn consume_number(&mut self) -> (f64, NumberType) {
        let mut number_type = NumberType::Integer;
        let mut repr = String::new();

        if matches!(self.next_input_code_point(), Some('+') | Some('-')) {
            repr.push(self.consume_next_input_code_point().unwrap());
        }

        loop {
            let character = self.next_input_code_point();

            match character {
                Some(c) if self.is_digit(c) => {
                    repr.push(self.consume_next_input_code_point().unwrap());
                }
                _ => break,
            }
        }

        if self.next_input_code_point() == Some('.')
            && matches!(self.peek(1), Some(c) if self.is_digit(c))
        {
            repr.push(self.consume_next_input_code_point().unwrap());
            repr.push(self.consume_next_input_code_point().unwrap());

            number_type = NumberType::Number;

            loop {
                let character = self.next_input_code_point();

                match character {
                    Some(c) if self.is_digit(c) => {
                        repr.push(self.consume_next_input_code_point().unwrap());
                    }
                    _ => break,
                }
            }
        }

        if matches!(self.next_input_code_point(), Some('E') | Some('e')) {
            let optional_part = matches!(self.peek(1), Some(c) if c == '+' || c == '-');
            let optional_part_with_digit = matches!(self.peek(2), Some(c) if self.is_digit(c));
            let not_optional_with_digit = matches!(self.peek(1), Some(c) if self.is_digit(c));

            if (optional_part && optional_part_with_digit) || not_optional_with_digit {
                if optional_part && optional_part_with_digit {
                    repr.push(self.consume_next_input_code_point().unwrap());
                    repr.push(self.consume_next_input_code_point().unwrap());
                } else {
                    repr.push(self.consume_next_input_code_point().unwrap());
                }

                number_type = NumberType::Number;

                loop {
                    let character = self.next_input_code_point();

                    match character {
                        Some(c) if self.is_digit(c) => {
                            repr.push(self.consume_next_input_code_point().unwrap());
                        }
                        _ => break,
                    }
                }
            }
        }

        (self.convert_repr(repr), number_type)
    }

    fn consume_ident_sequence(&mut self) -> String {
        let mut result = String::new();

        loop {
            let character = self.consume_next_input_code_point();

            match character {
                Some(c) if self.is_ident(c) => result.push(c),
                Some('\\') if self.next_input_code_point() != Some('\n') => {
                    result.push(self.consume_escaped_code_point());
                }
                _ => {
                    self.reconsume_current_input_code_point();
                    return result;
                }
            }
        }
    }

    fn consume_numeric_token(&mut self) -> Token {
        let number = self.consume_number();

        self.consume_next_input_code_point();

        let would_start_ident_sequence = self.check_if_would_start_ident_sequence();

        self.reconsume_current_input_code_point();

        if would_start_ident_sequence {
            let ident = self.consume_ident_sequence();

            Token::Dimension(number.0, number.1, ident)
        } else if let Some('%') = self.next_input_code_point() {
            self.consume_next_input_code_point();

            Token::Percentage(number.0)
        } else {
            Token::Number(number.0)
        }
    }

    fn consume_whitespace_without_token(&mut self) {
        loop {
            let character = self.next_input_code_point();

            if matches!(character, Some(c) if self.is_whitespace(c)) {
                self.consume_next_input_code_point();
            } else {
                break;
            }
        }
    }

    fn consume_remnants_of_bad_url(&mut self) {
        loop {
            let character = self.consume_next_input_code_point();

            match character {
                Some(')') => break,
                Some('\\') => {
                    if self.is_valid_escape() {
                        self.consume_escaped_code_point();
                    }
                }
                _ => {}
            }
        }
    }

    fn consume_url_token(&mut self) -> Token {
        self.consume_whitespace_without_token();

        let mut result = String::new();

        loop {
            let character = self.consume_next_input_code_point();

            match character {
                Some(')') => break,
                Some(c) if self.is_whitespace(c) => {
                    self.consume_whitespace_without_token();

                    if matches!(self.next_input_code_point(), Some(')') | None) {
                        self.consume_next_input_code_point();

                        break;
                    } else {
                        self.consume_remnants_of_bad_url();
                        return Token::BadUrl;
                    }
                }
                Some('"' | '\'' | '(') => {
                    self.consume_remnants_of_bad_url();
                    return Token::BadUrl;
                }
                Some(c) if self.is_non_printable(c) => {
                    self.consume_remnants_of_bad_url();
                    return Token::BadUrl;
                }
                Some('\\') => {
                    if self.is_valid_escape() {
                        result.push(self.consume_escaped_code_point());
                    } else {
                        self.consume_remnants_of_bad_url();
                        return Token::BadUrl;
                    }
                }
                None => break,
                _ => result.push(character.unwrap()),
            }
        }

        Token::Url(result)
    }

    fn consume_ident_like_token(&mut self) -> Token {
        let string = self.consume_ident_sequence();

        if string.to_lowercase().starts_with("url") && self.next_input_code_point() == Some('(') {
            self.consume_next_input_code_point();

            while let (Some(' '), Some(' ')) = (self.next_input_code_point(), self.peek(1)) {
                self.consume_next_input_code_point();
            }

            let quot_mark = matches!(self.next_input_code_point(), Some('"') | Some('\''));
            let space_and_quot_mark = matches!(
                (self.next_input_code_point(), self.peek(1)),
                (Some(' '), Some('"')) | (Some(' '), Some('\''))
            );

            if quot_mark || space_and_quot_mark {
                return Token::Function(string);
            } else {
                return self.consume_url_token();
            }
        }

        if self.next_input_code_point() == Some('(') {
            self.consume_next_input_code_point();

            return Token::Function(string);
        }

        Token::Ident(string)
    }

    fn consume_comments(&mut self) {
        loop {
            if !self.check_if_next_codepoints_are_and_consume("/*") {
                break;
            }

            loop {
                let character = self.consume_next_input_code_point();

                match character {
                    Some('*') => {
                        if self.next_input_code_point() == Some('/') {
                            self.consume_next_input_code_point();
                            break;
                        }
                    }
                    None => break,
                    _ => {}
                }
            }
        }
    }

    fn handle_number_sign(&mut self) -> Token {
        let next_is_ident = matches!(self.next_input_code_point(), Some(c) if self.is_ident(c));
        let next_two_valid_escape = matches!(self.next_input_code_point(), Some('\\'))
            && !matches!(self.peek(1), Some('\n'));

        if next_is_ident || next_two_valid_escape {
            self.consume_next_input_code_point();

            let would_start_ident_sequence = self.check_if_would_start_ident_sequence();

            self.reconsume_current_input_code_point();

            let hash_type = if would_start_ident_sequence {
                HashType::Id
            } else {
                HashType::Unrestricted
            };

            Token::Hash(self.consume_ident_sequence(), hash_type)
        } else {
            Token::Delim(self.current_input_code_point().unwrap())
        }
    }

    fn handle_less_than_sign(&mut self) -> Token {
        if self.check_if_next_codepoints_are_and_consume("!--") {
            return Token::Cdo;
        }

        Token::Delim(self.current_input_code_point().unwrap())
    }

    fn handle_digit(&mut self) -> Token {
        self.reconsume_current_input_code_point();
        self.consume_numeric_token()
    }

    fn handle_plus_sign(&mut self) -> Token {
        self.handle_full_stop()
    }

    fn handle_full_stop(&mut self) -> Token {
        if self.check_if_would_start_number() {
            self.reconsume_current_input_code_point();
            self.consume_numeric_token()
        } else {
            Token::Delim(self.current_input_code_point().unwrap())
        }
    }

    fn handle_hyphen_minus(&mut self) -> Token {
        if self.check_if_would_start_number() {
            self.reconsume_current_input_code_point();
            return self.consume_numeric_token();
        }

        if let (Some('-'), Some('>')) = (self.next_input_code_point(), self.peek(1)) {
            self.consume_next_input_code_point();
            self.consume_next_input_code_point();

            return Token::Cdc;
        }

        if self.check_if_would_start_ident_sequence() {
            self.reconsume_current_input_code_point();
            return self.consume_ident_like_token();
        }

        Token::Delim(self.current_input_code_point().unwrap())
    }

    fn handle_reverse_solidus(&mut self) -> Token {
        if self.next_input_code_point() != Some('\n') {
            self.reconsume_current_input_code_point();
            self.consume_ident_like_token()
        } else {
            Token::Delim(self.current_input_code_point().unwrap())
        }
    }

    fn handle_commercial_at(&mut self) -> Token {
        self.consume_next_input_code_point();

        let starts_ident_sequence = self.check_if_would_start_ident_sequence();

        self.reconsume_current_input_code_point();

        if starts_ident_sequence {
            Token::AtKeyword(self.consume_ident_sequence())
        } else {
            Token::Delim(self.current_input_code_point().unwrap())
        }
    }

    fn handle_ident_start(&mut self) -> Token {
        self.reconsume_current_input_code_point();
        self.consume_ident_like_token()
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_comments();

        let character = self.consume_next_input_code_point();

        match character {
            Some(c) if self.is_whitespace(c) => self.consume_whitespace(),
            Some('"') => self.consume_string(),
            Some('#') => self.handle_number_sign(),
            Some('\'') => self.consume_string(),
            Some('(') => Token::OpenParen,
            Some(')') => Token::CloseParen,
            Some('+') => self.handle_plus_sign(),
            Some(',') => Token::Comma,
            Some('-') => self.handle_hyphen_minus(),
            Some('.') => self.handle_full_stop(),
            Some(':') => Token::Colon,
            Some(';') => Token::Semicolon,
            Some('<') => self.handle_less_than_sign(),
            Some('@') => self.handle_commercial_at(),
            Some('[') => Token::OpenSquareBracket,
            Some('\\') => self.handle_reverse_solidus(),
            Some(']') => Token::CloseSquareBracket,
            Some('{') => Token::OpenCurlyBracket,
            Some('}') => Token::CloseCurlyBracket,
            Some(c) if self.is_digit(c) => self.handle_digit(),
            Some(c) if self.is_ident_start(c) => self.handle_ident_start(),
            None => Token::Eof,
            _ => Token::Delim(self.current_input_code_point().unwrap()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Function(String),
    AtKeyword(String),
    Hash(String, HashType),
    String(String),
    BadString,
    Url(String),
    BadUrl,
    Delim(char),
    Number(f64),
    Percentage(f64),
    Dimension(f64, NumberType, String),
    Whitespace,
    Cdo,
    Cdc,
    Colon,
    Semicolon,
    Comma,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenParen,
    CloseParen,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum NumberType {
    Integer,
    Number,
}

#[derive(Debug, PartialEq)]
pub enum HashType {
    Id,
    Unrestricted,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_all_tokens(input: &str) -> Vec<Token> {
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        loop {
            let token = tokenizer.next_token();

            if token == Token::Eof {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        tokens
    }

    #[test]
    fn test_consume_whitespace() {
        assert_eq!(get_all_tokens(" \t\n"), vec![Token::Whitespace, Token::Eof]);
    }

    #[test]
    fn test_consume_string() {
        assert_eq!(
            get_all_tokens("\"hello\"\n'world' 'text\"\n"),
            vec![
                Token::String(String::from("hello")),
                Token::Whitespace,
                Token::String(String::from("world")),
                Token::Whitespace,
                Token::BadString,
                Token::Whitespace,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_handle_less_than_sign() {
        assert_eq!(
            get_all_tokens("<'text'<!--"),
            vec![
                Token::Delim('<'),
                Token::String(String::from("text")),
                Token::Cdo,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_escape_sequences() {
        assert_eq!(
            get_all_tokens("\"\\41 a b\""),
            vec![Token::String(String::from("Aa b")), Token::Eof]
        );
    }

    #[test]
    fn test_numeric_tokens() {
        assert_eq!(
            get_all_tokens("4.5 3\\70\\78  4.3rem 40vh 98% 2.0e-2 -2.0e-2 +2.0e-2"),
            vec![
                Token::Number(4.5),
                Token::Whitespace,
                Token::Dimension(3.0, NumberType::Integer, String::from("px")),
                Token::Whitespace,
                Token::Dimension(4.3, NumberType::Number, String::from("rem")),
                Token::Whitespace,
                Token::Dimension(40.0, NumberType::Integer, String::from("vh")),
                Token::Whitespace,
                Token::Percentage(98.0),
                Token::Whitespace,
                Token::Number(2.0e-2),
                Token::Whitespace,
                Token::Number(-2.0e-2),
                Token::Whitespace,
                Token::Number(2.0e-2),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_full_stop() {
        assert_eq!(
            get_all_tokens(". .+5 .-7 .-.4 .4 .+.7 --> -px"),
            vec![
                Token::Delim('.'),
                Token::Whitespace,
                Token::Delim('.'),
                Token::Number(5.0),
                Token::Whitespace,
                Token::Delim('.'),
                Token::Number(-7.0),
                Token::Whitespace,
                Token::Delim('.'),
                Token::Number(-0.4),
                Token::Whitespace,
                Token::Number(0.4),
                Token::Whitespace,
                Token::Delim('.'),
                Token::Number(0.7),
                Token::Whitespace,
                Token::Cdc,
                Token::Whitespace,
                Token::Ident(String::from("-px")),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_reverse_solidus() {
        assert_eq!(
            get_all_tokens("\\41"),
            vec![Token::Ident(String::from("A")), Token::Eof]
        );
    }

    #[test]
    fn test_at_keyword() {
        assert_eq!(
            get_all_tokens("@ @media @counter-style"),
            vec![
                Token::Delim('@'),
                Token::Whitespace,
                Token::AtKeyword(String::from("media")),
                Token::Whitespace,
                Token::AtKeyword(String::from("counter-style")),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_functions_and_urls() {
        assert_eq!(
            get_all_tokens("url(foo\") rgb(255, 255, 255) url(https://example\\41.com)"),
            vec![
                Token::BadUrl,
                Token::Whitespace,
                Token::Function(String::from("rgb")),
                Token::Number(255.0),
                Token::Comma,
                Token::Whitespace,
                Token::Number(255.0),
                Token::Comma,
                Token::Whitespace,
                Token::Number(255.0),
                Token::CloseParen,
                Token::Whitespace,
                Token::Url(String::from("https://exampleA.com")),
                Token::Eof
            ]
        )
    }

    #[test]
    fn test_comments() {
        assert_eq!(
            get_all_tokens("/* text1 */ /* text2 *//* text3 */ /* text4"),
            vec![Token::Whitespace, Token::Whitespace, Token::Eof]
        );
    }

    #[test]
    fn test_css_rule() {
        assert_eq!(
            get_all_tokens("/*  text1   */@media (min-width: 600px)/*text2*//*text3 */ { body { background-color: lightblue; } }"),
            vec![
                Token::AtKeyword(String::from("media")),
                Token::Whitespace,
                Token::OpenParen,
                Token::Ident(String::from("min-width")),
                Token::Colon,
                Token::Whitespace,
                Token::Dimension(600.0, NumberType::Integer, String::from("px")),
                Token::CloseParen,
                Token::Whitespace,
                Token::OpenCurlyBracket,
                Token::Whitespace,
                Token::Ident(String::from("body")),
                Token::Whitespace,
                Token::OpenCurlyBracket,
                Token::Whitespace,
                Token::Ident(String::from("background-color")),
                Token::Colon,
                Token::Whitespace,
                Token::Ident(String::from("lightblue")),
                Token::Semicolon,
                Token::Whitespace,
                Token::CloseCurlyBracket,
                Token::Whitespace,
                Token::CloseCurlyBracket,
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_convert_repr() {
        let tokenizer = Tokenizer::new("4.5");

        assert!((tokenizer.convert_repr("4.57e-5".to_string()) - 4.57e-5).abs() < 0.0000001);
        assert!((tokenizer.convert_repr("+824.57e+2".to_string()) - 824.57e+2).abs() < 0.0000001);
    }
}
