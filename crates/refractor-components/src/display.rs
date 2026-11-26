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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));

        let mut card = optical_surface(&theme, SurfaceVariant::Slab)
            .w_full()
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(4.0));

        if let Some(kicker) = self.kicker {
            card = card.child(div().text_size(px(12.0)).text_color(muted).child(kicker));
        }
        if let Some(title) = self.title {
            card = card.child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(title),
            );
        }
        for child in self.children {
            card = card.child(
                div()
                    .mt(px(4.0))
                    .text_size(px(14.0))
                    .text_color(muted)
                    .child(child),
            );
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let dot_rgb = match self.tone {
            StatusTone::Success => RgbColor::new(16, 185, 129),
            StatusTone::Warning => RgbColor::new(245, 158, 11),
            StatusTone::Primary => RgbColor::new(249, 115, 22),
            StatusTone::Error => RgbColor::new(239, 68, 68),
            StatusTone::Neutral => RgbColor::new(148, 163, 184),
        };
        optical_surface(&theme, SurfaceVariant::Crystal)
            .rounded(px(9999.0))
            .px(px(14.0))
            .py(px(6.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg)
            .child(
                div()
                    .w(px(8.0))
                    .h(px(8.0))
                    .rounded(px(9999.0))
                    .bg(rgb_to_hsla(dot_rgb, 1.0)),
            )
            .child(self.label)
    }
}

/// Compact optical tag chip.
#[derive(Debug, Clone, IntoElement)]
pub struct TagChip {
    pub label: SharedString,
    pub selected: bool,
    pub base: BaseProps,
}

impl TagChip {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            selected: false,
            base: BaseProps::new(),
        }
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for TagChip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let (variant, fg) = if self.selected {
            (
                SurfaceVariant::Emerald,
                rgb_to_hsla(RgbColor::new(255, 255, 255), 1.0),
            )
        } else {
            (
                SurfaceVariant::Crystal,
                color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44)),
            )
        };
        optical_surface(&theme, variant)
            .h(px(32.0))
            .px(px(14.0))
            .rounded(px(9999.0))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
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
            size_px: 44.0,
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
            &theme.palette.info.contrast_text,
            RgbColor::new(255, 255, 255),
        );
        let dim = self.size_px.max(24.0);
        let font_px = if dim <= 38.0 {
            12.0
        } else if dim >= 52.0 {
            16.0
        } else {
            14.0
        };
        optical_surface(&theme, SurfaceVariant::Crystal)
            .w(px(dim))
            .h(px(dim))
            .rounded(px(9999.0))
            .p(px(2.0))
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .child(
                optical_surface(&theme, SurfaceVariant::CobaltButton)
                    .size_full()
                    .rounded(px(9999.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(font_px))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.initials),
            )
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let divider = color_str_to_hsla(&theme.palette.divider, RgbColor::new(226, 232, 240));
        let mut box_el = optical_surface(&theme, SurfaceVariant::Slab)
            .w_full()
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(
                div()
                    .w_full()
                    .px(px(20.0))
                    .py(px(14.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(16.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(fg)
                            .child(self.heading),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(muted)
                            .child(if self.expanded { "⌃" } else { "⌄" }),
                    ),
            );
        if self.expanded {
            box_el = box_el.child(
                div()
                    .w_full()
                    .px(px(20.0))
                    .pt(px(8.0))
                    .pb(px(16.0))
                    .border_t(px(1.0))
                    .border_color(divider)
                    .text_size(px(14.0))
                    .text_color(muted)
                    .child(self.body),
            );
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let delta_text = rgb_to_hsla(RgbColor::new(4, 120, 87), 1.0);
        let delta_fill = rgb_to_hsla(RgbColor::new(16, 185, 129), 0.15);
        let delta_border = rgb_to_hsla(RgbColor::new(16, 185, 129), 0.30);
        let mut card = optical_surface(&theme, SurfaceVariant::Slab)
            .w_full()
            .p(px(20.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(muted)
                    .child(self.label),
            )
            .child(
                div()
                    .text_size(px(26.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(fg)
                    .child(self.value),
            );
        if let Some(d) = self.delta {
            card = card.child(
                div().flex().flex_row().child(
                    div()
                        .px(px(10.0))
                        .py(px(3.0))
                        .rounded(px(9999.0))
                        .bg(delta_fill)
                        .border(px(1.0))
                        .border_color(delta_border)
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(delta_text)
                        .child(d),
                ),
            );
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
