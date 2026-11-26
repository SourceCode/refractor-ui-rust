//! Suite 8: Dynamic Gradient & AI-Glow FX (8 components — `refractor-ui` v1.50.0 parity).
//!
//! Components:
//! 1. [`PrismaticCursorHalo`]
//! 2. [`AiFocusRing`]
//! 3. [`FluidMeshCanvas`]
//! 4. [`GradientTextSelection`]
//! 5. [`AmbientAuraBleed`]
//! 6. [`ReactiveInputShimmer`]
//! 7. [`IridescentRimSurface`]
//! 8. [`StatefulEnergyRibbon`]

use gpui::{
    App, BorderStyle, Bounds, BoxShadow, Corners, Edges, FontWeight, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, linear_color_stop,
    linear_gradient, point, px, quad, size,
};
use refractor_core::{
    BaseProps, OpticalShaderKind, ShaderBudget, SurfaceVariant, active_theme,
    apply_component_override, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_security::clamp_f32;
use refractor_tokens::{RgbColor, ThemeMode};

// ─────────────────────────────────────────────────────────────────────────────
// 1. PrismaticCursorHalo
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HaloSpectrum {
    #[default]
    Prismatic,
    Aurora,
    Solar,
    Emerald,
}

impl HaloSpectrum {
    pub const fn colors(self) -> (RgbColor, RgbColor) {
        match self {
            Self::Prismatic => (RgbColor::new(231, 92, 36), RgbColor::new(56, 189, 248)),
            Self::Aurora => (RgbColor::new(139, 92, 246), RgbColor::new(34, 211, 238)),
            Self::Solar => (RgbColor::new(245, 158, 11), RgbColor::new(239, 68, 68)),
            Self::Emerald => (RgbColor::new(16, 185, 129), RgbColor::new(56, 189, 248)),
        }
    }
}

/// Cursor-tracking radial spotlight illuminating underlying optical glass borders & surfaces.
#[derive(Debug, Clone, IntoElement)]
pub struct PrismaticCursorHalo {
    pub title: SharedString,
    pub subtitle: SharedString,
    pub radius_px: f32,
    pub intensity: f32,
    pub spectrum: HaloSpectrum,
    pub border_glow: bool,
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub base: BaseProps,
}

impl Default for PrismaticCursorHalo {
    fn default() -> Self {
        Self::new()
    }
}

impl PrismaticCursorHalo {
    pub const fn component_name() -> &'static str {
        "PrismaticCursorHalo"
    }

    pub fn new() -> Self {
        Self {
            title: "Prismatic Cursor Halo".into(),
            subtitle: "Zero-re-render radial spotlight across optical borders".into(),
            radius_px: 240.0,
            intensity: 0.78,
            spectrum: HaloSpectrum::Prismatic,
            border_glow: true,
            pointer_x: 0.5,
            pointer_y: 0.35,
            base: BaseProps::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn radius_px(mut self, radius: f32) -> Self {
        self.radius_px = clamp_f32(radius, 48.0, 640.0, 240.0);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.78);
        self
    }

    pub const fn spectrum(mut self, spectrum: HaloSpectrum) -> Self {
        self.spectrum = spectrum;
        self
    }

    pub const fn border_glow(mut self, enabled: bool) -> Self {
        self.border_glow = enabled;
        self
    }

    pub fn pointer(mut self, x: f32, y: f32) -> Self {
        self.pointer_x = clamp_f32(x, 0.0, 1.0, 0.5);
        self.pointer_y = clamp_f32(y, 0.0, 1.0, 0.5);
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

impl RenderOnce for PrismaticCursorHalo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let budget = ShaderBudget::from_theme(&theme, false);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let (c1, c2) = self.spectrum.colors();
        let active_alpha = if self.base.disabled || !budget.performance_mode.allows_fx_overlays() {
            0.0
        } else {
            self.intensity * 0.28
        };
        let norm_x = self.pointer_x;
        let norm_y = self.pointer_y;
        let rad = self.radius_px;
        let border_glow = self.border_glow;

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if active_alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let center_x = w * norm_x;
                        let center_y = h * norm_y;
                        let d = rad.min(w.max(h) * 1.2).max(60.0);
                        let halo_bounds = Bounds::new(
                            point(
                                bounds.origin.x + px(center_x - d * 0.5),
                                bounds.origin.y + px(center_y - d * 0.5),
                            ),
                            size(px(d), px(d)),
                        );
                        window.paint_quad(quad(
                            halo_bounds,
                            Corners::all(px(d * 0.5)),
                            linear_gradient(
                                135.0,
                                linear_color_stop(rgb_to_hsla(c1, active_alpha), 0.0),
                                linear_color_stop(rgb_to_hsla(c2, active_alpha * 0.55), 1.0),
                            ),
                            Edges::all(px(0.0)),
                            rgb_to_hsla(c1, 0.0),
                            BorderStyle::Solid,
                        ));
                        if border_glow {
                            window.paint_quad(quad(
                                bounds,
                                Corners::all(px(16.0)),
                                rgb_to_hsla(c1, 0.0),
                                Edges::all(px(1.5)),
                                rgb_to_hsla(c2, active_alpha * 1.4),
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
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(muted)
                    .child(self.subtitle),
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
// 2. AiFocusRing
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AiFocusPalette {
    #[default]
    AiSpectrum,
    CobaltAmber,
    EmeraldCyan,
    RoseViolet,
}

impl AiFocusPalette {
    pub const fn endpoints(self) -> (RgbColor, RgbColor) {
        match self {
            Self::AiSpectrum => (RgbColor::new(139, 92, 246), RgbColor::new(56, 189, 248)),
            Self::CobaltAmber => (RgbColor::new(37, 99, 235), RgbColor::new(245, 158, 11)),
            Self::EmeraldCyan => (RgbColor::new(16, 185, 129), RgbColor::new(6, 182, 212)),
            Self::RoseViolet => (RgbColor::new(244, 63, 94), RgbColor::new(124, 58, 237)),
        }
    }
}

/// Rotating conic-gradient perimeter wrapping active prompt fields, agent cards, or panels.
#[derive(Debug, Clone, IntoElement)]
pub struct AiFocusRing {
    pub label: SharedString,
    pub active: bool,
    pub processing: bool,
    pub palette: AiFocusPalette,
    pub thickness_px: f32,
    pub speed_seconds: f32,
    pub base: BaseProps,
}

impl Default for AiFocusRing {
    fn default() -> Self {
        Self::new("AI Prompt Surface")
    }
}

impl AiFocusRing {
    pub const fn component_name() -> &'static str {
        "AiFocusRing"
    }

    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            active: false,
            processing: false,
            palette: AiFocusPalette::AiSpectrum,
            thickness_px: 2.0,
            speed_seconds: 3.2,
            base: BaseProps::new(),
        }
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub const fn processing(mut self, processing: bool) -> Self {
        self.processing = processing;
        self
    }

    pub const fn palette(mut self, palette: AiFocusPalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn thickness_px(mut self, px_val: f32) -> Self {
        self.thickness_px = clamp_f32(px_val, 1.0, 8.0, 2.0);
        self
    }

    pub fn speed_seconds(mut self, sec: f32) -> Self {
        self.speed_seconds = clamp_f32(sec, 0.4, 20.0, 3.2);
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

impl RenderOnce for AiFocusRing {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let (start, end) = self.palette.endpoints();
        let ring_alpha = if self.base.disabled {
            0.12
        } else if self.processing {
            0.95
        } else if self.active {
            0.8
        } else {
            0.35
        };

        let mut outer = div()
            .rounded(px(16.0))
            .p(px(self.thickness_px))
            .bg(linear_gradient(
                130.0,
                linear_color_stop(rgb_to_hsla(start, ring_alpha), 0.0),
                linear_color_stop(rgb_to_hsla(end, ring_alpha), 1.0),
            ));

        if (self.active || self.processing)
            && !self.base.disabled
            && theme.optics.performance_mode.allows_fx_overlays()
        {
            outer = outer.shadow(vec![BoxShadow {
                color: rgb_to_hsla(start, if self.processing { 0.34 } else { 0.22 }),
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(if self.processing { 20.0 } else { 12.0 }),
                spread_radius: px(0.0),
            }]);
        }

        let inner = optical_surface(&theme, SurfaceVariant::Crystal)
            .rounded(px(14.0))
            .px(px(14.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(fg)
                    .child(self.label),
            )
            .child(
                div()
                    .rounded(px(9999.0))
                    .px(px(8.0))
                    .py(px(2.0))
                    .bg(rgb_to_hsla(end, 0.18))
                    .text_size(px(10.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb_to_hsla(start, 0.98))
                    .child(if self.processing {
                        "Processing"
                    } else if self.active {
                        "Focused"
                    } else {
                        "Standby"
                    }),
            );

        let mut root = outer.child(inner);
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
// 3. FluidMeshCanvas
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FluidMeshMood {
    Calm,
    #[default]
    Creative,
    Thinking,
    Alert,
}

impl FluidMeshMood {
    pub const fn palette(self) -> (RgbColor, RgbColor, RgbColor) {
        match self {
            Self::Calm => (
                RgbColor::new(56, 189, 248),
                RgbColor::new(45, 212, 191),
                RgbColor::new(99, 102, 241),
            ),
            Self::Creative => (
                RgbColor::new(231, 92, 36),
                RgbColor::new(139, 92, 246),
                RgbColor::new(56, 189, 248),
            ),
            Self::Thinking => (
                RgbColor::new(99, 102, 241),
                RgbColor::new(168, 85, 247),
                RgbColor::new(34, 211, 238),
            ),
            Self::Alert => (
                RgbColor::new(239, 68, 68),
                RgbColor::new(245, 158, 11),
                RgbColor::new(231, 92, 36),
            ),
        }
    }
}

/// Live morphing multi-point color mesh ambient backdrop reacting to pointer hover, cadence & mood.
#[derive(Debug, Clone, IntoElement)]
pub struct FluidMeshCanvas {
    pub title: SharedString,
    pub mood: FluidMeshMood,
    pub cadence: f32,
    pub intensity: f32,
    pub animated: bool,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for FluidMeshCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl FluidMeshCanvas {
    pub const fn component_name() -> &'static str {
        "FluidMeshCanvas"
    }

    pub fn new() -> Self {
        Self {
            title: "Fluid Mesh Canvas".into(),
            mood: FluidMeshMood::Creative,
            cadence: 0.35,
            intensity: OpticalShaderKind::FluidMesh.default_intensity(),
            animated: true,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub const fn mood(mut self, mood: FluidMeshMood) -> Self {
        self.mood = mood;
        self
    }

    pub fn cadence(mut self, cadence: f32) -> Self {
        self.cadence = clamp_f32(cadence, 0.0, 2.0, 0.35);
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.85);
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

impl RenderOnce for FluidMeshCanvas {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let (c1, c2, c3) = self.mood.palette();
        let alpha = self.intensity * 0.32;
        let show_mesh = !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays();

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .min_h(px(96.0))
            .p(px(16.0))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if !show_mesh || alpha <= 0.001 {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let blobs = [
                            (0.18, 0.25, 0.65, c1),
                            (0.76, 0.32, 0.58, c2),
                            (0.50, 0.80, 0.60, c3),
                        ];
                        for (bx, by, scale, col) in blobs {
                            let d = w.min(h).max(72.0) * scale;
                            let r = Bounds::new(
                                point(
                                    bounds.origin.x + px(w * bx - d * 0.5),
                                    bounds.origin.y + px(h * by - d * 0.5),
                                ),
                                size(px(d), px(d)),
                            );
                            window.paint_quad(quad(
                                r,
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
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
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
// 4. GradientTextSelection
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientSelectionMode {
    #[default]
    Editable,
    Prose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientSelectionPalette {
    #[default]
    Spectrum,
    Aurora,
    Solar,
}

/// Spectrum caret & gradient text-selection highlight supporting editable and prose modes.
#[derive(Debug, Clone, IntoElement)]
pub struct GradientTextSelection {
    pub mode: GradientSelectionMode,
    pub value: SharedString,
    pub palette: GradientSelectionPalette,
    pub selected_phrase: Option<SharedString>,
    pub base: BaseProps,
}

impl Default for GradientTextSelection {
    fn default() -> Self {
        Self::new("Synthesize optical shaders...")
    }
}

impl GradientTextSelection {
    pub const fn component_name() -> &'static str {
        "GradientTextSelection"
    }

    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            mode: GradientSelectionMode::Editable,
            value: value.into(),
            palette: GradientSelectionPalette::Spectrum,
            selected_phrase: None,
            base: BaseProps::new(),
        }
    }

    pub const fn mode(mut self, mode: GradientSelectionMode) -> Self {
        self.mode = mode;
        self
    }

    pub const fn palette(mut self, palette: GradientSelectionPalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn selected_phrase(mut self, phrase: impl Into<SharedString>) -> Self {
        self.selected_phrase = Some(phrase.into());
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for GradientTextSelection {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let (c1, c2) = match self.palette {
            GradientSelectionPalette::Spectrum => {
                (RgbColor::new(231, 92, 36), RgbColor::new(56, 189, 248))
            }
            GradientSelectionPalette::Aurora => {
                (RgbColor::new(139, 92, 246), RgbColor::new(34, 211, 238))
            }
            GradientSelectionPalette::Solar => {
                (RgbColor::new(245, 158, 11), RgbColor::new(244, 63, 94))
            }
        };

        let variant = match self.mode {
            GradientSelectionMode::Editable => SurfaceVariant::Well,
            GradientSelectionMode::Prose => SurfaceVariant::Crystal,
        };

        let mut root = optical_surface(&theme, variant)
            .px(px(12.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .child(
                div()
                    .px(px(4.0))
                    .py(px(1.0))
                    .rounded(px(4.0))
                    .bg(linear_gradient(
                        90.0,
                        linear_color_stop(rgb_to_hsla(c1, 0.28), 0.0),
                        linear_color_stop(rgb_to_hsla(c2, 0.28), 1.0),
                    ))
                    .text_size(px(13.0))
                    .text_color(fg)
                    .child(self.selected_phrase.unwrap_or(self.value)),
            );

        if self.mode == GradientSelectionMode::Editable {
            root = root.child(
                div()
                    .w(px(2.0))
                    .h(px(16.0))
                    .rounded(px(1.0))
                    .bg(linear_gradient(
                        180.0,
                        linear_color_stop(rgb_to_hsla(c1, 1.0), 0.0),
                        linear_color_stop(rgb_to_hsla(c2, 1.0), 1.0),
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
// 5. AmbientAuraBleed
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuraTone {
    #[default]
    Prism,
    AiViolet,
    Emerald,
    Amber,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuraElevation {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Soft floating antigravity light bloom anchored beneath elevated optical cards or AI controls.
#[derive(Debug, Clone, IntoElement)]
pub struct AmbientAuraBleed {
    pub title: SharedString,
    pub tone: AuraTone,
    pub intensity: f32,
    pub elevation: AuraElevation,
    pub breathing: bool,
    pub active: bool,
    pub base: BaseProps,
}

impl Default for AmbientAuraBleed {
    fn default() -> Self {
        Self::new("Ambient Aura Surface")
    }
}

impl AmbientAuraBleed {
    pub const fn component_name() -> &'static str {
        "AmbientAuraBleed"
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            tone: AuraTone::Prism,
            intensity: 0.78,
            elevation: AuraElevation::Md,
            breathing: true,
            active: true,
            base: BaseProps::new(),
        }
    }

    pub const fn tone(mut self, tone: AuraTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.78);
        self
    }

    pub const fn elevation(mut self, elevation: AuraElevation) -> Self {
        self.elevation = elevation;
        self
    }

    pub const fn breathing(mut self, breathing: bool) -> Self {
        self.breathing = breathing;
        self
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for AmbientAuraBleed {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let glow_rgb = match self.tone {
            AuraTone::Prism => theme.palette.primary.rgb,
            AuraTone::AiViolet => RgbColor::new(139, 92, 246),
            AuraTone::Emerald => theme.palette.success.rgb,
            AuraTone::Amber => theme.palette.warning.rgb,
            AuraTone::Critical => theme.palette.error.rgb,
        };
        let (blur, offset_y) = match self.elevation {
            AuraElevation::Sm => (14.0, 6.0),
            AuraElevation::Md => (24.0, 10.0),
            AuraElevation::Lg => (36.0, 14.0),
        };

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(14.0))
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
            );

        if self.active && theme.optics.performance_mode.allows_fx_overlays() {
            root = root.shadow(vec![BoxShadow {
                color: rgb_to_hsla(glow_rgb, self.intensity * 0.34),
                offset: point(px(0.0), px(offset_y)),
                blur_radius: px(blur),
                spread_radius: px(-2.0),
            }]);
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
// 6. ReactiveInputShimmer
// ─────────────────────────────────────────────────────────────────────────────

/// Directional optical light sweep traveling across a text field or prompt bar on focus/submit.
#[derive(Debug, Clone, IntoElement)]
pub struct ReactiveInputShimmer {
    pub value: SharedString,
    pub placeholder: SharedString,
    pub suggestions: Vec<SharedString>,
    pub shimmer_trigger: u32,
    pub base: BaseProps,
}

impl Default for ReactiveInputShimmer {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactiveInputShimmer {
    pub const fn component_name() -> &'static str {
        "ReactiveInputShimmer"
    }

    pub fn new() -> Self {
        Self {
            value: "".into(),
            placeholder: "Ask Refractor to synthesize an optical shader...".into(),
            suggestions: vec![
                "Synthesize caustic rim".into(),
                "Benchmark 60fps surface".into(),
                "Switch to Obsidian Dark".into(),
            ],
            shimmer_trigger: 0,
            base: BaseProps::new(),
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn suggestions(mut self, items: Vec<SharedString>) -> Self {
        self.suggestions = items;
        self
    }

    pub const fn shimmer_trigger(mut self, count: u32) -> Self {
        self.shimmer_trigger = count;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for ReactiveInputShimmer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let accent = rgb_to_hsla(theme.palette.primary.rgb, 0.22);
        let display = if self.value.is_empty() {
            self.placeholder
        } else {
            self.value
        };

        let mut field = optical_surface(&theme, SurfaceVariant::Well)
            .relative()
            .overflow_hidden()
            .px(px(12.0))
            .py(px(9.0))
            .text_size(px(13.0))
            .text_color(if display.is_empty() { muted } else { fg })
            .child(display);

        if self.shimmer_trigger > 0 && theme.optics.performance_mode.allows_fx_overlays() {
            field = field.child(
                div()
                    .absolute()
                    .inset_0()
                    .bg(linear_gradient(
                        115.0,
                        linear_color_stop(rgb_to_hsla(theme.palette.primary.rgb, 0.0), 0.2),
                        linear_color_stop(accent, 0.55),
                    )),
            );
        }

        let mut root = div().flex().flex_col().gap(px(6.0)).child(field);
        if !self.suggestions.is_empty() {
            let mut chips = div().flex().flex_row().flex_wrap().gap(px(6.0));
            for s in self.suggestions.into_iter().take(4) {
                chips = chips.child(
                    optical_surface(&theme, SurfaceVariant::Badge)
                        .px(px(8.0))
                        .py(px(3.0))
                        .text_size(px(11.0))
                        .text_color(muted)
                        .child(s),
                );
            }
            root = root.child(chips);
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
// 7. IridescentRimSurface
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IridescentSpectrum {
    #[default]
    Dichroic,
    Prism,
    Arctic,
    AmberRose,
}

impl IridescentSpectrum {
    pub const fn colors(self) -> (RgbColor, RgbColor) {
        match self {
            Self::Dichroic => (RgbColor::new(56, 189, 248), RgbColor::new(192, 132, 252)),
            Self::Prism => (RgbColor::new(231, 92, 36), RgbColor::new(34, 211, 238)),
            Self::Arctic => (RgbColor::new(125, 211, 252), RgbColor::new(224, 242, 254)),
            Self::AmberRose => (RgbColor::new(245, 158, 11), RgbColor::new(244, 63, 94)),
        }
    }
}

/// Dynamic border-lighting optical surface shifting color temperature & specular angle.
#[derive(Debug, Clone, IntoElement)]
pub struct IridescentRimSurface {
    pub title: SharedString,
    pub intensity: f32,
    pub spectrum: IridescentSpectrum,
    pub enable_tilt: bool,
    pub elevated: bool,
    pub angle_deg: f32,
    pub base: BaseProps,
}

impl Default for IridescentRimSurface {
    fn default() -> Self {
        Self::new("Iridescent Rim Surface")
    }
}

impl IridescentRimSurface {
    pub const fn component_name() -> &'static str {
        "IridescentRimSurface"
    }

    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            intensity: 0.85,
            spectrum: IridescentSpectrum::Dichroic,
            enable_tilt: false,
            elevated: true,
            angle_deg: 132.0,
            base: BaseProps::new(),
        }
    }

    pub fn intensity(mut self, intensity: f32) -> Self {
        self.intensity = clamp_f32(intensity, 0.0, 1.0, 0.85);
        self
    }

    pub const fn spectrum(mut self, spectrum: IridescentSpectrum) -> Self {
        self.spectrum = spectrum;
        self
    }

    pub const fn enable_tilt(mut self, enable: bool) -> Self {
        self.enable_tilt = enable;
        self
    }

    pub const fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    pub fn angle_deg(mut self, deg: f32) -> Self {
        self.angle_deg = clamp_f32(deg, 0.0, 360.0, 132.0);
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

impl RenderOnce for IridescentRimSurface {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let (c1, c2) = self.spectrum.colors();
        let alpha = if self.base.disabled {
            0.18
        } else {
            self.intensity * 0.85
        };

        let mut frame = div()
            .rounded(px(18.0))
            .p(px(1.5))
            .bg(linear_gradient(
                self.angle_deg,
                linear_color_stop(rgb_to_hsla(c1, alpha), 0.0),
                linear_color_stop(rgb_to_hsla(c2, alpha), 1.0),
            ))
            .child(
                optical_surface(
                    &theme,
                    if self.elevated {
                        SurfaceVariant::Slab
                    } else {
                        SurfaceVariant::Crystal
                    },
                )
                .rounded(px(16.5))
                .p(px(14.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(fg)
                .child(self.title),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            frame = apply_component_override(frame, ov);
        }
        if let Some(ov) = &self.base.style_override {
            frame = apply_component_override(frame, ov);
        }
        frame
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. StatefulEnergyRibbon
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnergyRibbonState {
    #[default]
    Idle,
    Listening,
    Thinking,
    Streaming,
    Success,
    Error,
}

impl EnergyRibbonState {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Listening => "Listening",
            Self::Thinking => "Thinking",
            Self::Streaming => "Streaming",
            Self::Success => "Success",
            Self::Error => "Error",
        }
    }

    pub const fn colors(self) -> (RgbColor, RgbColor) {
        match self {
            Self::Idle => (RgbColor::new(148, 163, 184), RgbColor::new(203, 213, 225)),
            Self::Listening => (RgbColor::new(56, 189, 248), RgbColor::new(34, 211, 238)),
            Self::Thinking => (RgbColor::new(139, 92, 246), RgbColor::new(56, 189, 248)),
            Self::Streaming => (RgbColor::new(231, 92, 36), RgbColor::new(245, 158, 11)),
            Self::Success => (RgbColor::new(16, 185, 129), RgbColor::new(52, 211, 153)),
            Self::Error => (RgbColor::new(239, 68, 68), RgbColor::new(244, 63, 94)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RibbonPosition {
    Top,
    Bottom,
    #[default]
    Inline,
}

/// Slim animated gradient hairline anchored to top, bottom, or inline edges reflecting agent states.
#[derive(Debug, Clone, IntoElement)]
pub struct StatefulEnergyRibbon {
    pub state: EnergyRibbonState,
    pub position: RibbonPosition,
    pub thickness_px: f32,
    pub show_label: bool,
    pub paused: bool,
    pub base: BaseProps,
}

impl Default for StatefulEnergyRibbon {
    fn default() -> Self {
        Self::new(EnergyRibbonState::Idle)
    }
}

impl StatefulEnergyRibbon {
    pub const fn component_name() -> &'static str {
        "StatefulEnergyRibbon"
    }

    pub fn new(state: EnergyRibbonState) -> Self {
        Self {
            state,
            position: RibbonPosition::Inline,
            thickness_px: 3.0,
            show_label: false,
            paused: false,
            base: BaseProps::new(),
        }
    }

    pub const fn position(mut self, position: RibbonPosition) -> Self {
        self.position = position;
        self
    }

    pub fn thickness_px(mut self, thickness: f32) -> Self {
        self.thickness_px = clamp_f32(thickness, 1.0, 12.0, 3.0);
        self
    }

    pub const fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub const fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for StatefulEnergyRibbon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let is_dark = theme.palette.mode == ThemeMode::Dark;
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let (c1, c2) = self.state.colors();
        let alpha = if self.paused { 0.45 } else { 0.96 };

        let bar = div()
            .w_full()
            .h(px(self.thickness_px))
            .rounded(px(9999.0))
            .bg(linear_gradient(
                90.0,
                linear_color_stop(rgb_to_hsla(c1, alpha), 0.0),
                linear_color_stop(rgb_to_hsla(c2, alpha), 1.0),
            ));

        let mut root = div().w_full().flex().flex_col().gap(px(4.0));
        if self.position == RibbonPosition::Bottom && self.show_label {
            root = root
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(if is_dark {
                            rgb_to_hsla(c2, 0.95)
                        } else {
                            muted
                        })
                        .child(self.state.label()),
                )
                .child(bar);
        } else {
            root = root.child(bar);
            if self.show_label {
                root = root.child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(muted)
                        .child(self.state.label()),
                );
            }
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
