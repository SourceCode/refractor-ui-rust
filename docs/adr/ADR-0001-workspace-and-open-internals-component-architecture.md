# ADR-0001: Cargo Workspace Hierarchy & Open-Internals Component Architecture

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Refractor Principal Architecture & UX/UI Direction
- **Applies To:** Entire `refractor-ui-rust` workspace

---

## 1. Context

Porting Refractor Optical Glass UI Kit (`refractor-ui`) into a native Rust desktop UI framework powered by Zed's GPUI (`gpui`) and Iced-inspired application ergonomics requires strict crate boundaries so pure token math, security utilities, theme catalogs, GPU rendering, components, CLI tooling, and example apps remain independently testable, composable, and fast to compile.

## 2. Directives (Mandatory Invariants)

1. **Cargo Workspace Crate Boundaries:**
   - `crates/refractor-security`: Zero-dependency pure Rust security & sanitization primitives.
   - `crates/refractor-tokens`: Pure Rust deterministic color math, WCAG 2.2 AA derivation, `PrismTheme`, `create_prism_theme`, `define_theme`, 13 material archetypes, and 9 FX overlays. **MUST NEVER** depend on GPU or OS windowing crates.
   - `crates/refractor-themes`: All 67 full-transformation themes (5 Optical Glass presets + 62 multi-material themes).
   - `crates/refractor-core`: GPUI integration (`ThemeGlobal`, `OpticalSurface` GPU material painter, `BaseProps`, `ComponentDoc`, `ComponentRegistry`, optical shader engine) + Iced-inspired `RefractorApp` / `Task` / `Subscription` runtime.
   - `crates/refractor-components`: Complete 339-component catalog across all 58 categories. **MUST NEVER** import from `apps/*`.
   - `crates/refractor-cli`: Agent-ready CLI (`component`, `docs`, `theme`, `swizzle`, `check`).
   - `crates/refractor-xtask`: Repository governance & drift verification (`check-repo`).
   - `apps/workbench`: Interactive GPUI Desktop Optical Workbench & Theme Studio consuming public crate APIs exclusively.
2. **Open-Internals Component Contract:**
   Every component across all 58 categories MUST expose:
   - Typed Props struct embedding `BaseProps` with fluent builder methods.
   - Native GPUI `RenderOnce` implementation consuming `ThemeGlobal` and per-component `ComponentStyleOverride`.
   - Structured `ComponentDoc` metadata (`name`, `display_name`, `group`, `category`, `keywords`, `usage`, `best_practices`, `accessibility`, `props`, `examples`) registered in `all_component_docs()`.
   - Positive, negative, and adversarial tests.
3. **Five-Tier Open Internals Escalation Path:**
   1. Typed Props & fluent builders
   2. Theme Tokens (`create_prism_theme` / `define_theme`)
   3. Theme Component Overrides (`theme.components`)
   4. `BaseProps` (`id`, `style_override`, `data_attrs`, `aria_label`) & `OpticalSurface`
   5. Swizzle ejection via `refractor-cli swizzle <Name> --out <dir>`

## 3. Enforcement

- `cargo run -p refractor-xtask -- check-repo` fails CI if any crate manifest drifts, if any of the 339 components lack doc registration, or if any of the 67 themes are missing.
