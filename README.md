# NullReact: Compiler-First Reactive UI Engine

**Zero-runtime. Rust-powered. JSX-in, reactive JavaScript out.**

---

## Why NullReact

NullReact is not a framework — it’s a **reactivity compiler**. It transforms standard JSX into **fine-grained reactive JavaScript** without any virtual DOM, and with an optional runtime footprint of under 1KB.

**Zero-runtime JSX + Signals Compiler**  
A fast, minimal reactive engine and compiler-first JSX runtime built for **extreme speed**, **fine-grained reactivity**, and **<1KB runtime output**.

Built with ❤️ in **Rust + JS**. Inspired by SolidJS, Qwik, and React — reimagined from scratch.
- **Compiler-First** — All reactivity is handled at build time, not runtime.
- **No Virtual DOM** — Updates are precise and direct; no diffing or reconciliation.
- **Minimal Runtime (<1KB)** — Optional runtime handles hydration, batching, and effects.
- **Rust-Powered** — Fast, parallel compilation with full memory safety.
- **JSX-Compatible** — Familiar syntax, radically faster output.

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
│   ├── Cargo.toml              # Rust project config
│   └── Makefile                # Build, test, benchmark commands
│
├── runtime/                    # Lightweight DOM runtime (optional)
│   ├── dist/
│   │   └── runtime.js          # Compiled ESM runtime
│   ├── signals.js              # Signal system in JavaScript
│   ├── dom.js                  # DOM operations (insert/update/remove)
│   └── index.js                # Runtime entrypoint
│
├── examples/                   # JSX demos compiled by CLI
│   ├── 1m-signals.jsx
│   ├── 10k-filter.jsx
│   └── ui-benchmark.jsx
│
├── website/                    # Playground and demo UI
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

### 2. Compile JSX to Reactive JS

```bash
./target/release/nullreact examples/10k-filter.jsx -o dist/
```

### 3. Run in the Browser

Use the output JavaScript with `runtime/index.js` inside an HTML page.  
Example: `website/playground.html`.

---

## Benchmarks

Measured on real devices using raw DOM operations.

| Example            | NullReact       | React        |
|--------------------|------------------|--------------|
| 1M Signals         | ~19 ms           | Crashes      |
| 10K Filter Updates | ~6 ms            | ~180 ms      |
| UI Update Latency  | ~0.8 ms          | ~22 ms       |
| Runtime Size       | <1 KB            | ~43 KB       |

To run all benchmarks:

```bash
make bench
```

---

## Architecture

- **JSX Compiler** — Parses and transforms JSX to reactive instructions.
- **Signal Graph** — Updates are tracked and scheduled precisely.
- **Code Emitter** — Outputs pure JavaScript; runtime optional.
- **Runtime** — Handles hydration, batching, and effect flushing (if needed).

---

## Technology Stack

- **Rust** — Compiler backend, CLI, AST handling.
- **Signals** — Low-level reactivity system (in both Rust and JS).
- **JavaScript** — ESM-compatible output, no framework required.
- **No Babel, No React, No Overhead** — Just direct DOM instructions.

---

## Contributing & Testing

To test and compare performance:

```bash
make bench
```

You can modify or add new JSX files inside `examples/` and compile them using the CLI tool.

---

## Author

**Pawan Pediredla**  
GitHub: [@pavanscales](https://github.com/pavanscales)  
Email: pawanpediredla

---

## License

MIT License — Free to use, fork, and contribute.
