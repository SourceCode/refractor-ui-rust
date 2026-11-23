//! Form Inputs & Selection Controls (Category 2 of 55).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface,
};
use refractor_security::clamp_f32;
use refractor_tokens::RgbColor;

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
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let text_muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let is_empty = self.value.is_empty();
        let shown = if is_empty { self.placeholder } else { self.value };

        let mut wrapper = div().flex().flex_col().gap(px(6.0));
        if let Some(lbl) = self.label {
            wrapper = wrapper.child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_muted)
                    .child(lbl),
            );
        }
        wrapper.child(
            optical_surface(&theme, SurfaceVariant::Well)
                .h(px(40.0))
                .px(px(14.0))
                .flex()
                .items_center()
                .text_size(px(14.0))
                .text_color(if is_empty { text_muted } else { text_primary })
                .child(shown),
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
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let min_h = f32::from(self.rows.max(2)) * 22.0 + 16.0;
        let mut wrapper = div().flex().flex_col().gap(px(6.0));
        if let Some(lbl) = self.label {
            wrapper = wrapper.child(
                div()
                    .text_size(px(12.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.secondary,
                        RgbColor::new(100, 116, 139),
                    ))
                    .child(lbl),
            );
        }
        wrapper.child(
            optical_surface(&theme, SurfaceVariant::Well)
                .min_h(px(min_h))
                .p(px(12.0))
                .text_size(px(14.0))
                .text_color(text_primary)
                .child(self.value),
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
            .w(px(26.0))
            .h(px(26.0));

        let mut track = optical_surface(&theme, track_variant)
            .w(px(60.0))
            .h(px(34.0))
            .p(px(3.0))
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
            .gap(px(10.0))
            .child(track.child(thumb));

        if let Some(lbl) = self.label {
            row = row.child(
                div()
                    .text_size(px(14.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.primary,
                        RgbColor::new(21, 30, 44),
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
            SurfaceVariant::Well
        };
        let check_glyph = if self.checked { "✓" } else { "" };
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(
                optical_surface(&theme, box_variant)
                    .w(px(20.0))
                    .h(px(20.0))
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.success.contrast_text,
                        RgbColor::new(255, 255, 255),
                    ))
                    .child(check_glyph),
            )
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.primary,
                        RgbColor::new(21, 30, 44),
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
        let text_primary = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let mut col = div().flex().flex_col().gap(px(8.0));
        for (idx, opt) in self.options.into_iter().enumerate() {
            let active = idx == self.selected;
            let dot = optical_surface(
                &theme,
                if active {
                    SurfaceVariant::PrimaryButton
                } else {
                    SurfaceVariant::Well
                },
            )
            .w(px(18.0))
            .h(px(18.0))
            .rounded(px(999.0));

            col = col.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(dot)
                    .child(div().text_size(px(14.0)).text_color(text_primary).child(opt)),
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
        let track_w = 220.0_f32;
        let fill_px = (track_w * pct).clamp(0.0, track_w);
        let thumb_size = 20.0_f32;
        let thumb_left = (fill_px - thumb_size * 0.5).clamp(0.0, track_w - thumb_size);

        let mut col = div().flex().flex_col().gap(px(6.0));
        if let Some(lbl) = self.label {
            col = col.child(
                div()
                    .text_size(px(12.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.secondary,
                        RgbColor::new(100, 116, 139),
                    ))
                    .child(lbl),
            );
        }

        let track = div()
            .relative()
            .w(px(track_w))
            .h(px(22.0))
            .flex()
            .items_center()
            .child(
                optical_surface(&theme, SurfaceVariant::Well)
                    .w(px(track_w))
                    .h(px(10.0))
                    .rounded(px(999.0))
                    .child(
                        optical_surface(&theme, SurfaceVariant::PrimaryButton)
                            .w(px(fill_px))
                            .h(px(8.0))
                            .rounded(px(999.0)),
                    ),
            )
            .child(
                optical_surface(&theme, SurfaceVariant::Thumb)
                    .absolute()
                    .left(px(thumb_left))
                    .top(px(1.0))
                    .w(px(thumb_size))
                    .h(px(thumb_size))
                    .rounded(px(999.0)),
            );

        col.child(track)
    }
}

/// Optical glass dropdown select trigger.
#[derive(Debug, Clone, IntoElement)]
pub struct DropdownSelect {
    pub selected_label: SharedString,
    pub base: BaseProps,
}

impl DropdownSelect {
    pub fn new(selected_label: impl Into<SharedString>) -> Self {
        Self {
            selected_label: selected_label.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for DropdownSelect {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Well)
            .h(px(40.0))
            .px(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(12.0))
            .text_size(px(14.0))
            .text_color(fg)
            .child(self.selected_label)
            .child("▾")
    }
}
