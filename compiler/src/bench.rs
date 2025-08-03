use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::Cell;

mod signals;
mod jsx;
mod emitter;

use signals::Signal;
use jsx::parse_jsx;
use emitter::emit_js;

fn time_it<F: FnOnce()>(f: F) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn bench_signals() {
    let sig = Signal::new(0);
    let count = Rc::new(Cell::new(0));

    signals::effect({
        let sig = sig.clone();
        let count = count.clone();
        move || {
            let _ = sig.get();
            count.set(count.get() + 1);
        }
    });

    let dur = time_it(|| {
        for i in 0..1_000_000 {
            sig.set(i);
        }
    });

    println!("Signal set + notify 1,000,000 times: {:?}", dur);
}

fn bench_jsx_parse() {
    let large_jsx = include_str!("../examples/1m-signals.jsx");
    let dur = time_it(|| {
        for _ in 0..100 {
            let ast = parse_jsx(large_jsx);
            std::hint::black_box(ast);
        }
    });
    println!("Parse large JSX 100 times: {:?}", dur);
}

fn bench_emitter() {
    let large_jsx = include_str!("../examples/1m-signals.jsx");
    let ast = parse_jsx(large_jsx);

    let dur = time_it(|| {
        for _ in 0..100 {
            let js_code = emit_js(&ast);
            std::hint::black_box(js_code);
        }
    });

    println!("Emit JS from AST 100 times: {:?}", dur);
}

fn main() {
    println!("Starting extreme speed benchmarks...");
    bench_signals();
    bench_jsx_parse();
    bench_emitter();
    println!("Benchmarks done.");
}
