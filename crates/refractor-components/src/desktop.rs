//! Desktop OS, Workspace Layout, CLI Terminal, & AI Conversational components.

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, color_str_to_hsla, optical_surface,
};
use refractor_tokens::RgbColor;

/// Decorative window caption button style for [`WindowTitlebar`].
///
/// Defaults to [`Self::None`] so native OS-decorated desktop windows do not show
/// duplicate macOS traffic-light dots inside the client area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowChromeStyle {
    /// Clean app header bar with no duplicate OS caption buttons (recommended for native OS windows).
    #[default]
    None,
    /// Windows-style caption buttons (`─`, `□`, `✕`) aligned on the right.
    Windows,
    /// macOS-style traffic-light controls aligned on the left (for frameless/simulated Mac windows).
    MacTrafficLights,
}

/// Desktop OS window titlebar with optional app icon badge and configurable caption controls.
#[derive(Debug, Clone, IntoElement)]
pub struct WindowTitlebar {
    pub title: SharedString,
    pub icon: Option<SharedString>,
    pub chrome_style: WindowChromeStyle,
    pub base: BaseProps,
}

impl WindowTitlebar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            chrome_style: WindowChromeStyle::None,
            base: BaseProps::new(),
        }
    }

    /// Displays an optical-glass app icon badge (e.g. `"✓"`, `"R"`, `"◆"`) next to the title.
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Selects decorative caption controls (`None`, `Windows`, or `MacTrafficLights`).
    pub const fn chrome_style(mut self, style: WindowChromeStyle) -> Self {
        self.chrome_style = style;
        self
    }
}

impl RenderOnce for WindowTitlebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.fx.chrome_text, RgbColor::new(21, 30, 44));
        let contrast = color_str_to_hsla(
            &theme.palette.primary.contrast_text,
            RgbColor::new(255, 255, 255),
        );

        let mut title_group = div().flex().flex_row().items_center().gap(px(8.0));
        if let Some(glyph) = self.icon {
            title_group = title_group.child(
                optical_surface(&theme, SurfaceVariant::PrimaryButton)
                    .w(px(22.0))
                    .h(px(22.0))
                    .rounded(px(6.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(contrast)
                    .child(glyph),
            );
        }
        title_group = title_group.child(
            div()
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg)
                .child(self.title),
        );

        let bar = optical_surface(&theme, SurfaceVariant::Chrome)
            .h(px(theme.fx.title_bar_height_px.clamp(24.0, 48.0)))
            .px(px(14.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between();

        match self.chrome_style {
            WindowChromeStyle::None => bar.child(title_group),
            WindowChromeStyle::Windows => {
                let win_controls = div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .text_size(px(12.0))
                    .text_color(fg)
                    .child("─")
                    .child("□")
                    .child("✕");
                bar.child(title_group).child(win_controls)
            }
            WindowChromeStyle::MacTrafficLights => {
                let dots = div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded(px(999.0))
                            .bg(color_str_to_hsla(&theme.palette.error.main, theme.palette.error.rgb)),
                    )
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded(px(999.0))
                            .bg(color_str_to_hsla(
                                &theme.palette.warning.main,
                                theme.palette.warning.rgb,
                            )),
                    )
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded(px(999.0))
                            .bg(color_str_to_hsla(
                                &theme.palette.success.main,
                                theme.palette.success.rgb,
                            )),
                    );
                bar.child(dots).child(title_group).child(div().w(px(40.0)))
            }
        }
    }
}

/// Two-pane resizable workspace splitter container.
#[derive(IntoElement)]
pub struct WindowSplitter {
    pub left: AnyElement,
    pub right: AnyElement,
    pub base: BaseProps,
}

impl std::fmt::Debug for WindowSplitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSplitter")
            .field("base", &self.base)
            .finish_non_exhaustive()
    }
}

impl WindowSplitter {
    pub fn new(left: impl IntoElement, right: impl IntoElement) -> Self {
        Self {
            left: left.into_any_element(),
            right: right.into_any_element(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for WindowSplitter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let divider = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));
        div()
            .flex()
            .flex_row()
            .w_full()
            .gap(px(8.0))
            .child(div().flex_1().child(self.left))
            .child(div().w(px(2.0)).bg(divider))
            .child(div().flex_1().child(self.right))
    }
}

/// Desktop OS application dock bar.
#[derive(Debug, Clone, IntoElement)]
pub struct AppDockBar {
    pub apps: Vec<SharedString>,
    pub base: BaseProps,
}

impl AppDockBar {
    pub fn new(apps: Vec<SharedString>) -> Self {
        Self {
            apps,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for AppDockBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let mut dock = optical_surface(&theme, SurfaceVariant::Slab)
            .px(px(12.0))
            .py(px(8.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0));
        for app in self.apps {
            dock = dock.child(
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .px(px(12.0))
                    .py(px(8.0))
                    .text_size(px(12.0))
                    .text_color(fg)
                    .child(app),
            );
        }
        dock
    }
}

/// Bottom IDE/desktop workspace status bar.
#[derive(Debug, Clone, IntoElement)]
pub struct WorkspaceStatusBar {
    pub left_text: SharedString,
    pub right_text: SharedString,
    pub base: BaseProps,
}

impl WorkspaceStatusBar {
    pub fn new(left_text: impl Into<SharedString>, right_text: impl Into<SharedString>) -> Self {
        Self {
            left_text: left_text.into(),
            right_text: right_text.into(),
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for WorkspaceStatusBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.fx.chrome_text, RgbColor::new(21, 30, 44));
        optical_surface(&theme, SurfaceVariant::Chrome)
            .h(px(28.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .text_size(px(11.0))
            .text_color(fg)
            .child(self.left_text)
            .child(self.right_text)
    }
}

/// Native GPU-rendered terminal emulator surface using theme editor palette.
#[derive(Debug, Clone, IntoElement)]
pub struct TerminalEmulator {
    pub lines: Vec<SharedString>,
    pub base: BaseProps,
}

impl TerminalEmulator {
    pub fn new(lines: Vec<SharedString>) -> Self {
        Self {
            lines,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for TerminalEmulator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let bg = color_str_to_hsla(&theme.editor.background, RgbColor::new(15, 23, 42));
        let fg = color_str_to_hsla(&theme.editor.text, RgbColor::new(226, 232, 240));
        let mut term = optical_surface(&theme, SurfaceVariant::Crystal)
            .bg(bg)
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .text_size(px(12.0))
            .text_color(fg);
        for line in self.lines {
            term = term.child(div().child(line));
        }
        term
    }
}

/// AI conversational message stream surface.
#[derive(Debug, Clone, IntoElement)]
pub struct ChatMessageStream {
    pub messages: Vec<(SharedString, SharedString)>,
    pub base: BaseProps,
}

impl ChatMessageStream {
    pub fn new(messages: Vec<(SharedString, SharedString)>) -> Self {
        Self {
            messages,
            base: BaseProps::new(),
        }
    }
}

impl RenderOnce for ChatMessageStream {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);
        let mut col = div().flex().flex_col().gap(px(8.0));
        for (role, text) in self.messages {
            col = col.child(
                optical_surface(&theme, SurfaceVariant::Crystal)
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(4.0))
                    .child(div().text_size(px(11.0)).text_color(accent).child(role))
                    .child(div().text_size(px(13.0)).text_color(fg).child(text)),
            );
        }
        col
    }
}
