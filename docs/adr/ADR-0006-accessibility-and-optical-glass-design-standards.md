# ADR-0006: Accessibility Contracts, Iced/GPUI Ergonomics & Optical Glass Design Standard

- **Status:** Accepted
- **Date:** 2026-11-22
- **Decision Makers:** Creative UX/UI Director & Accessibility Lead
- **Applies To:** All 314 components, `refractor-core`, and `apps/workbench`

---

## 1. Context

A universal Rust desktop UI toolkit must deliver both museum-grade multi-material visual craftsmanship (Optical Glass + 68 themes) and frictionless developer ergonomics across complex desktop applications (IDEs, DAWs, NLEs, data grids, AI agent workbenches, OS shells).

## 2. Directives (Mandatory Invariants)

1. **Optical Material Hierarchy:**
   - `SurfaceVariant::PrimaryButton`: Warm refractive capsule with top specular arc and directional caustic glow.
   - `SurfaceVariant::SecondaryButton` / `CobaltButton`: Ice-crystal or deep cobalt glass pills.
   - `SurfaceVariant::Crystal` / `Slab`: Frosted translucent bezels and 24px optical slabs with outer hairline, inner rim highlight, and caustic shadow.
   - `SurfaceVariant::Well`: High-contrast recessed input wells guaranteeing >= 4.5:1 WCAG AA legibility.
   - `SurfaceVariant::Emerald` + `Thumb`: Translucent emerald active toggle paired with 3D porcelain thumb geometry.
2. **WCAG 2.2 AA & Keyboard Operability:**
   - All normal text achieves >= 4.5:1 contrast via deterministic WCAG luminance derivation in `refractor-tokens`.
   - Every interactive control carries semantic accessibility metadata (`aria_label`, `role`, keyboard contract) and respects disabled state without misleading pointer affordances.
3. **Dual Native GPUI + Iced-Inspired Application Ergonomics:**
   - Developers can build views directly with GPUI's `Render` / `RenderOnce` / `Entity<T>` OR use `RefractorApp` (`State`, `Message`, `update -> Task<Message>`, `view`, `theme`, `subscriptions`) modeled on Iced's Elm architecture, sharing the exact same 314-component catalog and 68 themes.

## 3. Enforcement

- Verified by `refractor-components` doc & contrast tests, `refractor-cli check --strict`, and live rendering in `apps/workbench`.
