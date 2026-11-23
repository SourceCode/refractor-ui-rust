//! Pure Rust optical glass design tokens, deterministic color math, WCAG 2.2 AA
//! contrast derivation, and dual MUI + Astryx theme engine (`create_prism_theme` & `define_theme`).

pub mod color_math;
pub mod engine;
pub mod types;

pub use color_math::{
    DEFAULT_FALLBACK_RGB, DerivedPaletteColor, HsvColor, RgbColor, RgbaColor,
    accessible_contrast_text, contrast_ratio, darken_hex, derive_palette_color, format_rgb_triplet,
    hex_to_hsv, hsv_to_hex, hsv_to_rgb, lighten_hex, mix_rgb, parse_color_to_rgb,
    parse_color_to_rgba, relative_luminance, rgb_to_hex, rgb_to_hsv,
};
pub use engine::{create_prism_theme, define_theme, resolve_defined_theme};
pub use types::{
    ComponentStyleOverride, DefinePrismThemeInput, DefinedPrismTheme, PaletteColorInput,
    PrismTheme, PrismThemeOptions, RefractionIntensity, ThemeBackground, ThemeCollectionCategory,
    ThemeControlsLayout, ThemeEditorColors, ThemeFxConfig, ThemeMaterial, ThemeMode,
    ThemeMotionConfig, ThemeOptics, ThemeOverlayFx, ThemePalette, ThemeShape, ThemeTextColors,
    ThemeTokenValue, ThemeTypography, TypographyScale,
};
