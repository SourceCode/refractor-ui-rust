# Refractor Optical Glass UI Framework for Rust — Repository Guidance (AGENTS.md)

This `AGENTS.md` is the canonical, tool-agnostic instruction surface for AI coding agents and human contributors working in **Refractor UI for Rust** (`refractor-ui-rust`).

## Architectural Blueprint (Open-Internals + GPUI + Iced Ergonomics)

Refractor for Rust adopts the Open-Internals component architecture and engineering discipline from `refractor-ui`, executed natively on **Zed's GPUI** GPU-accelerated rendering engine with **Iced-inspired** declarative application architecture (`RefractorApp`, `Task`, `Subscription`):

- **Cargo Workspace Layout:**
  - `crates/refractor-security`: Defense-in-depth security primitives (`sanitize_url`, `sanitize_html`, `sanitize_csv_cell`, `build_safe_csv`, `clamp_f32`/`clamp_f64`, `is_safe_object_key`, `split_literal_highlight`, safe swizzle path validation). Zero external dependencies.
  - `crates/refractor-tokens`: Pure Rust color math, WCAG 2.2 AA contrast derivation, `PrismTheme`, `create_prism_theme`, `define_theme`, 13 `ThemeMaterial` archetypes, 9 `ThemeOverlayFx` overlays, `ThemeFxConfig`, `ThemeMotionConfig`, `ThemeEditorColors`.
  - `crates/refractor-themes`: Complete 67-theme catalog (5 Optical Glass presets + 62 multi-material themes across 9 categories).
  - `crates/refractor-core`: GPUI integration (`ThemeGlobal`, concentric SDF `optical_surface` GPU material painter for all 13 materials, `OpticalBackdrop`, `FxOverlay` for all 9 overlays, `BaseProps`, `StyleOverride`, `ComponentDoc`, `ComponentRegistry`, IME-safe helpers) + Iced-inspired `RefractorApp` / `Task` / `Subscription` runtime.
  - `crates/refractor-components`: Complete 339-component catalog across all 58 categories with Open-Internals parity, GPUI `RenderOnce` implementations (including horizontal `MenuBarTrigger`/`MenuDropdownItem`, `WindowTitlebar` with `WindowChromeStyle`, and all 25 Dynamic Gradient & AI-Glow FX, Shader & Optical FX, and Loaders/Skeletons/Transitions components), typed Props, and `ComponentDoc` metadata.
  - `crates/refractor-cli`: Agent-ready CLI (`component`, `docs`, `theme`, `swizzle`, `check`).
  - `crates/refractor-xtask`: Governance & drift checker (`cargo run -p refractor-xtask -- check-repo`).
  - `apps/task-manager`: Interactive GPUI Optical Glass Task Manager with horizontal menu bar, live Visual FX Studio controls, and embedded Windows `app.ico`.
  - `apps/workbench`: Interactive Desktop Optical Workbench & Theme Studio GPUI application.
  - `apps/studio-showcase`: Multi-suite GPUI Desktop Showcase rendering all 7 bespoke component domain suites.
  - `docs/`: GitHub Pages site (`docs/index.html`) and authoritative Architecture Decision Records (`docs/adr/ADR-0001` through `ADR-0006`).

## Non-Negotiable Coding Rules

1. **Zero `unsafe` Code & Zero Panic Escape Hatches:**
   - Every crate MUST declare `#![forbid(unsafe_code)]` (inherited from workspace lints).
   - `clippy::unwrap_used`, `clippy::expect_used`, `clippy::panic`, `clippy::todo`, and `clippy::unimplemented` are **denied** in production code. Use explicit `Result`, `Option` combinators, and deterministic fallbacks.
2. **Open-Internals Component Contract:**
   - Every component in `refractor-components` MUST expose:
     - Strongly typed Props struct embedding `BaseProps` with fluent builder methods.
     - GPUI `RenderOnce` implementation reading active `ThemeGlobal` and respecting per-component `ComponentStyleOverride`.
     - Structured `ComponentDoc` metadata (`name`, `display_name`, `group`, `category`, `keywords`, `usage`, `best_practices`, `accessibility`, `props`, `examples`) registered in `all_component_docs()`.
     - Positive, negative, and adversarial tests.
3. **Two-Tier Linting:**
   - Local human dev: `cargo clippy --workspace --all-targets`.
   - CI & AI agents: `cargo clippy --workspace --all-targets -- -D warnings` (0 errors, 0 warnings).
4. **Escalation Path (Open Internals):**
   1. Typed Props & fluent builders
   2. Theme Tokens (`create_prism_theme` single-hex derivation OR `define_theme` cascade)
   3. Theme Component Overrides (`theme.components`)
   4. `BaseProps` (`id`, `style_override`, `data_attrs`, `aria_label`) & `OpticalSurface`
   5. Swizzle (`cargo run -p refractor-cli -- swizzle <Name> --out <dir>`)
5. **Security & Adversarial Invariants:**
   - Never render unsanitized HTML, allow `javascript:`/`vbscript:`/`data:text/html` URLs, export raw CSV cells without `sanitize_csv_cell`, construct regexes from untrusted user queries, or accept `..` path traversal in CLI swizzle.
6. **SemVer 2.0.0 & Release Discipline:**
   - Keep workspace `Cargo.toml`, all member `Cargo.toml` manifests, `CHANGELOG.md`, and Git tags strictly synchronized to SemVer 2.0.0.

## Agent Bootstrap Commands

```bash
cargo run -p refractor-cli -- help
cargo run -p refractor-cli -- component --list
cargo run -p refractor-cli -- component PrimaryButton --dense
cargo run -p refractor-cli -- theme list
cargo run -p refractor-xtask -- check-repo
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo check --workspace --all-targets
```
