//! Suite 9: Shader & Optical FX (8 components — `refractor-ui` v1.51.0 parity).
//!
//! Components:
//! 1. [`RefractiveLensContainer`]
//! 2. [`ChromaticDispersionEdge`]
//! 3. [`CausticLightLayer`]
//! 4. [`FrostedGrainOverlay`]
//! 5. [`LiquidRippleTrigger`]
//! 6. [`HolographicFoilSheen`]
//! 7. [`MetaballFluidBackdrop`]
//! 8. [`OpticalDepthVignette`]

use gpui::{
    App, BorderStyle, Bounds, BoxShadow, Corners, Edges, FontWeight, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, fill, linear_color_stop,
    linear_gradient, point, px, quad, size,
};
use refractor_core::{
    BaseProps, OpticalShaderKind, ShaderBudget, SurfaceVariant, TelemetryRingBuffer, active_theme,
    apply_component_override, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_security::clamp_f32;
use refractor_tokens::{RgbColor, ThemeMode};

// ─────────────────────────────────────────────────────────────────────────────
// 1. RefractiveLensContainer
// ─────────────────────────────────────────────────────────────────────────────

/// Optical acrylic / liquid-glass distortion container with configurable IOR (1.0–2.4),
/// focal lens radius, magnification, and Fresnel specular rim.
#[derive(Debug, Clone, IntoElement)]
pub struct RefractiveLensContainer {
    pub title: SharedString,
    pub ior: f32,
    pub lens_radius: f32,
    pub magnification: f32,
    pub intensity: f32,
    pub interactive: bool,
    pub disable_shader: bool,
    pub focus_x: f32,
    pub focus_y: f32,
    pub base: BaseProps,
}

impl Default for RefractiveLensContainer {
    fn default() -> Self {
        Self::new("Refractive Lens Container")
    }
}

impl RefractiveLensContainer {
    pub const fn component_name() -> &'static str {
        "RefractiveLensContainer"
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            ior: 1.49,
            lens_radius: 0.52,
            magnification: 1.04,
            intensity: OpticalShaderKind::RefractiveLens.default_intensity(),
            interactive: true,
            disable_shader: false,
            focus_x: 0.5,
            focus_y: 0.45,
            base: BaseProps::new(),
        }
    }

    pub fn ior(mut self, ior: f32) -> Self {
        self.ior = clamp_f32(ior, 1.0, 2.4, 1.49);
        self
    }

    pub fn lens_radius(mut self, radius: f32) -> Self {
        self.lens_radius = clamp_f32(radius, 0.1, 1.0, 0.52);
        self
    }

    pub fn magnification(mut self, mag: f32) -> Self {
        self.magnification = clamp_f32(mag, 0.9, 1.5, 1.04);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.9);
        self
    }

    pub const fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn focus(mut self, x: f32, y: f32) -> Self {
        self.focus_x = clamp_f32(x, 0.0, 1.0, 0.5);
        self.focus_y = clamp_f32(y, 0.0, 1.0, 0.5);
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for RefractiveLensContainer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let primary = theme.palette.primary.rgb;
        let secondary = theme.palette.secondary.rgb;
        let show_lens = !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays();
        let fx = self.focus_x;
        let fy = self.focus_y;
        let lr = self.lens_radius;
        let intensity = self.intensity;

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if !show_lens {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let d = w.min(h).max(56.0) * lr * 1.4;
                        let lens_rect = Bounds::new(
                            point(
                                bounds.origin.x + px(w * fx - d * 0.5),
                                bounds.origin.y + px(h * fy - d * 0.5),
                            ),
                            size(px(d), px(d)),
                        );
                        window.paint_quad(quad(
                            lens_rect,
                            Corners::all(px(d * 0.5)),
                            linear_gradient(
                                145.0,
                                linear_color_stop(rgb_to_hsla(RgbColor::new(255, 255, 255), 0.24 * intensity), 0.0),
                                linear_color_stop(rgb_to_hsla(secondary, 0.14 * intensity), 1.0),
                            ),
                            Edges::all(px(1.25)),
                            rgb_to_hsla(primary, 0.28 * intensity),
                            BorderStyle::Solid,
                        ));
                    },
                )
                .absolute()
                .inset_0(),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(muted)
                    .child(format!("IOR {:.2} · Magnification {:.2}x", self.ior, self.magnification)),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. ChromaticDispersionEdge
// ─────────────────────────────────────────────────────────────────────────────

/// Perimeter dispersion layer splitting light along curved borders into subtle RGB caustics.
#[derive(Debug, Clone, IntoElement)]
pub struct ChromaticDispersionEdge {
    pub title: SharedString,
    pub dispersion: f32,
    pub intensity: f32,
    pub animated: bool,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for ChromaticDispersionEdge {
    fn default() -> Self {
        Self::new("Chromatic Dispersion Edge")
    }
}

impl ChromaticDispersionEdge {
    pub const fn component_name() -> &'static str {
        "ChromaticDispersionEdge"
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            dispersion: 1.35,
            intensity: OpticalShaderKind::ChromaticDispersion.default_intensity(),
            animated: true,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub fn dispersion(mut self, dispersion: f32) -> Self {
        self.dispersion = clamp_f32(dispersion, 0.0, 4.0, 1.35);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.88);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for ChromaticDispersionEdge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let show = !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays();
        let spread = self.dispersion;
        let alpha = self.intensity * 0.45;

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(14.0))
            .text_size(px(13.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(fg)
            .child(self.title);

        if show {
            root = root.shadow(vec![
                BoxShadow {
                    color: rgb_to_hsla(RgbColor::new(244, 63, 94), alpha),
                    offset: point(px(-spread), px(0.0)),
                    blur_radius: px(2.0 + spread),
                    spread_radius: px(0.0),
                },
                BoxShadow {
                    color: rgb_to_hsla(RgbColor::new(56, 189, 248), alpha),
                    offset: point(px(spread), px(0.0)),
                    blur_radius: px(2.0 + spread),
                    spread_radius: px(0.0),
                },
            ]);
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. CausticLightLayer
// ─────────────────────────────────────────────────────────────────────────────

/// Underwater-style light-refraction filament projection overlay for cards, headers, and heroes.
#[derive(Debug, Clone, IntoElement)]
pub struct CausticLightLayer {
    pub intensity: f32,
    pub scale: f32,
    pub speed: f32,
    pub animated: bool,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for CausticLightLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl CausticLightLayer {
    pub const fn component_name() -> &'static str {
        "CausticLightLayer"
    }

    pub fn new() -> Self {
        Self {
            intensity: OpticalShaderKind::CausticLight.default_intensity(),
            scale: 1.2,
            speed: 1.0,
            animated: true,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.72);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = clamp_f32(scale, 0.25, 4.0, 1.2);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = clamp_f32(speed, 0.0, 4.0, 1.0);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for CausticLightLayer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let c1 = theme.palette.primary.rgb;
        let c2 = theme.palette.secondary.rgb;
        let show = !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays();
        let alpha = self.intensity * 0.22;

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .relative()
            .overflow_hidden()
            .min_h(px(72.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if !show || alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        for idx in 0..4_u32 {
                            let f = (idx as f32 + 0.5) / 4.0;
                            let band = Bounds::new(
                                point(
                                    bounds.origin.x + px(w * f * 0.65),
                                    bounds.origin.y + px(h * (0.15 + f * 0.45)),
                                ),
                                size(px(w * 0.42), px(6.0)),
                            );
                            let col = if idx % 2 == 0 { c1 } else { c2 };
                            window.paint_quad(quad(
                                band,
                                Corners::all(px(3.0)),
                                rgb_to_hsla(col, alpha),
                                Edges::all(px(0.0)),
                                rgb_to_hsla(col, 0.0),
                                BorderStyle::Solid,
                            ));
                        }
                    },
                )
                .absolute()
                .inset_0(),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. FrostedGrainOverlay
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GrainSize {
    #[default]
    Fine,
    Medium,
    Coarse,
}

impl GrainSize {
    pub const fn step_px(self) -> f32 {
        match self {
            Self::Fine => 4.0,
            Self::Medium => 7.0,
            Self::Coarse => 11.0,
        }
    }
}

/// Procedural micro-grain overlay preventing gradient banding and adding a matte-etched finish.
#[derive(Debug, Clone, IntoElement)]
pub struct FrostedGrainOverlay {
    pub grain_size: GrainSize,
    pub opacity: f32,
    pub animated: bool,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for FrostedGrainOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl FrostedGrainOverlay {
    pub const fn component_name() -> &'static str {
        "FrostedGrainOverlay"
    }

    pub fn new() -> Self {
        Self {
            grain_size: GrainSize::Fine,
            opacity: OpticalShaderKind::FrostedGrain.default_intensity(),
            animated: false,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub const fn grain_size(mut self, size: GrainSize) -> Self {
        self.grain_size = size;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = clamp_f32(opacity, 0.0, 0.4, 0.14);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for FrostedGrainOverlay {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let is_dark = theme.palette.mode == ThemeMode::Dark;
        let show = !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays();
        let step = self.grain_size.step_px();
        let alpha = self.opacity;
        let ink = if is_dark {
            RgbColor::new(255, 255, 255)
        } else {
            RgbColor::new(15, 23, 42)
        };

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .relative()
            .overflow_hidden()
            .min_h(px(64.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if !show || alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let cols = ((w / step) as u32).min(48);
                        let rows = ((h / step) as u32).min(24);
                        for r in 0..rows {
                            for c in 0..cols {
                                let hash = r.wrapping_mul(73) ^ c.wrapping_mul(193);
                                if hash % 3 == 0 {
                                    let dot = Bounds::new(
                                        point(
                                            bounds.origin.x + px(c as f32 * step),
                                            bounds.origin.y + px(r as f32 * step),
                                        ),
                                        size(px(1.5), px(1.5)),
                                    );
                                    window.paint_quad(fill(dot, rgb_to_hsla(ink, alpha * 0.6)));
                                }
                            }
                        }
                    },
                )
                .absolute()
                .inset_0(),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. LiquidRippleTrigger
// ─────────────────────────────────────────────────────────────────────────────

/// Normalized origin event emitted when a ripple shockwave is triggered.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RippleOriginEvent {
    pub x: f32,
    pub y: f32,
    pub timestamp_ms: u64,
}

/// Interactive click- and keyboard-originated displacement shockwave trigger with bounded wave pool.
#[derive(Debug, Clone, IntoElement)]
pub struct LiquidRippleTrigger {
    pub label: SharedString,
    pub intensity: f32,
    pub duration_ms: u32,
    pub max_ripples: usize,
    pub ripples: TelemetryRingBuffer<RippleOriginEvent, 8>,
    pub base: BaseProps,
}

impl Default for LiquidRippleTrigger {
    fn default() -> Self {
        Self::new("Trigger Optical Ripple")
    }
}

impl LiquidRippleTrigger {
    pub const fn component_name() -> &'static str {
        "LiquidRippleTrigger"
    }

    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            intensity: OpticalShaderKind::LiquidRipple.default_intensity(),
            duration_ms: 750,
            max_ripples: 3,
            ripples: TelemetryRingBuffer::new(),
            base: BaseProps::new(),
        }
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.9);
        self
    }

    pub fn duration_ms(mut self, ms: u32) -> Self {
        self.duration_ms = ms.clamp(150, 3000);
        self
    }

    pub fn max_ripples(mut self, count: usize) -> Self {
        self.max_ripples = count.clamp(1, 8);
        self
    }

    /// Pushes a bounded ripple origin (clamped to `0.0..=1.0`, respects `disabled`).
    pub fn push_ripple(&mut self, x: f32, y: f32, timestamp_ms: u64) {
        if self.base.disabled {
            return;
        }
        self.ripples.push(RippleOriginEvent {
            x: clamp_f32(x, 0.0, 1.0, 0.5),
            y: clamp_f32(y, 0.0, 1.0, 0.5),
            timestamp_ms,
        });
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.base.disabled = disabled;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for LiquidRippleTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let accent = theme.palette.secondary.rgb;
        let waves: Vec<RippleOriginEvent> = self
            .ripples
            .ordered()
            .into_iter()
            .rev()
            .take(self.max_ripples)
            .collect();
        let alpha = if self.base.disabled {
            0.0
        } else {
            self.intensity * 0.35
        };

        let mut root = optical_surface(&theme, SurfaceVariant::SecondaryButton)
            .relative()
            .overflow_hidden()
            .px(px(16.0))
            .py(px(10.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if alpha <= 0.001 || waves.is_empty() {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        for (idx, ev) in waves.iter().enumerate() {
                            let d = 28.0 + (idx as f32) * 18.0;
                            let r = Bounds::new(
                                point(
                                    bounds.origin.x + px(w * ev.x - d * 0.5),
                                    bounds.origin.y + px(h * ev.y - d * 0.5),
                                ),
                                size(px(d), px(d)),
                            );
                            window.paint_quad(quad(
                                r,
                                Corners::all(px(d * 0.5)),
                                rgb_to_hsla(accent, 0.0),
                                Edges::all(px(1.5)),
                                rgb_to_hsla(accent, alpha),
                                BorderStyle::Solid,
                            ));
                        }
                    },
                )
                .absolute()
                .inset_0(),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.label),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. HolographicFoilSheen
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HolographicPreset {
    #[default]
    Dichroic,
    Pearl,
    Titanium,
    PrismGold,
}

impl HolographicPreset {
    pub const fn gradient(self) -> (RgbColor, RgbColor) {
        match self {
            Self::Dichroic => (RgbColor::new(56, 189, 248), RgbColor::new(192, 132, 252)),
            Self::Pearl => (RgbColor::new(241, 245, 249), RgbColor::new(186, 230, 253)),
            Self::Titanium => (RgbColor::new(148, 163, 184), RgbColor::new(226, 232, 240)),
            Self::PrismGold => (RgbColor::new(251, 191, 36), RgbColor::new(231, 92, 36)),
        }
    }
}

/// Angle-reactive iridescent dichroic foil sheen with pearlescent metallic highlights.
#[derive(Debug, Clone, IntoElement)]
pub struct HolographicFoilSheen {
    pub title: SharedString,
    pub preset: HolographicPreset,
    pub intensity: f32,
    pub tilt_degrees: f32,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for HolographicFoilSheen {
    fn default() -> Self {
        Self::new("Holographic Foil Surface")
    }
}

impl HolographicFoilSheen {
    pub const fn component_name() -> &'static str {
        "HolographicFoilSheen"
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            preset: HolographicPreset::Dichroic,
            intensity: OpticalShaderKind::HolographicFoil.default_intensity(),
            tilt_degrees: 5.0,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub const fn preset(mut self, preset: HolographicPreset) -> Self {
        self.preset = preset;
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.85);
        self
    }

    pub fn tilt_degrees(mut self, deg: f32) -> Self {
        self.tilt_degrees = clamp_f32(deg, 0.0, 20.0, 5.0);
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.base.disabled = disabled;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for HolographicFoilSheen {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let (c1, c2) = self.preset.gradient();
        let show = !self.disable_shader
            && !self.base.disabled
            && theme.optics.performance_mode.allows_fx_overlays();
        let alpha = self.intensity * 0.28;

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .p(px(16.0))
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
            );

        if show {
            root = root.child(
                div()
                    .absolute()
                    .inset_0()
                    .bg(linear_gradient(
                        125.0 + self.tilt_degrees * 3.0,
                        linear_color_stop(rgb_to_hsla(c1, alpha), 0.0),
                        linear_color_stop(rgb_to_hsla(c2, alpha), 1.0),
                    )),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. MetaballFluidBackdrop
// ─────────────────────────────────────────────────────────────────────────────

/// GPU SDF metaball fluid backdrop where glowing optical orbs organically merge and separate.
#[derive(Debug, Clone, IntoElement)]
pub struct MetaballFluidBackdrop {
    pub orb_count: usize,
    pub intensity: f32,
    pub speed: f32,
    pub interactive: bool,
    pub animated: bool,
    pub base: BaseProps,
}

impl Default for MetaballFluidBackdrop {
    fn default() -> Self {
        Self::new()
    }
}

impl MetaballFluidBackdrop {
    pub const fn component_name() -> &'static str {
        "MetaballFluidBackdrop"
    }

    pub fn new() -> Self {
        Self {
            orb_count: 3,
            intensity: OpticalShaderKind::MetaballFluid.default_intensity(),
            speed: 1.0,
            interactive: true,
            animated: true,
            base: BaseProps::new(),
        }
    }

    pub fn orb_count(mut self, count: usize) -> Self {
        self.orb_count = count.clamp(1, 6);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.85);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = clamp_f32(speed, 0.0, 4.0, 1.0);
        self
    }

    pub const fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for MetaballFluidBackdrop {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let c1 = theme.palette.primary.rgb;
        let c2 = theme.palette.secondary.rgb;
        let c3 = theme.palette.info.rgb;
        let count = self.orb_count;
        let alpha = if theme.optics.performance_mode.allows_fx_overlays() {
            self.intensity * 0.34
        } else {
            0.0
        };

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .min_h(px(88.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let colors = [c1, c2, c3];
                        for idx in 0..count {
                            let frac = (idx as f32 + 1.0) / (count as f32 + 1.0);
                            let d = w.min(h).max(48.0) * 0.72;
                            let orb = Bounds::new(
                                point(
                                    bounds.origin.x + px(w * frac - d * 0.5),
                                    bounds.origin.y + px(h * (0.35 + (idx % 2) as f32 * 0.3) - d * 0.5),
                                ),
                                size(px(d), px(d)),
                            );
                            let col = colors[idx % colors.len()];
                            window.paint_quad(quad(
                                orb,
                                Corners::all(px(d * 0.5)),
                                rgb_to_hsla(col, alpha),
                                Edges::all(px(0.0)),
                                rgb_to_hsla(col, 0.0),
                                BorderStyle::Solid,
                            ));
                        }
                    },
                )
                .absolute()
                .inset_0(),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. OpticalDepthVignette
// ─────────────────────────────────────────────────────────────────────────────

/// Focus-management depth-of-field vignette progressively dimming background layers around a focal aperture.
#[derive(Debug, Clone, IntoElement)]
pub struct OpticalDepthVignette {
    pub active: bool,
    pub focus_x: f32,
    pub focus_y: f32,
    pub aperture: f32,
    pub blur_px: f32,
    pub dim_opacity: f32,
    pub base: BaseProps,
}

impl Default for OpticalDepthVignette {
    fn default() -> Self {
        Self::new()
    }
}

impl OpticalDepthVignette {
    pub const fn component_name() -> &'static str {
        "OpticalDepthVignette"
    }

    pub fn new() -> Self {
        Self {
            active: true,
            focus_x: 0.5,
            focus_y: 0.5,
            aperture: 0.36,
            blur_px: 8.0,
            dim_opacity: 0.58,
            base: BaseProps::new(),
        }
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn focus(mut self, x: f32, y: f32) -> Self {
        self.focus_x = clamp_f32(x, 0.0, 1.0, 0.5);
        self.focus_y = clamp_f32(y, 0.0, 1.0, 0.5);
        self
    }

    pub fn aperture(mut self, aperture: f32) -> Self {
        self.aperture = clamp_f32(aperture, 0.08, 0.95, 0.36);
        self
    }

    pub fn blur_px(mut self, blur: f32) -> Self {
        self.blur_px = clamp_f32(blur, 0.0, 32.0, 8.0);
        self
    }

    pub fn dim_opacity(mut self, opacity: f32) -> Self {
        self.dim_opacity = clamp_f32(opacity, 0.0, 0.92, 0.58);
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for OpticalDepthVignette {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let budget = ShaderBudget::from_theme(&theme, false);
        let alpha = if self.active { self.dim_opacity * 0.55 } else { 0.0 };
        let fx = self.focus_x;
        let fy = self.focus_y;
        let ap = self.aperture;
        let ring = theme.palette.secondary.rgb;

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .relative()
            .overflow_hidden()
            .min_h(px(76.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        window.paint_quad(fill(bounds, rgb_to_hsla(RgbColor::new(10, 14, 24), alpha)));
                        if budget.performance_mode.allows_fx_overlays() {
                            let d = w.min(h).max(40.0) * ap * 1.6;
                            let aperture_rect = Bounds::new(
                                point(
                                    bounds.origin.x + px(w * fx - d * 0.5),
                                    bounds.origin.y + px(h * fy - d * 0.5),
                                ),
                                size(px(d), px(d)),
                            );
                            window.paint_quad(quad(
                                aperture_rect,
                                Corners::all(px(d * 0.5)),
                                rgb_to_hsla(RgbColor::new(255, 255, 255), 0.14),
                                Edges::all(px(1.5)),
                                rgb_to_hsla(ring, 0.55),
                                BorderStyle::Solid,
                            ));
                        }
                    },
                )
                .absolute()
                .inset_0(),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}
