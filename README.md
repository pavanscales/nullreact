# nullReact

> Zero-Runtime · Compiler-First · Fine-Grained Reactivity  
> Built in Rust. Outputs `<1KB` JS. Blazing-fast.

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-rust-orange)](https://www.rust-lang.org)
[![runtime](https://img.shields.io/badge/runtime-<1KB-green)](#runtime)

---

**nullReact** is a new kind of frontend framework — not a library, but a **compiler-first engine**.  
It compiles JSX into **zero-runtime**, **reactive JavaScript** with **fine-grained updates** — all under **1KB**.

Built for developers who demand minimalism, performance, and control.

---

## Why nullReact?

- **Compiler-First** — JSX is turned into highly-optimized JavaScript at build time.
- **Signals & Effects** — Fine-grained reactivity without re-rendering components.
- **No Virtual DOM** — Real DOM operations for real performance.
- **Optional Runtime** — Ships a runtime under **1KB**, or none at all.
- **Rust-Powered** — Fast, safe, and parallel compilation pipeline.

---

## Quick Start

```bash
# Clone the repo
git clone https://github.com/pavanscales/nullreact
cd nullreact

# Build the compiler
make build

# Run performance benchmarks
make bench
Project Structure
graphql
Copy
Edit
nullreact/
├── compiler/                   # Rust-based JSX compiler + signal engine
│   ├── src/
│   │   ├── lib.rs              # Compiler CLI entry point
│   │   ├── jsx.rs              # JSX → AST parser
│   │   ├── emitter.rs          # AST → JavaScript emitter
│   │   ├── signals.rs          # Signals, batching, effects engine
│   │   ├── bench.rs            # Micro-benchmark suite
│   │   └── runtime.js          # Shared JS runtime for hydration
│   ├── Cargo.toml              # Rust crate config
│   └── Makefile                # Build, test, and bench commands
│
├── runtime/                    # JavaScript runtime for client-side DOM
│   ├── dist/
│   │   └── runtime.js          # Compiled bundle (<1KB)
│   ├── signals.js              # JS version of signal system
│   ├── dom.js                  # DOM mutation utilities
│   └── index.js                # Runtime entrypoint
│
├── examples/                   # JSX demos compiled by CLI
│   ├── 1m-signals.jsx          # 1 million signal stress test
│   ├── 10k-filter.jsx          # List filtering demo
│   └── ui-benchmark.jsx        # High-frequency DOM updates
│
├── website/                    # Playground and landing page
│   ├── public/
│   │   └── playground.html     # In-browser JSX → JS compiler
│   ├── src/
│   │   └── index.ts            # Playground logic
│   └── vite.config.ts          # Dev server config
│
├── README.md                   # Project documentation
├── LICENSE                     # MIT license
└── .gitignore                  # Ignore build/temp artifacts
Benchmarks
Test	React	nullReact
1 Million Signals	Crashes	Instant
10k Filter Updates	Laggy	Smooth
Cold Start Runtime	~43KB	<1KB

Run benchmarks with: make bench

Philosophy
Reactivity should be compiled, not interpreted.

Runtimes should be opt-in, not required.

Performance should be measured, not assumed.

License
MIT © Pawan Pediredla

Get Involved
Have ideas? Found bugs? Want to shape the future of reactive UI?

Join Discussions

Report Issues

Star the Repo

Built to change how the web thinks about reactivity.