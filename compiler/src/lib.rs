// lib.rs — nullreact Rust core compiler API — max perf & fully working

pub mod jsx;
pub mod emitter;
pub mod signals;

pub use jsx::parse_jsx;
pub use emitter::{Emitter, emit_js};
pub use signals::{Signal, effect};

use crate::jsx::JSXNode;

/// Parse JSX source string into AST
#[inline(always)]
pub fn parse(input: &str) -> JSXNode {
    parse_jsx(input)
}

/// Emit JavaScript code string from JSX AST
#[inline(always)]
pub fn emit(ast: &JSXNode) -> String {
    emit_js(ast)
}

/// Full compile pipeline: parse JSX source + emit JS code
#[inline(always)]
pub fn compile(input: &str) -> String {
    let ast = parse(input);
    emit(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_emit() {
        let jsx = r#"<div id="root"><p>Hello {name}</p></div>"#;
        let ast = parse(jsx);
        let js = emit(&ast);

        assert!(js.contains("React.createElement"));
        assert!(js.contains("\"div\""));
        assert!(js.contains("\"id\""));
        assert!(js.contains("\"root\""));
        assert!(js.contains("\"Hello \""));
        assert!(js.contains("name")); // expression
    }

    #[test]
    fn test_signal_reactivity() {
        let sig = Signal::new(0);
        let mut count = 0;

        let _effect = effect(|| {
            let _val = sig.get();
            count += 1;
        });

        assert_eq!(count, 1);

        sig.set(10);

        // Effects run immediately on set, so count increments
        assert!(count >= 2);
    }
}
