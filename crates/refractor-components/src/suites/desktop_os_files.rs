//! Domain Suite 6/7 (`RUI-21`): Desktop OS Windowing, File Explorer Navigation & Ops,
//! Window Anatomy & Tiling, Menus & Launchers, System Tray & Control Center, Hardware HUDs,
//! and Native System Dialogs (48 bespoke GPUI components).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface,
};
use refractor_security::clamp_f32;
use refractor_tokens::RgbColor;

use super::{gpu_matrix_canvas, gpu_meter, kv_rows, pill_strip, suite_header};

#[derive(Debug, Clone, Copy)]
enum OsKind {
    MenuBar,
    FileExplorer,
    TransferProgress,
    WindowChrome,
    TilingGrid,
    ControlCenter,
    SystemDialog,
    HardwareHud,
}

macro_rules! bespoke_os_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub badge: SharedString,
            pub value: SharedString,
            pub level: f32,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    badge: SharedString::from($badge),
                    value: SharedString::from($default_val),
                    level: 0.75,
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

            pub fn level(mut self, level: f32) -> Self {
                self.level = clamp_f32(level, 0.0, 1.0, 0.5);
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

                match OsKind::$kind {
                    OsKind::MenuBar => root.child(pill_strip(
                        &theme,
                        vec![
                            SharedString::from("File"),
                            SharedString::from("Edit"),
                            SharedString::from("View"),
                            SharedString::from("Window"),
                            SharedString::from("Help"),
                        ],
                        Some(0),
                    )),
                    OsKind::FileExplorer => root
                        .child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .px(px(10.0))
                                .py(px(6.0))
                                .text_size(px(12.0))
                                .text_color(fg)
                                .child(self.value),
                        )
                        .child(kv_rows(
                            &theme,
                            vec![
                                (SharedString::from("crates/"), SharedString::from("7 crates")),
                                (SharedString::from("apps/"), SharedString::from("2 apps")),
                            ],
                        )),
                    OsKind::TransferProgress => root
                        .child(gpu_meter(&theme, self.level, true))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    OsKind::WindowChrome => root.child(
                        optical_surface(&theme, SurfaceVariant::Chrome)
                            .px(px(10.0))
                            .py(px(6.0))
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .text_size(px(12.0))
                            .text_color(fg)
                            .child(self.value)
                            .child("─  □  ×"),
                    ),
                    OsKind::TilingGrid => root
                        .child(gpu_matrix_canvas(&theme, 2, 3, Vec::new(), 52.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    OsKind::ControlCenter => root
                        .child(pill_strip(
                            &theme,
                            vec![
                                SharedString::from("Wi-Fi On"),
                                SharedString::from("BT On"),
                                SharedString::from("Focus"),
                                SharedString::from("VPN"),
                            ],
                            Some(0),
                        ))
                        .child(gpu_meter(&theme, self.level, false)),
                    OsKind::SystemDialog => root
                        .child(div().text_size(px(12.0)).text_color(fg).child(self.value))
                        .child(pill_strip(
                            &theme,
                            vec![SharedString::from("Cancel"), SharedString::from("Confirm")],
                            Some(1),
                        )),
                    OsKind::HardwareHud => root
                        .child(gpu_meter(&theme, self.level, true))
                        .child(kv_rows(
                            &theme,
                            vec![(SharedString::from("Level"), self.value)],
                        )),
                }
            }
        }
    };
}

// Desktop OS & Windowing Metaphors (4)
bespoke_os_widget!(AppMenuBar, "App Menu Bar", "Global", "File • Edit • View • Help", MenuBar);
bespoke_os_widget!(MillerColumnsBrowser, "Miller Columns Browser", "3 Columns", "/workspace/crates/refractor-core", FileExplorer);
bespoke_os_widget!(QuickLookPreviewer, "Quick Look Previewer", "Spacebar", "README.md • 3.2 KB", SystemDialog);
bespoke_os_widget!(RadialContextMenu, "Radial Context Menu", "Pie", "6 radial actions", TilingGrid);

// File Explorer Navigation & Views (7)
bespoke_os_widget!(DirectoryFilterBar, "Directory Filter Bar", "Filter", "*.rs • Modified Today", FileExplorer);
bespoke_os_widget!(DualPaneCommander, "Dual-Pane Commander", "Left | Right", "crates/ ⇄ apps/", FileExplorer);
bespoke_os_widget!(EditablePathBar, "Editable Path Bar", "Path", "J:/code/refractor-ui-rust", FileExplorer);
bespoke_os_widget!(FileMetadataInspector, "File Metadata Inspector", "Stat", "Cargo.toml • UTF-8 • 1.8 KB", FileExplorer);
bespoke_os_widget!(FileViewModeBar, "File View Mode Bar", "Views", "Icons • List • Columns • Gallery", MenuBar);
bespoke_os_widget!(MarqueeSelectionBox, "Marquee Selection Box", "Rubberband", "4 items selected", TilingGrid);
bespoke_os_widget!(MountedVolumesSidebar, "Mounted Volumes Sidebar", "Drives", "NVMe C: • Workspace J:", FileExplorer);

// File Management & Operations (6)
bespoke_os_widget!(ArchiveInspector, "Archive Inspector", "ZIP/TAR", "refractor-v2.0.0.tar.gz • 42 files", FileExplorer);
bespoke_os_widget!(BatchRenameUtility, "Batch Rename Utility", "Pattern", "icon_{n:02}.svg (12 files)", SystemDialog);
bespoke_os_widget!(FileConflictDialog, "File Conflict Dialog", "Conflict", "Keep Both • Replace • Skip", SystemDialog);
bespoke_os_widget!(FileTransferQueue, "File Transfer Queue", "640 MB/s", "Copying 314 assets (75%)", TransferProgress);
bespoke_os_widget!(PermissionsSharingMatrix, "Permissions & Sharing", "ACL", "Owner RWX • Team R-X", FileExplorer);
bespoke_os_widget!(TrashRecoveryBanner, "Trash Recovery Banner", "Undo", "3 items in Trash • Restore", SystemDialog);

// Window Anatomy & Controls (5)
bespoke_os_widget!(FloatingInspectorPalette, "Floating Inspector Palette", "Utility", "Optical Properties", WindowChrome);
bespoke_os_widget!(MergedWindowTabBar, "Merged Window Tab Bar", "3 Tabs", "Workbench • Studio • Docs", WindowChrome);
bespoke_os_widget!(WindowAttachedSheet, "Window Attached Sheet", "Sheet", "Export Theme Preset JSON", SystemDialog);
bespoke_os_widget!(WindowResizeFrame, "Window Resize Frame", "1280×820", "8-edge resize handles", WindowChrome);
bespoke_os_widget!(WindowUtilityControls, "Window Utility Controls", "Pin", "Always on Top • Shade", WindowChrome);

// Window Management & Desktop Tiling (6)
bespoke_os_widget!(MinimizedWindowTray, "Minimized Window Tray", "2 Min", "Terminal • Audio Mixer", MenuBar);
bespoke_os_widget!(SnapZoneOverlay, "Snap Zone Overlay", "Zones", "Left Half • Right Half • Grid", TilingGrid);
bespoke_os_widget!(VirtualDesktopBar, "Virtual Desktop Bar", "Spaces", "Desktop 1 • Desktop 2 • Studio", MenuBar);
bespoke_os_widget!(WindowExposeGrid, "Window Exposé Grid", "Overview", "6 open windows", TilingGrid);
bespoke_os_widget!(WindowLayoutPopover, "Window Layout Popover", "Layouts", "50/50 • 70/30 • Quad", TilingGrid);
bespoke_os_widget!(WindowSwitcherOverlay, "Window Switcher Overlay", "Alt+Tab", "Refractor Workbench", TilingGrid);

// Menus & Launchers (5)
bespoke_os_widget!(AppLauncherMenu, "App Launcher Menu", "Start", "Pinned & Recent Apps", MenuBar);
bespoke_os_widget!(CascadingSubmenu, "Cascading Submenu", "Nested", "Theme › Cyber › Cyberpunk", MenuBar);
bespoke_os_widget!(RichContextMenu, "Rich Context Menu", "Context", "Copy • Swizzle • Inspect", MenuBar);
bespoke_os_widget!(SystemSpotlightLauncher, "System Spotlight Launcher", "⌘Space", "Search apps, files & themes...", FileExplorer);
bespoke_os_widget!(TearOffMenu, "Tear-Off Menu", "Floating", "Detached Tool Palette", WindowChrome);

// System Tray & Control Center (5)
bespoke_os_widget!(ClockCalendarPopover, "Clock & Calendar Popover", "12:59 PM", "Sun, Nov 22, 2026", SystemDialog);
bespoke_os_widget!(ControlCenterGrid, "Control Center Grid", "Quick", "Wi-Fi • Display • Sound", ControlCenter);
bespoke_os_widget!(NetworkVpnPopover, "Network & VPN Popover", "10 Gbps", "LAN Connected • WireGuard On", ControlCenter);
bespoke_os_widget!(PerAppAudioMixer, "Per-App Audio Mixer", "WASAPI", "Workbench 80% • DAW 100%", HardwareHud);
bespoke_os_widget!(SystemTrayCluster, "System Tray Cluster", "Tray", "Net • Vol • Bat 100% • 12:59", MenuBar);

// Hardware HUDs & System Overlays (5)
bespoke_os_widget!(HardwareOsdHud, "Hardware OSD HUD", "Volume", "75% Speaker Output", HardwareHud);
bespoke_os_widget!(LockScreenOverlay, "Lock Screen Overlay", "Locked", "Touch ID or Password", SystemDialog);
bespoke_os_widget!(ScreenCaptureHud, "Screen Capture HUD", "Capture", "Window • Region • Record", MenuBar);
bespoke_os_widget!(SessionPowerOverlay, "Session Power Overlay", "Power", "Sleep • Restart • Shut Down", SystemDialog);
bespoke_os_widget!(TaskManagerDialog, "Task Manager Dialog", "Processes", "CPU 12% • GPU 8% • 314 widgets", TransferProgress);

// Desktop Surfaces & Native System Dialogs (5)
bespoke_os_widget!(DesktopIconCanvas, "Desktop Icon Canvas", "Desktop", "6 desktop shortcuts", TilingGrid);
bespoke_os_widget!(DesktopWidgetFrame, "Desktop Widget Frame", "Widget", "Live Telemetry Glass Widget", WindowChrome);
bespoke_os_widget!(SystemAboutCard, "System About Card", "v2.0.0", "Refractor UI for Rust • GPUI", SystemDialog);
bespoke_os_widget!(SystemFileDialog, "System File Dialog", "Open", "Select Rust Workspace Folder", FileExplorer);
bespoke_os_widget!(SystemPreferencesShell, "System Preferences Shell", "Settings", "Appearance • Displays • Privacy", ControlCenter);
