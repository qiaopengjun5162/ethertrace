# Ethertrace

![Rust](https://img.shields.io/badge/Rust-98%25%20coverage-orange) ![Go](https://img.shields.io/badge/Go-Unit%20Tested-success) ![Python](https://img.shields.io/badge/Python-100%25%20coverage-brightgreen)

Multi-language Ethereum event tracer with high test coverage and flexible configuration (YAML/env vars).  
Rust: `~98%` coverage (main ignored), Go: Full unit tests, Python: `100%` coverage.

---

## 🚀 Features

• Multi-language: Rust (performance), Go (concurrency), Python (scripting)

• Config-driven:

  ```yaml
  rpc_url: "https://rpc.mevblocker.io"
  contract_addr: "0x5BD63a7ECc13b955C4F57e3F12A64c10263C14c1"
  ```

• Test Coverage:

  | Language | Coverage | Key Files |
  |----------|----------|-----------|
  | Rust     | 98%      | `load_config()`/`run()` 100% |
  | Go       | Full unit tests | `client_test.go` |
  | Python   | 100%     | `test_event_parser.py` |

---

## ⚡ Quick Start

Prerequisites

```bash
# Rust (nightly)
rustup install nightly && rustup default nightly

# Go
brew install go

# Python
uv pip install -r python/requirements.txt
```

Run All Implementations

```bash
# Rust
RUST_LOG=info cargo run --bin ethertrace

# Go
go run ./go/main.go -config ./config.yaml

# Python
uv run python/main.py
```

---

## 🔍 Testing & Coverage

Rust

```bash
cargo llvm-cov nextest --lcov --output-path lcov.info
genhtml lcov.info -o coverage_report
```

![Rust Coverage](.github/assets/rust-coverage.png) *(Example: main.rs excluded)*

Go

```bash
go test -coverprofile=coverage.out ./...
go tool cover -html=coverage.out
```

Python

```bash
pytest --cov=ethertrace --cov-report=html
```

---

## 🏗 Project Structure

```
ethertrace/
├── config.yaml              # Shared config
├── rust/                    # 🦀 High-perf implementation
│   ├── src/main.rs          # (Coverage-ignored entrypoint)
│   └── tests/integration.rs
├── go/                      # 🐹 Concurrent processor
│   ├── client_test.go       # Mocked HTTP tests
│   └── event_parser.go      
└── python/                  # 🐍 Scripting interface
    ├── requirements.txt     # Pinned deps
    └── tests/               # 100% coverage
```

---

## 📊 Coverage Reports

| Language | Command | Output | Target |
|----------|---------|--------|--------|
| Rust     | `cargo llvm-cov` | HTML/LCOV | `src/lib.rs` |
| Go       | `go test -cover` | `coverage.out` | All packages |
| Python   | `pytest --cov`   | `htmlcov/`    | `ethertrace/` |

---

## 🤝 Contributing

1. Fork → Branch → Test → PR  
2. Coverage Requirements:
   • Rust: New code must maintain ≥95% coverage

   • Python: 100% coverage enforced via CI

   • Go: Unit tests for all exported funcs

---

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
**Happy Coding!** 🚀
