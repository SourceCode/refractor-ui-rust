//! Feedback, System Status & Overlay components (Categories 5 & 6 of 55).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface,
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let accent = match self.tone {
            StatusTone::Success => theme.palette.success.main.as_str(),
            StatusTone::Warning => theme.palette.warning.main.as_str(),
            StatusTone::Error => theme.palette.error.main.as_str(),
            _ => theme.palette.primary.main.as_str(),
        };

        optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(14.0))
            .border_l(px(4.0))
            .border_color(color_str_to_hsla(accent, theme.palette.primary.rgb))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(div().text_size(px(14.0)).text_color(fg).child(self.title))
            .child(div().text_size(px(13.0)).text_color(muted).child(self.message))
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Toast)
            .px(px(16.0))
            .py(px(10.0))
            .text_size(px(13.0))
            .text_color(fg)
            .child(self.message)
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
        let fill = 240.0 * (self.progress_pct / 100.0);
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
        col.child(
            optical_surface(&theme, SurfaceVariant::Well)
                .w(px(240.0))
                .h(px(10.0))
                .rounded(px(999.0))
                .child(
                    optical_surface(&theme, SurfaceVariant::Emerald)
                        .w(px(fill))
                        .h(px(8.0))
                        .rounded(px(999.0)),
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
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(24.0))
            .flex()
            .flex_col()
            .items_center()
            .gap(px(6.0))
            .child(div().text_size(px(16.0)).text_color(fg).child(self.title))
            .child(div().text_size(px(13.0)).text_color(muted).child(self.description))
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
