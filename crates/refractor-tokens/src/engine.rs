//! Optical Glass Theme Engine (`create_prism_theme` and `define_theme`).

use std::collections::BTreeMap;

use refractor_security::clamp_f32;

use crate::color_math::{derive_palette_color, format_rgb_triplet};
use crate::types::{
    ComponentStyleOverride, DefinePrismThemeInput, DefinedPrismTheme, PaletteColorInput,
    PrismTheme, PrismThemeOptions, RefractionIntensity, ThemeBackground, ThemeCollectionCategory,
    ThemeControlsLayout, ThemeEditorColors, ThemeFxConfig, ThemeMaterial, ThemeMode,
    ThemeMotionConfig, ThemeOptics, ThemeOverlayFx, ThemePalette, ThemePerformanceMode, ThemeShape,
    ThemeTextColors, ThemeTypography, TypographyScale,
};

fn refraction_multiplier(refraction: RefractionIntensity) -> f32 {
    match refraction {
        RefractionIntensity::Subtle => 0.45,
        RefractionIntensity::Standard => 1.0,
        RefractionIntensity::Vivid => 1.35,
    }
}

/// Creates a complete [`PrismTheme`] from ergonomic partial [`PrismThemeOptions`],
/// automatically deriving tonal variants, WCAG 2.2 AA contrast text, optical glass
/// caustics, and multi-material FX defaults.
pub fn create_prism_theme(options: PrismThemeOptions) -> PrismTheme {
    let mode = options.mode.unwrap_or(ThemeMode::Light);
    let is_dark = mode == ThemeMode::Dark;
    let material = options.material.unwrap_or(ThemeMaterial::Glass);
    let refraction = options.refraction.unwrap_or(RefractionIntensity::Standard);
    let name = options
        .name
        .unwrap_or_else(|| if is_dark { "obsidian" } else { "alabaster" }.to_owned());
    let display_name = options.display_name.unwrap_or_else(|| name.clone());
    let description = options.description.unwrap_or_default();
    let category = options
        .category
        .unwrap_or(ThemeCollectionCategory::OpticalGlass);

    let derive_opt = |opt: Option<PaletteColorInput>, default_main: &str| match opt {
        Some(p) => derive_palette_color(
            &p.main,
            p.light.as_deref(),
            p.dark.as_deref(),
            p.contrast_text.as_deref(),
        ),
        None => derive_palette_color(default_main, None, None, None),
    };

    let primary = derive_opt(options.primary, "#E75C24");
    let secondary = derive_opt(options.secondary, "#38BDF8");
    let info = derive_opt(options.info, "#1D4ED8");
    let success = derive_opt(options.success, "#26BC92");
    let warning = derive_opt(options.warning, "#F59E0B");
    let error = derive_opt(options.error, "#EF4444");

    let background = options.background.unwrap_or_else(|| {
        if is_dark {
            ThemeBackground {
                default: "#0B1120".to_owned(),
                start: "#0F172A".to_owned(),
                mid: "#111827".to_owned(),
                end: "#0B1120".to_owned(),
                paper: "rgba(15, 23, 42, 0.78)".to_owned(),
                well: "rgba(15, 23, 42, 0.92)".to_owned(),
            }
        } else {
            ThemeBackground {
                default: "#E4EAF3".to_owned(),
                start: "#EEF2F7".to_owned(),
                mid: "#E2E8F1".to_owned(),
                end: "#D8E0EC".to_owned(),
                paper: "rgba(255, 255, 255, 0.72)".to_owned(),
                well: "rgba(241, 245, 250, 0.88)".to_owned(),
            }
        }
    });

    let text = options.text.unwrap_or_else(|| {
        if is_dark {
            ThemeTextColors {
                primary: "#F1F5F9".to_owned(),
                secondary: "#94A3B8".to_owned(),
                disabled: "#64748B".to_owned(),
            }
        } else {
            ThemeTextColors {
                primary: "#151E2C".to_owned(),
                secondary: "#475569".to_owned(),
                disabled: "#94A3B8".to_owned(),
            }
        }
    });

    let divider = options.divider.unwrap_or_else(|| {
        if is_dark {
            "rgba(148, 163, 184, 0.22)".to_owned()
        } else {
            "rgba(15, 23, 42, 0.12)".to_owned()
        }
    });

    let sharp = material.is_sharp();
    let default_radius = if sharp { 0.0 } else { 14.0 };
    let border_radius_px = clamp_f32(
        options.border_radius_px.unwrap_or(default_radius),
        0.0,
        36.0,
        default_radius,
    );
    let slab_radius_px = clamp_f32(
        options
            .slab_radius_px
            .unwrap_or(if sharp { 0.0 } else { 24.0 }),
        0.0,
        48.0,
        24.0,
    );

    let blur_default = material.default_blur_px();
    let blur_px = clamp_f32(
        options.blur_px.unwrap_or(blur_default),
        0.0,
        48.0,
        blur_default,
    );
    let mult = refraction_multiplier(refraction);

    let performance_mode = options
        .performance_mode
        .unwrap_or(ThemePerformanceMode::Balanced);
    let optics = ThemeOptics {
        refraction,
        performance_mode,
        blur_px,
        slab_blur_px: clamp_f32((blur_px * 1.167).round(), 0.0, 48.0, 14.0),
        caustic_opacity: clamp_f32(0.55 * mult, 0.0, 1.0, 0.55),
        specular_opacity: clamp_f32(0.75 * mult, 0.0, 1.0, 0.75),
        caustic_warm_rgb: format_rgb_triplet(primary.rgb),
        caustic_cool_rgb: format_rgb_triplet(secondary.rgb),
        specular_highlight: options.specular_highlight.unwrap_or(true),
    };

    let fx = options.fx.unwrap_or_else(|| ThemeFxConfig {
        wallpaper: background.default.clone(),
        wallpaper_overlay: ThemeOverlayFx::None,
        overlay_opacity: ThemeOverlayFx::None.default_opacity(),
        glow_color: primary.main.clone(),
        glow_intensity: if matches!(material, ThemeMaterial::Neon | ThemeMaterial::Crt) {
            1.0
        } else {
            0.35
        },
        border_width_px: if sharp { 2.0 } else { 1.0 },
        window_bg: background.paper.clone(),
        surface_bg: background.paper.clone(),
        surface_highlight: if is_dark {
            "rgba(255, 255, 255, 0.08)".to_owned()
        } else {
            "rgba(255, 255, 255, 0.90)".to_owned()
        },
        window_border: divider.clone(),
        chrome_bg: background.start.clone(),
        chrome_text: text.primary.clone(),
        window_shadow: "0 18px 48px rgba(15, 23, 42, 0.18)".to_owned(),
        control_shadow: "0 6px 18px rgba(15, 23, 42, 0.12)".to_owned(),
        well_shadow: "inset 0 2px 6px rgba(15, 23, 42, 0.12)".to_owned(),
        controls_layout: ThemeControlsLayout::Right,
        title_bar_height_px: 36.0,
        adaptive_ink: material != ThemeMaterial::Glass,
        sharp_corners: sharp,
    });

    let editor = options.editor.unwrap_or_else(|| {
        if is_dark {
            ThemeEditorColors {
                background: "#0F172A".to_owned(),
                caret: primary.main.clone(),
                comment: "#64748B".to_owned(),
                gutter: "#1E293B".to_owned(),
                keyword: secondary.main.clone(),
                line: "#1E293B".to_owned(),
                number: warning.main.clone(),
                selection: "rgba(56, 189, 248, 0.24)".to_owned(),
                selection_text: "#F8FAFC".to_owned(),
                string: success.main.clone(),
                text: "#E2E8F0".to_owned(),
            }
        } else {
            ThemeEditorColors {
                background: "#F8FAFC".to_owned(),
                caret: primary.main.clone(),
                comment: "#64748B".to_owned(),
                gutter: "#E2E8F0".to_owned(),
                keyword: info.main.clone(),
                line: "#E2E8F0".to_owned(),
                number: primary.dark.clone(),
                selection: "rgba(231, 92, 36, 0.18)".to_owned(),
                selection_text: "#0F172A".to_owned(),
                string: success.dark.clone(),
                text: "#0F172A".to_owned(),
            }
        }
    });

    PrismTheme {
        name,
        display_name,
        description,
        category,
        material,
        palette: ThemePalette {
            mode,
            primary,
            secondary,
            info,
            success,
            warning,
            error,
            background,
            text,
            divider,
        },
        typography: ThemeTypography {
            font_family: options
                .font_family
                .unwrap_or_else(|| "Inter, system-ui, sans-serif".to_owned()),
            mono_family: options
                .mono_family
                .unwrap_or_else(|| "JetBrains Mono, monospace".to_owned()),
            letter_spacing: options.letter_spacing.unwrap_or_else(|| "-0.01em".to_owned()),
            h1: TypographyScale {
                font_size_px: 28.0,
                font_weight: 700,
                line_height: 1.2,
            },
            h2: TypographyScale {
                font_size_px: 20.0,
                font_weight: 600,
                line_height: 1.3,
            },
            body1: TypographyScale {
                font_size_px: 14.0,
                font_weight: 400,
                line_height: 1.5,
            },
            caption: TypographyScale {
                font_size_px: 12.0,
                font_weight: 500,
                line_height: 1.4,
            },
        },
        shape: ThemeShape {
            border_radius_px,
            button_radius_px: if sharp { 0.0 } else { 999.0 },
            pill_radius_px: if sharp { 0.0 } else { 999.0 },
            slab_radius_px,
        },
        optics,
        fx,
        motion: ThemeMotionConfig {
            duration_fast_ms: 120,
            duration_normal_ms: 200,
            duration_slow_ms: 340,
            easing_standard: "cubic-bezier(0.2, 0.0, 0.0, 1.0)".to_owned(),
            easing_emphasized: "cubic-bezier(0.2, 0.8, 0.2, 1.0)".to_owned(),
        },
        editor,
        components: options.components,
        custom_tokens: options.custom_tokens,
    }
}

/// Cascading theme definition supporting inheritance (`extends`),
/// `[light, dark]` token pairs, and per-component overrides.
pub fn define_theme(input: DefinePrismThemeInput) -> DefinedPrismTheme {
    let mut merged_options = input
        .extends
        .as_ref()
        .map(|parent| parent.options.clone())
        .unwrap_or_default();

    merge_options(&mut merged_options, input.options);
    merged_options.name = Some(input.name.clone());

    let mut merged_tokens = input
        .extends
        .as_ref()
        .map(|parent| parent.tokens.clone())
        .unwrap_or_default();
    for (k, v) in input.tokens {
        merged_tokens.insert(k, v);
    }

    let mut merged_components: BTreeMap<String, ComponentStyleOverride> = input
        .extends
        .as_ref()
        .map(|parent| parent.components.clone())
        .unwrap_or_default();
    for (k, v) in input.components {
        merged_components.insert(k, v);
    }

    DefinedPrismTheme {
        name: input.name,
        options: merged_options,
        tokens: merged_tokens,
        components: merged_components,
    }
}

/// Resolves a [`DefinedPrismTheme`] into a concrete [`PrismTheme`] for the given mode.
pub fn resolve_defined_theme(defined: &DefinedPrismTheme, mode_override: Option<ThemeMode>) -> PrismTheme {
    let mut opts = defined.options.clone();
    if let Some(m) = mode_override {
        opts.mode = Some(m);
    }
    let resolved_mode = opts.mode.unwrap_or(ThemeMode::Light);
    for (k, v) in &defined.tokens {
        opts.custom_tokens
            .insert(k.clone(), v.resolve(resolved_mode).to_owned());
    }
    for (k, v) in &defined.components {
        opts.components.insert(k.clone(), v.clone());
    }
    create_prism_theme(opts)
}

fn merge_options(base: &mut PrismThemeOptions, overlay: PrismThemeOptions) {
    if overlay.name.is_some() {
        base.name = overlay.name;
    }
    if overlay.display_name.is_some() {
        base.display_name = overlay.display_name;
    }
    if overlay.description.is_some() {
        base.description = overlay.description;
    }
    if overlay.category.is_some() {
        base.category = overlay.category;
    }
    if overlay.material.is_some() {
        base.material = overlay.material;
    }
    if overlay.mode.is_some() {
        base.mode = overlay.mode;
    }
    if overlay.primary.is_some() {
        base.primary = overlay.primary;
    }
    if overlay.secondary.is_some() {
        base.secondary = overlay.secondary;
    }
    if overlay.info.is_some() {
        base.info = overlay.info;
    }
    if overlay.success.is_some() {
        base.success = overlay.success;
    }
    if overlay.warning.is_some() {
        base.warning = overlay.warning;
    }
    if overlay.error.is_some() {
        base.error = overlay.error;
    }
    if overlay.background.is_some() {
        base.background = overlay.background;
    }
    if overlay.text.is_some() {
        base.text = overlay.text;
    }
    if overlay.divider.is_some() {
        base.divider = overlay.divider;
    }
    if overlay.font_family.is_some() {
        base.font_family = overlay.font_family;
    }
    if overlay.mono_family.is_some() {
        base.mono_family = overlay.mono_family;
    }
    if overlay.letter_spacing.is_some() {
        base.letter_spacing = overlay.letter_spacing;
    }
    if overlay.border_radius_px.is_some() {
        base.border_radius_px = overlay.border_radius_px;
    }
    if overlay.slab_radius_px.is_some() {
        base.slab_radius_px = overlay.slab_radius_px;
    }
    if overlay.refraction.is_some() {
        base.refraction = overlay.refraction;
    }
    if overlay.performance_mode.is_some() {
        base.performance_mode = overlay.performance_mode;
    }
    if overlay.blur_px.is_some() {
        base.blur_px = overlay.blur_px;
    }
    if overlay.specular_highlight.is_some() {
        base.specular_highlight = overlay.specular_highlight;
    }
    if overlay.fx.is_some() {
        base.fx = overlay.fx;
    }
    if overlay.editor.is_some() {
        base.editor = overlay.editor;
    }
    for (k, v) in overlay.components {
        base.components.insert(k, v);
    }
    for (k, v) in overlay.custom_tokens {
        base.custom_tokens.insert(k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ThemeTokenValue;

    #[test]
    fn single_hex_derivation_creates_full_theme() {
        let theme = create_prism_theme(PrismThemeOptions {
            primary: Some(PaletteColorInput::from_main("#7C3AED")),
            refraction: Some(RefractionIntensity::Vivid),
            ..PrismThemeOptions::default()
        });
        assert_eq!(theme.palette.primary.main, "#7C3AED");
        assert_eq!(theme.palette.primary.contrast_text, "#FFFFFF");
        assert!(theme.optics.caustic_opacity > 0.7);
    }

    #[test]
    fn define_theme_supports_inheritance_and_mode_tokens() {
        let base = define_theme(DefinePrismThemeInput {
            name: "brand-base".to_owned(),
            options: PrismThemeOptions {
                primary: Some(PaletteColorInput::from_main("#0EA5E9")),
                ..PrismThemeOptions::default()
            },
            tokens: BTreeMap::from([(
                "--prism-accent-glow".to_owned(),
                ThemeTokenValue::ModePair {
                    light: "#BAE6FD".to_owned(),
                    dark: "#0284C7".to_owned(),
                },
            )]),
            ..DefinePrismThemeInput::default()
        });

        let child = define_theme(DefinePrismThemeInput {
            name: "brand-dark".to_owned(),
            extends: Some(Box::new(base)),
            options: PrismThemeOptions {
                mode: Some(ThemeMode::Dark),
                ..PrismThemeOptions::default()
            },
            ..DefinePrismThemeInput::default()
        });

        let resolved = resolve_defined_theme(&child, None);
        assert_eq!(resolved.name, "brand-dark");
        assert_eq!(resolved.palette.mode, ThemeMode::Dark);
        assert_eq!(resolved.palette.primary.main, "#0EA5E9");
        assert_eq!(
            resolved.custom_tokens.get("--prism-accent-glow").map(String::as_str),
            Some("#0284C7")
        );
    }
}
