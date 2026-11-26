//! GPU-accelerated visual FX overlay & studio stage backdrop engine for GPUI (`RUI-11` / ADR-0003).
//!
//! Renders:
//! 1. [`OpticalBackdrop`]: Multi-stop studio wallpaper gradient (`145deg`, `bg.start -> bg.end`)
//!    with atmospheric warm & cool caustic light pools that give frosted glass slabs genuine depth.
//! 2. [`FxOverlay`]: All 9 [`ThemeOverlayFx`] variants (`None`, `Noise`, `Scanlines`, `CyberGrid`,
//!    `RetroGrid`, `DotMatrix`, `PaperGrain`, `CrtRgb`, `HazardStripes`) painted on the GPU
//!    using `canvas`, `pattern_slash`, and `paint_quad` while clamping opacity to `[0.0, 0.4]`,
//!    respecting `reduced_motion`, and never intercepting pointer events.

use gpui::{
    App, Bounds, BoxShadow, Hsla, IntoElement, ParentElement, Pixels, RenderOnce, Styled, Window,
    canvas, div, fill, linear_color_stop, linear_gradient, pattern_slash, point, px, size,
};
use refractor_security::clamp_f32;
use refractor_tokens::{PrismTheme, RefractionIntensity, RgbColor, ThemeMode, ThemeOverlayFx};

use crate::gpui_bridge::{active_theme, color_str_to_hsla, rgb_to_hsla};

/// Maximum permitted overlay opacity per ADR-0003 / `ThemeFxConfig` contract.
pub const MAX_OVERLAY_OPACITY: f32 = 0.4;

/// Clamps an overlay opacity into `[0.0, 0.4]` with NaN/Infinity defense.
pub fn clamp_overlay_opacity(opacity: f32) -> f32 {
    clamp_f32(opacity, 0.0, MAX_OVERLAY_OPACITY, 0.0)
}

/// Resolved paint parameters for an overlay layer (pure data for deterministic testing).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FxOverlaySpec {
    pub overlay: ThemeOverlayFx,
    pub opacity: f32,
    pub step_px: f32,
    pub active: bool,
}

impl FxOverlaySpec {
    /// Derives safe overlay paint parameters from a [`PrismTheme`] and reduced-motion flag.
    pub fn from_theme(theme: &PrismTheme, reduced_motion: bool) -> Self {
        if !theme.optics.performance_mode.allows_fx_overlays() {
            return Self {
                overlay: ThemeOverlayFx::None,
                opacity: 0.0,
                step_px: 0.0,
                active: false,
            };
        }
        let overlay = theme.fx.wallpaper_overlay;
        let raw = if overlay == ThemeOverlayFx::None {
            0.0
        } else if theme.fx.overlay_opacity > 0.0 {
            theme.fx.overlay_opacity
        } else {
            overlay.default_opacity()
        };
        let clamped = clamp_overlay_opacity(raw);
        let opacity = if reduced_motion {
            clamped * 0.5
        } else {
            clamped
        };
        let step_px = match overlay {
            ThemeOverlayFx::Scanlines | ThemeOverlayFx::CrtRgb => 4.0,
            ThemeOverlayFx::DotMatrix => 8.0,
            ThemeOverlayFx::Noise => 6.0,
            ThemeOverlayFx::PaperGrain => 10.0,
            ThemeOverlayFx::HazardStripes => 24.0,
            ThemeOverlayFx::RetroGrid => 28.0,
            ThemeOverlayFx::CyberGrid => 36.0,
            ThemeOverlayFx::None => 0.0,
        };
        Self {
            overlay,
            opacity,
            step_px,
            active: overlay != ThemeOverlayFx::None && opacity > 0.001,
        }
    }
}

/// Atmospheric multi-stop optical studio backdrop with ambient caustic light pools
/// and integrated [`FxOverlay`] layer (Rust equivalent of `.prism-studio-surface`).
#[derive(Debug, Clone, Default, IntoElement)]
pub struct OpticalBackdrop {
    pub reduced_motion: bool,
    pub overlay_opacity_override: Option<f32>,
}

impl OpticalBackdrop {
    pub const fn new() -> Self {
        Self {
            reduced_motion: false,
            overlay_opacity_override: None,
        }
    }

    pub const fn reduced_motion(mut self, reduced: bool) -> Self {
        self.reduced_motion = reduced;
        self
    }

    pub fn overlay_opacity(mut self, opacity: f32) -> Self {
        self.overlay_opacity_override = Some(clamp_overlay_opacity(opacity));
        self
    }
}

impl RenderOnce for OpticalBackdrop {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let is_dark = theme.palette.mode == ThemeMode::Dark;

        let start = color_str_to_hsla(
            &theme.palette.background.start,
            if is_dark {
                RgbColor::new(13, 18, 26)
            } else {
                RgbColor::new(243, 245, 248)
            },
        );
        let end = color_str_to_hsla(
            &theme.palette.background.end,
            if is_dark {
                RgbColor::new(11, 16, 23)
            } else {
                RgbColor::new(222, 228, 238)
            },
        );

        let refraction_mult = match theme.optics.refraction {
            RefractionIntensity::Subtle => 0.45,
            RefractionIntensity::Standard => 1.0,
            RefractionIntensity::Vivid => 1.4,
        };
        let bloom_scale = if self.reduced_motion {
            0.35
        } else {
            (refraction_mult * theme.optics.caustic_opacity).clamp(0.0, 1.4)
        };

        let cool_bloom = rgb_to_hsla(
            theme.palette.secondary.rgb,
            (if is_dark { 0.14 } else { 0.18 } * bloom_scale).clamp(0.0, 0.34),
        );
        let warm_bloom = rgb_to_hsla(
            theme.palette.primary.light_rgb,
            (if is_dark { 0.11 } else { 0.16 } * bloom_scale).clamp(0.0, 0.30),
        );
        let studio_wash = rgb_to_hsla(
            RgbColor::new(255, 255, 255),
            if is_dark { 0.04 } else { 0.52 },
        );

        let mut fx = FxOverlay::new().reduced_motion(self.reduced_motion);
        if let Some(ov) = self.overlay_opacity_override {
            fx = fx.opacity(ov);
        }

        let mut stage = div()
            .relative()
            .size_full()
            .overflow_hidden()
            .bg(linear_gradient(
                148.0,
                linear_color_stop(start, 0.0),
                linear_color_stop(end, 1.0),
            ))
            // Soft top-left studio skylight wash
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(px(340.0))
                    .bg(linear_gradient(
                        165.0,
                        linear_color_stop(studio_wash, 0.0),
                        linear_color_stop(rgb_to_hsla(RgbColor::new(255, 255, 255), 0.0), 1.0),
                    )),
            );

        // Ambient caustic light pools that illuminate frosted glass slabs from behind
        if bloom_scale > 0.05 {
            stage = stage
                .child(
                    div()
                        .absolute()
                        .top(px(-140.0))
                        .left(px(-110.0))
                        .w(px(420.0))
                        .h(px(420.0))
                        .rounded(px(999.0))
                        .bg(cool_bloom)
                        .shadow(vec![BoxShadow {
                            color: cool_bloom,
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(110.0),
                            spread_radius: px(36.0),
                        }]),
                )
                .child(
                    div()
                        .absolute()
                        .bottom(px(-160.0))
                        .right(px(-100.0))
                        .w(px(440.0))
                        .h(px(440.0))
                        .rounded(px(999.0))
                        .bg(warm_bloom)
                        .shadow(vec![BoxShadow {
                            color: warm_bloom,
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(120.0),
                            spread_radius: px(36.0),
                        }]),
                );
        }

        stage.child(div().absolute().inset_0().child(fx))
    }
}

/// Non-interactive GPU overlay element layered over a Refractor desktop stage.
#[derive(Debug, Clone, Default, IntoElement)]
pub struct FxOverlay {
    pub reduced_motion: bool,
    pub opacity_override: Option<f32>,
}

impl FxOverlay {
    pub const fn new() -> Self {
        Self {
            reduced_motion: false,
            opacity_override: None,
        }
    }

    pub const fn reduced_motion(mut self, reduced: bool) -> Self {
        self.reduced_motion = reduced;
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity_override = Some(clamp_overlay_opacity(opacity));
        self
    }
}

impl RenderOnce for FxOverlay {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let mut spec = FxOverlaySpec::from_theme(&theme, self.reduced_motion);
        if let Some(ov) = self.opacity_override {
            spec.opacity = ov;
            spec.active = spec.overlay != ThemeOverlayFx::None && ov > 0.001;
        }

        if !spec.active {
            return div().size_full().into_any_element();
        }

        let glow = color_str_to_hsla(&theme.fx.glow_color, theme.palette.primary.rgb);
        let accent = rgb_to_hsla(theme.palette.primary.rgb, spec.opacity);
        let ink = rgb_to_hsla(RgbColor::new(10, 14, 22), spec.opacity);
        let light_line = rgb_to_hsla(RgbColor::new(255, 255, 255), spec.opacity);
        let tint = Hsla {
            a: spec.opacity,
            ..glow
        };

        // Native GPU diagonal slash pattern for HazardStripes & PaperGrain
        if spec.overlay == ThemeOverlayFx::HazardStripes {
            return div()
                .size_full()
                .bg(pattern_slash(accent, 10.0, 24.0))
                .into_any_element();
        }
        if spec.overlay == ThemeOverlayFx::PaperGrain {
            let grain_color = rgb_to_hsla(RgbColor::new(24, 32, 44), (spec.opacity * 0.6).min(0.2));
            return div()
                .size_full()
                .bg(pattern_slash(grain_color, 1.0, 6.0))
                .into_any_element();
        }

        let crt_r = rgb_to_hsla(RgbColor::new(255, 48, 64), (spec.opacity * 0.45).min(0.18));
        let crt_g = rgb_to_hsla(RgbColor::new(48, 255, 112), (spec.opacity * 0.35).min(0.14));
        let crt_b = rgb_to_hsla(RgbColor::new(64, 128, 255), (spec.opacity * 0.45).min(0.18));

        canvas(
            |_bounds, _window, _cx| {},
            move |bounds: Bounds<Pixels>, (), window, _cx| {
                let width: f32 = bounds.size.width.into();
                let height: f32 = bounds.size.height.into();
                let step = spec.step_px.max(4.0);

                match spec.overlay {
                    ThemeOverlayFx::None
                    | ThemeOverlayFx::HazardStripes
                    | ThemeOverlayFx::PaperGrain => {}
                    ThemeOverlayFx::Scanlines => {
                        let mut y = 0.0_f32;
                        while y < height {
                            let bar = Bounds::new(
                                point(bounds.origin.x, bounds.origin.y + px(y)),
                                size(bounds.size.width, px(2.0)),
                            );
                            window.paint_quad(fill(bar, ink));
                            y += step;
                        }
                    }
                    ThemeOverlayFx::CrtRgb => {
                        // Horizontal CRT scanlines + vertical RGB phosphor subpixel columns
                        let mut y = 0.0_f32;
                        while y < height {
                            let bar = Bounds::new(
                                point(bounds.origin.x, bounds.origin.y + px(y)),
                                size(bounds.size.width, px(1.5)),
                            );
                            window.paint_quad(fill(bar, ink));
                            y += step;
                        }
                        let mut x = 0.0_f32;
                        let mut col = 0_usize;
                        while x < width {
                            let color = match col % 3 {
                                0 => crt_r,
                                1 => crt_g,
                                _ => crt_b,
                            };
                            let vbar = Bounds::new(
                                point(bounds.origin.x + px(x), bounds.origin.y),
                                size(px(1.0), bounds.size.height),
                            );
                            window.paint_quad(fill(vbar, color));
                            x += 3.0;
                            col += 1;
                        }
                    }
                    ThemeOverlayFx::CyberGrid | ThemeOverlayFx::RetroGrid => {
                        let grid_color = if spec.overlay == ThemeOverlayFx::RetroGrid {
                            light_line
                        } else {
                            tint
                        };
                        let mut x = 0.0_f32;
                        while x < width {
                            let vline = Bounds::new(
                                point(bounds.origin.x + px(x), bounds.origin.y),
                                size(px(1.0), bounds.size.height),
                            );
                            window.paint_quad(fill(vline, grid_color));
                            x += step;
                        }
                        let mut y = 0.0_f32;
                        while y < height {
                            let hline = Bounds::new(
                                point(bounds.origin.x, bounds.origin.y + px(y)),
                                size(bounds.size.width, px(1.0)),
                            );
                            window.paint_quad(fill(hline, grid_color));
                            y += step;
                        }
                    }
                    ThemeOverlayFx::DotMatrix => {
                        let mut y = 0.0_f32;
                        while y < height {
                            let mut x = 0.0_f32;
                            while x < width {
                                let dot = Bounds::new(
                                    point(bounds.origin.x + px(x), bounds.origin.y + px(y)),
                                    size(px(1.5), px(1.5)),
                                );
                                window.paint_quad(fill(dot, ink));
                                x += step;
                            }
                            y += step;
                        }
                    }
                    ThemeOverlayFx::Noise => {
                        // Staggered dual-tone micro-grain (light & dark speckles)
                        let mut y = 0.0_f32;
                        let mut row = 0_usize;
                        while y < height {
                            let offset = if row.is_multiple_of(2) {
                                0.0
                            } else {
                                step * 0.5
                            };
                            let mut x = offset;
                            while x < width {
                                let color = if (row + (x as usize)).is_multiple_of(2) {
                                    light_line
                                } else {
                                    ink
                                };
                                let grain = Bounds::new(
                                    point(bounds.origin.x + px(x), bounds.origin.y + px(y)),
                                    size(px(1.5), px(1.5)),
                                );
                                window.paint_quad(fill(grain, color));
                                x += step;
                            }
                            y += step;
                            row += 1;
                        }
                    }
                }
            },
        )
        .size_full()
        .into_any_element()
    }
}
