# Refractor Optical Glass UI Framework for Rust (`refractor-ui-rust`)

[![Release v1.0.0](https://img.shields.io/badge/release-v1.0.0-e75c24?style=flat-square)](https://github.com/SourceCode/refractor-ui-rust/releases/tag/v1.0.0)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-1d4ed8?style=flat-square)](./LICENSE)
[![Rust: 1.88+](https://img.shields.io/badge/rust-1.88%2B-26bc92?style=flat-square)](./Cargo.toml)
[![Site](https://img.shields.io/badge/docs-GitHub%20Pages-38bdf8?style=flat-square)](https://sourcecode.github.io/refractor-ui-rust/)

**A 100% native, cross-platform, GPU-accelerated Rust desktop UI framework combining the Refractor Optical Glass UI Kit (`glass-ui-v1`), Zed's GPUI (`gpui`), and Iced-inspired declarative application ergonomics — featuring 314 components across 55 categories, 68 full-transformation multi-material themes, a concentric GPU SDF optical surface painter, Astryx Open-Internals architecture, and strict ADR-0001..ADR-0006 governance.**

**Project Site:** https://sourcecode.github.io/refractor-ui-rust/

---

## Highlights

- **Concentric GPU SDF Optical Surface Painter (`refractor-core`):** Hardware-accelerated `gpui::linear_gradient` fills, mathematically concentric SDF specular glass arcs and inner catchlight rims, bottom depth rims, recessed input wells, dual-corner neumorphic extrusions, classic 3D Win95/Amiga bevels, and multi-layer caustic floor blooms.
- **Atmospheric Stage Backdrop & 9 GPU FX Overlays (`OpticalBackdrop` & `FxOverlay`):** Multi-stop studio wallpaper gradients with warm/cool ambient caustic light pools plus all 9 GPU-accelerated overlays (`None`, `Noise`, `Scanlines`, `CyberGrid`, `RetroGrid`, `DotMatrix`, `PaperGrain`, `CrtRgb`, `HazardStripes`).
- **Dual Application Ergonomics (GPUI Native + Iced-Inspired):** Build apps using either GPUI's native `Render`/`Entity<T>` model OR the Iced-inspired `RefractorApp` trait (`State`, `Message`, `update -> Task<Message>`, `view`, `theme`, `subscriptions`).
- **314 Components Across 55 Categories (`refractor-components`):** Strongly-typed Rust `RenderOnce` GPUI elements including horizontal desktop menu bars (`MenuBarTrigger`, `MenuDropdownItem`), configurable `WindowTitlebar` (`WindowChromeStyle` + `.icon()`), tactile `RangeSlider` with 3D porcelain thumb, semantic `BadgeStatus`, and 7 bespoke domain suites.
- **68 Full-Transformation Themes (`refractor-themes`):** 5 flagship Optical Glass presets (`alabaster`, `obsidian`, `amethyst`, `emerald-tide`, `solar-prism`) + 63 WebOS-v3 themes spanning **13 material archetypes** (`glass`, `mica`, `bevel`, `brutalist`, `neon`, `crt`, `neumorphic`, `clay`, `paper`, `flat`, `chrome`, `pixel`, `resin`).
- **Native Windows App Icon Embedding:** Zero-dependency `build.rs` pattern embedding multi-resolution `assets/app.ico` at Win32 Resource ID `1`, automatically loaded by GPUI onto the taskbar, Alt-Tab switcher, Explorer executable, and native titlebar.
- **Strict Rust Safety & Defense-in-Depth Security (`refractor-security`):** `#![forbid(unsafe_code)]`, zero `unwrap`/`expect`/`panic` in production code, URL protocol allowlisting, strict HTML sanitization, OWASP CSV formula neutralization, ReDoS-free literal highlighting, NaN/Infinity clamping, and path-traversal-hardened CLI swizzling.

## Workspace Crates & Desktop Apps

```text
├── crates/
│   ├── refractor-security/    # Zero-dep defense-in-depth sanitization & validation (ADR-0005)
│   ├── refractor-tokens/      # Pure Rust color math, WCAG 2.2 AA & theme engine (ADR-0003)
│   ├── refractor-themes/      # 68 full-transformation multi-material themes (5 Optical + 63 WebOS-v3)
│   ├── refractor-core/        # GPUI ThemeGlobal, Concentric SDF OpticalSurface, OpticalBackdrop, FxOverlay, RefractorApp
│   ├── refractor-components/  # Complete 314-component GPUI catalog across 55 categories
│   ├── refractor-cli/         # Agent-ready CLI (`component`, `docs`, `theme`, `swizzle`, `check`)
│   └── refractor-xtask/       # Automated ADR, SemVer lockstep, & governance checker (`check-repo`)
├── apps/
│   ├── task-manager/          # Interactive Optical Glass Task Manager with horizontal menu bar, Visual FX Studio & app.ico
│   ├── workbench/             # Interactive GPUI Desktop Optical Workbench & 68-Theme Studio
│   └── studio-showcase/       # Multi-suite desktop showcase rendering all 7 bespoke component domain suites
└── docs/
    ├── index.html             # Live GitHub Pages project site
    └── adr/                   # Authoritative Architecture Decision Records (ADR-0001..0006)
```

## Quick Start

```bash
# Run all 46 unit, integration, and adversarial tests
cargo test --workspace

# Run strict Clippy lint gate (ADR-0002 — 0 warnings, 0 errors)
cargo clippy --workspace --all-targets -- -D warnings

# Verify repository governance, ADRs, SemVer lockstep & catalog parity
cargo run -p refractor-xtask -- check-repo

# Explore the 314 components and 68 themes via CLI
cargo run -p refractor-cli -- component --list
cargo run -p refractor-cli -- component PrimaryButton --dense
cargo run -p refractor-cli -- theme list
cargo run -p refractor-cli -- swizzle PrimaryButton --out src/ui

# Launch any of the 3 native GPUI desktop applications
cargo run -p refractor-task-manager
cargo run -p refractor-workbench
cargo run -p refractor-studio-showcase
```

## Setting Custom App Icons (Windows / GPUI)

GPUI's Windows backend automatically loads Win32 icon Resource ID `1` from the executable. Place your multi-resolution `.ico` at `assets/app.ico` and include the zero-dependency `build.rs` pattern from [`apps/task-manager/build.rs`](./apps/task-manager/build.rs) to embed it automatically on build.

## License

Apache License 2.0 — see [`LICENSE`](./LICENSE) and [`NOTICE`](./NOTICE).
