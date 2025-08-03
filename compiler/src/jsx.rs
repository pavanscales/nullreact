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
    consume_whitespace(chars);
    assert_eq!(chars.next(), Some('<'));

    let tag = parse_identifier(chars);
    let attrs = parse_attributes(chars);

    let is_self_closing = if let Some('/') = chars.peek() {
        chars.next();
        assert_eq!(chars.next(), Some('>'));
        true
    } else {
        assert_eq!(chars.next(), Some('>'));
        false
    };

    let mut children = vec![];

    if !is_self_closing {
        loop {
            consume_whitespace(chars);

            match chars.peek() {
                Some('<') => {
                    if lookahead(chars, "</") {
                        consume_until(chars, '>');
                        chars.next(); // consume '>'
                        break;
                    } else {
                        children.push(parse_element(chars));
                    }
                }
                Some('{') => {
                    chars.next(); // {
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

    JSXNode::Element {
        tag,
        attrs,
        children,
    }
}

fn parse_identifier<I>(chars: &mut std::iter::Peekable<I>) -> String
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

fn parse_attributes<I>(chars: &mut std::iter::Peekable<I>) -> HashMap<String, String>
where
    I: Iterator<Item = char>,
{
    let mut attrs = HashMap::new();
    loop {
        consume_whitespace(chars);
        match chars.peek() {
            Some('>') | Some('/') => break,
            Some(_) => {
                let name = parse_identifier(chars);
                consume_whitespace(chars);
                assert_eq!(chars.next(), Some('='));
                consume_whitespace(chars);
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

fn consume_whitespace<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
        chars.next();
    }
}

fn consume_until<I>(chars: &mut std::iter::Peekable<I>, end: char) -> String
where
    I: Iterator<Item = char>,
{
    let mut result = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == end {
            break;
        }
        result.push(ch);
        chars.next();
    }
    result
}

fn consume_text<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut result = String::new();
    while let Some(&ch) = chars.peek() {
        if ch == '<' || ch == '{' {
            break;
        }
        result.push(ch);
        chars.next();
    }
    result
}

fn lookahead<I>(chars: &mut std::iter::Peekable<I>, pat: &str) -> bool
where
    I: Iterator<Item = char> + Clone,
{
    let mut clone = chars.clone();
    for c in pat.chars() {
        if clone.next() != Some(c) {
            return false;
        }
    }
    true
}
