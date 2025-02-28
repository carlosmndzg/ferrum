use core::panic;

use super::types::{Declaration, Rgb, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};

pub(crate) struct CssParser {
    input: Vec<char>,
    next_pos: usize,
}

impl CssParser {
    pub(crate) fn new(input: &str) -> CssParser {
        CssParser {
            input: input.chars().collect(),
            next_pos: 0,
        }
    }

    fn consume_next_code_point(&mut self) -> Option<char> {
        let next_code_point = self.input.get(self.next_pos);
        self.next_pos += 1;

        next_code_point.copied()
    }

    fn next_code_point(&self) -> Option<char> {
        self.input.get(self.next_pos).copied()
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.input.get(self.next_pos + n).copied()
    }

    fn reconsume_current_code_point(&mut self) {
        self.next_pos -= 1;
    }

    fn create_color_from_i32(&self, r: i32, g: i32, b: i32) -> Value {
        if !(0..=255).contains(&r) || !(0..=255).contains(&g) || !(0..=255).contains(&b) {
            panic!("Invalid color values");
        }

        Value::Rgb(Rgb::new(r as u8, g as u8, b as u8, 1.))
    }

    fn consume_if_starts(&mut self, s: &str) -> bool {
        let chars = s.chars();

        for (i, c) in chars.enumerate() {
            if self.peek(i) != Some(c) {
                return false;
            }
        }

        for _ in 0..s.len() {
            self.consume_next_code_point();
        }

        true
    }

    fn consume_until(&mut self, f: impl Fn(char) -> bool) {
        while let Some(code_point) = self.consume_next_code_point() {
            if f(code_point) {
                self.reconsume_current_code_point();
                break;
            }
        }
    }

    fn consume_until_and_return(&mut self, f: impl Fn(char) -> bool) -> String {
        let mut result = String::new();

        while let Some(code_point) = self.consume_next_code_point() {
            if f(code_point) {
                self.reconsume_current_code_point();
                break;
            }

            result.push(code_point);
        }

        result
    }

    fn consume_identifier(&mut self) -> String {
        self.consume_until_and_return(|c| !c.is_alphanumeric() && c != '-')
    }

    fn consume_selector(&mut self) -> Selector {
        let mut simple_selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: vec![],
        };

        loop {
            let code_point = self.consume_next_code_point();

            if code_point.is_none()
                || matches!(code_point, Some(c) if c.is_whitespace() || c == '{' || c == ',')
            {
                self.reconsume_current_code_point();
                break;
            }

            match code_point {
                Some('#') => {
                    simple_selector.id = Some(self.consume_identifier());
                }
                Some('.') => {
                    simple_selector.class.push(self.consume_identifier());
                }
                Some('*') => {}
                _ => {
                    self.reconsume_current_code_point();
                    simple_selector.tag_name = Some(self.consume_identifier());
                }
            }
        }

        Selector::Simple(simple_selector)
    }

    fn consume_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = vec![];

        loop {
            self.consume_until(|c| !c.is_whitespace());

            let code_point = self.next_code_point();

            if let Some('{') | None = code_point {
                self.consume_next_code_point();
                break;
            }

            if let Some(',') = code_point {
                self.consume_next_code_point();
                self.consume_until(|c| !c.is_whitespace());
            }

            selectors.push(self.consume_selector());
        }

        selectors
    }

    fn consume_number(&mut self) -> i32 {
        self.consume_until(|c| !c.is_whitespace());
        self.consume_until_and_return(|c| !c.is_numeric())
            .parse()
            .unwrap()
    }

    fn consume_value(&mut self) -> Value {
        if self.consume_if_starts("rgb(") {
            let r = self.consume_number();
            self.consume_next_code_point();
            let g = self.consume_number();
            self.consume_next_code_point();
            let b = self.consume_number();
            self.consume_next_code_point();

            self.create_color_from_i32(r, g, b)
        } else if matches!(self.next_code_point(), Some(c) if c.is_ascii_digit()) {
            let number = self.consume_number();

            match self.next_code_point() {
                Some('%') => {
                    self.consume_next_code_point();
                    Value::Percentage(number as f32)
                }
                _ => {
                    let unit = self.consume_identifier();

                    match unit.as_str() {
                        "px" => Value::Dimension(number as f32, Unit::Px),
                        _ => Value::Dimension(number as f32, Unit::None),
                    }
                }
            }
        } else {
            Value::Keyword(self.consume_until_and_return(|c| !c.is_alphabetic()))
        }
    }

    fn consume_declaration(&mut self) -> Declaration {
        self.consume_until(|c| !c.is_whitespace());

        let name = self.consume_identifier();

        self.consume_until(|c| !c.is_whitespace());

        if self.consume_next_code_point() != Some(':') {
            panic!("Expected ':'");
        }

        self.consume_until(|c| !c.is_whitespace());

        let value = self.consume_value();

        self.consume_until(|c| !c.is_whitespace());

        if !matches!(self.consume_next_code_point(), Some(';') | None) {
            panic!("Expected ';'");
        }

        Declaration { name, value }
    }

    fn consume_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = vec![];

        loop {
            self.consume_until(|c| !c.is_whitespace());

            let code_point = self.consume_next_code_point();

            if let None | Some('}') = code_point {
                break;
            }

            self.reconsume_current_code_point();
            declarations.push(self.consume_declaration());
        }

        declarations
    }

    fn consume_rule(&mut self) -> Vec<Rule> {
        let selectors = self.consume_selectors();
        let declarations = self.consume_declarations();

        let mut result = Vec::new();

        for selector in selectors {
            result.push(Rule {
                selector,
                declarations: declarations.clone(),
            });
        }

        result
    }

    pub(crate) fn parse(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet { rules: vec![] };

        loop {
            self.consume_until(|c| !c.is_whitespace());

            let code_point = self.next_code_point();

            if code_point.is_none() {
                break;
            }

            for rule in self.consume_rule() {
                stylesheet.rules.push(rule);
            }
        }

        stylesheet
    }

    pub(crate) fn parse_list_of_declarations(&mut self) -> Vec<Declaration> {
        self.consume_declarations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_css() {
        let mut parser = CssParser::new(
            "
            p {
                color: red;
                width: 100%;
                margin-left: 4px;
            }

            h1.title, h2, #unique {
                display: block;
                color: rgb(0, 0, 255);
            }
        ",
        );

        assert_eq!(
            parser.parse(),
            Stylesheet {
                rules: vec![
                    Rule {
                        selector: Selector::Simple(SimpleSelector {
                            tag_name: Some("p".to_string()),
                            id: None,
                            class: vec![],
                        }),
                        declarations: vec![
                            Declaration {
                                name: "color".to_string(),
                                value: Value::Keyword("red".to_string()),
                            },
                            Declaration {
                                name: "width".to_string(),
                                value: Value::Percentage(100.0),
                            },
                            Declaration {
                                name: "margin-left".to_string(),
                                value: Value::Dimension(4.0, Unit::Px),
                            }
                        ],
                    },
                    Rule {
                        selector: Selector::Simple(SimpleSelector {
                            tag_name: Some("h1".to_string()),
                            id: None,
                            class: vec!["title".to_string()],
                        }),
                        declarations: vec![
                            Declaration {
                                name: "display".to_string(),
                                value: Value::Keyword("block".to_string()),
                            },
                            Declaration {
                                name: "color".to_string(),
                                value: Value::Rgb(Rgb::new(0, 0, 255, 1.)),
                            },
                        ],
                    },
                    Rule {
                        selector: Selector::Simple(SimpleSelector {
                            tag_name: Some("h2".to_string()),
                            id: None,
                            class: vec![],
                        }),
                        declarations: vec![
                            Declaration {
                                name: "display".to_string(),
                                value: Value::Keyword("block".to_string()),
                            },
                            Declaration {
                                name: "color".to_string(),
                                value: Value::Rgb(Rgb::new(0, 0, 255, 1.)),
                            },
                        ],
                    },
                    Rule {
                        selector: Selector::Simple(SimpleSelector {
                            tag_name: None,
                            id: Some("unique".to_string()),
                            class: vec![],
                        }),
                        declarations: vec![
                            Declaration {
                                name: "display".to_string(),
                                value: Value::Keyword("block".to_string()),
                            },
                            Declaration {
                                name: "color".to_string(),
                                value: Value::Rgb(Rgb::new(0, 0, 255, 1.)),
                            },
                        ],
                    },
                ]
            }
        );
    }
}
