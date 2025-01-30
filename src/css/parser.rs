use core::panic;

use super::types::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Value};

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
        self.consume_until_and_return(|c| !c.is_alphanumeric())
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

    fn consume_number(&mut self) -> u8 {
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

            Value::ColorValue(Color { r, g, b })
        } else {
            Value::Keyword(self.consume_until_and_return(|c| c == ';'))
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

        if self.consume_next_code_point() != Some(';') {
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
                        declarations: vec![Declaration {
                            name: "color".to_string(),
                            value: Value::Keyword("red".to_string()),
                        },],
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
                                value: Value::ColorValue(Color { r: 0, g: 0, b: 255 }),
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
                                value: Value::ColorValue(Color { r: 0, g: 0, b: 255 }),
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
                                value: Value::ColorValue(Color { r: 0, g: 0, b: 255 }),
                            },
                        ],
                    },
                ]
            }
        );
    }
}
