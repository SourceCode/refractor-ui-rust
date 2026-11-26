//! Buttons & Actions components (Category 1 of 55).

use gpui::{
    App, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface,
};
use refractor_tokens::RgbColor;

/// Capsule height and padding scale for buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl ButtonSize {
    pub const fn height_px(self) -> f32 {
        match self {
            Self::Sm => 36.0,
            Self::Md => 48.0,
            Self::Lg => 56.0,
        }
    }

    pub const fn padding_x_px(self) -> f32 {
        match self {
            Self::Sm => 20.0,
            Self::Md => 28.0,
            Self::Lg => 36.0,
        }
    }

    pub const fn font_size_px(self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Md => 15.0,
            Self::Lg => 16.0,
        }
    }
}

/// Highest-emphasis optical glass capsule CTA button.
#[derive(Debug, Clone, IntoElement)]
pub struct PrimaryButton {
    pub label: SharedString,
    pub size: ButtonSize,
    pub leading_icon: Option<SharedString>,
    pub trailing_icon: Option<SharedString>,
    pub base: BaseProps,
}

impl PrimaryButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            size: ButtonSize::Md,
            leading_icon: None,
            trailing_icon: None,
            base: BaseProps::new(),
        }
    }

    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.base.disabled = disabled;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for PrimaryButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.primary.contrast_text, RgbColor::new(255, 255, 255));
        let mut surface = optical_surface(&theme, SurfaceVariant::PrimaryButton)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(10.0))
            .h(px(self.size.height_px()))
            .px(px(self.size.padding_x_px()))
            .text_size(px(self.size.font_size_px()))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg);

        if self.base.disabled {
            surface = surface.opacity(0.5);
        } else {
            let hover_border =
                color_str_to_hsla(&theme.palette.primary.light, theme.palette.primary.light_rgb);
            surface = surface
                .cursor_pointer()
                .hover(move |s| s.opacity(0.95).border_color(hover_border));
        }

        if let Some(ov) = theme.components.get("PrimaryButton") {
            surface = apply_component_override(surface, ov);
        }
        if let Some(ref ov) = self.base.style_override {
            surface = apply_component_override(surface, ov);
        }

        if let Some(icon) = self.leading_icon {
            surface = surface.child(div().child(icon));
        }
        surface = surface.child(self.label);
        if let Some(icon) = self.trailing_icon {
            surface = surface.child(div().child(icon));
        }
        surface
    }
}

/// Secondary ice-crystal or cobalt optical glass button.
#[derive(Debug, Clone, IntoElement)]
pub struct SecondaryButton {
    pub label: SharedString,
    pub size: ButtonSize,
    pub cobalt: bool,
    pub base: BaseProps,
}

impl SecondaryButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            size: ButtonSize::Md,
            cobalt: false,
            base: BaseProps::new(),
        }
    }

    pub const fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub const fn cobalt(mut self, cobalt: bool) -> Self {
        self.cobalt = cobalt;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.base.disabled = disabled;
        self
    }
}

impl RenderOnce for SecondaryButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let variant = if self.cobalt {
            SurfaceVariant::CobaltButton
        } else {
            SurfaceVariant::SecondaryButton
        };
        let fg = if self.cobalt {
            color_str_to_hsla(&theme.palette.info.contrast_text, RgbColor::new(255, 255, 255))
        } else {
            color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44))
        };

        let mut surface = optical_surface(&theme, variant)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .h(px(self.size.height_px()))
            .px(px(self.size.padding_x_px()))
            .text_size(px(self.size.font_size_px()))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg)
            .child(self.label);

        if self.base.disabled {
            surface = surface.opacity(0.5);
        } else {
            let hover_border =
                color_str_to_hsla(&theme.palette.secondary.main, theme.palette.secondary.rgb);
            surface = surface
                .cursor_pointer()
                .hover(move |s| s.border_color(hover_border));
        }
        surface
    }
}

/// Compact square/circular optical icon button.
#[derive(Debug, Clone, IntoElement)]
pub struct IconButton {
    pub glyph: SharedString,
    pub size_px: f32,
    pub base: BaseProps,
}

impl IconButton {
    pub fn new(glyph: impl Into<SharedString>) -> Self {
        Self {
            glyph: glyph.into(),
            size_px: 48.0,
            base: BaseProps::new(),
        }
    }

    pub const fn size_px(mut self, size_px: f32) -> Self {
        self.size_px = size_px;
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(24, 32, 44));
        let side = self.size_px.max(24.0);
        let radius = if side <= 38.0 {
            13.0
        } else if side >= 54.0 {
            20.0
        } else {
            17.0
        };
        optical_surface(&theme, SurfaceVariant::Crystal)
            .flex()
            .items_center()
            .justify_center()
            .w(px(side))
            .h(px(side))
            .rounded(px(radius))
            .text_color(fg)
            .cursor_pointer()
            .child(self.glyph)
    }
}

/// Optical glass multi-option segmented toggle bar.
#[derive(Debug, Clone, IntoElement)]
pub struct SegmentedControl {
    pub options: Vec<SharedString>,
    pub selected_index: usize,
    pub base: BaseProps,
}

impl SegmentedControl {
    pub fn new(options: Vec<SharedString>, selected_index: usize) -> Self {
        Self {
            options,
            selected_index,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for SegmentedControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let active_fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let inactive_fg =
            color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let mut container = optical_surface(&theme, SurfaceVariant::Crystal)
            .flex()
            .flex_row()
            .items_center()
            .rounded(px(9999.0))
            .gap(px(4.0))
            .p(px(4.0));

        for (idx, opt) in self.options.into_iter().enumerate() {
            let active = idx == self.selected_index;
            let pill = if active {
                optical_surface(&theme, SurfaceVariant::Well)
                    .rounded(px(9999.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(active_fg)
            } else {
                div()
                    .rounded(px(9999.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(inactive_fg)
            };
            container = container.child(
                pill.flex()
                    .items_center()
                    .justify_center()
                    .h(px(36.0))
                    .px(px(16.0))
                    .text_size(px(12.0))
                    .child(opt),
            );
        }
        container
    }
}

/// Primary action paired with a secondary dropdown trigger.
#[derive(Debug, Clone, IntoElement)]
pub struct SplitButton {
    pub label: SharedString,
    pub base: BaseProps,
}

impl SplitButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for SplitButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.primary.contrast_text, RgbColor::new(255, 255, 255));
        let sep = refractor_core::rgb_to_hsla(RgbColor::new(255, 255, 255), 0.25);
        optical_surface(&theme, SurfaceVariant::PrimaryButton)
            .flex()
            .flex_row()
            .items_center()
            .h(px(48.0))
            .text_size(px(15.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg)
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .pl(px(24.0))
                    .pr(px(16.0))
                    .child(self.label),
            )
            .child(div().w(px(1.0)).h(px(28.0)).bg(sep))
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .px(px(14.0))
                    .text_size(px(12.0))
                    .child("⌄"),
            )
    }
}

/// Elevated floating action capsule button.
#[derive(Debug, Clone, IntoElement)]
pub struct FloatingActionButton {
    pub glyph: SharedString,
    pub label: Option<SharedString>,
    pub base: BaseProps,
}

impl FloatingActionButton {
    pub fn new(glyph: impl Into<SharedString>) -> Self {
        Self {
            glyph: glyph.into(),
            label: None,
            base: BaseProps::new(),
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for FloatingActionButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.primary.contrast_text, RgbColor::new(255, 255, 255));
        let mut btn = optical_surface(&theme, SurfaceVariant::PrimaryButton)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .h(px(56.0))
            .text_size(px(14.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg);
        if let Some(lbl) = self.label {
            btn = btn.px(px(24.0)).child(self.glyph).child(lbl);
        } else {
            btn = btn.w(px(56.0)).child(self.glyph);
        }
        btn
    }
}
