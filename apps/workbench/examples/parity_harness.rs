#![forbid(unsafe_code)]

use gpui::{
    AnyElement, App, AppContext, Application, Bounds, Context, IntoElement, ParentElement, Render,
    Styled, Window, WindowBounds, WindowOptions, div, px, rgb, size,
};
use refractor_components::{
    AlertBanner, Avatar, BadgeStatus, Breadcrumbs, ButtonSize, Checkbox, DropdownSelect,
    FloatingActionButton, IconButton, Pagination, PrimaryButton, PrismCard, PrismTabs,
    ProgressAndSpinner, RadioGroup, RangeSlider, SecondaryButton, SegmentedControl, SplitButton,
    StatusTone, Stepper, TagChip, TextArea, TextInput, ToastSnackbar, ToggleSwitch,
};
use refractor_core::ThemeGlobal;
use refractor_themes::resolve_theme_by_id;
use refractor_tokens::ThemeMode;

struct ParityView {
    sheet: String,
    theme_id: String,
}

impl ParityView {
    fn cell(children: Vec<AnyElement>) -> AnyElement {
        let mut c = div()
            .w(px(400.0))
            .h(px(200.0))
            .p(px(20.0))
            .flex()
            .flex_col()
            .justify_center()
            .items_start()
            .gap(px(10.0))
            .overflow_hidden();
        for ch in children {
            c = c.child(ch);
        }
        c.into_any_element()
    }

    fn render_sheet_1() -> AnyElement {
        let row0 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(PrimaryButton::new("Small").size(ButtonSize::Sm))
                    .child(PrimaryButton::new("Start project").size(ButtonSize::Md))
                    .into_any_element(),
                PrimaryButton::new("Launch")
                    .size(ButtonSize::Lg)
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(SecondaryButton::new("Secondary").size(ButtonSize::Md))
                    .child(
                        SecondaryButton::new("Pro plan")
                            .size(ButtonSize::Md)
                            .cobalt(true),
                    )
                    .into_any_element(),
                SecondaryButton::new("Disabled")
                    .size(ButtonSize::Sm)
                    .disabled(true)
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(IconButton::new("+"))
                    .child(FloatingActionButton::new("+"))
                    .into_any_element(),
                SplitButton::new("Deploy").into_any_element(),
            ]));

        let row1 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                SegmentedControl::new(vec!["Day".into(), "Week".into(), "Month".into()], 1)
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                ToggleSwitch::new(true).label("Optics").into_any_element(),
                ToggleSwitch::new(false).label("Muted").into_any_element(),
            ]))
            .child(Self::cell(vec![
                Checkbox::new(true, "Enable HDR").into_any_element(),
                Checkbox::new(false, "Auto-sync").into_any_element(),
            ]));

        let row2 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                div()
                    .w_full()
                    .child(TextInput::new("Refractor Optical").label("Project name"))
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                div()
                    .w_full()
                    .child(
                        TextArea::new("Optical glass surface.")
                            .label("Notes")
                            .rows(2),
                    )
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                div()
                    .w_full()
                    .child(DropdownSelect::new("Optical Glass").label("Material"))
                    .into_any_element(),
            ]));

        let row3 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                div()
                    .w_full()
                    .child(RangeSlider::new(64.0, 0.0, 100.0).label("Blur"))
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                RadioGroup::new(vec!["Alabaster".into(), "Obsidian".into()], 0).into_any_element(),
            ]))
            .child(Self::cell(vec![
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(8.0))
                    .child(BadgeStatus::new("Online", StatusTone::Success))
                    .child(BadgeStatus::new("Warn", StatusTone::Warning))
                    .child(BadgeStatus::new("Offline", StatusTone::Neutral))
                    .into_any_element(),
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(px(8.0))
                    .child(TagChip::new("Glass"))
                    .child(TagChip::new("GPUI").selected(true))
                    .into_any_element(),
            ]));

        div()
            .flex()
            .flex_col()
            .w(px(1200.0))
            .h(px(800.0))
            .child(row0)
            .child(row1)
            .child(row2)
            .child(row3)
            .into_any_element()
    }

    fn render_sheet_2() -> AnyElement {
        let row0 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                PrismCard::new()
                    .kicker("OPTICS")
                    .title("Optical Slab")
                    .child("Refractive frosted glass card surface.")
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                AlertBanner::new(
                    "Optical Ready",
                    "GPU shader cache verified.",
                    StatusTone::Primary,
                )
                .into_any_element(),
            ]))
            .child(Self::cell(vec![
                AlertBanner::new(
                    "Build Passed",
                    "Zero warnings across workspace.",
                    StatusTone::Success,
                )
                .into_any_element(),
            ]));

        let row1 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                ToastSnackbar::new("Deployment verified").into_any_element(),
            ]))
            .child(Self::cell(vec![
                ProgressAndSpinner::new(72.0)
                    .label("Shader cache")
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                PrismTabs::new(
                    vec!["Overview".into(), "Tokens".into(), "Preview".into()],
                    0,
                )
                .into_any_element(),
            ]));

        let row2 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                Breadcrumbs::new(vec![
                    "Workspace".into(),
                    "Components".into(),
                    "Buttons".into(),
                ])
                .into_any_element(),
            ]))
            .child(Self::cell(vec![Pagination::new(2, 5).into_any_element()]))
            .child(Self::cell(vec![
                Stepper::new(vec!["Tokens".into(), "Build".into(), "Ship".into()], 1)
                    .into_any_element(),
            ]));

        let row3 = div()
            .flex()
            .flex_row()
            .w(px(1200.0))
            .h(px(200.0))
            .child(Self::cell(vec![
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .child(Avatar::new("RR").size_px(36.0))
                    .child(Avatar::new("OG").size_px(44.0))
                    .child(Avatar::new("RU").size_px(56.0))
                    .into_any_element(),
            ]))
            .child(Self::cell(vec![
                AlertBanner::new(
                    "Contrast Check",
                    "Review low-alpha borders.",
                    StatusTone::Warning,
                )
                .into_any_element(),
            ]))
            .child(Self::cell(vec![
                AlertBanner::new(
                    "Sync Blocked",
                    "Reconnect remote endpoint.",
                    StatusTone::Error,
                )
                .into_any_element(),
            ]));

        div()
            .flex()
            .flex_col()
            .w(px(1200.0))
            .h(px(800.0))
            .child(row0)
            .child(row1)
            .child(row2)
            .child(row3)
            .into_any_element()
    }
}

impl Render for ParityView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = resolve_theme_by_id(&self.theme_id);
        let is_dark = theme.palette.mode == ThemeMode::Dark;
        cx.set_global(ThemeGlobal::new(theme));
        let bg = if is_dark {
            rgb(0x000f_1621)
        } else {
            rgb(0x00ea_edf2)
        };

        div()
            .w(px(1200.0))
            .h(px(800.0))
            .bg(bg)
            .font_family("Segoe UI")
            .overflow_hidden()
            .child(match self.sheet.as_str() {
                "2" => Self::render_sheet_2(),
                _ => Self::render_sheet_1(),
            })
    }
}

fn main() {
    let mut sheet = "1".to_owned();
    let mut theme_id = "alabaster".to_owned();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--sheet" {
            if let Some(v) = args.next() {
                sheet = v;
            }
        } else if arg == "--theme"
            && let Some(v) = args.next()
        {
            theme_id = v;
        }
    }

    Application::new().run(move |cx: &mut App| {
        let theme = resolve_theme_by_id(&theme_id);
        cx.set_global(ThemeGlobal::new(theme));
        let bounds = Bounds::centered(None, size(px(1200.0), px(800.0)), cx);
        let sheet_clone = sheet.clone();
        let theme_clone = theme_id.clone();
        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("REFRACTOR_PARITY_HARNESS".into()),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                ..Default::default()
            },
            move |_window, cx| {
                cx.new(|_cx| ParityView {
                    sheet: sheet_clone,
                    theme_id: theme_clone,
                })
            },
        );
    });
}
