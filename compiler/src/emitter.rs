use std::collections::HashMap;
use std::ptr;

#[derive(Debug)]
pub enum JSXNode<'a> {
    Element {
        tag: &'a str,
        attrs: HashMap<&'a str, &'a str>,
        children: Vec<JSXNode<'a>>,
    },
    Text(&'a str),
    Expression(&'a str),
}

pub struct Emitter {
    buf: Vec<u8>, // raw output buffer
    pos: usize,   // current write position
    cap: usize,   // capacity (for bounds check)
}

impl Emitter {
    /// Create a new emitter with given capacity (pre-allocated buffer)
    pub fn with_capacity(cap: usize) -> Self {
        let mut buf = Vec::with_capacity(cap);
        unsafe { buf.set_len(cap); } // set len upfront to allow unsafe writes
        Self { buf, pos: 0, cap }
    }

    #[inline(always)]
    fn push_byte(&mut self, b: u8) {
        debug_assert!(self.pos < self.cap);
        unsafe {
            *self.buf.get_unchecked_mut(self.pos) = b;
        }
        self.pos += 1;
    }

    #[inline(always)]
    fn push_bytes(&mut self, bytes: &[u8]) {
        debug_assert!(self.pos + bytes.len() <= self.cap);
        unsafe {
            let dst = self.buf.as_mut_ptr().add(self.pos);
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len());
        }
        self.pos += bytes.len();
    }

    /// Emit a string with JS escaping (only basic escaping for quotes, backslash, and newlines)
    fn emit_string_escaped(&mut self, s: &str) {
        self.push_byte(b'"');
        for &b in s.as_bytes() {
            match b {
                b'\\' => self.push_bytes(b"\\\\"),
                b'"' => self.push_bytes(b"\\\""),
                b'\n' => self.push_bytes(b"\\n"),
                b'\r' => self.push_bytes(b"\\r"),
                b'\t' => self.push_bytes(b"\\t"),
                _ => self.push_byte(b),
            }
        }
        self.push_byte(b'"');
    }

    /// Emit attributes as JS object literal, or null if empty
    fn emit_attrs(&mut self, attrs: &HashMap<&str, &str>) {
        if attrs.is_empty() {
            self.push_bytes(b"null");
            return;
        }

        self.push_byte(b'{');

        let mut first = true;
        for (&k, &v) in attrs.iter() {
            if !first {
                self.push_bytes(b", ");
            }
            self.emit_string_escaped(k);
            self.push_bytes(b": ");
            self.emit_string_escaped(v);
            first = false;
        }

        self.push_byte(b'}');
    }

    /// Emit the JSXNode into the buffer as React.createElement calls
    pub fn emit_node(&mut self, node: &JSXNode) {
        match node {
            JSXNode::Element { tag, attrs, children } => {
                self.push_bytes(b"React.createElement(");
                self.emit_string_escaped(tag);
                self.push_bytes(b", ");
                self.emit_attrs(attrs);
                self.push_bytes(b", ");

                if children.is_empty() {
                    self.push_bytes(b"null");
                } else {
                    for (i, child) in children.iter().enumerate() {
                        if i > 0 {
                            self.push_bytes(b", ");
                        }
                        self.emit_node(child);
                    }
                }

                self.push_byte(b')');
            }
            JSXNode::Text(text) => {
                self.emit_string_escaped(text);
            }
            JSXNode::Expression(expr) => {
                self.push_bytes(expr.as_bytes());
            }
        }
    }

    /// Finalize and convert internal buffer to String
    pub fn finish(mut self) -> String {
        unsafe {
            self.buf.set_len(self.pos);
        }
        // Safety: all emitted bytes are valid UTF-8 (assuming valid input strings)
        unsafe { String::from_utf8_unchecked(self.buf) }
    }
}
