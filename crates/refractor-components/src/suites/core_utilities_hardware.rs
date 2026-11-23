//! Domain Suite 7/7 (`RUI-22`): Data Display, Feedback/Overlays, Layout & Workspace,
//! Foundational Micro-Utilities, Mobile/Touch & Hardware/IoT Tactile Surfaces
//! (32 bespoke GPUI components).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface,
};
use refractor_security::clamp_f32;
use refractor_tokens::RgbColor;

use super::{
    gpu_bar_canvas, gpu_matrix_canvas, gpu_meter, gpu_scatter_canvas, kv_rows, pill_strip,
    suite_header,
};

#[derive(Debug, Clone, Copy)]
enum UtilKind {
    DisplayList,
    CalendarGrid,
    OverlayPopover,
    WorkspacePane,
    MicroUtility,
    MobileBar,
    HardwareTactile,
    LedReadout,
}

macro_rules! bespoke_util_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub badge: SharedString,
            pub value: SharedString,
            pub numeric: f32,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    badge: SharedString::from($badge),
                    value: SharedString::from($default_val),
                    numeric: 0.72,
                    base: BaseProps::new(),
                }
            }

            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.title = title.into();
                self
            }

            pub fn value(mut self, value: impl Into<SharedString>) -> Self {
                self.value = value.into();
                self
            }

            pub fn numeric(mut self, numeric: f32) -> Self {
                self.numeric = clamp_f32(numeric, 0.0, 1.0, 0.5);
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

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl RenderOnce for $name {
            fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
                let theme = active_theme(cx);
                let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
                let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);
                let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(suite_header(&theme, self.title, Some(self.badge)));
                if self.base.disabled {
                    root = root.opacity(0.5);
                }
                if let Some(ov) = theme.components.get(stringify!($name)) {
                    root = apply_component_override(root, ov);
                }
                if let Some(ref ov) = self.base.style_override {
                    root = apply_component_override(root, ov);
                }

                match UtilKind::$kind {
                    UtilKind::DisplayList => root.child(kv_rows(
                        &theme,
                        vec![
                            (SharedString::from("Item 1"), self.value),
                            (SharedString::from("Item 2"), SharedString::from("Optical Glass Ready")),
                        ],
                    )),
                    UtilKind::CalendarGrid => root
                        .child(gpu_matrix_canvas(&theme, 5, 7, Vec::new(), 56.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    UtilKind::OverlayPopover => root.child(
                        optical_surface(&theme, SurfaceVariant::Toast)
                            .p(px(10.0))
                            .text_size(px(12.0))
                            .text_color(fg)
                            .child(self.value),
                    ),
                    UtilKind::WorkspacePane => root
                        .child(gpu_bar_canvas(&theme, vec![0.4, 0.7, 0.55, 0.85, 0.65], 44.0, false))
                        .child(div().text_size(px(12.0)).text_color(fg).child(self.value)),
                    UtilKind::MicroUtility => root.child(
                        optical_surface(&theme, SurfaceVariant::Well)
                            .px(px(10.0))
                            .py(px(6.0))
                            .text_size(px(12.0))
                            .text_color(fg)
                            .child(self.value),
                    ),
                    UtilKind::MobileBar => root.child(pill_strip(
                        &theme,
                        vec![
                            SharedString::from("Home"),
                            SharedString::from("Explore"),
                            SharedString::from("Studio"),
                            SharedString::from("Profile"),
                        ],
                        Some(2),
                    )),
                    UtilKind::HardwareTactile => root
                        .child(gpu_scatter_canvas(&theme, vec![(self.numeric, 0.5, 16.0)], 56.0))
                        .child(gpu_meter(&theme, self.numeric, true))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    UtilKind::LedReadout => {
                        let bg = color_str_to_hsla(&theme.editor.background, RgbColor::new(10, 14, 22));
                        root.child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .bg(bg)
                                .px(px(14.0))
                                .py(px(10.0))
                                .text_size(px(20.0))
                                .text_color(accent)
                                .child(self.value),
                        )
                    }
                }
            }
        }
    };
}

// Data Display & Content (6)
bespoke_util_widget!(CalendarView, "Calendar View", "Nov 2026", "22 Nov • Release v2.0.0", CalendarGrid);
bespoke_util_widget!(CarouselSlider, "Carousel Slider", "Slide 1/4", "Optical Glass Specimen", WorkspacePane);
bespoke_util_widget!(OpticalCharts, "Optical Charts", "Suite", "Token-Driven GPU Charts", WorkspacePane);
bespoke_util_widget!(PrismList, "Prism List", "List", "314 Native Components", DisplayList);
bespoke_util_widget!(Timeline, "Timeline", "Events", "v1.0.0 → v2.0.0 Parity", DisplayList);
bespoke_util_widget!(TreeView, "Tree View", "Hierarchy", "crates/ › refractor-components/", DisplayList);

// Feedback & Overlays (4)
bespoke_util_widget!(SkeletonLoader, "Skeleton Loader", "Shimmer", "Loading optical layout...", WorkspacePane);
bespoke_util_widget!(ContextMenuDropdown, "Context Menu Dropdown", "Menu", "Inspect • Swizzle • Copy", OverlayPopover);
bespoke_util_widget!(ContextualPopover, "Contextual Popover", "Popover", "Anchored Glass Details", OverlayPopover);
bespoke_util_widget!(SlideOverSheet, "Slide-Over Sheet", "Drawer", "Theme Inspector Sheet", OverlayPopover);

// Layout & Workspace Controls (5)
bespoke_util_widget!(DockablePanel, "Dockable Panel", "Dock", "Inspector Docked Right", WorkspacePane);
bespoke_util_widget!(MasonryGrid, "Masonry Grid", "3 Cols", "Variable-height cards", CalendarGrid);
bespoke_util_widget!(MasterDetailView, "Master-Detail View", "Split", "Catalog List | Component Preview", DisplayList);
bespoke_util_widget!(StickyActionBar, "Sticky Action Bar", "Footer", "Save Changes • Discard", MobileBar);
bespoke_util_widget!(VirtualizedList, "Virtualized List", "100k Rows", "Windowed 60fps Viewport", DisplayList);

// Foundational Primitives & Micro-Utilities (7)
bespoke_util_widget!(AvatarStack, "Avatar Stack", "+5", "RR • AL • MK • +5", MobileBar);
bespoke_util_widget!(ExpandableTextClamp, "Expandable Text Clamp", "2 Lines", "Refractor combines GPUI + Iced ergonomics...", MicroUtility);
bespoke_util_widget!(InlineCopyTrigger, "Inline Copy Trigger", "Copy", "cargo add refractor-components", MicroUtility);
bespoke_util_widget!(MarqueeTicker, "Marquee Ticker", "Live", "GPUI 0.2.2 • 314 Components • 68 Themes", MicroUtility);
bespoke_util_widget!(PropertyGrid, "Property Grid", "Key/Val", "blur_px: 18.0 • material: glass", DisplayList);
bespoke_util_widget!(ScrollShadowArea, "Scroll Shadow Area", "Scroll", "Edge-faded optical scroll container", MicroUtility);
bespoke_util_widget!(ShortcutCheatSheet, "Shortcut Cheat Sheet", "Keys", "⌘K Palette • ⌘T Cycle Theme", DisplayList);

// Mobile, Touch & System Utilities (5)
bespoke_util_widget!(BottomNavigationBar, "Bottom Navigation Bar", "Mobile", "Home • Explore • Studio • Profile", MobileBar);
bespoke_util_widget!(CookieConsentBanner, "Cookie Consent Banner", "Privacy", "Essential Only • Accept All", OverlayPopover);
bespoke_util_widget!(KeycapBadge, "Keycap Badge", "Shortcut", "⌘ + Shift + P", MicroUtility);
bespoke_util_widget!(PullToRefresh, "Pull To Refresh", "Sync", "Pull down to refresh telemetry", MicroUtility);
bespoke_util_widget!(SwipeableActionRow, "Swipeable Action Row", "Swipe", "Archive ⇄ Pin", MicroUtility);

// Hardware, IoT & Tactile Surfaces (5)
bespoke_util_widget!(DigitalLedReadout, "Digital LED Readout", "7-Seg", "120.0 FPS", LedReadout);
bespoke_util_widget!(HardwarePortStrip, "Hardware Port Strip", "I/O", "USB-C • HDMI • MIDI • ETH", MobileBar);
bespoke_util_widget!(StepSequencerMatrix, "Step Sequencer Matrix", "16-Step", "128 BPM • 4/4 Pattern A", CalendarGrid);
bespoke_util_widget!(ThermostatDial, "Thermostat Dial", "HVAC", "70.5°F • Eco Glass", HardwareTactile);
bespoke_util_widget!(VirtualJoystickPad, "Virtual Joystick Pad", "2-Axis", "X: +0.24  Y: +0.68", HardwareTactile);
