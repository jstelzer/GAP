# Contributing to GAP

Thanks for helping build GAP! This project aims to keep the **protocol open** and the **code easy to adopt**.

## Licenses

- **Code (Rust/Python/etc.)**: [Apache-2.0](LICENSE)
- **Spec/Docs (e.g., SPEC.md)**: [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/)

By submitting a contribution, you agree your code is licensed under Apache-2.0 and your documentation/spec changes under CC-BY-4.0. You confirm you have the right to contribute this work (no third-party restrictions).

> Why Apache-2.0? It includes an explicit **patent grant**, which helps keep GAP unencumbered as it grows.

## How to contribute

1. **Fork** the repo and create a feature branch:
```bash
   git checkout -b feat/your-thing
```

2.	Build & run locally
```
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```


3.	Open a PR
- Keep changes focused and small when possible.
- Link any related Issues.
- Include a brief rationale and screenshots/GIFs for user-visible changes.

### Coding guidelines
- Rust: stable toolchain, formatted with cargo fmt, no clippy warnings.
- Keep modules small and focused (schema, gap, world, etc.).
- Prefer clear, minimal public APIs over cleverness.

### Commit messages (lightweight)

Use clear prefixes when helpful:
- feat: new capability
- fix: bug fix
- docs: docs/spec only
- refactor:, chore:, test:
