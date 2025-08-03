use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum JSXNode {
    Element {
        tag: String,
        attrs: HashMap<String, String>,
        children: Vec<JSXNode>,
    },
    Text(String),
    Expression(String),
}

pub fn parse_jsx(input: &str) -> JSXNode {
    let mut chars = input.chars().peekable();
    parse_element(&mut chars)
}

fn parse_element<I>(chars: &mut std::iter::Peekable<I>) -> JSXNode
where
    I: Iterator<Item = char>,
{
    skip_ws(chars);
    expect(chars, '<');

    let tag = parse_ident(chars);
    let attrs = parse_attrs(chars);

    let self_closing = match chars.peek() {
        Some('/') => {
            chars.next();
            expect(chars, '>');
            true
        }
        Some('>') => {
            chars.next();
            false
        }
        _ => panic!("Invalid JSX format"),
    };

    let mut children = vec![];

    if !self_closing {
        loop {
            skip_ws(chars);
            match chars.peek() {
                Some('<') if peek_match(chars, "</") => {
                    consume(chars, "</");
                    parse_ident(chars);
                    expect(chars, '>');
                    break;
                }
                Some('<') => children.push(parse_element(chars)),
                Some('{') => {
                    chars.next(); // consume {
                    let expr = consume_until(chars, '}');
                    chars.next(); // }
                    children.push(JSXNode::Expression(expr.trim().to_string()));
                }
                Some(_) => {
                    let text = consume_text(chars);
                    if !text.trim().is_empty() {
                        children.push(JSXNode::Text(text));
                    }
                }
                None => break,
            }
        }
    }

    JSXNode::Element { tag, attrs, children }
}

fn parse_ident<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut ident = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '-' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    ident
}

fn parse_attrs<I>(chars: &mut std::iter::Peekable<I>) -> HashMap<String, String>
where
    I: Iterator<Item = char>,
{
    let mut attrs = HashMap::new();
    loop {
        skip_ws(chars);
        match chars.peek() {
            Some('>') | Some('/') => break,
            Some(_) => {
                let name = parse_ident(chars);
                skip_ws(chars);
                expect(chars, '=');
                skip_ws(chars);
                let quote = chars.next().unwrap();
                assert!(quote == '"' || quote == '\'');
                let value = consume_until(chars, quote);
                chars.next(); // closing quote
                attrs.insert(name, value);
            }
            None => break,
        }
    }
    attrs
}

fn skip_ws<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) {
    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }
}

fn consume_until<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>, end: char) -> String {
    let mut out = String::new();
    while let Some(&c) = chars.peek() {
        if c == end {
            break;
        }
        out.push(c);
        chars.next();
    }
    out
}

fn consume_text<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> String {
    let mut out = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '<' || ch == '{' {
            break;
        }
        out.push(ch);
        chars.next();
    }
    out
}

fn peek_match<I: Iterator<Item = char> + Clone>(chars: &std::iter::Peekable<I>, pat: &str) -> bool {
    let mut clone = chars.clone();
    for c in pat.chars() {
        if clone.next() != Some(c) {
            return false;
        }
    }
    true
}

fn consume<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>, pat: &str) {
    for expected in pat.chars() {
        assert_eq!(chars.next(), Some(expected));
    }
}

fn expect<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>, expected: char) {
    match chars.next() {
        Some(c) if c == expected => {}
        _ => panic!("Expected '{}'", expected),
    }
}
