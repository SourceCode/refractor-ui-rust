//! Pure deterministic sRGB/HSV color math and WCAG 2.2 AA contrast derivation.

use std::sync::LazyLock;

use refractor_security::clamp_f64;
use serde::{Deserialize, Serialize};

/// Precomputed 256-entry sRGB-to-linear luminance lookup table (eliminates hot-path `powf(2.4)`).
static SRGB_LINEAR_LUT: LazyLock<[f64; 256]> = LazyLock::new(|| {
    let mut table = [0.0_f64; 256];
    let mut i = 0_usize;
    while i < 256 {
        let s = (i as f64) / 255.0;
        table[i] = if s <= 0.03928 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        };
        i += 1;
    }
    table
});

/// 8-bit sRGB color triplet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn with_alpha(self, a: f32) -> RgbaColor {
        RgbaColor {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }
}

/// 8-bit sRGB color with floating-point alpha in `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl RgbaColor {
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        let safe_a = if a.is_finite() {
            a.clamp(0.0, 1.0)
        } else {
            1.0
        };
        Self { r, g, b, a: safe_a }
    }

    pub const fn rgb(self) -> RgbColor {
        RgbColor {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

/// Hue (`0..=360`), Saturation (`0..=100`), Value (`0..=100`) color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HsvColor {
    pub h: u16,
    pub s: u8,
    pub v: u8,
}

/// Fully derived tonal palette entry with WCAG 2.2 AA foreground contrast text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedPaletteColor {
    pub main: String,
    pub light: String,
    pub dark: String,
    pub contrast_text: String,
    pub rgb: RgbColor,
    pub light_rgb: RgbColor,
    pub dark_rgb: RgbColor,
}

/// Default warm tangerine fallback matching Refractor Alabaster primary.
pub const DEFAULT_FALLBACK_RGB: RgbColor = RgbColor::new(231, 92, 36);

fn clamp_channel(n: f64) -> u8 {
    clamp_f64(n.round(), 0.0, 255.0, 0.0) as u8
}

/// Parses `#RGB`, `#RRGGBB`, `#RRGGBBAA`, `rgb(r, g, b)`, or `rgba(r, g, b, a)` into [`RgbColor`].
/// Falls back cleanly to `fallback` if input is malformed.
pub fn parse_color_to_rgb(input: &str, fallback: RgbColor) -> RgbColor {
    parse_color_to_rgba(input, fallback.with_alpha(1.0)).rgb()
}

/// Parses `#RGB`, `#RRGGBB`, `#RRGGBBAA`, `rgb(...)`, or `rgba(...)` into [`RgbaColor`].
pub fn parse_color_to_rgba(input: &str, fallback: RgbaColor) -> RgbaColor {
    let trimmed = input.trim();

    if let Some(hex) = trimmed.strip_prefix('#') {
        let bytes = hex.as_bytes();
        if bytes.len() == 3
            && let (Some(r), Some(g), Some(b)) = (
                parse_hex_nibble(bytes[0]),
                parse_hex_nibble(bytes[1]),
                parse_hex_nibble(bytes[2]),
            )
        {
            return RgbaColor::new(r * 17, g * 17, b * 17, 1.0);
        } else if (bytes.len() == 6 || bytes.len() == 8)
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            )
        {
            let a = if bytes.len() == 8 {
                u8::from_str_radix(&hex[6..8], 16).map_or(1.0, |v| f32::from(v) / 255.0)
            } else {
                1.0
            };
            return RgbaColor::new(r, g, b, a);
        }
    }

    let lower = trimmed.to_ascii_lowercase();
    let inner = lower
        .strip_prefix("rgba(")
        .or_else(|| lower.strip_prefix("rgb("))
        .and_then(|s| s.strip_suffix(')'));

    if let Some(body) = inner {
        let parts: Vec<&str> = body.split(',').map(str::trim).collect();
        if parts.len() >= 3
            && let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].parse::<f64>(),
                parts[1].parse::<f64>(),
                parts[2].parse::<f64>(),
            )
        {
            let a = parts
                .get(3)
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(1.0);
            return RgbaColor::new(clamp_channel(r), clamp_channel(g), clamp_channel(b), a);
        }
    }

    fallback
}

const fn parse_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Formats an [`RgbColor`] as uppercase `#RRGGBB`.
pub fn rgb_to_hex(rgb: RgbColor) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

/// Formats an [`RgbColor`] as `"R, G, B"` channel triplet.
pub fn format_rgb_triplet(rgb: RgbColor) -> String {
    format!("{}, {}, {}", rgb.r, rgb.g, rgb.b)
}

/// Linearly mixes two RGB colors (`amount` clamped to `0.0..=1.0`).
pub fn mix_rgb(base: RgbColor, target: RgbColor, amount: f64) -> RgbColor {
    let t = clamp_f64(amount, 0.0, 1.0, 0.0);
    let r = f64::from(base.r) + (f64::from(target.r) - f64::from(base.r)) * t;
    let g = f64::from(base.g) + (f64::from(target.g) - f64::from(base.g)) * t;
    let b = f64::from(base.b) + (f64::from(target.b) - f64::from(base.b)) * t;
    RgbColor::new(clamp_channel(r), clamp_channel(g), clamp_channel(b))
}

/// Lightens a hex color toward `#FFFFFF`.
pub fn lighten_hex(hex: &str, amount: f64) -> String {
    let base = parse_color_to_rgb(hex, DEFAULT_FALLBACK_RGB);
    rgb_to_hex(mix_rgb(base, RgbColor::new(255, 255, 255), amount))
}

/// Darkens a hex color toward obsidian slate `#0A0E16`.
pub fn darken_hex(hex: &str, amount: f64) -> String {
    let base = parse_color_to_rgb(hex, DEFAULT_FALLBACK_RGB);
    rgb_to_hex(mix_rgb(base, RgbColor::new(10, 14, 22), amount))
}

#[inline]
fn linearize_channel(channel: u8) -> f64 {
    SRGB_LINEAR_LUT[usize::from(channel)]
}

/// WCAG 2.2 relative luminance in `0.0..=1.0`.
pub fn relative_luminance(rgb: RgbColor) -> f64 {
    0.2126 * linearize_channel(rgb.r)
        + 0.7152 * linearize_channel(rgb.g)
        + 0.0722 * linearize_channel(rgb.b)
}

/// WCAG 2.2 contrast ratio in `1.0..=21.0`.
pub fn contrast_ratio(color_a: &str, color_b: &str) -> f64 {
    let lum_a = relative_luminance(parse_color_to_rgb(color_a, DEFAULT_FALLBACK_RGB));
    let lum_b = relative_luminance(parse_color_to_rgb(color_b, DEFAULT_FALLBACK_RGB));
    let lighter = lum_a.max(lum_b);
    let darker = lum_a.min(lum_b);
    (lighter + 0.05) / (darker + 0.05)
}

/// Selects high-contrast foreground text (`#FFFFFF` or `#151E2C`) maximizing WCAG readability.
pub fn accessible_contrast_text(background_hex: &str) -> String {
    let white_ratio = contrast_ratio(background_hex, "#FFFFFF");
    let ink_ratio = contrast_ratio(background_hex, "#151E2C");
    if white_ratio >= ink_ratio {
        "#FFFFFF".to_owned()
    } else {
        "#151E2C".to_owned()
    }
}

/// MUI-style automatic palette derivation from a single `main` color string,
/// with optional explicit overrides for `light`, `dark`, and `contrast_text`.
pub fn derive_palette_color(
    main: &str,
    light: Option<&str>,
    dark: Option<&str>,
    contrast_text: Option<&str>,
) -> DerivedPaletteColor {
    let main_rgb = parse_color_to_rgb(main, DEFAULT_FALLBACK_RGB);
    let normalized_main = rgb_to_hex(main_rgb);
    let light_hex = light.map_or_else(
        || lighten_hex(&normalized_main, 0.24),
        |l| rgb_to_hex(parse_color_to_rgb(l, main_rgb)),
    );
    let dark_hex = dark.map_or_else(
        || darken_hex(&normalized_main, 0.22),
        |d| rgb_to_hex(parse_color_to_rgb(d, main_rgb)),
    );
    let contrast = contrast_text
        .map_or_else(|| accessible_contrast_text(&normalized_main), str::to_owned);

    DerivedPaletteColor {
        main: normalized_main,
        light_rgb: parse_color_to_rgb(&light_hex, main_rgb),
        dark_rgb: parse_color_to_rgb(&dark_hex, main_rgb),
        light: light_hex,
        dark: dark_hex,
        contrast_text: contrast,
        rgb: main_rgb,
    }
}

/// Converts [`HsvColor`] to [`RgbColor`].
pub fn hsv_to_rgb(hsv: HsvColor) -> RgbColor {
    let hue = f64::from(hsv.h % 360);
    let sat = f64::from(hsv.s.min(100)) / 100.0;
    let val = f64::from(hsv.v.min(100)) / 100.0;

    let chroma = val * sat;
    let second = chroma * (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs());
    let offset = val - chroma;

    let (r_p, g_p, b_p) = if hue < 60.0 {
        (chroma, second, 0.0)
    } else if hue < 120.0 {
        (second, chroma, 0.0)
    } else if hue < 180.0 {
        (0.0, chroma, second)
    } else if hue < 240.0 {
        (0.0, second, chroma)
    } else if hue < 300.0 {
        (second, 0.0, chroma)
    } else {
        (chroma, 0.0, second)
    };

    RgbColor::new(
        clamp_channel((r_p + offset) * 255.0),
        clamp_channel((g_p + offset) * 255.0),
        clamp_channel((b_p + offset) * 255.0),
    )
}

/// Converts [`RgbColor`] to [`HsvColor`].
pub fn rgb_to_hsv(rgb: RgbColor) -> HsvColor {
    let red = f64::from(rgb.r) / 255.0;
    let green = f64::from(rgb.g) / 255.0;
    let blue = f64::from(rgb.b) / 255.0;
    let max = red.max(green).max(blue);
    let min = red.min(green).min(blue);
    let delta = max - min;

    let mut hue = if delta == 0.0 {
        0.0
    } else if (max - red).abs() < f64::EPSILON {
        60.0 * (((green - blue) / delta) % 6.0)
    } else if (max - green).abs() < f64::EPSILON {
        60.0 * ((blue - red) / delta + 2.0)
    } else {
        60.0 * ((red - green) / delta + 4.0)
    };
    if hue < 0.0 {
        hue += 360.0;
    }

    let sat = if max == 0.0 { 0.0 } else { (delta / max) * 100.0 };
    let val = max * 100.0;

    HsvColor {
        h: hue.round() as u16 % 360,
        s: sat.round().clamp(0.0, 100.0) as u8,
        v: val.round().clamp(0.0, 100.0) as u8,
    }
}

pub fn hsv_to_hex(hsv: HsvColor) -> String {
    rgb_to_hex(hsv_to_rgb(hsv))
}

pub fn hex_to_hsv(hex: &str) -> HsvColor {
    rgb_to_hsv(parse_color_to_rgb(hex, DEFAULT_FALLBACK_RGB))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_and_rgb_and_handles_invalid_inputs() {
        assert_eq!(parse_color_to_rgb("#E75C24", DEFAULT_FALLBACK_RGB), RgbColor::new(231, 92, 36));
        assert_eq!(parse_color_to_rgb("#0f8", DEFAULT_FALLBACK_RGB), RgbColor::new(0, 255, 136));
        assert_eq!(
            parse_color_to_rgb("rgba(10, 20, 30, 0.5)", DEFAULT_FALLBACK_RGB),
            RgbColor::new(10, 20, 30)
        );
        // Adversarial malformed color falls back deterministically
        assert_eq!(parse_color_to_rgb("not-a-color", RgbColor::new(1, 2, 3)), RgbColor::new(1, 2, 3));
    }

    #[test]
    fn wcag_contrast_and_derivation_meet_aa() {
        let ratio = contrast_ratio("#FFFFFF", "#000000");
        assert!((ratio - 21.0).abs() < 0.01);

        let dark_primary = derive_palette_color("#7C3AED", None, None, None);
        assert_eq!(dark_primary.contrast_text, "#FFFFFF");
        assert!(contrast_ratio(&dark_primary.main, &dark_primary.contrast_text) >= 4.5);

        let light_primary = derive_palette_color("#FDE047", None, None, None);
        assert_eq!(light_primary.contrast_text, "#151E2C");
        assert!(contrast_ratio(&light_primary.main, &light_primary.contrast_text) >= 4.5);
    }

    #[test]
    fn hsv_roundtrip_is_consistent() {
        for hue in [0_u16, 60, 120, 180, 210, 270, 330] {
            let hex = hsv_to_hex(HsvColor { h: hue, s: 80, v: 90 });
            let back = hex_to_hsv(&hex);
            assert!((i32::from(back.s) - 80).abs() <= 2);
            assert!((i32::from(back.v) - 90).abs() <= 2);
        }
        assert_eq!(rgb_to_hsv(RgbColor::new(0, 0, 0)), HsvColor { h: 0, s: 0, v: 0 });
        assert_eq!(parse_color_to_rgba("#11223380", RgbColor::new(0, 0, 0).with_alpha(1.0)).r, 0x11);
        assert!((RgbaColor::new(1, 2, 3, f32::NAN).a - 1.0).abs() < f32::EPSILON);
    }
}
