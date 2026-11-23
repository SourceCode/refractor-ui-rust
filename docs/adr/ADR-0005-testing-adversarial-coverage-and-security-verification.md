# ADR-0005: High-Confidence Testing, Adversarial Validation & Security Invariants

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Refractor Security & Quality Engineering
- **Applies To:** All crates and components in `refractor-ui-rust`

---

## 1. Context

Desktop applications built with Refractor process untrusted documents, rich text, CSV exports, URLs, query builders, terminal streams, and CLI swizzle targets. Happy-path smoke tests are insufficient to prevent injection, formula execution, path traversal, ReDoS, or floating-point NaN/Infinity panics.

## 2. Directives (Mandatory Invariants)

1. **Three-Pillar Testing Standard:**
   - **Positive Tests:** Happy-path state transitions, builder ergonomics, theme resolution, WCAG contrast derivation, and component doc completeness.
   - **Negative Tests:** Disabled states, empty collections, out-of-range numeric bounds, invalid hex/rgb strings, and unknown IDs.
   - **Adversarial Tests:** XSS payloads (`<script>alert(1)</script>`, `<img src=x onerror=alert(1)>`, `javascript:alert(1)`), prototype-pollution keys (`__proto__`, `constructor`, `prototype`), OWASP CSV formula payloads (`=CMD|' /C calc'!A0`, `+1+1`, `@SUM(1+1)`), `f32::NAN` / `f32::INFINITY` / `f64::NEG_INFINITY` geometry inputs, ReDoS query strings, and `../` path traversal attacks in CLI swizzle.
2. **Runtime Security Primitives (`crates/refractor-security`):**
   - `sanitize_url(raw, allow_raster_data_uri)`
   - `sanitize_html(raw)`
   - `sanitize_csv_cell(value)` & `build_safe_csv(headers, rows)`
   - `clamp_f32` / `clamp_f64`
   - `is_safe_object_key(key)`
   - `split_literal_highlight(source, query)`
   - `validate_swizzle_target(component_name, out_dir)`

## 3. Enforcement

- `cargo test --workspace` executes all unit, integration, and adversarial suites across every crate.
