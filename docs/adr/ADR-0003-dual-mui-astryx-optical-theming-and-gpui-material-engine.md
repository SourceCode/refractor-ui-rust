# ADR-0003: Dual MUI + Astryx Optical Glass Theme Engine & GPUI Multi-Material Renderer

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Refractor Creative UX/UI Direction & Core Architecture
- **Applies To:** `crates/refractor-tokens`, `crates/refractor-themes`, `crates/refractor-core`

---

## 1. Context

Refractor's visual identity combines tactile prismatic refractive glass with 67 full-transformation themes spanning 13 material archetypes and 9 GPU-friendly FX overlays. In Rust + GPUI, CSS custom properties are replaced by a zero-allocation-friendly `ThemeGlobal` (`gpui::Global`) and an `OpticalSurface` GPU renderer that translates theme tokens into GPUI `Hsla`/`Rgba` colors, `LinearGradient` specular arcs, multi-layer `BoxShadow` caustic glows, crisp hairline borders, and recessed input wells.

## 2. Directives (Mandatory Invariants)

1. **MUI-Ergonomic `create_prism_theme(options)`:**
   - Accepting a partial `PrismThemeOptions` struct automatically derives missing tonal variants (`light`, `dark`, WCAG 2.2 AA `contrast_text`, and `RgbColor` triplets) via pure deterministic color math in `refractor-tokens::color_math`.
2. **Astryx-Style `define_theme(input)` & Scoped Inheritance:**
   - `define_theme` produces a serializable `DefinedPrismTheme` supporting theme inheritance (`extends`), light/dark token pairs, and per-component `ComponentStyleOverride` maps.
   - `ThemeGlobal` provides reactive GPUI root theming while `ThemeScope` allows nested isolated theme subtrees.
3. **13 Material Archetypes & 9 FX Overlays:**
   - `OpticalSurface` faithfully renders `Glass`, `Mica`, `Bevel`, `Brutalist`, `Neon`, `Crt`, `Neumorphic`, `Clay`, `Paper`, `Flat`, `Chrome`, `Pixel`, and `Resin`, plus visual FX overlay treatments (`None`, `Noise`, `Scanlines`, `CyberGrid`, `RetroGrid`, `DotMatrix`, `PaperGrain`, `CrtRgb`, `HazardStripes`).
4. **67 Shipped Themes (`refractor-themes`):**
   - 5 Optical Glass presets (`alabaster`, `obsidian`, `amethyst`, `emerald-tide`, `solar-prism`) + 62 WebOS-v3 themes across all 9 categories.

## 3. Enforcement

- `refractor-tokens` and `refractor-themes` unit suites verify all 67 themes resolve deterministically with valid RGB derivation and WCAG contrast ratios.
