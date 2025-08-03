use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JSXNode<'a> {
    Element {
        tag: &'a str,
        attrs: HashMap<&'a str, &'a str>,
        children: Vec<JSXNode<'a>>,
    },
    Text(&'a str),
    Expression(&'a str),
}

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
    len: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            len: input.len(),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.len {
            None
        } else {
            let ch = self.peek_char()?;
            self.pos += ch.len_utf8();
            Some(ch)
        }
    }

    fn skip_ws(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn expect_char(&mut self, expected: char) {
        match self.next_char() {
            Some(ch) if ch == expected => {}
            _ => panic!("Expected '{}'", expected),
        }
    }

    fn parse_ident(&mut self) -> &'a str {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '-' {
                self.next_char();
            } else {
                break;
            }
        }
        &self.input[start..self.pos]
    }

    fn parse_until(&mut self, end: char) -> &'a str {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch == end {
                break;
            }
            self.next_char();
        }
        &self.input[start..self.pos]
    }

    fn consume_str(&mut self, s: &str) -> bool {
        if self.input[self.pos..].starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    pub fn parse_element(&mut self) -> JSXNode<'a> {
        self.skip_ws();
        self.expect_char('<');

        let tag = self.parse_ident();
        let attrs = self.parse_attrs();

        let self_closing = if self.consume_str("/>") {
            true
        } else {
            self.expect_char('>');
            false
        };

        let mut children = Vec::new();

        if !self_closing {
            loop {
                self.skip_ws();
                if self.consume_str("</") {
                    let close_tag = self.parse_ident();
                    assert_eq!(close_tag, tag, "Mismatched closing tag");
                    self.expect_char('>');
                    break;
                } else if self.peek_char() == Some('<') {
                    children.push(self.parse_element());
                } else if self.peek_char() == Some('{') {
                    self.next_char(); // consume '{'
                    let expr = self.parse_until('}');
                    self.expect_char('}');
                    children.push(JSXNode::Expression(expr.trim()));
                } else if self.peek_char().is_some() {
                    let text = self.parse_text();
                    if !text.trim().is_empty() {
                        children.push(JSXNode::Text(text));
                    }
                } else {
                    break;
                }
            }
        }

        JSXNode::Element { tag, attrs, children }
    }

    fn parse_attrs(&mut self) -> HashMap<&'a str, &'a str> {
        let mut attrs = HashMap::new();
        loop {
            self.skip_ws();
            let ch = self.peek_char();
            if ch == Some('>') || ch == Some('/') || ch.is_none() {
                break;
            }
            let name = self.parse_ident();
            self.skip_ws();
            self.expect_char('=');
            self.skip_ws();
            let quote = self.next_char().expect("Expected quote");
            assert!(quote == '"' || quote == '\'');
            let value = self.parse_until(quote);
            self.expect_char(quote);
            attrs.insert(name, value);
        }
        attrs
    }

    fn parse_text(&mut self) -> &'a str {
        let start = self.pos;
        while let Some(ch) = self.peek_char() {
            if ch == '<' || ch == '{' {
                break;
            }
            self.next_char();
        }
        &self.input[start..self.pos]
    }
}

pub fn parse_jsx(input: &str) -> JSXNode {
    let mut parser = Parser::new(input);
    parser.parse_element()
}
