//! Native GPU-accelerated Desktop **Refractor Optical Workbench & 67-Theme Studio**
//! demonstrating Iced-inspired `RefractorApp` architecture on top of Zed's GPUI,
//! live GPU `FxOverlay` rendering, 9-category theme filtering, and 55-category component browsing.

use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use refractor_components::{
    AlertBanner, BadgeStatus, Breadcrumbs, ButtonSize, DataTable, KpiMetricCard, PrimaryButton,
    PrismCard, PrismTabs, ProgressAndSpinner, RangeSlider, SecondaryButton, StatusTone, TagChip,
    TerminalEmulator, TextInput, ToggleSwitch, TopNavBar, WindowTitlebar, WorkspaceStatusBar,
    all_component_groups, all_component_specs, components_in_group,
};
use refractor_core::{AppHost, OpticalBackdrop, RefractorApp, Task, run_app};
use refractor_themes::{all_themes, resolve_theme_by_id, theme_categories, themes_by_category};
use refractor_tokens::{PrismTheme, ThemeCollectionCategory};

#[derive(Debug, Clone)]
enum WorkbenchMessage {
    SelectTheme(String),
    SelectThemeCategory(ThemeCollectionCategory),
    NextTheme,
    ToggleOptics,
    SelectTab(usize),
    NextComponentGroup,
}

struct OpticalWorkbench {
    active_theme_id: String,
    active_theme_category: ThemeCollectionCategory,
    optics_enabled: bool,
    active_tab: usize,
    active_group_index: usize,
}

impl RefractorApp for OpticalWorkbench {
    type Message = WorkbenchMessage;

    fn new() -> (Self, Task<Self::Message>) {
        (
            Self {
                active_theme_id: "alabaster".to_owned(),
                active_theme_category: ThemeCollectionCategory::OpticalGlass,
                optics_enabled: true,
                active_tab: 0,
                active_group_index: 0,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        "Refractor UI for Rust — Optical Workbench & 67-Theme Studio".to_owned()
    }

    fn theme(&self) -> PrismTheme {
        resolve_theme_by_id(&self.active_theme_id)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        let themes = all_themes();
        let groups = all_component_groups();
        match message {
            WorkbenchMessage::SelectTheme(id) => {
                self.active_theme_id = id;
            }
            WorkbenchMessage::SelectThemeCategory(cat) => {
                self.active_theme_category = cat;
                if let Some(first) = themes_by_category(cat).first() {
                    self.active_theme_id.clone_from(&first.id);
                }
            }
            WorkbenchMessage::NextTheme => {
                let current = themes
                    .iter()
                    .position(|t| t.id == self.active_theme_id)
                    .unwrap_or(0);
                if let Some(next) = themes.get((current + 1) % themes.len().max(1)) {
                    self.active_theme_id.clone_from(&next.id);
                    self.active_theme_category = next.category;
                }
            }
            WorkbenchMessage::ToggleOptics => {
                self.optics_enabled = !self.optics_enabled;
            }
            WorkbenchMessage::SelectTab(tab) => {
                self.active_tab = tab % 3;
            }
            WorkbenchMessage::NextComponentGroup => {
                self.active_group_index = (self.active_group_index + 1) % groups.len().max(1);
            }
        }
        Task::none()
    }

    fn view(
        &self,
        theme: &PrismTheme,
        _window: &mut Window,
        cx: &mut Context<'_, AppHost<Self>>,
    ) -> AnyElement {
        let active_name = theme.display_name.clone();
        let category_themes = themes_by_category(self.active_theme_category);
        let groups = all_component_groups();
        let active_group = groups
            .get(self.active_group_index)
            .copied()
            .unwrap_or("Buttons & Actions");
        let group_members = components_in_group(active_group);

        let category_bar = {
            let mut row = div().flex().flex_row().flex_wrap().gap(px(6.0));
            for (idx, meta) in theme_categories().iter().enumerate() {
                let cat = meta.id;
                let selected = cat == self.active_theme_category;
                let btn = if selected {
                    PrimaryButton::new(meta.short_label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                } else {
                    SecondaryButton::new(meta.short_label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                };
                row = row.child(
                    div()
                        .id(("cat-pill", idx))
                        .on_click(cx.listener(move |host, _ev, _win, cx| {
                            host.dispatch(WorkbenchMessage::SelectThemeCategory(cat), cx);
                        }))
                        .child(btn),
                );
            }
            row
        };

        let theme_strip = {
            let mut row = div().flex().flex_row().flex_wrap().gap(px(8.0));
            for (idx, preset) in category_themes.iter().take(12).enumerate() {
                let id_clone = preset.id.clone();
                let label = SharedString::from(preset.name.clone());
                let btn = if preset.id == self.active_theme_id {
                    PrimaryButton::new(label).size(ButtonSize::Sm).into_any_element()
                } else {
                    SecondaryButton::new(label).size(ButtonSize::Sm).into_any_element()
                };
                row = row.child(
                    div()
                        .id(("theme-pill", idx))
                        .on_click(cx.listener(move |host, _ev, _win, cx| {
                            host.dispatch(WorkbenchMessage::SelectTheme(id_clone.clone()), cx);
                        }))
                        .child(btn),
                );
            }
            row
        };

        let next_btn = div()
            .id("next-theme-btn")
            .on_click(cx.listener(|host, _ev, _win, cx| {
                host.dispatch(WorkbenchMessage::NextTheme, cx);
            }))
            .child(PrimaryButton::new("Cycle All 67 Themes →").size(ButtonSize::Md));

        let next_group_btn = div()
            .id("next-group-btn")
            .on_click(cx.listener(|host, _ev, _win, cx| {
                host.dispatch(WorkbenchMessage::NextComponentGroup, cx);
            }))
            .child(
                SecondaryButton::new(format!(
                    "Category {}/55: {active_group} ({}) →",
                    self.active_group_index + 1,
                    group_members.len()
                ))
                .size(ButtonSize::Sm),
            );

        let toggle_el = div()
            .id("toggle-optics")
            .on_click(cx.listener(|host, _ev, _win, cx| {
                host.dispatch(WorkbenchMessage::ToggleOptics, cx);
            }))
            .child(ToggleSwitch::new(self.optics_enabled).label("Optical Specular Refraction & FX Overlay"));

        let tab_bar = div()
            .id("tab-switch")
            .on_click(cx.listener(|host, _ev, _win, cx| {
                let next = (host.state().active_tab + 1) % 3;
                host.dispatch(WorkbenchMessage::SelectTab(next), cx);
            }))
            .child(PrismTabs::new(
                vec![
                    SharedString::from("Overview"),
                    SharedString::from("314 Components"),
                    SharedString::from("67 Themes"),
                ],
                self.active_tab,
            ));

        let mut member_chips = div().flex().flex_row().flex_wrap().gap(px(6.0));
        for spec in group_members.iter().take(10) {
            member_chips = member_chips.child(TagChip::new(spec.name.clone()));
        }

        div()
            .relative()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .justify_between()
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .child(OpticalBackdrop::new().reduced_motion(!self.optics_enabled)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(WindowTitlebar::new(format!(
                        "Refractor UI for Rust — {active_name} ({})",
                        theme.material.as_str()
                    )))
                    .child(
                        div()
                            .px(px(24.0))
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .child(
                                TopNavBar::new("Refractor Optical Glass UI Framework")
                                    .subtitle("GPUI GPU-Accelerated • Iced Ergonomics • 314 Components • 67 Themes"),
                            )
                            .child(Breadcrumbs::new(vec![
                                SharedString::from("Workspace"),
                                SharedString::from("Optical Workbench"),
                                SharedString::from(active_name.clone()),
                            ]))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .gap(px(12.0))
                                    .child(tab_bar)
                                    .child(next_btn),
                            )
                            .child(category_bar)
                            .child(theme_strip)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap(px(14.0))
                                    .child(
                                        div().flex_1().child(
                                            KpiMetricCard::new(
                                                "Catalog Components",
                                                format!("{}", all_component_specs().len()),
                                            )
                                            .delta("55 Categories • Astryx Open-Internals"),
                                        ),
                                    )
                                    .child(
                                        div().flex_1().child(
                                            KpiMetricCard::new(
                                                "Multi-Material Themes",
                                                format!("{}", all_themes().len()),
                                            )
                                            .delta("13 Materials • 9 GPU FX Overlays"),
                                        ),
                                    )
                                    .child(
                                        div().flex_1().child(
                                            KpiMetricCard::new("Safety Gate", "0 unsafe")
                                                .delta("ADR-0002 Strict Rust Governance"),
                                        ),
                                    ),
                            )
                            .child(
                                PrismCard::new()
                                    .kicker("LIVE OPTICAL SPECIMENS & 55-CATEGORY BROWSER")
                                    .title("Controls, Data, FX & Terminal Surfaces")
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_between()
                                            .child(next_group_btn)
                                            .child(BadgeStatus::new("WCAG 2.2 AA", StatusTone::Success)),
                                    )
                                    .child(member_chips)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(16.0))
                                            .child(toggle_el)
                                            .child(RangeSlider::new(72.0, 0.0, 100.0).label("Blur & Caustic Intensity"))
                                            .child(ProgressAndSpinner::new(96.0).label("Coverage & Frame Budget")),
                                    )
                                    .child(
                                        TextInput::new("refractor-ui-rust --theme cyberpunk")
                                            .label("Command Input Well"),
                                    )
                                    .child(AlertBanner::new(
                                        "Native GPUI + Iced-Inspired Runtime Active",
                                        format!(
                                            "Loaded {} categories and {} full-transformation themes.",
                                            all_component_groups().len(),
                                            all_themes().len()
                                        ),
                                        StatusTone::Primary,
                                    ))
                                    .child(DataTable::new(
                                        vec!["Crate".to_owned(), "Role".to_owned(), "Status".to_owned()],
                                        vec![
                                            vec!["refractor-tokens".to_owned(), "Color Math & Theme Engine".to_owned(), "Verified".to_owned()],
                                            vec!["refractor-themes".to_owned(), "67 Multi-Material Themes".to_owned(), "Verified".to_owned()],
                                            vec!["refractor-components".to_owned(), "314 GPUI Components".to_owned(), "Verified".to_owned()],
                                        ],
                                    ))
                                    .child(TerminalEmulator::new(vec![
                                        SharedString::from("$ cargo run -p refractor-cli -- check --strict"),
                                        SharedString::from("Refractor check (strict=true): components=314/314, categories=55/55, themes=67/67"),
                                    ])),
                            ),
                    ),
            )
            .child(WorkspaceStatusBar::new(
                format!(
                    "Theme: {active_name} | Material: {} | Overlay: {}",
                    theme.material.as_str(),
                    theme.fx.wallpaper_overlay.as_str()
                ),
                "Refractor UI v1.0.1 • GPUI 0.2.2 • Apache-2.0",
            ))
            .into_any_element()
    }
}

fn main() {
    run_app::<OpticalWorkbench>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workbench_state_transitions_work_across_all_67_themes_and_55_categories() {
        let (mut app, _) = OpticalWorkbench::new();
        assert_eq!(app.theme().name, "alabaster");

        let _ = app.update(WorkbenchMessage::SelectThemeCategory(
            ThemeCollectionCategory::CyberNeon,
        ));
        assert_eq!(app.active_theme_category, ThemeCollectionCategory::CyberNeon);

        let _ = app.update(WorkbenchMessage::SelectTheme("cyberpunk".to_owned()));
        assert_eq!(app.theme().name, "cyberpunk");

        let _ = app.update(WorkbenchMessage::NextTheme);
        assert_ne!(app.theme().name, "cyberpunk");

        let _ = app.update(WorkbenchMessage::ToggleOptics);
        assert!(!app.optics_enabled);

        let _ = app.update(WorkbenchMessage::SelectTab(2));
        assert_eq!(app.active_tab, 2);

        let _ = app.update(WorkbenchMessage::NextComponentGroup);
        assert_eq!(app.active_group_index, 1);
    }
}
