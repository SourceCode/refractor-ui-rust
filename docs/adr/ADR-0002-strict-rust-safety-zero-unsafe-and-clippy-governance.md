# ADR-0002: Strict Rust Safety (Zero `unsafe`, Zero `unwrap`/`panic`) & Two-Tier Clippy Governance

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Refractor Principal Architecture
- **Applies To:** All Rust crates and binaries in `refractor-ui-rust`

---

## 1. Context

Just as `glass-ui-v1` enforces zero `any` and zero `unknown` in TypeScript to eliminate runtime surprises and improve AI agent reliability, a native Rust UI framework must eliminate memory-safety escape hatches (`unsafe`), runtime panics (`unwrap`, `expect`, `panic!`, `todo!`), and implicit type erosion.

## 2. Directives (Mandatory Invariants)

1. **Zero `unsafe` Policy:**
   - `[workspace.lints.rust] unsafe_code = "forbid"` is enabled workspace-wide and inherited by every crate via `[lints] workspace = true`.
2. **Zero Panic Escape Hatches in Production Code:**
   - `clippy::unwrap_used = "deny"`, `clippy::expect_used = "deny"`, `clippy::panic = "deny"`, `clippy::todo = "deny"`, and `clippy::unimplemented = "deny"` are enforced in `[workspace.lints.clippy]`.
   - All parsing, color math, sanitization, theme lookup, and UI state transitions MUST return typed `Option`/`Result` values or deterministic fallbacks.
3. **Strong Algebraic Modeling:**
   - Every theme material, overlay FX, theme ID, component category, button size, alert severity, and control state is modeled as an exhaustive Rust `enum` or strongly typed struct — never untyped string bags or dynamic `Box<dyn Any>` escape hatches.
4. **Two-Tier Linting Philosophy:**
   - **Local Development:** `cargo clippy --workspace --all-targets` surfaces pedantic advisories as warnings while keeping soundness/panic rules as hard errors.
   - **CI & AI Agents (Strict Mode):** `cargo clippy --workspace --all-targets -- -D warnings` elevates every warning to a hard error. AI agents and CI MUST pass with 0 errors and 0 warnings.

## 3. Enforcement

- Enforced by `[workspace.lints]` in root `Cargo.toml`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo run -p refractor-xtask -- check-repo`.
