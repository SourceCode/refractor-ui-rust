//! Complete 68-theme multi-material & optical glass catalog ported from `glass-ui-v1`:
//! - 5 Optical Glass Presets (`alabaster`, `obsidian`, `amethyst`, `emerald-tide`, `solar-prism`)
//! - 63 WebOS-v3 Full-Transformation Themes across 9 categories, 13 material archetypes, and 9 FX overlays.

use std::sync::OnceLock;

use refractor_tokens::{
    DefinePrismThemeInput, DefinedPrismTheme, PaletteColorInput, PrismTheme, PrismThemeOptions,
    RefractionIntensity, ThemeBackground, ThemeCollectionCategory, ThemeControlsLayout,
    ThemeEditorColors, ThemeFxConfig, ThemeMaterial, ThemeMode, ThemeOverlayFx, ThemeTextColors,
    define_theme, resolve_defined_theme,
};
use serde::Deserialize;

const WEBOS_V3_JSON: &str = include_str!("webos_v3_themes.json");

/// Metadata for a theme collection category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeCategoryMeta {
    pub id: ThemeCollectionCategory,
    pub label: &'static str,
    pub short_label: &'static str,
    pub description: &'static str,
}

/// Complete catalog entry for one of the 68 shipped themes.
#[derive(Debug, Clone)]
pub struct ThemeCatalogEntry {
    pub id: String,
    pub source_id: String,
    pub name: String,
    pub description: String,
    pub mode: ThemeMode,
    pub category: ThemeCollectionCategory,
    pub material: ThemeMaterial,
    pub wallpaper_overlay: ThemeOverlayFx,
    pub font_family: String,
    pub swatches: [String; 4],
    pub wallpaper: String,
    pub defined: DefinedPrismTheme,
}

impl ThemeCatalogEntry {
    /// Resolves this theme into a ready-to-render [`PrismTheme`].
    pub fn resolve(&self) -> PrismTheme {
        resolve_defined_theme(&self.defined, None)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawWebOsTheme {
    id: String,
    source_id: String,
    name: String,
    description: String,
    mode: ThemeMode,
    category: ThemeCollectionCategory,
    material: ThemeMaterial,
    wallpaper_overlay: ThemeOverlayFx,
    overlay_opacity: Option<f32>,
    sharp_corners: Option<bool>,
    letter_spacing: Option<String>,
    font_family: String,
    mono_family: Option<String>,
    colors: RawWebOsColors,
    window: RawWebOsWindow,
    editor: RawWebOsEditor,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawWebOsColors {
    primary: String,
    primary_text: String,
    secondary: String,
    accent: String,
    info: Option<String>,
    success: String,
    warning: String,
    error: String,
    text: String,
    text_muted: String,
    border: String,
    window_bg: String,
    window_border: String,
    surface: String,
    surface_highlight: String,
    chrome_bg: String,
    chrome_text: String,
    wallpaper: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawWebOsWindow {
    radius_px: f32,
    border_width_px: f32,
    backdrop_blur_px: f32,
    shadow: String,
    controls_layout: ThemeControlsLayout,
    title_bar_height_px: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawWebOsEditor {
    background: String,
    caret: String,
    comment: String,
    gutter: String,
    keyword: String,
    line: String,
    number: String,
    selection: String,
    selection_text: String,
    string: String,
    text: String,
}

/// Returns metadata for all 9 theme categories.
pub fn theme_categories() -> &'static [ThemeCategoryMeta] {
    &[
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::OpticalGlass,
            label: "Optical Glass",
            short_label: "Optical",
            description: "Signature refractive prismatic glass presets.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::ClassicOs,
            label: "Classic OS",
            short_label: "Classic OS",
            description: "Retro workstation and classic desktop metaphors.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::ModernOs,
            label: "Modern OS",
            short_label: "Modern OS",
            description: "Contemporary desktop, mica, and frosted OS themes.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::CyberNeon,
            label: "Cyber & Neon",
            short_label: "Cyber",
            description: "High-contrast luminous cyberpunk, synthwave, and CRT themes.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::RetroGaming,
            label: "Retro Gaming",
            short_label: "Gaming",
            description: "Iconic console and handheld palettes.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::TactileMaterial,
            label: "Tactile & Material",
            short_label: "Tactile",
            description: "Clay, paper, brutalist, chrome, neumorphic, and resin surfaces.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::DeveloperIde,
            label: "Developer IDE",
            short_label: "IDE",
            description: "Beloved code editor themes tuned for long sessions.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::NatureElements,
            label: "Nature & Elements",
            short_label: "Elements",
            description: "Elemental palettes inspired by air, earth, fire, water, and ether.",
        },
        ThemeCategoryMeta {
            id: ThemeCollectionCategory::Seasons,
            label: "Four Seasons",
            short_label: "Seasons",
            description: "Seasonal color atmospheres across spring, summer, autumn, and winter.",
        },
    ]
}

fn build_optical_preset(
    id: &str,
    name: &str,
    description: &str,
    mode: ThemeMode,
    primary: (&str, &str, &str, &str),
    secondary: (&str, &str, &str, &str),
    info: (&str, &str, &str, &str),
    success: (&str, &str, &str, &str),
    refraction: RefractionIntensity,
    blur_px: f32,
) -> ThemeCatalogEntry {
    let opts = PrismThemeOptions {
        name: Some(id.to_owned()),
        display_name: Some(name.to_owned()),
        description: Some(description.to_owned()),
        category: Some(ThemeCollectionCategory::OpticalGlass),
        material: Some(ThemeMaterial::Glass),
        mode: Some(mode),
        primary: Some(PaletteColorInput {
            main: primary.0.to_owned(),
            light: Some(primary.1.to_owned()),
            dark: Some(primary.2.to_owned()),
            contrast_text: Some(primary.3.to_owned()),
        }),
        secondary: Some(PaletteColorInput {
            main: secondary.0.to_owned(),
            light: Some(secondary.1.to_owned()),
            dark: Some(secondary.2.to_owned()),
            contrast_text: Some(secondary.3.to_owned()),
        }),
        info: Some(PaletteColorInput {
            main: info.0.to_owned(),
            light: Some(info.1.to_owned()),
            dark: Some(info.2.to_owned()),
            contrast_text: Some(info.3.to_owned()),
        }),
        success: Some(PaletteColorInput {
            main: success.0.to_owned(),
            light: Some(success.1.to_owned()),
            dark: Some(success.2.to_owned()),
            contrast_text: Some(success.3.to_owned()),
        }),
        refraction: Some(refraction),
        blur_px: Some(blur_px),
        specular_highlight: Some(true),
        ..PrismThemeOptions::default()
    };

    let wallpaper = if mode == ThemeMode::Dark {
        "#0B1120".to_owned()
    } else {
        "#E4EAF3".to_owned()
    };

    let defined = define_theme(DefinePrismThemeInput {
        name: id.to_owned(),
        options: opts,
        ..DefinePrismThemeInput::default()
    });

    ThemeCatalogEntry {
        id: id.to_owned(),
        source_id: id.to_owned(),
        name: name.to_owned(),
        description: description.to_owned(),
        mode,
        category: ThemeCollectionCategory::OpticalGlass,
        material: ThemeMaterial::Glass,
        wallpaper_overlay: ThemeOverlayFx::None,
        font_family: "Inter, system-ui, sans-serif".to_owned(),
        swatches: [
            primary.0.to_owned(),
            secondary.0.to_owned(),
            success.0.to_owned(),
            wallpaper.clone(),
        ],
        wallpaper,
        defined,
    }
}

fn build_webos_entry(raw: RawWebOsTheme) -> ThemeCatalogEntry {
    let overlay_opacity = raw
        .overlay_opacity
        .unwrap_or_else(|| raw.wallpaper_overlay.default_opacity());
    let sharp = raw.sharp_corners.unwrap_or_else(|| raw.material.is_sharp());

    let fx = ThemeFxConfig {
        wallpaper: raw.colors.wallpaper.clone(),
        wallpaper_overlay: raw.wallpaper_overlay,
        overlay_opacity,
        glow_color: raw.colors.accent.clone(),
        glow_intensity: if matches!(raw.material, ThemeMaterial::Neon | ThemeMaterial::Crt) {
            1.15
        } else {
            0.35
        },
        border_width_px: raw.window.border_width_px,
        window_bg: raw.colors.window_bg.clone(),
        surface_bg: raw.colors.surface.clone(),
        surface_highlight: raw.colors.surface_highlight.clone(),
        window_border: raw.colors.window_border.clone(),
        chrome_bg: raw.colors.chrome_bg.clone(),
        chrome_text: raw.colors.chrome_text.clone(),
        window_shadow: raw.window.shadow.clone(),
        control_shadow: raw.window.shadow.clone(),
        well_shadow: "inset 0 2px 4px rgba(0, 0, 0, 0.14)".to_owned(),
        controls_layout: raw.window.controls_layout,
        title_bar_height_px: raw.window.title_bar_height_px,
        adaptive_ink: raw.material != ThemeMaterial::Glass,
        sharp_corners: sharp,
    };

    let editor = ThemeEditorColors {
        background: raw.editor.background,
        caret: raw.editor.caret,
        comment: raw.editor.comment,
        gutter: raw.editor.gutter,
        keyword: raw.editor.keyword,
        line: raw.editor.line,
        number: raw.editor.number,
        selection: raw.editor.selection,
        selection_text: raw.editor.selection_text,
        string: raw.editor.string,
        text: raw.editor.text,
    };

    let swatches = [
        raw.colors.primary.clone(),
        raw.colors.secondary.clone(),
        raw.colors.accent.clone(),
        raw.colors.window_bg.clone(),
    ];

    let opts = PrismThemeOptions {
        name: Some(raw.id.clone()),
        display_name: Some(raw.name.clone()),
        description: Some(raw.description.clone()),
        category: Some(raw.category),
        material: Some(raw.material),
        mode: Some(raw.mode),
        primary: Some(PaletteColorInput {
            main: raw.colors.primary,
            light: None,
            dark: None,
            contrast_text: Some(raw.colors.primary_text),
        }),
        secondary: Some(PaletteColorInput::from_main(raw.colors.secondary)),
        info: Some(PaletteColorInput::from_main(
            raw.colors.info.unwrap_or(raw.colors.accent),
        )),
        success: Some(PaletteColorInput::from_main(raw.colors.success)),
        warning: Some(PaletteColorInput::from_main(raw.colors.warning)),
        error: Some(PaletteColorInput::from_main(raw.colors.error)),
        background: Some(ThemeBackground {
            default: raw.colors.wallpaper.clone(),
            start: raw.colors.window_bg.clone(),
            mid: raw.colors.surface.clone(),
            end: raw.colors.chrome_bg,
            paper: raw.colors.surface,
            well: raw.colors.surface_highlight,
        }),
        text: Some(ThemeTextColors {
            primary: raw.colors.text,
            secondary: raw.colors.text_muted.clone(),
            disabled: raw.colors.text_muted,
        }),
        divider: Some(raw.colors.border),
        font_family: Some(raw.font_family.clone()),
        mono_family: raw.mono_family,
        letter_spacing: raw.letter_spacing,
        border_radius_px: Some(raw.window.radius_px),
        slab_radius_px: Some(if sharp {
            raw.window.radius_px
        } else {
            (raw.window.radius_px + 8.0).min(32.0)
        }),
        blur_px: Some(raw.window.backdrop_blur_px),
        fx: Some(fx),
        editor: Some(editor),
        ..PrismThemeOptions::default()
    };

    let defined = define_theme(DefinePrismThemeInput {
        name: raw.id.clone(),
        options: opts,
        ..DefinePrismThemeInput::default()
    });

    ThemeCatalogEntry {
        id: raw.id,
        source_id: raw.source_id,
        name: raw.name,
        description: raw.description,
        mode: raw.mode,
        category: raw.category,
        material: raw.material,
        wallpaper_overlay: raw.wallpaper_overlay,
        font_family: raw.font_family,
        swatches,
        wallpaper: raw.colors.wallpaper,
        defined,
    }
}

fn build_catalog() -> Vec<ThemeCatalogEntry> {
    let mut entries = vec![
        build_optical_preset(
            "alabaster",
            "Alabaster Light",
            "Signature warm-tangerine & ice-crystal optical glass light theme.",
            ThemeMode::Light,
            ("#E75C24", "#F59C28", "#C94114", "#FFFFFF"),
            ("#38BDF8", "#BAE6FD", "#0284C7", "#151E2C"),
            ("#1D4ED8", "#0EA5E9", "#1E3A8A", "#FFFFFF"),
            ("#26BC92", "#5CE1B8", "#159673", "#FFFFFF"),
            RefractionIntensity::Standard,
            18.0,
        ),
        build_optical_preset(
            "obsidian",
            "Obsidian Dark",
            "High-contrast dark optical glass workbench with luminous caustics.",
            ThemeMode::Dark,
            ("#F06429", "#FBA93B", "#C84312", "#FFFFFF"),
            ("#38BDF8", "#7DD3FC", "#0369A1", "#F1F5F9"),
            ("#2563EB", "#38BDF8", "#1E3A8A", "#FFFFFF"),
            ("#22C58E", "#5EEAD4", "#0F766E", "#FFFFFF"),
            RefractionIntensity::Standard,
            20.0,
        ),
        build_optical_preset(
            "amethyst",
            "Amethyst Prism",
            "Violet & cyan vivid optical refraction preset.",
            ThemeMode::Light,
            ("#7C3AED", "#A78BFA", "#5B21B6", "#FFFFFF"),
            ("#06B6D4", "#A5F3FC", "#0E7490", "#151E2C"),
            ("#4F46E5", "#818CF8", "#312E81", "#FFFFFF"),
            ("#10B981", "#6EE7B7", "#047857", "#FFFFFF"),
            RefractionIntensity::Vivid,
            20.0,
        ),
        build_optical_preset(
            "emerald-tide",
            "Emerald Tide",
            "Jade & ocean teal refractive glass preset.",
            ThemeMode::Light,
            ("#0D9488", "#2DD4BF", "#115E59", "#FFFFFF"),
            ("#0EA5E9", "#BAE6FD", "#0369A1", "#151E2C"),
            ("#0284C7", "#38BDF8", "#0C4A6E", "#FFFFFF"),
            ("#16A34A", "#4ADE80", "#166534", "#FFFFFF"),
            RefractionIntensity::Standard,
            18.0,
        ),
        build_optical_preset(
            "solar-prism",
            "Solar Prism",
            "Amber gold & cobalt vivid refraction preset.",
            ThemeMode::Light,
            ("#D97706", "#FBBF24", "#92400E", "#FFFFFF"),
            ("#2563EB", "#93C5FD", "#1E40AF", "#151E2C"),
            ("#1D4ED8", "#60A5FA", "#1E3A8A", "#FFFFFF"),
            ("#059669", "#34D399", "#065F46", "#FFFFFF"),
            RefractionIntensity::Vivid,
            20.0,
        ),
    ];

    if let Ok(raw_list) = serde_json::from_str::<Vec<RawWebOsTheme>>(WEBOS_V3_JSON) {
        entries.extend(raw_list.into_iter().map(build_webos_entry));
    }

    entries
}

static CATALOG: OnceLock<Vec<ThemeCatalogEntry>> = OnceLock::new();

/// Returns all 68 shipped Refractor themes.
pub fn all_themes() -> &'static [ThemeCatalogEntry] {
    CATALOG.get_or_init(build_catalog).as_slice()
}

/// Looks up a theme catalog entry by ID (case-insensitive).
pub fn find_theme(id: &str) -> Option<&'static ThemeCatalogEntry> {
    let needle = id.trim();
    all_themes()
        .iter()
        .find(|t| t.id.eq_ignore_ascii_case(needle))
}

/// Resolves a theme by ID, falling back to `alabaster` if unknown.
pub fn resolve_theme_by_id(id: &str) -> PrismTheme {
    find_theme(id)
        .or_else(|| all_themes().first())
        .map_or_else(
            || refractor_tokens::create_prism_theme(PrismThemeOptions::default()),
            ThemeCatalogEntry::resolve,
        )
}

/// Returns all themes matching a given [`ThemeCollectionCategory`].
pub fn themes_by_category(category: ThemeCollectionCategory) -> Vec<&'static ThemeCatalogEntry> {
    all_themes()
        .iter()
        .filter(|t| t.category == category)
        .collect()
}

/// Returns all themes matching a given [`ThemeMaterial`].
pub fn themes_by_material(material: ThemeMaterial) -> Vec<&'static ThemeCatalogEntry> {
    all_themes()
        .iter()
        .filter(|t| t.material == material)
        .collect()
}

/// Convenience constructors for the 5 flagship Optical Glass themes.
pub fn alabaster_theme() -> PrismTheme {
    resolve_theme_by_id("alabaster")
}

pub fn obsidian_theme() -> PrismTheme {
    resolve_theme_by_id("obsidian")
}

pub fn amethyst_theme() -> PrismTheme {
    resolve_theme_by_id("amethyst")
}

pub fn emerald_tide_theme() -> PrismTheme {
    resolve_theme_by_id("emerald-tide")
}

pub fn solar_prism_theme() -> PrismTheme {
    resolve_theme_by_id("solar-prism")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_all_68_themes() {
        let themes = all_themes();
        assert_eq!(themes.len(), 68, "expected 5 optical glass + 63 WebOS-v3 themes");
    }

    #[test]
    fn every_theme_resolves_cleanly() {
        for entry in all_themes() {
            let resolved = entry.resolve();
            assert_eq!(resolved.name, entry.id);
            assert_ne!(resolved.palette.primary.main, "");
            assert_ne!(resolved.palette.primary.contrast_text, "");
        }
    }

    #[test]
    fn lookup_and_fallback_work() {
        assert!(find_theme("alabaster").is_some());
        assert!(find_theme("cyberpunk").is_some());
        assert!(find_theme("windows95").is_some());
        assert!(find_theme("tokyo-night").is_some());
        assert!(find_theme("nonexistent-theme").is_none());

        let fallback = resolve_theme_by_id("nonexistent-theme");
        assert_eq!(fallback.name, "alabaster");
    }

    #[test]
    fn all_nine_categories_are_populated() {
        assert_eq!(theme_categories().len(), 9);
        for cat in ThemeCollectionCategory::ALL {
            assert!(
                !themes_by_category(cat).is_empty(),
                "category {cat:?} should have themes"
            );
        }
        assert!(!themes_by_material(ThemeMaterial::Glass).is_empty());
        assert_eq!(alabaster_theme().name, "alabaster");
        assert_eq!(obsidian_theme().name, "obsidian");
        assert_eq!(amethyst_theme().name, "amethyst");
        assert_eq!(emerald_tide_theme().name, "emerald-tide");
        assert_eq!(solar_prism_theme().name, "solar-prism");
    }
}
