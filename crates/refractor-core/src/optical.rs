//! GPU-accelerated multi-material Optical Surface Painter for GPUI (ADR-0003 & ADR-0006).
//!
//! Faithfully ports Refractor's 13 material archetypes (`Glass`, `Mica`, `Bevel`, `Brutalist`,
//! `Neon`, `Crt`, `Neumorphic`, `Clay`, `Paper`, `Flat`, `Chrome`, `Pixel`, `Resin`) and
//! optical glass visual FX from `glass-ui-v1` (`refractor.css`):
//! - Multi-stop hardware-accelerated `linear_gradient` fills
//! - Concentric GPU SDF `canvas` painter (`gpui::quad`) that reads exact runtime element
//!   dimensions so top specular arcs, inner catchlight rims, bottom depth rims, 3D Win95
//!   bevels, recessed well shadows, and toast amber rings follow every corner radius
//!   mathematically with zero poke-out or clipping artifacts at any size
//! - Multi-layer directional caustic floor blooms, dual-corner neumorphic extrusions, and neon glows

use gpui::{
    BorderStyle, Bounds, BoxShadow, Corners, Div, Edges, Hsla, ParentElement, Pixels, Styled,
    canvas, div, linear_color_stop, linear_gradient, point, px, quad, size,
};
use refractor_security::clamp_f32;
use refractor_tokens::{
    ComponentStyleOverride, PrismTheme, RefractionIntensity, RgbColor, ThemeMaterial, ThemeMode,
};

use crate::gpui_bridge::{color_str_to_hsla, rgb_to_hsla};

/// Semantic optical surface role matching ADR-0006 material hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceVariant {
    /// Highest-emphasis refractive primary CTA capsule (`.prism-btn-primary`).
    PrimaryButton,
    /// Ice-crystal secondary glass button (`.prism-btn-secondary`).
    SecondaryButton,
    /// Deep cobalt glass button (`.prism-btn-cobalt`).
    CobaltButton,
    /// Frosted translucent crystal card / control frame (`.prism-crystal`).
    Crystal,
    /// Large 24px structural optical glass slab (`.prism-slab`).
    Slab,
    /// Recessed high-contrast input well (`.prism-well`).
    Well,
    /// Active translucent emerald toggle / positive indicator (`.prism-emerald`).
    Emerald,
    /// Tactile 3D porcelain thumb geometry (`.prism-thumb`).
    Thumb,
    /// Floating notification / toast surface (`.prism-toast`).
    Toast,
    /// Desktop window titlebar / chrome bar.
    Chrome,
    /// Desktop window outer shell.
    WindowFrame,
    /// Compact pill badge surface.
    Badge,
}

/// Blends two `RgbColor` values linearly (`ratio = 0.0` returns `start`, `ratio = 1.0` returns `end`).
fn lerp_rgb(start: RgbColor, end: RgbColor, ratio: f32) -> RgbColor {
    let factor = ratio.clamp(0.0, 1.0);
    let mix = |from_ch: u8, to_ch: u8| -> u8 {
        let blended = f32::from(from_ch) + (f32::from(to_ch) - f32::from(from_ch)) * factor;
        blended.round().clamp(0.0, 255.0) as u8
    };
    RgbColor::new(
        mix(start.r, end.r),
        mix(start.g, end.g),
        mix(start.b, end.b),
    )
}

/// Effective caustic intensity multiplier derived from `theme.optics.refraction` and `caustic_opacity`.
fn caustic_scale(theme: &PrismTheme) -> f32 {
    let base = match theme.optics.refraction {
        RefractionIntensity::Subtle => 0.45,
        RefractionIntensity::Standard => 1.0,
        RefractionIntensity::Vivid => 1.35,
    };
    clamp_f32(base * theme.optics.caustic_opacity, 0.0, 1.5, 1.0)
}

/// Effective specular highlight opacity multiplier.
fn specular_scale(theme: &PrismTheme) -> f32 {
    if !theme.optics.specular_highlight {
        return 0.0;
    }
    clamp_f32(theme.optics.specular_opacity, 0.0, 1.2, 1.0)
}

/// Computes the surface corner radius respecting material sharpness and variant hierarchy.
fn resolve_radius(theme: &PrismTheme, variant: SurfaceVariant) -> f32 {
    let sharp = theme.fx.sharp_corners || theme.material.is_sharp();
    if sharp {
        return if theme.material == ThemeMaterial::Pixel {
            2.0
        } else {
            0.0
        };
    }
    match variant {
        SurfaceVariant::PrimaryButton
        | SurfaceVariant::SecondaryButton
        | SurfaceVariant::CobaltButton
        | SurfaceVariant::Emerald
        | SurfaceVariant::Thumb
        | SurfaceVariant::Toast
        | SurfaceVariant::Badge => theme.shape.button_radius_px,
        SurfaceVariant::Slab | SurfaceVariant::WindowFrame => theme.shape.slab_radius_px,
        SurfaceVariant::Chrome => 0.0,
        SurfaceVariant::Crystal | SurfaceVariant::Well => theme.shape.border_radius_px,
    }
}

/// Builds a GPUI [`Div`] pre-styled with the active [`PrismTheme`]'s full material physics:
/// GPU linear gradients, mathematically concentric SDF specular arcs and inner rims,
/// 3D bevels, recessed well depth, border geometry, and multi-layer caustic/elevation shadows.
pub fn optical_surface(theme: &PrismTheme, variant: SurfaceVariant) -> Div {
    let is_dark = theme.palette.mode == ThemeMode::Dark;
    let radius = resolve_radius(theme, variant);
    let border_w = match theme.material {
        ThemeMaterial::Neumorphic => 0.0,
        ThemeMaterial::Brutalist | ThemeMaterial::Pixel => theme.fx.border_width_px.max(2.0),
        _ => theme.fx.border_width_px.max(1.0),
    };

    let border_color = resolve_border_color(theme, variant, is_dark);
    let shadows = build_material_shadows(theme, variant);

    let mut el = div().relative().rounded(px(radius));

    if border_w > 0.0 {
        el = el.border(px(border_w)).border_color(border_color);
    }

    el = apply_material_fill(el, theme, variant, is_dark);

    if !shadows.is_empty() {
        el = el.shadow(shadows);
    }

    el = attach_concentric_fx_canvas(el, theme, variant, radius, border_w, is_dark);

    el
}

fn resolve_border_color(theme: &PrismTheme, variant: SurfaceVariant, is_dark: bool) -> Hsla {
    match theme.material {
        ThemeMaterial::Neon | ThemeMaterial::Crt => match variant {
            SurfaceVariant::PrimaryButton => rgb_to_hsla(theme.palette.primary.light_rgb, 0.92),
            SurfaceVariant::Emerald => rgb_to_hsla(theme.palette.success.light_rgb, 0.9),
            _ => color_str_to_hsla(&theme.fx.glow_color, theme.palette.primary.rgb),
        },
        ThemeMaterial::Brutalist | ThemeMaterial::Pixel => {
            color_str_to_hsla(&theme.fx.window_border, RgbColor::new(15, 23, 42))
        }
        ThemeMaterial::Bevel => {
            color_str_to_hsla(&theme.fx.window_border, RgbColor::new(64, 64, 64))
        }
        _ => match variant {
            SurfaceVariant::PrimaryButton => {
                rgb_to_hsla(theme.palette.primary.dark_rgb, if is_dark { 0.45 } else { 0.35 })
            }
            SurfaceVariant::CobaltButton => rgb_to_hsla(theme.palette.info.dark_rgb, 0.40),
            SurfaceVariant::Emerald => rgb_to_hsla(theme.palette.success.dark_rgb, 0.38),
            SurfaceVariant::Thumb => rgb_to_hsla(RgbColor::new(255, 255, 255), 0.94),
            SurfaceVariant::Well => {
                if is_dark {
                    rgb_to_hsla(RgbColor::new(148, 163, 184), 0.24)
                } else {
                    rgb_to_hsla(RgbColor::new(132, 148, 172), 0.34)
                }
            }
            SurfaceVariant::Badge => rgb_to_hsla(theme.palette.primary.rgb, 0.30),
            _ => color_str_to_hsla(
                &theme.fx.window_border,
                if is_dark {
                    RgbColor::new(148, 163, 184)
                } else {
                    RgbColor::new(122, 138, 162)
                },
            ),
        },
    }
}

fn apply_material_fill(el: Div, theme: &PrismTheme, variant: SurfaceVariant, is_dark: bool) -> Div {
    // Solid archetypes (Bevel, Brutalist, Pixel, Paper, Flat, Crt)
    if matches!(
        theme.material,
        ThemeMaterial::Bevel
            | ThemeMaterial::Brutalist
            | ThemeMaterial::Pixel
            | ThemeMaterial::Paper
            | ThemeMaterial::Flat
            | ThemeMaterial::Crt
    ) {
        let solid = match variant {
            SurfaceVariant::PrimaryButton => rgb_to_hsla(theme.palette.primary.rgb, 1.0),
            SurfaceVariant::CobaltButton => rgb_to_hsla(theme.palette.info.rgb, 1.0),
            SurfaceVariant::Emerald => rgb_to_hsla(theme.palette.success.rgb, 1.0),
            SurfaceVariant::Thumb => rgb_to_hsla(RgbColor::new(255, 255, 255), 0.98),
            SurfaceVariant::Well => {
                color_str_to_hsla(&theme.palette.background.well, RgbColor::new(235, 240, 246))
            }
            SurfaceVariant::Chrome => {
                color_str_to_hsla(&theme.fx.chrome_bg, RgbColor::new(220, 226, 234))
            }
            SurfaceVariant::Slab | SurfaceVariant::WindowFrame => {
                color_str_to_hsla(&theme.fx.window_bg, RgbColor::new(241, 245, 249))
            }
            SurfaceVariant::Badge => rgb_to_hsla(theme.palette.primary.rgb, 0.2),
            _ => color_str_to_hsla(&theme.fx.surface_bg, RgbColor::new(245, 247, 250)),
        };
        return el.bg(solid);
    }

    // Gradient-driven optical materials (Glass, Mica, Chrome, Resin, Clay, Neumorphic, Neon)
    match variant {
        SurfaceVariant::PrimaryButton => {
            // Matches `refractor.css` .prism-btn-primary: subtle deeper top -> luminous primary light bottom
            let rich_top = lerp_rgb(theme.palette.primary.rgb, theme.palette.primary.dark_rgb, 0.28);
            let (angle, top, bottom) = match theme.material {
                ThemeMaterial::Glass => (
                    180.0,
                    rgb_to_hsla(rich_top, 0.98),
                    rgb_to_hsla(theme.palette.primary.light_rgb, 0.98),
                ),
                ThemeMaterial::Chrome | ThemeMaterial::Resin => (
                    135.0,
                    rgb_to_hsla(theme.palette.primary.light_rgb, 0.98),
                    rgb_to_hsla(rich_top, 0.98),
                ),
                _ => (
                    180.0,
                    rgb_to_hsla(theme.palette.primary.light_rgb, 0.96),
                    rgb_to_hsla(theme.palette.primary.rgb, 0.98),
                ),
            };
            el.bg(linear_gradient(
                angle,
                linear_color_stop(top, 0.0),
                linear_color_stop(bottom, 1.0),
            ))
        }
        SurfaceVariant::CobaltButton => {
            let deep = lerp_rgb(theme.palette.info.rgb, theme.palette.info.dark_rgb, 0.35);
            el.bg(linear_gradient(
                135.0,
                linear_color_stop(rgb_to_hsla(theme.palette.info.light_rgb, 0.96), 0.0),
                linear_color_stop(rgb_to_hsla(deep, 0.98), 1.0),
            ))
        }
        SurfaceVariant::Emerald => {
            let deep = lerp_rgb(theme.palette.success.rgb, theme.palette.success.dark_rgb, 0.35);
            el.bg(linear_gradient(
                145.0,
                linear_color_stop(rgb_to_hsla(deep, 0.96), 0.0),
                linear_color_stop(rgb_to_hsla(theme.palette.success.light_rgb, 0.96), 1.0),
            ))
        }
        SurfaceVariant::Thumb => el.bg(linear_gradient(
            180.0,
            linear_color_stop(rgb_to_hsla(RgbColor::new(255, 255, 255), 1.0), 0.0),
            linear_color_stop(rgb_to_hsla(RgbColor::new(232, 238, 246), 1.0), 1.0),
        )),
        SurfaceVariant::SecondaryButton => {
            if is_dark {
                let top = color_str_to_hsla(&theme.fx.surface_bg, RgbColor::new(40, 54, 76));
                let bottom = color_str_to_hsla(&theme.fx.window_bg, RgbColor::new(22, 32, 48));
                el.bg(linear_gradient(
                    180.0,
                    linear_color_stop(Hsla { a: 0.92, ..top }, 0.0),
                    linear_color_stop(Hsla { a: 0.88, ..bottom }, 1.0),
                ))
            } else {
                // Cool crystal ice-blue glass matching `.prism-btn-secondary`
                let top = rgb_to_hsla(RgbColor::new(240, 246, 254), 0.92);
                let bottom = rgb_to_hsla(
                    lerp_rgb(RgbColor::new(208, 226, 246), theme.palette.secondary.light_rgb, 0.45),
                    0.84,
                );
                el.bg(linear_gradient(
                    180.0,
                    linear_color_stop(top, 0.0),
                    linear_color_stop(bottom, 1.0),
                ))
            }
        }
        SurfaceVariant::Crystal => {
            let base = color_str_to_hsla(
                &theme.fx.surface_bg,
                if is_dark {
                    RgbColor::new(34, 45, 64)
                } else {
                    RgbColor::new(242, 246, 251)
                },
            );
            let top = Hsla {
                l: (base.l + 0.025).clamp(0.0, 1.0),
                a: base.a.max(0.84),
                ..base
            };
            let bottom = Hsla {
                l: (base.l - 0.03).clamp(0.0, 1.0),
                a: base.a.max(0.78),
                ..base
            };
            el.bg(linear_gradient(
                180.0,
                linear_color_stop(top, 0.0),
                linear_color_stop(bottom, 1.0),
            ))
        }
        SurfaceVariant::Slab | SurfaceVariant::WindowFrame => {
            let base = color_str_to_hsla(
                &theme.fx.window_bg,
                if is_dark {
                    RgbColor::new(26, 36, 52)
                } else {
                    RgbColor::new(241, 245, 251)
                },
            );
            let top = Hsla {
                l: (base.l + if is_dark { 0.03 } else { 0.025 }).clamp(0.0, 1.0),
                a: base.a.max(0.84),
                ..base
            };
            let bottom = Hsla {
                l: (base.l - 0.03).clamp(0.0, 1.0),
                a: base.a.max(0.78),
                ..base
            };
            let angle = if theme.material == ThemeMaterial::Chrome {
                150.0
            } else {
                180.0
            };
            el.bg(linear_gradient(
                angle,
                linear_color_stop(top, 0.0),
                linear_color_stop(bottom, 1.0),
            ))
        }
        SurfaceVariant::Well => {
            let start = color_str_to_hsla(
                &theme.palette.background.well,
                if is_dark {
                    RgbColor::new(13, 19, 29)
                } else {
                    RgbColor::new(236, 241, 248)
                },
            );
            let end = Hsla {
                l: (start.l + if is_dark { 0.025 } else { 0.03 }).clamp(0.0, 1.0),
                a: 0.96,
                ..start
            };
            el.bg(linear_gradient(
                180.0,
                linear_color_stop(Hsla { a: 0.94, ..start }, 0.0),
                linear_color_stop(end, 1.0),
            ))
        }
        SurfaceVariant::Toast => {
            let hi = color_str_to_hsla(
                &theme.fx.surface_highlight,
                if is_dark {
                    RgbColor::new(30, 41, 59)
                } else {
                    RgbColor::new(252, 253, 255)
                },
            );
            let lo = color_str_to_hsla(
                &theme.fx.surface_bg,
                if is_dark {
                    RgbColor::new(20, 28, 42)
                } else {
                    RgbColor::new(235, 241, 249)
                },
            );
            el.bg(linear_gradient(
                180.0,
                linear_color_stop(hi, 0.0),
                linear_color_stop(lo, 1.0),
            ))
        }
        SurfaceVariant::Chrome => {
            let base = color_str_to_hsla(
                &theme.fx.chrome_bg,
                if is_dark {
                    RgbColor::new(20, 28, 42)
                } else {
                    RgbColor::new(238, 243, 250)
                },
            );
            let bottom = Hsla {
                l: (base.l - 0.02).clamp(0.0, 1.0),
                ..base
            };
            el.bg(linear_gradient(
                180.0,
                linear_color_stop(base, 0.0),
                linear_color_stop(bottom, 1.0),
            ))
        }
        SurfaceVariant::Badge => el.bg(linear_gradient(
            180.0,
            linear_color_stop(rgb_to_hsla(theme.palette.primary.light_rgb, 0.24), 0.0),
            linear_color_stop(rgb_to_hsla(theme.palette.primary.rgb, 0.14), 1.0),
        )),
    }
}

/// Attaches a zero-layout GPU SDF `canvas` painter that renders mathematically concentric
/// inner rims, specular glass arcs, 3D bevels, recessed well shadows, and toast rings using
/// the exact runtime element bounds — completely eliminating corner poke-out or clipping defects.
fn attach_concentric_fx_canvas(
    el: Div,
    theme: &PrismTheme,
    variant: SurfaceVariant,
    radius: f32,
    border_w: f32,
    is_dark: bool,
) -> Div {
    let material = theme.material;
    let spec = specular_scale(theme);
    let caustic = caustic_scale(theme);

    let primary_dark = theme.palette.primary.dark_rgb;
    let info_dark = theme.palette.info.dark_rgb;
    let success_dark = theme.palette.success.dark_rgb;
    let secondary_dark = theme.palette.secondary.dark_rgb;
    let warning_rgb = theme.palette.warning.rgb;
    let divider_color = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));
    let caustic_tint = if material == ThemeMaterial::Resin {
        color_str_to_hsla(&theme.fx.glow_color, theme.palette.primary.rgb)
    } else {
        rgb_to_hsla(theme.palette.secondary.rgb, 1.0)
    };

    let overlay = canvas(
        |_bounds, _window, _cx| {},
        move |bounds: Bounds<Pixels>, (), window, _cx| {
            let w: f32 = bounds.size.width.into();
            let h: f32 = bounds.size.height.into();
            if w <= 2.0 || h <= 2.0 {
                return;
            }

            let transparent = rgb_to_hsla(RgbColor::new(0, 0, 0), 0.0);
            let inset = border_w.max(0.5);
            let iw = (w - inset * 2.0).max(0.0);
            let ih = (h - inset * 2.0).max(0.0);
            if iw <= 1.0 || ih <= 1.0 {
                return;
            }

            let effective_r = radius.min(h * 0.5).min(w * 0.5);
            let inner_r = (effective_r - inset).max(0.0);
            let inner_bounds = Bounds::new(
                point(bounds.origin.x + px(inset), bounds.origin.y + px(inset)),
                size(px(iw), px(ih)),
            );

            // 1. Classic Win95 / Amiga 3D Bevel inner highlight & shadow borders
            if material == ThemeMaterial::Bevel {
                let (tl_col, br_col) = if variant == SurfaceVariant::Well {
                    (
                        rgb_to_hsla(RgbColor::new(0, 0, 0), 0.45),
                        rgb_to_hsla(RgbColor::new(255, 255, 255), 0.85),
                    )
                } else {
                    (
                        rgb_to_hsla(RgbColor::new(255, 255, 255), 0.85),
                        rgb_to_hsla(RgbColor::new(0, 0, 0), 0.40),
                    )
                };
                window.paint_quad(quad(
                    inner_bounds,
                    Corners::all(px(inner_r)),
                    transparent,
                    Edges {
                        top: px(2.0),
                        left: px(2.0),
                        bottom: px(0.0),
                        right: px(0.0),
                    },
                    tl_col,
                    BorderStyle::Solid,
                ));
                window.paint_quad(quad(
                    inner_bounds,
                    Corners::all(px(inner_r)),
                    transparent,
                    Edges {
                        top: px(0.0),
                        left: px(0.0),
                        bottom: px(2.0),
                        right: px(2.0),
                    },
                    br_col,
                    BorderStyle::Solid,
                ));
                return;
            }

            // 2. Recessed Well concentric inner shadow + bottom catchlight lip
            if variant == SurfaceVariant::Well
                && !matches!(
                    material,
                    ThemeMaterial::Flat | ThemeMaterial::Paper | ThemeMaterial::Brutalist
                )
            {
                let top_shadow = if is_dark {
                    rgb_to_hsla(RgbColor::new(0, 0, 0), 0.42)
                } else {
                    rgb_to_hsla(RgbColor::new(15, 23, 42), 0.09)
                };
                window.paint_quad(quad(
                    inner_bounds,
                    Corners::all(px(inner_r)),
                    transparent,
                    Edges {
                        top: px(2.0),
                        left: px(1.0),
                        right: px(1.0),
                        bottom: px(0.0),
                    },
                    top_shadow,
                    BorderStyle::Solid,
                ));
                if !is_dark {
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(0.0),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(1.0),
                        },
                        rgb_to_hsla(RgbColor::new(255, 255, 255), 0.85),
                        BorderStyle::Solid,
                    ));
                }
                return;
            }

            // 3. Toast concentric warm amber inner ring (`.prism-toast::after`)
            if variant == SurfaceVariant::Toast && iw > 10.0 && ih > 10.0 {
                let ring_inset = 3.5_f32;
                let ring_bounds = Bounds::new(
                    point(
                        bounds.origin.x + px(ring_inset),
                        bounds.origin.y + px(ring_inset),
                    ),
                    size(px((w - ring_inset * 2.0).max(0.0)), px((h - ring_inset * 2.0).max(0.0))),
                );
                let ring_r = (effective_r - ring_inset).max(0.0);
                window.paint_quad(quad(
                    ring_bounds,
                    Corners::all(px(ring_r)),
                    transparent,
                    Edges::all(px(1.5)),
                    rgb_to_hsla(warning_rgb, 0.72),
                    BorderStyle::Solid,
                ));
            }

            // 4. Chrome bottom hairline divider
            if variant == SurfaceVariant::Chrome {
                window.paint_quad(quad(
                    bounds,
                    Corners::all(px(0.0)),
                    transparent,
                    Edges {
                        top: px(0.0),
                        left: px(0.0),
                        right: px(0.0),
                        bottom: px(1.0),
                    },
                    divider_color,
                    BorderStyle::Solid,
                ));
                return;
            }

            // Skip specular rims for non-reflective materials or when specular is disabled
            if spec <= 0.01
                || matches!(
                    material,
                    ThemeMaterial::Brutalist
                        | ThemeMaterial::Pixel
                        | ThemeMaterial::Paper
                        | ThemeMaterial::Flat
                        | ThemeMaterial::Crt
                )
            {
                return;
            }

            match variant {
                SurfaceVariant::PrimaryButton
                | SurfaceVariant::CobaltButton
                | SurfaceVariant::Emerald => {
                    let bottom_rgb = match variant {
                        SurfaceVariant::PrimaryButton => primary_dark,
                        SurfaceVariant::CobaltButton => info_dark,
                        _ => success_dark,
                    };

                    // Concentric top inner specular rim (curves cleanly around pill ends)
                    let top_rim_alpha = if matches!(
                        material,
                        ThemeMaterial::Glass
                            | ThemeMaterial::Chrome
                            | ThemeMaterial::Resin
                            | ThemeMaterial::Clay
                    ) {
                        (0.68 * spec).clamp(0.0, 0.82)
                    } else {
                        (0.32 * spec).clamp(0.0, 0.45)
                    };
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(1.5),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(0.0),
                        },
                        rgb_to_hsla(RgbColor::new(255, 255, 255), top_rim_alpha),
                        BorderStyle::Solid,
                    ));

                    // Concentric bottom inner depth rim (`inset 0 -2px ...`)
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(0.0),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(2.0),
                        },
                        rgb_to_hsla(bottom_rgb, 0.34),
                        BorderStyle::Solid,
                    ));

                    // Concentric full-perimeter warm/cool optical inner hairline
                    if matches!(material, ThemeMaterial::Glass | ThemeMaterial::Chrome) {
                        window.paint_quad(quad(
                            inner_bounds,
                            Corners::all(px(inner_r)),
                            transparent,
                            Edges::all(px(1.0)),
                            rgb_to_hsla(RgbColor::new(255, 245, 235), (0.28 * spec).clamp(0.0, 0.40)),
                            BorderStyle::Solid,
                        ));
                    }

                    // Nested specular glass capsule arc (`.prism-btn-primary::before`)
                    // Drawn ONLY when element is large enough, with mathematically computed
                    // horizontal clearance from the outer corner curve so it NEVER pokes out.
                    if matches!(
                        material,
                        ThemeMaterial::Glass
                            | ThemeMaterial::Chrome
                            | ThemeMaterial::Resin
                            | ThemeMaterial::Clay
                    ) && iw >= 34.0
                        && ih >= 24.0
                    {
                        let arc_pad_top = 2.0_f32;
                        let arc_h = (ih * 0.34).clamp(5.0, 12.0);
                        // Exact horizontal distance from outer rounded corner at y = arc_pad_top
                        let dy = (inner_r - arc_pad_top).max(0.0);
                        let corner_inset = if inner_r > 1.0 {
                            inner_r - (inner_r * inner_r - dy * dy).max(0.0).sqrt()
                        } else {
                            2.0
                        };
                        let arc_inset_x = (corner_inset + 3.0).max(5.0);
                        let arc_w = iw - arc_inset_x * 2.0;

                        if arc_w >= 14.0 {
                            let arc_r = (arc_h * 0.5).min((inner_r - 2.0).max(0.0));
                            let arc_bounds = Bounds::new(
                                point(
                                    inner_bounds.origin.x + px(arc_inset_x),
                                    inner_bounds.origin.y + px(arc_pad_top),
                                ),
                                size(px(arc_w), px(arc_h)),
                            );
                            window.paint_quad(quad(
                                arc_bounds,
                                Corners::all(px(arc_r)),
                                linear_gradient(
                                    180.0,
                                    linear_color_stop(
                                        rgb_to_hsla(
                                            RgbColor::new(255, 255, 255),
                                            (0.36 * spec).clamp(0.0, 0.48),
                                        ),
                                        0.0,
                                    ),
                                    linear_color_stop(
                                        rgb_to_hsla(RgbColor::new(255, 255, 255), 0.0),
                                        1.0,
                                    ),
                                ),
                                Edges::default(),
                                transparent,
                                BorderStyle::Solid,
                            ));
                        }
                    }
                }
                SurfaceVariant::SecondaryButton | SurfaceVariant::Crystal => {
                    let top_rim_alpha =
                        if is_dark { 0.26 * spec } else { 0.90 * spec }.clamp(0.0, 0.95);
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(1.5),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(0.0),
                        },
                        rgb_to_hsla(RgbColor::new(255, 255, 255), top_rim_alpha),
                        BorderStyle::Solid,
                    ));
                    if !is_dark {
                        let bottom_col = if variant == SurfaceVariant::SecondaryButton {
                            rgb_to_hsla(secondary_dark, 0.18)
                        } else {
                            rgb_to_hsla(RgbColor::new(30, 41, 59), 0.10)
                        };
                        window.paint_quad(quad(
                            inner_bounds,
                            Corners::all(px(inner_r)),
                            transparent,
                            Edges {
                                top: px(0.0),
                                left: px(0.0),
                                right: px(0.0),
                                bottom: px(1.5),
                            },
                            bottom_col,
                            BorderStyle::Solid,
                        ));
                    }
                }
                SurfaceVariant::Slab | SurfaceVariant::WindowFrame => {
                    // Subtle diagonal prismatic glass wash strictly inside inner_bounds
                    if matches!(
                        material,
                        ThemeMaterial::Glass | ThemeMaterial::Chrome | ThemeMaterial::Resin
                    ) {
                        let top_wash =
                            if is_dark { 0.08 * spec } else { 0.28 * spec }.clamp(0.0, 0.36);
                        let bottom_wash = Hsla {
                            a: if is_dark { 0.05 } else { 0.08 } * caustic.min(1.0),
                            ..caustic_tint
                        };
                        window.paint_quad(quad(
                            inner_bounds,
                            Corners::all(px(inner_r)),
                            linear_gradient(
                                128.0,
                                linear_color_stop(
                                    rgb_to_hsla(RgbColor::new(255, 255, 255), top_wash),
                                    0.0,
                                ),
                                linear_color_stop(bottom_wash, 1.0),
                            ),
                            Edges::default(),
                            transparent,
                            BorderStyle::Solid,
                        ));
                    }
                    // Concentric top specular catchlight rim
                    let rim_alpha =
                        if is_dark { 0.24 * spec } else { 0.92 * spec }.clamp(0.0, 0.96);
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(1.5),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(0.0),
                        },
                        rgb_to_hsla(RgbColor::new(255, 255, 255), rim_alpha),
                        BorderStyle::Solid,
                    ));
                }
                SurfaceVariant::Thumb => {
                    // Porcelain 3D dome top specular rim + bottom subtle shade
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(1.5),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(0.0),
                        },
                        rgb_to_hsla(RgbColor::new(255, 255, 255), 0.98),
                        BorderStyle::Solid,
                    ));
                    window.paint_quad(quad(
                        inner_bounds,
                        Corners::all(px(inner_r)),
                        transparent,
                        Edges {
                            top: px(0.0),
                            left: px(0.0),
                            right: px(0.0),
                            bottom: px(1.5),
                        },
                        rgb_to_hsla(RgbColor::new(148, 163, 184), 0.30),
                        BorderStyle::Solid,
                    ));
                }
                SurfaceVariant::Toast
                | SurfaceVariant::Badge
                | SurfaceVariant::Well
                | SurfaceVariant::Chrome => {}
            }
        },
    )
    .size_full();

    el.child(div().absolute().inset_0().child(overlay))
}

fn build_material_shadows(theme: &PrismTheme, variant: SurfaceVariant) -> Vec<BoxShadow> {
    let is_dark = theme.palette.mode == ThemeMode::Dark;
    let caustic = caustic_scale(theme);

    match theme.material {
        ThemeMaterial::Brutalist => vec![BoxShadow {
            color: color_str_to_hsla(&theme.fx.window_border, RgbColor::new(0, 0, 0)),
            offset: point(px(3.0), px(3.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
        }],
        ThemeMaterial::Pixel => vec![BoxShadow {
            color: rgb_to_hsla(theme.palette.primary.dark_rgb, 0.9),
            offset: point(px(0.0), px(3.0)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
        }],
        ThemeMaterial::Neon | ThemeMaterial::Crt => {
            let glow_alpha = (0.48 * theme.fx.glow_intensity).clamp(0.14, 0.82);
            let glow_col = if variant == SurfaceVariant::PrimaryButton {
                rgb_to_hsla(theme.palette.primary.rgb, glow_alpha)
            } else {
                let c = color_str_to_hsla(&theme.fx.glow_color, theme.palette.primary.rgb);
                Hsla { a: glow_alpha, ..c }
            };
            vec![
                BoxShadow {
                    color: glow_col,
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(16.0 * theme.fx.glow_intensity.clamp(0.4, 1.6)),
                    spread_radius: px(1.0),
                },
                BoxShadow {
                    color: rgb_to_hsla(RgbColor::new(0, 0, 0), 0.45),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(20.0),
                    spread_radius: px(0.0),
                },
            ]
        }
        ThemeMaterial::Neumorphic => {
            if variant == SurfaceVariant::Well {
                return Vec::new();
            }
            let dark_drop = if is_dark {
                rgb_to_hsla(RgbColor::new(0, 0, 0), 0.55)
            } else {
                rgb_to_hsla(RgbColor::new(163, 177, 198), 0.58)
            };
            let light_lift = if is_dark {
                rgb_to_hsla(RgbColor::new(255, 255, 255), 0.08)
            } else {
                rgb_to_hsla(RgbColor::new(255, 255, 255), 0.78)
            };
            vec![
                BoxShadow {
                    color: dark_drop,
                    offset: point(px(6.0), px(6.0)),
                    blur_radius: px(14.0),
                    spread_radius: px(0.0),
                },
                BoxShadow {
                    color: light_lift,
                    offset: point(px(-6.0), px(-6.0)),
                    blur_radius: px(14.0),
                    spread_radius: px(0.0),
                },
            ]
        }
        ThemeMaterial::Clay => vec![
            BoxShadow {
                color: rgb_to_hsla(theme.palette.primary.rgb, if is_dark { 0.35 } else { 0.24 }),
                offset: point(px(0.0), px(12.0)),
                blur_radius: px(24.0),
                spread_radius: px(-4.0),
            },
            BoxShadow {
                color: rgb_to_hsla(RgbColor::new(15, 23, 42), if is_dark { 0.42 } else { 0.12 }),
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(10.0),
                spread_radius: px(0.0),
            },
        ],
        ThemeMaterial::Resin => {
            let glow = color_str_to_hsla(&theme.fx.glow_color, theme.palette.primary.rgb);
            vec![
                BoxShadow {
                    color: Hsla {
                        a: (0.42 * theme.fx.glow_intensity).clamp(0.18, 0.65),
                        ..glow
                    },
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(22.0),
                    spread_radius: px(-2.0),
                },
                BoxShadow {
                    color: rgb_to_hsla(RgbColor::new(0, 0, 0), if is_dark { 0.5 } else { 0.16 }),
                    offset: point(px(0.0), px(12.0)),
                    blur_radius: px(28.0),
                    spread_radius: px(-6.0),
                },
            ]
        }
        ThemeMaterial::Paper => match variant {
            SurfaceVariant::PrimaryButton => vec![BoxShadow {
                color: rgb_to_hsla(RgbColor::new(0, 0, 0), 0.22),
                offset: point(px(0.0), px(2.0)),
                blur_radius: px(0.0),
                spread_radius: px(0.0),
            }],
            _ => Vec::new(),
        },
        ThemeMaterial::Flat => match variant {
            SurfaceVariant::PrimaryButton => vec![BoxShadow {
                color: rgb_to_hsla(theme.palette.primary.rgb, 0.30),
                offset: point(px(0.0), px(2.0)),
                blur_radius: px(8.0),
                spread_radius: px(0.0),
            }],
            SurfaceVariant::Slab | SurfaceVariant::WindowFrame => vec![BoxShadow {
                color: rgb_to_hsla(RgbColor::new(0, 0, 0), if is_dark { 0.36 } else { 0.08 }),
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(14.0),
                spread_radius: px(0.0),
            }],
            _ => Vec::new(),
        },
        ThemeMaterial::Bevel => match variant {
            SurfaceVariant::Slab | SurfaceVariant::WindowFrame => vec![BoxShadow {
                color: rgb_to_hsla(RgbColor::new(0, 0, 0), 0.28),
                offset: point(px(2.0), px(2.0)),
                blur_radius: px(0.0),
                spread_radius: px(0.0),
            }],
            _ => Vec::new(),
        },
        // Optical Glass, Mica, Chrome — dual ambient + directional caustic floor bloom
        ThemeMaterial::Glass | ThemeMaterial::Mica | ThemeMaterial::Chrome => match variant {
            SurfaceVariant::PrimaryButton => {
                let mut list = vec![BoxShadow {
                    color: rgb_to_hsla(theme.palette.primary.rgb, 0.42),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(18.0),
                    spread_radius: px(-5.0),
                }];
                if caustic > 0.05 {
                    list.push(BoxShadow {
                        color: rgb_to_hsla(
                            theme.palette.primary.light_rgb,
                            (0.38 * caustic).clamp(0.0, 0.56),
                        ),
                        offset: point(px(5.0), px(9.0)),
                        blur_radius: px(15.0),
                        spread_radius: px(-5.0),
                    });
                }
                list
            }
            SurfaceVariant::CobaltButton => {
                let mut list = vec![BoxShadow {
                    color: rgb_to_hsla(theme.palette.info.rgb, 0.42),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(18.0),
                    spread_radius: px(-5.0),
                }];
                if caustic > 0.05 {
                    list.push(BoxShadow {
                        color: rgb_to_hsla(
                            theme.palette.info.light_rgb,
                            (0.36 * caustic).clamp(0.0, 0.52),
                        ),
                        offset: point(px(5.0), px(9.0)),
                        blur_radius: px(14.0),
                        spread_radius: px(-4.0),
                    });
                }
                list
            }
            SurfaceVariant::Emerald => vec![BoxShadow {
                color: rgb_to_hsla(theme.palette.success.rgb, 0.36),
                offset: point(px(0.0), px(6.0)),
                blur_radius: px(14.0),
                spread_radius: px(-4.0),
            }],
            SurfaceVariant::Thumb => vec![BoxShadow {
                color: rgb_to_hsla(RgbColor::new(15, 23, 42), 0.24),
                offset: point(px(0.0), px(3.0)),
                blur_radius: px(7.0),
                spread_radius: px(0.0),
            }],
            SurfaceVariant::Slab | SurfaceVariant::WindowFrame | SurfaceVariant::Toast => {
                let mut list = vec![BoxShadow {
                    color: rgb_to_hsla(
                        RgbColor::new(15, 23, 42),
                        if is_dark { 0.54 } else { 0.18 },
                    ),
                    offset: point(px(0.0), px(16.0)),
                    blur_radius: px(30.0),
                    spread_radius: px(-10.0),
                }];
                if !is_dark && caustic > 0.05 && theme.material == ThemeMaterial::Glass {
                    list.push(BoxShadow {
                        color: rgb_to_hsla(
                            theme.palette.primary.light_rgb,
                            (0.26 * caustic).clamp(0.0, 0.40),
                        ),
                        offset: point(px(8.0), px(12.0)),
                        blur_radius: px(20.0),
                        spread_radius: px(-9.0),
                    });
                }
                list
            }
            SurfaceVariant::SecondaryButton | SurfaceVariant::Crystal => {
                let mut list = vec![BoxShadow {
                    color: rgb_to_hsla(
                        RgbColor::new(28, 52, 88),
                        if is_dark { 0.46 } else { 0.16 },
                    ),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(18.0),
                    spread_radius: px(-6.0),
                }];
                if !is_dark && caustic > 0.05 && theme.material == ThemeMaterial::Glass {
                    let caustic_rgb = if variant == SurfaceVariant::SecondaryButton {
                        theme.palette.secondary.rgb
                    } else {
                        theme.palette.primary.light_rgb
                    };
                    list.push(BoxShadow {
                        color: rgb_to_hsla(caustic_rgb, (0.26 * caustic).clamp(0.0, 0.40)),
                        offset: point(px(6.0), px(9.0)),
                        blur_radius: px(14.0),
                        spread_radius: px(-5.0),
                    });
                }
                list
            }
            SurfaceVariant::Well | SurfaceVariant::Chrome | SurfaceVariant::Badge => Vec::new(),
        },
    }
}

/// Applies Astryx Tier 3 / Tier 4 [`ComponentStyleOverride`] onto a GPUI [`Div`].
pub fn apply_component_override(mut el: Div, ov: &ComponentStyleOverride) -> Div {
    if let Some(ref bg) = ov.background {
        el = el.bg(color_str_to_hsla(bg, RgbColor::new(255, 255, 255)));
    }
    if let Some(ref fg) = ov.foreground {
        el = el.text_color(color_str_to_hsla(fg, RgbColor::new(21, 30, 44)));
    }
    if let Some(ref border) = ov.border_color {
        el = el.border_color(color_str_to_hsla(border, RgbColor::new(148, 163, 184)));
    }
    if let Some(radius) = ov.border_radius_px {
        el = el.rounded(px(radius.max(0.0)));
    }
    if let Some(px_val) = ov.padding_x_px {
        el = el.px(px(px_val.max(0.0)));
    }
    if let Some(py_val) = ov.padding_y_px {
        el = el.py(px(py_val.max(0.0)));
    }
    if let Some(opacity) = ov.opacity {
        el = el.opacity(opacity.clamp(0.0, 1.0));
    }
    el
}
