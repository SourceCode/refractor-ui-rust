//! Form Inputs & Selection Controls (Category 2 of 55).

use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
    relative,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_security::clamp_f32;
use refractor_tokens::{RgbColor, ThemeMode};

/// Recessed optical glass single-line text input well.
#[derive(Debug, Clone, IntoElement)]
pub struct TextInput {
    pub label: Option<SharedString>,
    pub value: SharedString,
    pub placeholder: SharedString,
    pub base: BaseProps,
}

impl TextInput {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            label: None,
            value: value.into(),
            placeholder: SharedString::from(""),
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl RenderOnce for TextInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let text_muted =
            color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(75, 88, 108));
        let is_empty = self.value.is_empty();
        let shown = if is_empty { self.placeholder } else { self.value };

        let mut wrapper = div().w_full().flex().flex_col();
        if let Some(lbl) = self.label {
            wrapper = wrapper.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .mb(px(6.0))
                    .ml(px(4.0))
                    .child(lbl),
            );
        }
        wrapper.child(
            optical_surface(&theme, SurfaceVariant::Crystal)
                .w_full()
                .rounded(px(9999.0))
                .p(px(4.0))
                .child(
                    optical_surface(&theme, SurfaceVariant::Well)
                        .w_full()
                        .rounded(px(9999.0))
                        .h(px(44.0))
                        .px(px(16.0))
                        .flex()
                        .items_center()
                        .text_size(px(14.0))
                        .text_color(if is_empty { text_muted } else { text_primary })
                        .child(shown),
                ),
        )
    }
}

/// Multi-line optical glass recessed text area.
#[derive(Debug, Clone, IntoElement)]
pub struct TextArea {
    pub label: Option<SharedString>,
    pub value: SharedString,
    pub rows: u16,
    pub base: BaseProps,
}

impl TextArea {
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            label: None,
            value: value.into(),
            rows: 4,
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub const fn rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }
}

impl RenderOnce for TextArea {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let min_h = (f32::from(self.rows.max(2)) * 22.0 + 48.0).max(104.0);
        let mut wrapper = div().w_full().flex().flex_col();
        if let Some(lbl) = self.label {
            wrapper = wrapper.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .mb(px(6.0))
                    .px(px(4.0))
                    .child(lbl),
            );
        }
        wrapper.child(
            optical_surface(&theme, SurfaceVariant::Crystal)
                .w_full()
                .rounded(px(22.0))
                .p(px(6.0))
                .child(
                    optical_surface(&theme, SurfaceVariant::Well)
                        .w_full()
                        .rounded(px(17.0))
                        .min_h(px(min_h))
                        .px(px(16.0))
                        .py(px(12.0))
                        .text_size(px(14.0))
                        .text_color(text_primary)
                        .child(self.value),
                ),
        )
    }
}

/// Tactile emerald optical toggle switch with 3D porcelain thumb (ADR-0006).
#[derive(Debug, Clone, IntoElement)]
pub struct ToggleSwitch {
    pub checked: bool,
    pub label: Option<SharedString>,
    pub base: BaseProps,
}

impl ToggleSwitch {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            label: None,
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for ToggleSwitch {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let track_variant = if self.checked {
            SurfaceVariant::Emerald
        } else {
            SurfaceVariant::Crystal
        };

        let thumb = optical_surface(&theme, SurfaceVariant::Thumb)
            .w(px(34.0))
            .h(px(34.0))
            .rounded(px(9999.0));

        let mut track = optical_surface(&theme, track_variant)
            .w(px(78.0))
            .h(px(42.0))
            .rounded(px(9999.0))
            .p(px(4.0))
            .flex()
            .flex_row()
            .items_center();

        track = if self.checked {
            track.justify_end()
        } else {
            track.justify_start()
        };

        let mut row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(track.child(thumb));

        if let Some(lbl) = self.label {
            row = row.child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.primary,
                        RgbColor::new(24, 32, 44),
                    ))
                    .child(lbl),
            );
        }
        row
    }
}

/// Accessible optical glass checkbox control.
#[derive(Debug, Clone, IntoElement)]
pub struct Checkbox {
    pub checked: bool,
    pub label: SharedString,
    pub base: BaseProps,
}

impl Checkbox {
    pub fn new(checked: bool, label: impl Into<SharedString>) -> Self {
        Self {
            checked,
            label: label.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let box_variant = if self.checked {
            SurfaceVariant::Emerald
        } else {
            SurfaceVariant::Crystal
        };
        let check_glyph = if self.checked { "✓" } else { "" };
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(
                optical_surface(&theme, box_variant)
                    .w(px(28.0))
                    .h(px(28.0))
                    .rounded(px(9.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(color_str_to_hsla(
                        &theme.palette.success.contrast_text,
                        RgbColor::new(255, 255, 255),
                    ))
                    .child(check_glyph),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.primary,
                        RgbColor::new(24, 32, 44),
                    ))
                    .child(self.label),
            )
    }
}

/// Optical glass radio group.
#[derive(Debug, Clone, IntoElement)]
pub struct RadioGroup {
    pub options: Vec<SharedString>,
    pub selected: usize,
    pub base: BaseProps,
}

impl RadioGroup {
    pub fn new(options: Vec<SharedString>, selected: usize) -> Self {
        Self {
            options,
            selected,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let mut col = div().flex().flex_col().gap(px(10.0));
        for (idx, opt) in self.options.into_iter().enumerate() {
            let active = idx == self.selected;
            let mut ring = optical_surface(&theme, SurfaceVariant::Crystal)
                .w(px(24.0))
                .h(px(24.0))
                .rounded(px(9999.0))
                .flex()
                .items_center()
                .justify_center();
            if active {
                ring = ring.child(
                    optical_surface(&theme, SurfaceVariant::Emerald)
                        .w(px(14.0))
                        .h(px(14.0))
                        .rounded(px(9999.0)),
                );
            }

            col = col.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .p(px(10.0))
                    .child(ring)
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_primary)
                            .child(opt),
                    ),
            );
        }
        col
    }
}

/// NaN-safe optical range slider with illuminated progress track.
#[derive(Debug, Clone, IntoElement)]
pub struct RangeSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub label: Option<SharedString>,
    pub base: BaseProps,
}

impl RangeSlider {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value,
            min,
            max,
            label: None,
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn normalized_fraction(&self) -> f32 {
        let safe_min = if self.min.is_finite() { self.min } else { 0.0 };
        let safe_max = if self.max.is_finite() { self.max } else { 100.0 };
        let span = (safe_max - safe_min).abs();
        if span <= f32::EPSILON {
            return 0.0;
        }
        let clamped = clamp_f32(self.value, safe_min, safe_max, safe_min);
        ((clamped - safe_min.min(safe_max)) / span).clamp(0.0, 1.0)
    }
}

impl RenderOnce for RangeSlider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let pct = self.normalized_fraction();
        let safe_val = clamp_f32(self.value, self.min, self.max, self.min);
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(51, 65, 85));

        let mut col = div().w_full().flex().flex_col();
        if let Some(lbl) = self.label {
            col = col.child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .mb(px(8.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(text_primary)
                            .child(lbl),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(text_primary)
                            .child(format!("{}%", safe_val.round() as i32)),
                    ),
            );
        }

        let track = div()
            .relative()
            .w_full()
            .h(px(32.0))
            .flex()
            .items_center()
            .child(
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .w_full()
                    .h(px(14.0))
                    .rounded(px(9999.0))
                    .p(px(2.0))
                    .overflow_hidden()
                    .child(
                        optical_surface(&theme, SurfaceVariant::Emerald)
                            .border_0()
                            .w(relative(pct))
                            .h_full()
                            .rounded(px(9999.0)),
                    ),
            )
            .child(
                optical_surface(&theme, SurfaceVariant::Thumb)
                    .absolute()
                    .left(relative(pct))
                    .ml(px(-12.0))
                    .top(px(4.0))
                    .w(px(24.0))
                    .h(px(24.0))
                    .rounded(px(9999.0)),
            );

        col.child(track)
    }
}

/// Optical glass dropdown select trigger.
#[derive(Debug, Clone, IntoElement)]
pub struct DropdownSelect {
    pub label: Option<SharedString>,
    pub selected_label: SharedString,
    pub base: BaseProps,
}

impl DropdownSelect {
    pub fn new(selected_label: impl Into<SharedString>) -> Self {
        Self {
            label: None,
            selected_label: selected_label.into(),
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for DropdownSelect {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let is_dark = theme.palette.mode == ThemeMode::Dark;
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let tray_bg = if is_dark {
            rgb_to_hsla(RgbColor::new(255, 255, 255), 0.04)
        } else {
            rgb_to_hsla(RgbColor::new(15, 23, 42), 0.03)
        };
        let tray_border = if is_dark {
            rgb_to_hsla(RgbColor::new(255, 255, 255), 0.10)
        } else {
            rgb_to_hsla(RgbColor::new(255, 255, 255), 0.50)
        };

        let mut wrapper = div().w_full().flex().flex_col();
        if let Some(lbl) = self.label {
            wrapper = wrapper.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .mb(px(6.0))
                    .ml(px(4.0))
                    .child(lbl),
            );
        }

        wrapper.child(
            optical_surface(&theme, SurfaceVariant::Crystal)
                .w_full()
                .h(px(56.0))
                .rounded(px(9999.0))
                .pl(px(20.0))
                .pr(px(10.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(px(12.0))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(12.0))
                        .child(
                            div()
                                .text_size(px(18.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(fg)
                                .child("C"),
                        )
                        .child(
                            div()
                                .text_size(px(15.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(fg)
                                .child(self.selected_label),
                        ),
                )
                .child(
                    div()
                        .h(px(40.0))
                        .pl(px(20.0))
                        .pr(px(6.0))
                        .rounded(px(9999.0))
                        .bg(tray_bg)
                        .border(px(1.0))
                        .border_color(tray_border)
                        .flex()
                        .items_center()
                        .justify_end()
                        .child(
                            optical_surface(&theme, SurfaceVariant::Emerald)
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded(px(8.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb_to_hsla(RgbColor::new(255, 255, 255), 1.0))
                                .child("✓"),
                        ),
                ),
        )
    }
}
