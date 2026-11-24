//! Strongly typed algebraic design token definitions for Refractor UI for Rust.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::color_math::DerivedPaletteColor;

/// Light or dark color scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMode {
    #[default]
    Light,
    Dark,
}

/// Optical refraction intensity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RefractionIntensity {
    Subtle,
    #[default]
    Standard,
    Vivid,
}

/// All 13 Refractor multi-material surface archetypes ported from `glass-ui-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeMaterial {
    #[default]
    Glass,
    Mica,
    Bevel,
    Brutalist,
    Neon,
    Crt,
    Neumorphic,
    Clay,
    Paper,
    Flat,
    Chrome,
    Pixel,
    Resin,
}

impl ThemeMaterial {
    /// All 13 supported material archetypes.
    pub const ALL: [Self; 13] = [
        Self::Glass,
        Self::Mica,
        Self::Bevel,
        Self::Brutalist,
        Self::Neon,
        Self::Crt,
        Self::Neumorphic,
        Self::Clay,
        Self::Paper,
        Self::Flat,
        Self::Chrome,
        Self::Pixel,
        Self::Resin,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Glass => "glass",
            Self::Mica => "mica",
            Self::Bevel => "bevel",
            Self::Brutalist => "brutalist",
            Self::Neon => "neon",
            Self::Crt => "crt",
            Self::Neumorphic => "neumorphic",
            Self::Clay => "clay",
            Self::Paper => "paper",
            Self::Flat => "flat",
            Self::Chrome => "chrome",
            Self::Pixel => "pixel",
            Self::Resin => "resin",
        }
    }

    pub const fn is_sharp(self) -> bool {
        matches!(self, Self::Bevel | Self::Brutalist | Self::Crt | Self::Pixel)
    }

    pub const fn default_blur_px(self) -> f32 {
        match self {
            Self::Glass | Self::Mica | Self::Resin => 18.0,
            Self::Chrome => 14.0,
            Self::Neon => 4.0,
            _ => 0.0,
        }
    }
}

/// All 9 GPU-friendly decorative overlay FX layers ported from `glass-ui-v1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeOverlayFx {
    #[default]
    None,
    Noise,
    Scanlines,
    CyberGrid,
    RetroGrid,
    DotMatrix,
    PaperGrain,
    CrtRgb,
    HazardStripes,
}

impl ThemeOverlayFx {
    pub const ALL: [Self; 9] = [
        Self::None,
        Self::Noise,
        Self::Scanlines,
        Self::CyberGrid,
        Self::RetroGrid,
        Self::DotMatrix,
        Self::PaperGrain,
        Self::CrtRgb,
        Self::HazardStripes,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Noise => "noise",
            Self::Scanlines => "scanlines",
            Self::CyberGrid => "cyber-grid",
            Self::RetroGrid => "retro-grid",
            Self::DotMatrix => "dot-matrix",
            Self::PaperGrain => "paper-grain",
            Self::CrtRgb => "crt-rgb",
            Self::HazardStripes => "hazard-stripes",
        }
    }

    pub const fn default_opacity(self) -> f32 {
        match self {
            Self::None => 0.0,
            Self::Noise => 0.04,
            Self::PaperGrain => 0.06,
            Self::DotMatrix => 0.12,
            Self::HazardStripes => 0.14,
            Self::Scanlines | Self::CrtRgb => 0.16,
            Self::CyberGrid | Self::RetroGrid => 0.18,
        }
    }
}

/// Preferred OS window control alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeControlsLayout {
    Left,
    #[default]
    Right,
}

/// All 9 theme collection categories spanning the 67 shipped themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeCollectionCategory {
    #[default]
    OpticalGlass,
    ClassicOs,
    ModernOs,
    CyberNeon,
    RetroGaming,
    TactileMaterial,
    DeveloperIde,
    NatureElements,
    Seasons,
}

impl ThemeCollectionCategory {
    pub const ALL: [Self; 9] = [
        Self::OpticalGlass,
        Self::ClassicOs,
        Self::ModernOs,
        Self::CyberNeon,
        Self::RetroGaming,
        Self::TactileMaterial,
        Self::DeveloperIde,
        Self::NatureElements,
        Self::Seasons,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpticalGlass => "optical-glass",
            Self::ClassicOs => "classic-os",
            Self::ModernOs => "modern-os",
            Self::CyberNeon => "cyber-neon",
            Self::RetroGaming => "retro-gaming",
            Self::TactileMaterial => "tactile-material",
            Self::DeveloperIde => "developer-ide",
            Self::NatureElements => "nature-elements",
            Self::Seasons => "seasons",
        }
    }
}

/// Partial MUI-style color input; only `main` is required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaletteColorInput {
    pub main: String,
    pub light: Option<String>,
    pub dark: Option<String>,
    pub contrast_text: Option<String>,
}

impl PaletteColorInput {
    pub fn from_main(main: impl Into<String>) -> Self {
        Self {
            main: main.into(),
            light: None,
            dark: None,
            contrast_text: None,
        }
    }
}

/// Per-component typed style override (Astryx Tier 3 customization).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComponentStyleOverride {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub border_color: Option<String>,
    pub border_radius_px: Option<f32>,
    pub padding_x_px: Option<f32>,
    pub padding_y_px: Option<f32>,
    pub opacity: Option<f32>,
}

/// Light/dark pair or single token value for `define_theme`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeTokenValue {
    Single(String),
    ModePair { light: String, dark: String },
}

impl ThemeTokenValue {
    pub fn resolve(&self, mode: ThemeMode) -> &str {
        match self {
            Self::Single(val) => val,
            Self::ModePair { light, dark } => match mode {
                ThemeMode::Light => light,
                ThemeMode::Dark => dark,
            },
        }
    }
}

/// Desktop wallpaper, surface chrome, shadows, and material FX parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeFxConfig {
    pub wallpaper: String,
    pub wallpaper_overlay: ThemeOverlayFx,
    pub overlay_opacity: f32,
    pub glow_color: String,
    pub glow_intensity: f32,
    pub border_width_px: f32,
    pub window_bg: String,
    pub surface_bg: String,
    pub surface_highlight: String,
    pub window_border: String,
    pub chrome_bg: String,
    pub chrome_text: String,
    pub window_shadow: String,
    pub control_shadow: String,
    pub well_shadow: String,
    pub controls_layout: ThemeControlsLayout,
    pub title_bar_height_px: f32,
    pub adaptive_ink: bool,
    pub sharp_corners: bool,
}

/// Motion timing tokens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeMotionConfig {
    pub duration_fast_ms: u16,
    pub duration_normal_ms: u16,
    pub duration_slow_ms: u16,
    pub easing_standard: String,
    pub easing_emphasized: String,
}

/// Syntax-highlighted code editor palette.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeEditorColors {
    pub background: String,
    pub caret: String,
    pub comment: String,
    pub gutter: String,
    pub keyword: String,
    pub line: String,
    pub number: String,
    pub selection: String,
    pub selection_text: String,
    pub string: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeBackground {
    pub default: String,
    pub start: String,
    pub mid: String,
    pub end: String,
    pub paper: String,
    pub well: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemeTextColors {
    pub primary: String,
    pub secondary: String,
    pub disabled: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThemePalette {
    pub mode: ThemeMode,
    pub primary: DerivedPaletteColor,
    pub secondary: DerivedPaletteColor,
    pub info: DerivedPaletteColor,
    pub success: DerivedPaletteColor,
    pub warning: DerivedPaletteColor,
    pub error: DerivedPaletteColor,
    pub background: ThemeBackground,
    pub text: ThemeTextColors,
    pub divider: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypographyScale {
    pub font_size_px: f32,
    pub font_weight: u16,
    pub line_height: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeTypography {
    pub font_family: String,
    pub mono_family: String,
    pub letter_spacing: String,
    pub h1: TypographyScale,
    pub h2: TypographyScale,
    pub body1: TypographyScale,
    pub caption: TypographyScale,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ThemeShape {
    pub border_radius_px: f32,
    pub button_radius_px: f32,
    pub pill_radius_px: f32,
    pub slab_radius_px: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeOptics {
    pub refraction: RefractionIntensity,
    pub blur_px: f32,
    pub slab_blur_px: f32,
    pub caustic_opacity: f32,
    pub specular_opacity: f32,
    pub caustic_warm_rgb: String,
    pub caustic_cool_rgb: String,
    pub specular_highlight: bool,
}

/// Complete resolved Refractor Multi-Material & Optical Glass Theme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrismTheme {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: ThemeCollectionCategory,
    pub material: ThemeMaterial,
    pub palette: ThemePalette,
    pub typography: ThemeTypography,
    pub shape: ThemeShape,
    pub optics: ThemeOptics,
    pub fx: ThemeFxConfig,
    pub motion: ThemeMotionConfig,
    pub editor: ThemeEditorColors,
    pub components: BTreeMap<String, ComponentStyleOverride>,
    pub custom_tokens: BTreeMap<String, String>,
}

/// Partial ergonomic theme customization input for `create_prism_theme`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PrismThemeOptions {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub category: Option<ThemeCollectionCategory>,
    pub material: Option<ThemeMaterial>,
    pub mode: Option<ThemeMode>,
    pub primary: Option<PaletteColorInput>,
    pub secondary: Option<PaletteColorInput>,
    pub info: Option<PaletteColorInput>,
    pub success: Option<PaletteColorInput>,
    pub warning: Option<PaletteColorInput>,
    pub error: Option<PaletteColorInput>,
    pub background: Option<ThemeBackground>,
    pub text: Option<ThemeTextColors>,
    pub divider: Option<String>,
    pub font_family: Option<String>,
    pub mono_family: Option<String>,
    pub letter_spacing: Option<String>,
    pub border_radius_px: Option<f32>,
    pub slab_radius_px: Option<f32>,
    pub refraction: Option<RefractionIntensity>,
    pub blur_px: Option<f32>,
    pub specular_highlight: Option<bool>,
    pub fx: Option<ThemeFxConfig>,
    pub editor: Option<ThemeEditorColors>,
    pub components: BTreeMap<String, ComponentStyleOverride>,
    pub custom_tokens: BTreeMap<String, String>,
}

/// Input to Astryx-style `define_theme`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DefinePrismThemeInput {
    pub name: String,
    pub extends: Option<Box<DefinedPrismTheme>>,
    pub options: PrismThemeOptions,
    pub tokens: BTreeMap<String, ThemeTokenValue>,
    pub components: BTreeMap<String, ComponentStyleOverride>,
}

/// Serializable cascade theme definition supporting inheritance and mode-aware tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefinedPrismTheme {
    pub name: String,
    pub options: PrismThemeOptions,
    pub tokens: BTreeMap<String, ThemeTokenValue>,
    pub components: BTreeMap<String, ComponentStyleOverride>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_materials_overlays_and_categories_format_cleanly() {
        for mat in ThemeMaterial::ALL {
            assert_ne!(mat.as_str(), "");
            let _ = mat.is_sharp();
            let _ = mat.default_blur_px();
        }
        for ov in ThemeOverlayFx::ALL {
            assert_ne!(ov.as_str(), "");
            assert!(ov.default_opacity() >= 0.0);
        }
        for cat in ThemeCollectionCategory::ALL {
            assert_ne!(cat.as_str(), "");
        }
        let single = ThemeTokenValue::Single("#123456".to_owned());
        assert_eq!(single.resolve(ThemeMode::Light), "#123456");
        assert_eq!(single.resolve(ThemeMode::Dark), "#123456");
    }
}
