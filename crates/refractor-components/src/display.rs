//! Data Display & Content components (Category 4 of 55).

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_security::build_safe_csv;
use refractor_tokens::RgbColor;

/// Flagship 24px optical glass structural card slab.
#[derive(IntoElement)]
pub struct PrismCard {
    pub title: Option<SharedString>,
    pub kicker: Option<SharedString>,
    pub children: Vec<AnyElement>,
    pub base: BaseProps,
}

impl std::fmt::Debug for PrismCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrismCard")
            .field("title", &self.title)
            .field("kicker", &self.kicker)
            .field("children_count", &self.children.len())
            .field("base", &self.base)
            .finish()
    }
}

impl PrismCard {
    pub fn new() -> Self {
        Self {
            title: None,
            kicker: None,
            children: Vec::new(),
            base: BaseProps::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn kicker(mut self, kicker: impl Into<SharedString>) -> Self {
        self.kicker = Some(kicker.into());
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for PrismCard {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for PrismCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);

        let mut card = optical_surface(&theme, SurfaceVariant::Slab)
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(10.0));

        if let Some(kicker) = self.kicker {
            card = card.child(div().text_size(px(11.0)).text_color(accent).child(kicker));
        }
        if let Some(title) = self.title {
            card = card.child(div().text_size(px(18.0)).text_color(fg).child(title));
        }
        for child in self.children {
            card = card.child(child);
        }
        card
    }
}

/// Enterprise data table with OWASP formula-safe CSV export (ADR-0005).
#[derive(Debug, Clone, IntoElement)]
pub struct DataTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub base: BaseProps,
}

impl DataTable {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            headers,
            rows,
            base: BaseProps::new(),
        }
    }

    /// Exports table contents using OWASP formula-neutralized RFC 4180 CSV (ADR-0005).
    pub fn export_safe_csv(&self) -> String {
        build_safe_csv(&self.headers, &self.rows)
    }
}

impl RenderOnce for DataTable {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));

        let mut table = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(12.0))
            .flex()
            .flex_col()
            .gap(px(6.0));

        let mut header_row = div().flex().flex_row().gap(px(12.0)).pb(px(6.0));
        for h in self.headers {
            header_row = header_row.child(div().flex_1().text_size(px(12.0)).text_color(muted).child(h));
        }
        table = table.child(header_row);

        for row in self.rows {
            let mut r = optical_surface(&theme, SurfaceVariant::Well)
                .px(px(10.0))
                .py(px(8.0))
                .flex()
                .flex_row()
                .gap(px(12.0));
            for cell in row {
                r = r.child(div().flex_1().text_size(px(13.0)).text_color(fg).child(cell));
            }
            table = table.child(r);
        }
        table
    }
}

/// Semantic status tone for badges and alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusTone {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Error,
}

/// Optical status badge pill.
#[derive(Debug, Clone, IntoElement)]
pub struct BadgeStatus {
    pub label: SharedString,
    pub tone: StatusTone,
    pub base: BaseProps,
}

impl BadgeStatus {
    pub fn new(label: impl Into<SharedString>, tone: StatusTone) -> Self {
        Self {
            label: label.into(),
            tone,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for BadgeStatus {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        match self.tone {
            StatusTone::Primary => {
                let fg = color_str_to_hsla(
                    &theme.palette.primary.contrast_text,
                    RgbColor::new(255, 255, 255),
                );
                optical_surface(&theme, SurfaceVariant::PrimaryButton)
                    .px(px(10.0))
                    .py(px(4.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.label)
            }
            StatusTone::Success => {
                let fg = color_str_to_hsla(
                    &theme.palette.success.contrast_text,
                    RgbColor::new(255, 255, 255),
                );
                optical_surface(&theme, SurfaceVariant::Emerald)
                    .px(px(10.0))
                    .py(px(4.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.label)
            }
            StatusTone::Warning => {
                let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
                optical_surface(&theme, SurfaceVariant::Badge)
                    .bg(rgb_to_hsla(theme.palette.warning.rgb, 0.22))
                    .border_color(rgb_to_hsla(theme.palette.warning.rgb, 0.55))
                    .px(px(10.0))
                    .py(px(4.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.label)
            }
            StatusTone::Error => {
                let fg = color_str_to_hsla(
                    &theme.palette.error.contrast_text,
                    RgbColor::new(255, 255, 255),
                );
                optical_surface(&theme, SurfaceVariant::Badge)
                    .bg(rgb_to_hsla(theme.palette.error.rgb, 0.90))
                    .border_color(rgb_to_hsla(theme.palette.error.light_rgb, 0.65))
                    .px(px(10.0))
                    .py(px(4.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.label)
            }
            StatusTone::Neutral => {
                let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .px(px(10.0))
                    .py(px(4.0))
                    .rounded(px(999.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(fg)
                    .child(self.label)
            }
        }
    }
}

/// Compact optical tag chip.
#[derive(Debug, Clone, IntoElement)]
pub struct TagChip {
    pub label: SharedString,
    pub base: BaseProps,
}

impl TagChip {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for TagChip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Crystal)
            .px(px(10.0))
            .py(px(4.0))
            .rounded(px(999.0))
            .text_size(px(12.0))
            .text_color(fg)
            .child(self.label)
    }
}

/// Circular optical user avatar.
#[derive(Debug, Clone, IntoElement)]
pub struct Avatar {
    pub initials: SharedString,
    pub size_px: f32,
    pub base: BaseProps,
}

impl Avatar {
    pub fn new(initials: impl Into<SharedString>) -> Self {
        Self {
            initials: initials.into(),
            size_px: 40.0,
            base: BaseProps::new(),
        }
    }

    pub const fn size_px(mut self, size_px: f32) -> Self {
        self.size_px = size_px;
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(
            &theme.palette.primary.contrast_text,
            RgbColor::new(255, 255, 255),
        );
        optical_surface(&theme, SurfaceVariant::PrimaryButton)
            .w(px(self.size_px.max(24.0)))
            .h(px(self.size_px.max(24.0)))
            .rounded(px(999.0))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(13.0))
            .text_color(fg)
            .child(self.initials)
    }
}

/// Collapsible optical accordion section.
#[derive(Debug, Clone, IntoElement)]
pub struct Accordion {
    pub heading: SharedString,
    pub body: SharedString,
    pub expanded: bool,
    pub base: BaseProps,
}

impl Accordion {
    pub fn new(heading: impl Into<SharedString>, body: impl Into<SharedString>, expanded: bool) -> Self {
        Self {
            heading: heading.into(),
            body: body.into(),
            expanded,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let mut box_el = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .text_size(px(14.0))
                    .text_color(fg)
                    .child(self.heading)
                    .child(if self.expanded { "▾" } else { "▸" }),
            );
        if self.expanded {
            box_el = box_el.child(div().text_size(px(13.0)).text_color(muted).child(self.body));
        }
        box_el
    }
}

/// Executive KPI metric card with tabular numerals.
#[derive(Debug, Clone, IntoElement)]
pub struct KpiMetricCard {
    pub label: SharedString,
    pub value: SharedString,
    pub delta: Option<SharedString>,
    pub base: BaseProps,
}

impl KpiMetricCard {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            delta: None,
            base: BaseProps::new(),
        }
    }

    pub fn delta(mut self, delta: impl Into<SharedString>) -> Self {
        self.delta = Some(delta.into());
        self
    }
}

impl RenderOnce for KpiMetricCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let positive = color_str_to_hsla(&theme.palette.success.main, theme.palette.success.rgb);
        let mut card = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(16.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(div().text_size(px(12.0)).text_color(muted).child(self.label))
            .child(div().text_size(px(24.0)).text_color(fg).child(self.value));
        if let Some(d) = self.delta {
            card = card.child(div().text_size(px(12.0)).text_color(positive).child(d));
        }
        card
    }
}

/// Subtle refractive hairline divider.
#[derive(Debug, Clone, Default, IntoElement)]
pub struct OpticalDivider {
    pub base: BaseProps,
}

impl OpticalDivider {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RenderOnce for OpticalDivider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let line = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));
        div().w_full().h(px(1.0)).bg(line)
    }
}
