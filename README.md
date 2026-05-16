# rust-attribute-macros-level-01

A hands-on learning project for Rust procedural attribute macros using `syn` and `quote`. Built step by step as part of a structured tutoring session.

---
```bash
git clone https://github.com/your-username/rust-attribute-macros
cd rust-attribute-macros-level-01
cargo run
```

---

## Macros

| Macro | Applies to | What it does |
|-------|-----------|--------------|
| `#[log_call]` | functions | Wraps with enter/exit logs using `stringify!` at runtime |
| `#[log_call_comp]` | functions | Same, but function name baked in as a string literal at compile time |
| `#[double_output]` | functions | Doubles the return value |
| `#[multiply(n)]` | functions | Multiplies the return value by `n` |
| `#[log_multiply(factor=n, log=bool)]` | functions | Multiplies return value, optionally logs - multiple parameters |
| `#[describe]` | structs | Prints struct name and field names via a `describe()` method |
| `#[builder]` | structs | Generates a `new()` constructor and `display()` method from fields |

---
## Viewing macro expansion

```bash
cargo install cargo-expand
cargo expand
```
