# NullReact: Compiler-First Reactive UI Engine

**Blazing-fast. Zero-runtime. Rust-powered JSX compiler for fine-grained, DOM-first reactivity.**

---

## Why NullReact?

- **Compiler-First** — Transforms JSX into direct DOM operations at build time.
- **No Virtual DOM** — Uses signals and reactive graphs, not diffing engines.
- **Minimal Runtime (<1KB)** — Optional runtime for hydration and scheduling.
- **Rust-Powered** — Fast, parallel, and memory-safe compilation.
- **JSX-Compatible** — Familiar syntax, radically improved performance.

---

## Project Structure

```
nullreact/
├── compiler/                   # Rust JSX compiler + signal engine
│   ├── src/
│   │   ├── lib.rs              # CLI and entry point
│   │   ├── jsx.rs              # JSX → AST parser
│   │   ├── emitter.rs          # AST → optimized JS code
│   │   ├── signals.rs          # Core signal system: batching, effects
│   │   ├── bench.rs            # Performance test suite
│   │   └── runtime.js          # Shared runtime hooks
│   ├── Cargo.toml              # Rust config
│   └── Makefile                # Build, test, bench commands
│
├── runtime/                    # Lightweight DOM runtime (optional)
│   ├── dist/
│   │   └── runtime.js          # Compiled ESM runtime
│   ├── signals.js              # JS-based signal fallback
│   ├── dom.js                  # DOM insert/update/delete
│   └── index.js                # Runtime entrypoint
│
├── examples/                   # JSX demos (compiled via CLI)
│   ├── 1m-signals.jsx
│   ├── 10k-filter.jsx
│   └── ui-benchmark.jsx
│
├── website/                    # Playground & dev UI
│   ├── public/playground.html
│   ├── src/index.ts
│   └── vite.config.ts
│
├── README.md
├── LICENSE
└── .gitignore
```

---

## Getting Started

### 1. Build the Compiler

```bash
cd compiler
cargo build --release
```

### 2. Compile JSX

```bash
./target/release/nullreact examples/10k-filter.jsx -o dist/
```

### 3. Run in the Browser

Use output JS with `runtime/index.js` in any HTML page. See `/website` for example.

---

## Benchmarks

| Example           | NullReact      | React         |
|------------------|----------------|---------------|
| 1M Signals        | ~19ms          | ❌ crashes     |
| 10k Filter Updates| ~6ms           | ~180ms        |
| UI Update Latency | ~0.8ms         | ~22ms         |
| Runtime Payload   | <1KB           | ~43KB         |

Run:  
```bash
make bench
```

---

## Architecture Overview

- **Compile-Time Reactivity** — JSX is parsed and lowered into reactive primitives.
- **Signals as Dataflow** — Fine-grained updates without components.
- **Minimal Runtime** — Just hydration, effects, and scheduling.
- **No Dependencies** — Pure ESM + Rust output.

---

## Tech Stack

- 🦀 **Rust** – Compiler core, CLI, AST transform
- 🧠 **Signals** – Fine-grained reactivity model
- 🧩 **JavaScript** – Output-ready, no build tools needed
- 🧼 **No VDOM** – No diffing, just direct DOM updates

---

## Contributing & Benchmarks

Run performance tests across examples:

```bash
make bench
```

Compare results with React, Solid, Preact using included demos.

---

## Author

**Pawan Pediredla**  
GitHub: [@pavanscales](https://github.com/pavanscales)  
Email: pawanpediredla

---

## License

MIT © 2025 — Use it. Break it. Ship it.
