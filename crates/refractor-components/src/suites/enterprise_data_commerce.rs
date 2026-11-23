//! Domain Suite 5/7 (`RUI-20`): Enterprise Data & Viz, Advanced Search, Workflows & API,
//! Document/Knowledge/Spreadsheet, Security & Auth, Commerce & Billing, Collaboration,
//! and Design/3D/Shader Inspectors (43 bespoke GPUI components).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface,
};
use refractor_security::{clamp_f32, sanitize_url};
use refractor_tokens::RgbColor;

use super::{
    gpu_matrix_canvas, gpu_meter, gpu_scatter_canvas, gpu_tracks_canvas, kv_rows, pill_strip,
    suite_header,
};

#[derive(Debug, Clone, Copy)]
enum EntKind {
    GridTable,
    GanttTracks,
    SearchFacet,
    AuthSecret,
    CommerceCard,
    DocPage,
    Inspector3d,
    CollabFeed,
}

macro_rules! bespoke_enterprise_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub subtitle: Option<SharedString>,
            pub badge: SharedString,
            pub value: SharedString,
            pub strength: f32,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    subtitle: None,
                    badge: SharedString::from($badge),
                    value: SharedString::from($default_val),
                    strength: 0.85,
                    base: BaseProps::new(),
                }
            }

            pub fn title(mut self, title: impl Into<SharedString>) -> Self {
                self.title = title.into();
                self
            }

            pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
                self.subtitle = Some(subtitle.into());
                self
            }

            pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
                self.badge = badge.into();
                self
            }

            pub fn value(mut self, value: impl Into<SharedString>) -> Self {
                self.value = value.into();
                self
            }

            pub fn strength(mut self, strength: f32) -> Self {
                self.strength = clamp_f32(strength, 0.0, 1.0, 0.5);
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

            pub const fn component_name() -> &'static str {
                stringify!($name)
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
                let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
                let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(suite_header(&theme, self.title, Some(self.badge)));
                if let Some(sub) = self.subtitle {
                    root = root.child(div().text_size(px(11.0)).text_color(muted).child(sub));
                }
                if self.base.disabled {
                    root = root.opacity(0.5);
                }
                if let Some(ov) = theme.components.get(stringify!($name)) {
                    root = apply_component_override(root, ov);
                }
                if let Some(ref ov) = self.base.style_override {
                    root = apply_component_override(root, ov);
                }

                match EntKind::$kind {
                    EntKind::GridTable => root.child(kv_rows(
                        &theme,
                        vec![
                            (SharedString::from("Dataset"), self.value),
                            (SharedString::from("CSV Export"), SharedString::from("OWASP Safe")),
                        ],
                    )),
                    EntKind::GanttTracks => root
                        .child(gpu_tracks_canvas(&theme, Vec::new(), Some(0.45), 60.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    EntKind::SearchFacet => root
                        .child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .px(px(10.0))
                                .py(px(6.0))
                                .text_size(px(12.0))
                                .text_color(fg)
                                .child(self.value),
                        )
                        .child(pill_strip(
                            &theme,
                            vec![
                                SharedString::from("status:active"),
                                SharedString::from("tier:pro"),
                                SharedString::from("wcag:aa"),
                            ],
                            Some(0),
                        )),
                    EntKind::AuthSecret => root
                        .child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .px(px(10.0))
                                .py(px(6.0))
                                .text_size(px(12.0))
                                .text_color(fg)
                                .child(self.value),
                        )
                        .child(gpu_meter(&theme, self.strength, true)),
                    EntKind::CommerceCard => root
                        .child(div().text_size(px(16.0)).text_color(fg).child(self.value))
                        .child(pill_strip(
                            &theme,
                            vec![SharedString::from("Monthly"), SharedString::from("Annual -20%")],
                            Some(1),
                        )),
                    EntKind::DocPage => {
                        let safe_hint = if self.value.starts_with("http") {
                            sanitize_url(self.value.as_ref(), false)
                        } else {
                            self.value.to_string()
                        };
                        root.child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .p(px(10.0))
                                .text_size(px(12.0))
                                .text_color(fg)
                                .child(safe_hint),
                        )
                    }
                    EntKind::Inspector3d => root
                        .child(gpu_scatter_canvas(&theme, vec![(0.5, 0.5, 14.0)], 52.0))
                        .child(kv_rows(
                            &theme,
                            vec![
                                (SharedString::from("Transform"), self.value),
                                (SharedString::from("IOR"), SharedString::from("1.52 Glass")),
                            ],
                        )),
                    EntKind::CollabFeed => root
                        .child(gpu_matrix_canvas(&theme, 2, 6, Vec::new(), 36.0))
                        .child(div().text_size(px(12.0)).text_color(fg).child(self.value)),
                }
            }
        }
    };
}

// Advanced Data & Visualization (5)
bespoke_enterprise_widget!(AdvancedDataGrid, "Advanced Data Grid", "Virtualized", "10,000 rows • Sort/Filter", GridTable);
bespoke_enterprise_widget!(DiffViewer, "Diff Viewer", "Unified", "+128 -14 lines", DocPage);
bespoke_enterprise_widget!(GanttChart, "Gantt Chart", "Schedule", "Release v2.0.0 Milestones", GanttTracks);
bespoke_enterprise_widget!(KanbanBoard, "Kanban Board", "3 Columns", "Backlog • In Review • Done", CollabFeed);
bespoke_enterprise_widget!(PivotTable, "Pivot Table", "Map-Safe", "Revenue by Region × Quarter", GridTable);

// Advanced Search & Filtering (4)
bespoke_enterprise_widget!(FacetedFilterBar, "Faceted Filter Bar", "Facets", "3 active filters", SearchFacet);
bespoke_enterprise_widget!(GlobalSearchModal, "Global Search Modal", "⌘K", "Search across 314 components...", SearchFacet);
bespoke_enterprise_widget!(MultiSelectCombobox, "Multi-Select Combobox", "Multi", "Rust, GPUI, Optical Glass", SearchFacet);
bespoke_enterprise_widget!(VisualQueryBuilder, "Visual Query Builder", "AST", "category = 'charts' AND wcag >= 4.5", SearchFacet);

// Enterprise Workflows, Data Ops & API Tooling (6)
bespoke_enterprise_widget!(ApiRequestBuilder, "API Request Builder", "HTTP", "POST https://api.refractor.dev/v2", SearchFacet);
bespoke_enterprise_widget!(ApprovalChainStrip, "Approval Chain Strip", "3/3 Signed", "Arch ✓ • Sec ✓ • QA ✓", CollabFeed);
bespoke_enterprise_widget!(AuditTrailFeed, "Audit Trail Feed", "Immutable", "v2.0.0 verified by Kestrel", CollabFeed);
bespoke_enterprise_widget!(CsvColumnMapper, "CSV Column Mapper", "RFC 4180", "5 columns mapped (OWASP safe)", GridTable);
bespoke_enterprise_widget!(FeatureFlagRollout, "Feature Flag Rollout", "Canary", "gpu_fx_overlays: 100%", AuthSecret);
bespoke_enterprise_widget!(SlaCountdownTimer, "SLA Countdown Timer", "P1 SLA", "03h 42m remaining", AuthSecret);

// Document, Knowledge & Spreadsheet Tools (7)
bespoke_enterprise_widget!(BlockTransformHandle, "Block Transform Handle", "Block", "⋮⋮ Drag or Convert Block", DocPage);
bespoke_enterprise_widget!(CalloutAdmonition, "Callout Admonition", "Important", "All crates forbid unsafe code.", DocPage);
bespoke_enterprise_widget!(DocumentPageViewer, "Document Page Viewer", "Page 1/4", "Refractor Architecture Guide", DocPage);
bespoke_enterprise_widget!(FloatingTableOfContents, "Floating Table of Contents", "Outline", "1. Tokens • 2. GPUI • 3. Suites", DocPage);
bespoke_enterprise_widget!(FormulaSpreadsheetGrid, "Formula Spreadsheet Grid", "A1:F12", "=SUM(B2:B10) -> 314", GridTable);
bespoke_enterprise_widget!(LinkPreviewCard, "Link Preview Card", "Safe URL", "https://github.com/SourceCode/glass-ui-v1", DocPage);
bespoke_enterprise_widget!(SlashCommandMenu, "Slash Command Menu", "/command", "/table • /callout • /chart", SearchFacet);

// Security, Auth & Account Management (5)
bespoke_enterprise_widget!(ActiveSessionsList, "Active Sessions List", "2 Devices", "Windows Workstation • Active now", AuthSecret);
bespoke_enterprise_widget!(OtpCodeInput, "OTP Code Input", "6-Digit", "4 • 8 • 2 • 9 • 1 • 0", AuthSecret);
bespoke_enterprise_widget!(PasswordStrengthMeter, "Password Strength Meter", "Strong", "Entropy: 96 bits", AuthSecret);
bespoke_enterprise_widget!(RolePermissionsMatrix, "Role Permissions Matrix", "RBAC", "Admin • Editor • Viewer", GridTable);
bespoke_enterprise_widget!(SecretKeyField, "Secret Key Field", "Masked", "rf_live_••••••••••••••••", AuthSecret);

// Commerce, Billing & Conversion (5)
bespoke_enterprise_widget!(PaymentCardInput, "Payment Card Input", "PCI", "•••• •••• •••• 4242 • 12/28", CommerceCard);
bespoke_enterprise_widget!(PricingTierMatrix, "Pricing Tier Matrix", "Plans", "$49 / month — Pro Studio", CommerceCard);
bespoke_enterprise_widget!(ProductVariantSelector, "Product Variant Selector", "SKU", "Obsidian Dark • 16-inch", CommerceCard);
bespoke_enterprise_widget!(PromoCouponInput, "Promo Coupon Input", "Discount", "REFRACTOR20 (-20%)", CommerceCard);
bespoke_enterprise_widget!(ShoppingCartDrawer, "Shopping Cart Drawer", "2 Items", "Subtotal: $148.00", CommerceCard);

// Collaboration & Guidance (4)
bespoke_enterprise_widget!(CommentThread, "Comment Thread", "2 Replies", "Looks crisp in Alabaster & Obsidian!", CollabFeed);
bespoke_enterprise_widget!(LivePresenceIndicator, "Live Presence Indicator", "Online", "3 collaborators viewing", CollabFeed);
bespoke_enterprise_widget!(NotificationCenter, "Notification Center", "Unread 2", "Build v2.0.0 passed all gates", CollabFeed);
bespoke_enterprise_widget!(ProductTour, "Product Tour", "Step 2/5", "Switch across 68 themes live", CollabFeed);

// Design, 3D & Shader Inspector Controls (7)
bespoke_enterprise_widget!(AlignmentToolbar, "Alignment Toolbar", "Layout", "Align Left • Center • Distribute", SearchFacet);
bespoke_enterprise_widget!(AssetSwatchGrid, "Asset Swatch Grid", "Swatches", "68 Theme Palettes", CollabFeed);
bespoke_enterprise_widget!(GradientStopEditor, "Gradient Stop Editor", "Linear", "0% #E75C24 → 100% #38BDF8", Inspector3d);
bespoke_enterprise_widget!(OpticalMaterialInspector, "Optical Material Inspector", "13 Materials", "Glass • Blur 18px • Specular", Inspector3d);
bespoke_enterprise_widget!(SpringEasingPicker, "Spring Easing Picker", "Physics", "stiffness=280 damping=24", Inspector3d);
bespoke_enterprise_widget!(TransformInspector, "Transform Inspector", "XYZ", "X: 0.0  Y: 0.0  Scale: 1.0", Inspector3d);
bespoke_enterprise_widget!(VectorXyPad, "Vector XY Pad", "2D Pad", "X: +0.42  Y: -0.18", Inspector3d);
