# nullReact

> 🧬 Zero-Runtime · ⚡ Compiler-First · 🧠 Fine-Grained Reactivity  
> Built in 🦀 Rust. Outputs `<1KB` JS. Blazing-fast.

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Built with Rust](https://img.shields.io/badge/built%20with-rust-orange)](https://www.rust-lang.org)
[![runtime](https://img.shields.io/badge/runtime-<1KB-green)](#runtime)

---

`nullReact` is a new kind of web framework — **not a library**, but a **compiler-first engine**.  
It compiles JSX into **zero-runtime**, **reactive JavaScript**, with **fine-grained signal-based updates** — all under 1KB.

Perfect for those who demand 🧠 minimalism, ⚡ raw performance, and 🛠️ full control.

---

## 🚀 Why nullReact?

- 🛠️ **Compiler-First** — JSX is compiled to highly-optimized JavaScript.
- 🧬 **Signals + Effects** — Fine-grained reactivity without component re-renders.
- 🚫 **No Virtual DOM** — Direct DOM operations. Real speed.
- 📦 **Minimal Runtime** — Optional runtime under **1KB**.
- 🦀 **Rust-Powered** — Fast, safe, and parallel compilation.

---

## 🔧 Quick Start

```bash
# 🌀 Clone the repo
git clone https://github.com/pavanscales/nullreact
cd nullreact

# 🛠 Build the compiler
make build

# ⏱ Run benchmarks
make bench
📁 Project Structure
graphql
Copy
Edit
nullreact/
├── compiler/                   # ⚙️ Rust-based compiler: parses JSX → signals
│   ├── src/
│   │   ├── lib.rs              # Entry point + CLI tool
│   │   ├── jsx.rs              # JSX parser to AST
│   │   ├── emitter.rs          # AST → optimized JavaScript
│   │   ├── signals.rs          # Core reactivity engine (signals, batching)
│   │   ├── bench.rs            # Performance test suites
│   │   └── runtime.js          # Shared runtime JS (for hydration/hooks)
│   ├── Cargo.toml              # Rust crate definition
│   └── Makefile                # Build/test/benchmark commands
│
├── runtime/                    # 🧪 Minimal JS runtime for DOM updates
│   ├── dist/
│   │   └── runtime.js          # Compiled runtime bundle (<1KB)
│   ├── signals.js              # Signals in JavaScript (for fallback)
│   ├── dom.js                  # Reactive DOM API (insert/update/remove)
│   └── index.js                # Entrypoint for apps using JS runtime
│
├── examples/                   # 🧩 Real-world demos compiled by the CLI
│   ├── 1m-signals.jsx          # Benchmarks 1M reactive signals
│   ├── 10k-filter.jsx          # List filtering performance
│   └── ui-benchmark.jsx        # High-frequency DOM ops test
│
├── website/                    # 🌍 Playground or landing page
│   ├── public/
│   │   └── playground.html     # Try JSX-to-JS in the browser
│   ├── src/
│   │   └── index.ts            # Playground compiler logic
│   └── vite.config.ts          # Local dev setup via Vite
│
├── README.md                   # 📘 Project overview and usage
├── LICENSE                     # 📄 MIT open-source license
└── .gitignore                  # 🔒 Ignore build, temp, and cache files
📊 Benchmarks
🧪 Test	🧱 React	⚡ nullReact
1 Million Signals	❌ Crashes	✅ Instant
10k Filter Updates	⚠️ Laggy	✅ Smooth
Cold Start Runtime	~43KB	<1KB

⏱ Run all tests: make bench

📣 Philosophy
🔥 Reactivity should be compiled, not interpreted.

🧩 Runtimes should be opt-in, not bloated.

📏 Performance should be measurable, not theoretical.

📜 License
MIT © Pawan Pediredla

💬 Get Involved
Found bugs 🐞? Have ideas 💡? Want to shape the future of reactivity?

👥 Join Discussions

🐛 Create an Issue

⭐ Star the Repo

Built with ❤️ to change how the web thinks about reactivity.