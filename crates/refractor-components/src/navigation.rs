//! Navigation components (Category 3 of 55).

use gpui::{
    App, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface, rgb_to_hsla,
};
use refractor_tokens::RgbColor;

/// Top optical glass application navigation bar.
#[derive(Debug, Clone, IntoElement)]
pub struct TopNavBar {
    pub brand: SharedString,
    pub subtitle: Option<SharedString>,
    pub base: BaseProps,
}

impl TopNavBar {
    pub fn new(brand: impl Into<SharedString>) -> Self {
        Self {
            brand: brand.into(),
            subtitle: None,
            base: BaseProps::new(),
        }
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl RenderOnce for TopNavBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let primary_fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted_fg = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let mut bar = optical_surface(&theme, SurfaceVariant::Chrome)
            .h(px(52.0))
            .px(px(20.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(div().text_size(px(16.0)).text_color(primary_fg).child(self.brand));
        if let Some(sub) = self.subtitle {
            bar = bar.child(div().text_size(px(12.0)).text_color(muted_fg).child(sub));
        }
        bar
    }
}

/// Optical glass tab strip.
#[derive(Debug, Clone, IntoElement)]
pub struct PrismTabs {
    pub tabs: Vec<SharedString>,
    pub active_index: usize,
    pub base: BaseProps,
}

impl PrismTabs {
    pub fn new(tabs: Vec<SharedString>, active_index: usize) -> Self {
        Self {
            tabs,
            active_index,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for PrismTabs {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let mut row = optical_surface(&theme, SurfaceVariant::Well)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .p(px(4.0));

        for (idx, tab) in self.tabs.into_iter().enumerate() {
            let active = idx == self.active_index;
            let item = if active {
                optical_surface(&theme, SurfaceVariant::Crystal).text_color(color_str_to_hsla(
                    &theme.palette.text.primary,
                    RgbColor::new(21, 30, 44),
                ))
            } else {
                div().text_color(color_str_to_hsla(
                    &theme.palette.text.secondary,
                    RgbColor::new(100, 116, 139),
                ))
            };
            row = row.child(item.px(px(14.0)).py(px(7.0)).text_size(px(13.0)).child(tab));
        }
        row
    }
}

/// Hierarchical breadcrumb trail.
#[derive(Debug, Clone, IntoElement)]
pub struct Breadcrumbs {
    pub segments: Vec<SharedString>,
    pub base: BaseProps,
}

impl Breadcrumbs {
    pub fn new(segments: Vec<SharedString>) -> Self {
        Self {
            segments,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Breadcrumbs {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let primary_fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted_fg = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let len = self.segments.len();
        let mut row = div().flex().flex_row().items_center().gap(px(6.0));
        for (idx, seg) in self.segments.into_iter().enumerate() {
            let is_last = idx + 1 == len;
            row = row.child(
                div()
                    .text_size(px(13.0))
                    .text_color(if is_last { primary_fg } else { muted_fg })
                    .child(seg),
            );
            if !is_last {
                row = row.child(div().text_size(px(12.0)).text_color(muted_fg).child("/"));
            }
        }
        row
    }
}

/// Multi-step progress stepper.
#[derive(Debug, Clone, IntoElement)]
pub struct Stepper {
    pub steps: Vec<SharedString>,
    pub current: usize,
    pub base: BaseProps,
}

impl Stepper {
    pub fn new(steps: Vec<SharedString>, current: usize) -> Self {
        Self {
            steps,
            current,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Stepper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let mut row = div().flex().flex_row().items_center().gap(px(12.0));
        for (idx, step) in self.steps.into_iter().enumerate() {
            let variant = match idx.cmp(&self.current) {
                std::cmp::Ordering::Less => SurfaceVariant::Emerald,
                std::cmp::Ordering::Equal => SurfaceVariant::PrimaryButton,
                std::cmp::Ordering::Greater => SurfaceVariant::Well,
            };
            let badge = optical_surface(&theme, variant)
                .w(px(26.0))
                .h(px(26.0))
                .rounded(px(999.0))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(12.0))
                .text_color(color_str_to_hsla(
                    if idx <= self.current {
                        &theme.palette.primary.contrast_text
                    } else {
                        &theme.palette.text.secondary
                    },
                    RgbColor::new(255, 255, 255),
                ))
                .child(format!("{}", idx + 1));

            row = row.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(badge)
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(color_str_to_hsla(
                                &theme.palette.text.primary,
                                RgbColor::new(21, 30, 44),
                            ))
                            .child(step),
                    ),
            );
        }
        row
    }
}

/// Pagination control strip.
#[derive(Debug, Clone, IntoElement)]
pub struct Pagination {
    pub page: usize,
    pub total_pages: usize,
    pub base: BaseProps,
}

impl Pagination {
    pub fn new(page: usize, total_pages: usize) -> Self {
        Self {
            page: page.max(1),
            total_pages: total_pages.max(1),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Crystal)
            .px(px(14.0))
            .py(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.0))
            .text_size(px(13.0))
            .text_color(fg)
            .child("‹ Prev")
            .child(format!("Page {} of {}", self.page, self.total_pages))
            .child("Next ›")
    }
}

/// Collapsible sidebar navigation drawer.
#[derive(Debug, Clone, IntoElement)]
pub struct SidebarDrawer {
    pub title: SharedString,
    pub items: Vec<SharedString>,
    pub active_index: usize,
    pub base: BaseProps,
}

impl SidebarDrawer {
    pub fn new(title: impl Into<SharedString>, items: Vec<SharedString>, active_index: usize) -> Self {
        Self {
            title: title.into(),
            items,
            active_index,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for SidebarDrawer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let mut col = optical_surface(&theme, SurfaceVariant::Slab)
            .w(px(240.0))
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(div().text_size(px(12.0)).text_color(muted).child(self.title));

        for (idx, item) in self.items.into_iter().enumerate() {
            let active = idx == self.active_index;
            let row = if active {
                optical_surface(&theme, SurfaceVariant::PrimaryButton).text_color(color_str_to_hsla(
                    &theme.palette.primary.contrast_text,
                    RgbColor::new(255, 255, 255),
                ))
            } else {
                div().text_color(fg)
            };
            col = col.child(row.px(px(12.0)).py(px(8.0)).text_size(px(13.0)).child(item));
        }
        col
    }
}

/// Spotlight-style keyboard command palette.
#[derive(Debug, Clone, IntoElement)]
pub struct CommandPalette {
    pub query: SharedString,
    pub results: Vec<SharedString>,
    pub base: BaseProps,
}

impl CommandPalette {
    pub fn new(query: impl Into<SharedString>, results: Vec<SharedString>) -> Self {
        Self {
            query: query.into(),
            results,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for CommandPalette {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let mut shell = optical_surface(&theme, SurfaceVariant::Slab)
            .w(px(480.0))
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(
                optical_surface(&theme, SurfaceVariant::Well)
                    .h(px(40.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .text_color(fg)
                    .child(if self.query.is_empty() {
                        SharedString::from("Type a command or search...")
                    } else {
                        self.query
                    }),
            );
        for item in self.results {
            shell = shell.child(
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .px(px(12.0))
                    .py(px(8.0))
                    .text_size(px(13.0))
                    .text_color(fg)
                    .child(item),
            );
        }
        shell
    }
}

/// Top-level horizontal menu bar trigger button (`File`, `Edit`, `View`, `Theme`, `Help`).
#[derive(Debug, Clone, IntoElement)]
pub struct MenuBarTrigger {
    pub label: SharedString,
    pub active: bool,
    pub base: BaseProps,
}

impl MenuBarTrigger {
    pub fn new(label: impl Into<SharedString>, active: bool) -> Self {
        Self {
            label: label.into(),
            active,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for MenuBarTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.fx.chrome_text, RgbColor::new(21, 30, 44));
        let hover_bg = rgb_to_hsla(theme.palette.primary.rgb, 0.14);

        if self.active {
            let contrast = color_str_to_hsla(
                &theme.palette.primary.contrast_text,
                RgbColor::new(255, 255, 255),
            );
            optical_surface(&theme, SurfaceVariant::PrimaryButton)
                .h(px(26.0))
                .px(px(10.0))
                .rounded(px(7.0))
                .flex()
                .items_center()
                .text_size(px(12.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(contrast)
                .cursor_pointer()
                .child(self.label)
        } else {
            div()
                .h(px(26.0))
                .px(px(10.0))
                .rounded(px(7.0))
                .flex()
                .items_center()
                .text_size(px(12.5))
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg)
                .cursor_pointer()
                .hover(move |s| s.bg(hover_bg))
                .child(self.label)
        }
    }
}

/// Interactive row inside an optical-glass horizontal menu bar dropdown.
#[derive(Debug, Clone, IntoElement)]
pub struct MenuDropdownItem {
    pub label: SharedString,
    pub shortcut: Option<SharedString>,
    pub checked: bool,
    pub base: BaseProps,
}

impl MenuDropdownItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            checked: false,
            base: BaseProps::new(),
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub const fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
}

impl RenderOnce for MenuDropdownItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);
        let hover_bg = rgb_to_hsla(theme.palette.primary.rgb, 0.14);

        let mut row = div()
            .w_full()
            .px(px(10.0))
            .py(px(6.0))
            .rounded(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(16.0))
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .w(px(14.0))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(accent)
                            .child(if self.checked { "✓" } else { "" }),
                    )
                    .child(div().text_size(px(12.5)).text_color(fg).child(self.label)),
            );

        if let Some(sc) = self.shortcut {
            row = row.child(div().text_size(px(11.0)).text_color(muted).child(sc));
        }

        row
    }
}
