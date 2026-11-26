# Changelog

All notable changes to **Refractor UI for Rust** (`refractor-ui-rust`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-11-26

### Added
- **1:1 Parity with `refractor-ui` v1.48.0–v1.52.0 (339 Components Across 58 Categories):**
  - **Dynamic Gradient & AI-Glow FX Suite (`suites::gradient_ai_glow_fx`, 8 components):** `PrismaticCursorHalo`, `AiFocusRing`, `FluidMeshCanvas`, `GradientTextSelection`, `AmbientAuraBleed`, `ReactiveInputShimmer`, `IridescentRimSurface`, and `StatefulEnergyRibbon`.
  - **Shader & Optical FX Suite (`suites::shader_optical_fx`, 8 components):** `RefractiveLensContainer`, `ChromaticDispersionEdge`, `CausticLightLayer`, `FrostedGrainOverlay`, `LiquidRippleTrigger`, `HolographicFoilSheen`, `MetaballFluidBackdrop`, and `OpticalDepthVignette`.
  - **Loaders, Skeletons & Data Transitions Suite (`suites::loaders_skeletons_transitions`, 9 components):** `RefractiveSkeleton`, `OrbitalRingSpinner`, `AiThinkingOrb`, `SegmentedGlassProgress`, `StreamingTokenReveal`, `DataGridRowShimmer`, `AcousticActivityBar`, `PipelinePulseLoader`, and `ScanlineRadarSweep`.
- **Optical Shader & High-Performance Frame Engine (`refractor-core::shader`):**
  - Added `OpticalShaderKind` (all 10 shader presets with native WGSL entry-point metadata), `ShaderBudget` (DPR cap `<= 1.5`, dimension clamping `<= 768px`, offscreen/window/reduced-motion/performance-mode guards), `ShaderUniforms`, leak-free `OpticalShaderRegistry`, 60fps `PointerCoalescer`, bounded `TelemetryRingBuffer<T, N>`, and `BlurHostContext` nested blur-elision helper.
- **Three-Tier GPU Performance Profiles & Zero-Allocation Hot Paths (`refractor-tokens`, `refractor-themes`, `refractor-components`):**
  - Added `ThemePerformanceMode` (`Balanced`, `High`, `Max`) integrated into `ThemeOptics`, `optical_surface`, `FxOverlaySpec`, and `ShaderBudget`.
  - Added precomputed 256-entry `SRGB_LINEAR_LUT` in `refractor-tokens::color_math` eliminating hot-path `powf(2.4)` calls.
  - Added O(1) `HashMap` lookup indices, pre-resolved theme caching (`RESOLVED_CACHE`), and precomputed lowercased search corpora (`search_themes`, `search_components`) in `refractor-themes` and `refractor-components::registry`.
- **Visual Parity Harness & Expanded Adversarial/Performance Test Suite:**
  - Added `apps/workbench/examples/parity_harness.rs` and expanded `crates/refractor-components/tests/app_validation.rs` with positive, negative, adversarial (`NaN`/`Infinity`/bounds), 10/10 shader disposal, 200x high-frequency pointer coalescing, and 10,000x sRGB LUT throughput benchmarks.

### Changed
- **GPU Blur Kernel Optimization & Optical Surface Fidelity (`refractor-tokens`, `refractor-themes`, `refractor-core`):** Lowered default optical blur radii (`18px -> 12px` glass/mica/resin, `14px -> 10px` chrome, `22px -> 14px` slabs) and refined concentric SDF optical surfaces and flagship component styling (`buttons`, `inputs`, `display`, `feedback`, `navigation`, `suites`).
- **Synchronized Tooling, Apps & Documentation:** Updated `refractor-cli`, `refractor-xtask`, `apps/workbench`, `apps/studio-showcase`, `README.md`, `AGENTS.md`, and `docs/index.html` to 339 components across 58 categories and 67 themes.

## [1.0.1] - 2026-11-24

### Changed
- **Theme Catalog Maintenance (`refractor-themes`):** Curated the shipped WebOS-v3 theme catalog to 62 themes (67 total multi-material themes including the 5 Optical Glass presets) and synchronized catalog counts across CLI, governance checks, desktop apps, and documentation.

## [1.0.0] - 2026-11-22

### Added
- **Refractor Optical Glass UI Framework for Rust (`v1.0.0` Public Release):** Native GPU-accelerated Rust port of the Refractor Optical Glass UI Kit (`glass-ui-v1`) built on **Zed GPUI** (`gpui 0.2.2`) with **Iced-inspired** declarative application ergonomics (`RefractorApp`, `Task`, `Subscription`).
- **Concentric GPU SDF Optical Surface Engine (`refractor-core`):** Hardware-accelerated `gpui::linear_gradient` fills, runtime-exact concentric SDF specular highlight arcs and inner catchlight rims (`gpui::quad`), bottom depth rims, recessed input wells, 3D Win95/Amiga bevels, dual-corner neumorphic extrusions, clay/resin/neon/crt/chrome/pixel physics across all **13 material archetypes**, and multi-layer caustic floor blooms.
- **Studio Stage Backdrop & 9 GPU FX Overlays (`refractor-core`):** Added `OpticalBackdrop` (148deg studio wallpaper gradient + skylight wash + ambient warm/cool caustic light pools) and upgraded `FxOverlay` with native GPU `pattern_slash` (`HazardStripes`, `PaperGrain`), RGB subpixel `CrtRgb`, `Scanlines`, `CyberGrid`, `RetroGrid`, `DotMatrix`, and staggered dual-tone `Noise`.
- **314 Components Across 55 Categories (`refractor-components`):** Native GPUI `RenderOnce` implementations across 7 bespoke domain suites, including:
  - Horizontal desktop menu bar primitives (`MenuBarTrigger`, `MenuDropdownItem`).
  - Configurable `WindowTitlebar` with `WindowChromeStyle` (`None`, `Windows`, `MacTrafficLights`) and `.icon()` optical app badge.
  - Tactile `PrimaryButton`/`SecondaryButton` hover/luminance feedback, `RangeSlider` with 3D porcelain thumb, and semantic `BadgeStatus` tones.
- **68 Multi-Material Themes (`refractor-themes` & `refractor-tokens`):** 5 Optical Glass presets + 63 WebOS-v3 themes across 9 collections, WCAG 2.2 AA single-hex derivation (`create_prism_theme`), and cascading `define_theme`.
- **Defense-in-Depth Security (`refractor-security`):** Zero-dependency URL, HTML, OWASP CSV formula neutralization, NaN/Infinity clamping, prototype-key, ReDoS-safe literal highlight, and swizzle path-traversal guards.
- **3 Native GPUI Desktop Applications:**
  - `apps/task-manager`: Interactive Optical Glass Task Manager featuring a horizontal top menu bar (`File`, `Edit`, `View`, `Theme`, `Help`), live **Visual FX & Aesthetic Studio** controls, OWASP-safe CSV export, and embedded multi-resolution Windows `app.ico` (Resource ID `1` via `build.rs`).
  - `apps/workbench`: Interactive Desktop Optical Workbench & 68-Theme Studio.
  - `apps/studio-showcase`: Multi-suite desktop application rendering all 7 bespoke component domain suites.
- **Tooling, Documentation & GitHub Pages Site:**
  - `refractor-cli` (`component`, `docs`, `theme`, `swizzle`, `check`) and `refractor-xtask` (`check-repo`).
  - Live interactive GitHub Pages documentation site at `docs/index.html` (`https://sourcecode.github.io/refractor-ui-rust/`).
  - Authoritative Architecture Decision Records (`docs/adr/ADR-0001` through `ADR-0006`).
