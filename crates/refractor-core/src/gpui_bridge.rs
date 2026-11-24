//! GPUI color conversion and reactive `ThemeGlobal` state bridge.

use gpui::{App, Global, Hsla, Rgba};
use refractor_tokens::{
    DEFAULT_FALLBACK_RGB, PrismTheme, RgbColor, RgbaColor, parse_color_to_rgba,
};

/// Global GPUI theme container reactive across all windows and components.
#[derive(Debug, Clone)]
pub struct ThemeGlobal {
    pub theme: PrismTheme,
}

impl Global for ThemeGlobal {}

impl ThemeGlobal {
    pub fn new(theme: PrismTheme) -> Self {
        Self { theme }
    }
}

/// Initializes [`ThemeGlobal`] on a GPUI [`App`] with the provided [`PrismTheme`].
pub fn init_theme(cx: &mut App, theme: PrismTheme) {
    cx.set_global(ThemeGlobal::new(theme));
}

/// Updates the active [`PrismTheme`] globally across the application.
pub fn set_theme(cx: &mut App, theme: PrismTheme) {
    cx.set_global(ThemeGlobal::new(theme));
}

/// Switches the active theme by one of the 67 shipped theme IDs (e.g. `"alabaster"`, `"obsidian"`, `"cyberpunk"`).
pub fn set_theme_by_id(cx: &mut App, id: &str) {
    let theme = refractor_themes::resolve_theme_by_id(id);
    set_theme(cx, theme);
}

/// Reads the active [`PrismTheme`] from GPUI [`App`], falling back to Alabaster Light if uninitialized.
pub fn active_theme(cx: &App) -> PrismTheme {
    cx.try_global::<ThemeGlobal>()
        .map_or_else(refractor_themes::alabaster_theme, |g| g.theme.clone())
}

/// Converts a pure [`RgbColor`] into a GPUI [`Hsla`].
pub fn rgb_to_hsla(rgb: RgbColor, alpha: f32) -> Hsla {
    rgba_to_hsla(rgb.with_alpha(alpha))
}

/// Converts a pure [`RgbaColor`] into a GPUI [`Hsla`].
pub fn rgba_to_hsla(rgba: RgbaColor) -> Hsla {
    let gpui_rgba = Rgba {
        r: f32::from(rgba.r) / 255.0,
        g: f32::from(rgba.g) / 255.0,
        b: f32::from(rgba.b) / 255.0,
        a: rgba.a.clamp(0.0, 1.0),
    };
    gpui_rgba.into()
}

/// Parses a `#RGB`, `#RRGGBB`, `rgb(...)`, or `rgba(...)` string directly into a GPUI [`Hsla`].
pub fn color_str_to_hsla(input: &str, fallback: RgbColor) -> Hsla {
    rgba_to_hsla(parse_color_to_rgba(input, fallback.with_alpha(1.0)))
}

/// Parses a hex or rgba color string into [`Hsla`] with Refractor's default fallback.
pub fn hex_to_hsla(input: &str) -> Hsla {
    color_str_to_hsla(input, DEFAULT_FALLBACK_RGB)
}
