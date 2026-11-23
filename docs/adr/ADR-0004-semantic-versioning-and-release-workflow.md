# ADR-0004: Semantic Versioning 2.0.0, Keep a Changelog & Release Governance

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Refractor Release Engineering
- **Applies To:** All Cargo manifests, Git tags, Releases, and `CHANGELOG.md`

---

## 1. Context

A multi-crate Rust UI framework must keep every workspace crate, CLI binary, example app, `CHANGELOG.md`, and Git tag strictly synchronized to Semantic Versioning 2.0.0.

## 2. Directives (Mandatory Invariants)

1. **Strict Semantic Versioning 2.0.0:**
   - Root `[workspace.package].version` and all member crates (`version.workspace = true`) MUST remain in exact lockstep.
2. **Keep a Changelog 1.1.0 (`CHANGELOG.md`):**
   - Every release MUST have a dated `## [X.Y.Z] - YYYY-MM-DD` section grouped by `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, and `Security`.
3. **Verified Release Gate:**
   - Before cutting any tag `vX.Y.Z`, all gates (`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo run -p refractor-xtask -- check-repo`) MUST exit 0.

## 3. Enforcement

- `cargo run -p refractor-xtask -- check-repo` validates SemVer 2.0.0 format, workspace lockstep, and `CHANGELOG.md` coverage.
