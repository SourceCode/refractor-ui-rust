# Contributing to Refractor UI for Rust

Thank you for contributing to **Refractor Optical Glass UI Framework for Rust**!

## Standards & Governance

1. Read [`AGENTS.md`](./AGENTS.md) and [`docs/adr/README.md`](./docs/adr/README.md) before making changes.
2. Follow all six Architecture Decision Records (`ADR-0001` through `ADR-0006`):
   - `#![forbid(unsafe_code)]` across all crates.
   - Zero `unwrap()`, `expect()`, or `panic!()` in library code.
   - Every component must provide typed Props with `BaseProps`, a GPUI `RenderOnce` implementation, structured `ComponentDoc` metadata, and positive/negative/adversarial tests.
3. Verify all gates before opening a pull request:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo run -p refractor-xtask -- check-repo
   ```
