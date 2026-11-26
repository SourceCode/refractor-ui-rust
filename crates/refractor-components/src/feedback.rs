//! Feedback, System Status & Overlay components (Categories 5 & 6 of 55).

use gpui::{
    App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
    relative,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_security::clamp_f32;
use refractor_tokens::RgbColor;

use crate::display::StatusTone;

/// Semantic optical alert banner.
#[derive(Debug, Clone, IntoElement)]
pub struct AlertBanner {
    pub title: SharedString,
    pub message: SharedString,
    pub tone: StatusTone,
    pub base: BaseProps,
}

impl AlertBanner {
    pub fn new(title: impl Into<SharedString>, message: impl Into<SharedString>, tone: StatusTone) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            tone,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for AlertBanner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let (icon_glyph, accent_rgb) = match self.tone {
            StatusTone::Success => ("✓", RgbColor::new(16, 185, 129)),
            StatusTone::Warning => ("⚠", RgbColor::new(245, 158, 11)),
            StatusTone::Error => ("✕", RgbColor::new(239, 68, 68)),
            _ => ("i", RgbColor::new(2, 132, 199)),
        };

        optical_surface(&theme, SurfaceVariant::Slab)
            .w_full()
            .px(px(16.0))
            .py(px(14.0))
            .border_color(rgb_to_hsla(accent_rgb, 0.55))
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .child(
                div()
                    .w(px(20.0))
                    .h(px(20.0))
                    .mt(px(2.0))
                    .rounded(px(9999.0))
                    .border(px(1.5))
                    .border_color(rgb_to_hsla(accent_rgb, 0.90))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb_to_hsla(accent_rgb, 1.0))
                    .child(icon_glyph),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(fg)
                            .child(self.title),
                    )
                    .child(div().text_size(px(12.0)).text_color(muted).child(self.message)),
            )
    }
}

/// Floating optical toast notification.
#[derive(Debug, Clone, IntoElement)]
pub struct ToastSnackbar {
    pub message: SharedString,
    pub base: BaseProps,
}

impl ToastSnackbar {
    pub fn new(message: impl Into<SharedString>) -> Self {
        Self {
            message: message.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for ToastSnackbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(30, 41, 59));
        let diamond_col = rgb_to_hsla(theme.palette.warning.rgb, 0.95);
        optical_surface(&theme, SurfaceVariant::Toast)
            .min_h(px(48.0))
            .px(px(20.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .child(
                div()
                    .text_size(px(16.0))
                    .text_color(diamond_col)
                    .child("◆"),
            )
            .child(
                div()
                    .text_size(px(15.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(fg)
                    .child(self.message),
            )
    }
}

/// NaN-safe optical progress bar.
#[derive(Debug, Clone, IntoElement)]
pub struct ProgressAndSpinner {
    pub progress_pct: f32,
    pub label: Option<SharedString>,
    pub base: BaseProps,
}

impl ProgressAndSpinner {
    pub fn new(progress_pct: f32) -> Self {
        Self {
            progress_pct: clamp_f32(progress_pct, 0.0, 100.0, 0.0),
            label: None,
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for ProgressAndSpinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let pct = (self.progress_pct / 100.0).clamp(0.0, 1.0);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(30, 41, 59));
        let mut col = div().w_full().flex().flex_col();
        if let Some(lbl) = self.label {
            col = col.child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .mb(px(6.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(fg)
                            .child(lbl),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(fg)
                            .child(format!("{}%", self.progress_pct.round() as i32)),
                    ),
            );
        }
        col.child(
            optical_surface(&theme, SurfaceVariant::Crystal)
                .w_full()
                .h(px(16.0))
                .rounded(px(9999.0))
                .p(px(2.0))
                .overflow_hidden()
                .child(
                    optical_surface(&theme, SurfaceVariant::PrimaryButton)
                        .w(relative(pct))
                        .h_full()
                        .rounded(px(9999.0)),
                ),
        )
    }
}

/// Structured empty state surface.
#[derive(Debug, Clone, IntoElement)]
pub struct EmptyState {
    pub title: SharedString,
    pub description: SharedString,
    pub base: BaseProps,
}

impl EmptyState {
    pub fn new(title: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        optical_surface(&theme, SurfaceVariant::Slab)
            .w_full()
            .p(px(24.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(6.0))
            .child(
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .w(px(48.0))
                    .h(px(48.0))
                    .rounded(px(16.0))
                    .mb(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.0))
                    .text_color(fg)
                    .child("□"),
            )
            .child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(self.title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(muted)
                    .child(self.description),
            )
    }
}

/// Elevated optical modal dialog surface.
#[derive(Debug, Clone, IntoElement)]
pub struct ModalDialog {
    pub title: SharedString,
    pub body: SharedString,
    pub base: BaseProps,
}

impl ModalDialog {
    pub fn new(title: impl Into<SharedString>, body: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for ModalDialog {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        optical_surface(&theme, SurfaceVariant::Slab)
            .w(px(440.0))
            .p(px(22.0))
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(div().text_size(px(18.0)).text_color(fg).child(self.title))
            .child(div().text_size(px(14.0)).text_color(muted).child(self.body))
    }
}

/// Compact optical tooltip pill.
#[derive(Debug, Clone, IntoElement)]
pub struct Tooltip {
    pub content: SharedString,
    pub base: BaseProps,
}

impl Tooltip {
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Toast)
            .px(px(10.0))
            .py(px(6.0))
            .text_size(px(12.0))
            .text_color(fg)
            .child(self.content)
    }
}
