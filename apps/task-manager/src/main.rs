//! Native GPUI Optical Glass Task Manager built with **Refractor UI for Rust**.
//!
//! Demonstrates full interactive desktop state management, keyboard text entry,
//! filtering, priority management, live progress tracking, multi-material optical
//! glass rendering (13 materials & 9 GPU FX overlays), and an interactive **Visual FX
//! & Aesthetic Studio** drawer for live-tuning refraction, specular highlights,
//! corner geometry, material archetypes, and overlay FX.

use gpui::{
    App, AppContext, Application, Bounds, Context, FocusHandle, FontWeight, InteractiveElement,
    IntoElement, KeyDownEvent, ParentElement, Render, SharedString, StatefulInteractiveElement,
    Styled, TitlebarOptions, Window, WindowBounds, WindowOptions, div, px, size,
};
use refractor_components::{
    BadgeStatus, ButtonSize, Checkbox, EmptyState, KpiMetricCard, MenuBarTrigger, MenuDropdownItem,
    PrimaryButton, PrismCard, ProgressAndSpinner, RangeSlider, SecondaryButton, StatusTone,
    TagChip, TextInput, ToggleSwitch, TopNavBar, WorkspaceStatusBar,
};
use refractor_core::{
    KeyAction, KeyboardEvent, OpticalBackdrop, SurfaceVariant, clamp_overlay_opacity,
    color_str_to_hsla, init_theme, optical_surface, set_theme, should_handle_key,
};
use refractor_security::{build_safe_csv, clamp_f32};
use refractor_themes::resolve_theme_by_id;
use refractor_tokens::{
    PrismTheme, RefractionIntensity, RgbColor, ThemeMaterial, ThemeOverlayFx,
};

/// Priority level for a task item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    const fn label(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }

    const fn tone(self) -> StatusTone {
        match self {
            Self::Low => StatusTone::Neutral,
            Self::Medium => StatusTone::Warning,
            Self::High => StatusTone::Error,
        }
    }

    const fn next(self) -> Self {
        match self {
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High => Self::Low,
        }
    }
}

/// Category tag for organizing tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Work,
    Engineering,
    Design,
    Personal,
}

impl Category {
    const ALL: [Self; 4] = [Self::Work, Self::Engineering, Self::Design, Self::Personal];

    const fn label(self) -> &'static str {
        match self {
            Self::Work => "Work",
            Self::Engineering => "Engineering",
            Self::Design => "Design",
            Self::Personal => "Personal",
        }
    }
}

/// Active task list filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TaskFilter {
    All,
    Active,
    Completed,
    HighPriority,
}

impl TaskFilter {
    const ALL: [Self; 4] = [Self::All, Self::Active, Self::Completed, Self::HighPriority];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Active => "Active",
            Self::Completed => "Completed",
            Self::HighPriority => "High Priority",
        }
    }
}

/// Live visual FX & aesthetic overrides matching `glass-ui-v1`'s `ThemeCustomizerDrawer`.
#[derive(Debug, Clone)]
struct AestheticOverrides {
    refraction: RefractionIntensity,
    specular_highlight: bool,
    material_override: Option<ThemeMaterial>,
    overlay_override: Option<ThemeOverlayFx>,
    overlay_opacity: f32,
    slab_radius_override: Option<f32>,
    fx_studio_open: bool,
}

impl Default for AestheticOverrides {
    fn default() -> Self {
        Self {
            refraction: RefractionIntensity::Standard,
            specular_highlight: true,
            material_override: None,
            overlay_override: None,
            overlay_opacity: 0.16,
            slab_radius_override: None,
            fx_studio_open: true,
        }
    }
}

/// Individual task item stored in application state.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskItem {
    id: u64,
    title: String,
    completed: bool,
    priority: Priority,
    category: Category,
}

/// Top-level horizontal menu bar sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopMenu {
    File,
    Edit,
    View,
    Theme,
    Help,
}

impl TopMenu {
    const ALL: [Self; 5] = [Self::File, Self::Edit, Self::View, Self::Theme, Self::Help];

    const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Theme => "Theme",
            Self::Help => "Help",
        }
    }

    const fn left_offset_px(self) -> f32 {
        match self {
            Self::File => 44.0,
            Self::Edit => 92.0,
            Self::View => 142.0,
            Self::Theme => 196.0,
            Self::Help => 262.0,
        }
    }
}

/// Pure, testable state for the Refractor Task Manager.
#[derive(Debug, Clone)]
struct TaskManagerState {
    tasks: Vec<TaskItem>,
    next_id: u64,
    draft_title: String,
    draft_priority: Priority,
    draft_category: Category,
    filter: TaskFilter,
    theme_id: String,
    optics_enabled: bool,
    aesthetics: AestheticOverrides,
    open_menu: Option<TopMenu>,
    status_message: String,
}

impl TaskManagerState {
    fn new() -> Self {
        Self {
            tasks: vec![
                TaskItem {
                    id: 1,
                    title: "Review optical glass specular arcs, caustic blooms & 13 materials"
                        .to_owned(),
                    completed: false,
                    priority: Priority::High,
                    category: Category::Design,
                },
                TaskItem {
                    id: 2,
                    title: "Verify zero-unsafe & strict clippy governance checks".to_owned(),
                    completed: true,
                    priority: Priority::High,
                    category: Category::Engineering,
                },
                TaskItem {
                    id: 3,
                    title: "Prepare sprint demo checklist for desktop UI release".to_owned(),
                    completed: false,
                    priority: Priority::Medium,
                    category: Category::Work,
                },
                TaskItem {
                    id: 4,
                    title: "Plan weekend hike and pack camera gear".to_owned(),
                    completed: false,
                    priority: Priority::Low,
                    category: Category::Personal,
                },
            ],
            next_id: 5,
            draft_title: String::new(),
            draft_priority: Priority::Medium,
            draft_category: Category::Work,
            filter: TaskFilter::All,
            theme_id: "alabaster".to_owned(),
            optics_enabled: true,
            aesthetics: AestheticOverrides::default(),
            open_menu: None,
            status_message: "Ready • Type a task title and press Enter to add".to_owned(),
        }
    }

    fn theme(&self) -> PrismTheme {
        let mut theme = resolve_theme_by_id(&self.theme_id);
        theme.optics.refraction = self.aesthetics.refraction;
        theme.optics.specular_highlight = self.aesthetics.specular_highlight;
        if let Some(mat) = self.aesthetics.material_override {
            theme.material = mat;
            theme.fx.sharp_corners = mat.is_sharp();
        }
        if let Some(ov) = self.aesthetics.overlay_override {
            theme.fx.wallpaper_overlay = ov;
            theme.fx.overlay_opacity = clamp_overlay_opacity(self.aesthetics.overlay_opacity);
        }
        if let Some(radius) = self.aesthetics.slab_radius_override {
            theme.shape.slab_radius_px = radius;
            theme.shape.border_radius_px = (radius * 0.66).clamp(0.0, 24.0);
            theme.fx.sharp_corners = radius <= 0.5;
        }
        theme
    }

    fn push_draft_char(&mut self, text: &str) {
        if self.draft_title.chars().count() < 120 {
            self.draft_title.push_str(text);
        }
    }

    fn pop_draft_char(&mut self) {
        let _ = self.draft_title.pop();
    }

    fn clear_draft(&mut self) {
        self.draft_title.clear();
        "Cleared draft input".clone_into(&mut self.status_message);
    }

    fn add_draft_task(&mut self) -> bool {
        let trimmed = self.draft_title.trim();
        if trimmed.is_empty() {
            "Enter a task title before adding".clone_into(&mut self.status_message);
            return false;
        }
        let title = trimmed.to_owned();
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        self.tasks.insert(
            0,
            TaskItem {
                id,
                title: title.clone(),
                completed: false,
                priority: self.draft_priority,
                category: self.draft_category,
            },
        );
        self.draft_title.clear();
        self.status_message = format!("Added task: {title}");
        true
    }

    fn toggle_task(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = !task.completed;
            self.status_message = if task.completed {
                format!("Completed: {}", task.title)
            } else {
                format!("Reopened: {}", task.title)
            };
        }
    }

    fn cycle_task_priority(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.priority = task.priority.next();
            self.status_message = format!(
                "Priority set to {} for: {}",
                task.priority.label(),
                task.title
            );
        }
    }

    fn delete_task(&mut self, id: u64) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            let removed = self.tasks.remove(pos);
            self.status_message = format!("Deleted task: {}", removed.title);
        }
    }

    fn clear_completed(&mut self) {
        let before = self.tasks.len();
        self.tasks.retain(|t| !t.completed);
        let removed = before.saturating_sub(self.tasks.len());
        self.status_message = if removed == 0 {
            "No completed tasks to clear".to_owned()
        } else {
            format!("Cleared {removed} completed task(s)")
        };
    }

    fn mark_all_completed(&mut self) {
        for task in &mut self.tasks {
            task.completed = true;
        }
        self.status_message = format!("Marked all {} task(s) completed", self.tasks.len());
    }

    fn reopen_all_tasks(&mut self) {
        for task in &mut self.tasks {
            task.completed = false;
        }
        self.status_message = format!("Reopened all {} task(s)", self.tasks.len());
    }

    fn reset_sample_tasks(&mut self) {
        let fresh = Self::new();
        self.tasks = fresh.tasks;
        self.next_id = fresh.next_id;
        self.filter = TaskFilter::All;
        "Restored 4 starter sample tasks".clone_into(&mut self.status_message);
    }

    fn export_tasks_csv(&mut self) -> String {
        let headers = vec![
            "ID".to_owned(),
            "Title".to_owned(),
            "Status".to_owned(),
            "Priority".to_owned(),
            "Category".to_owned(),
        ];
        let rows: Vec<Vec<String>> = self
            .tasks
            .iter()
            .map(|t| {
                vec![
                    t.id.to_string(),
                    t.title.clone(),
                    if t.completed { "Done" } else { "Active" }.to_owned(),
                    t.priority.label().to_owned(),
                    t.category.label().to_owned(),
                ]
            })
            .collect();
        let csv = build_safe_csv(&headers, &rows);
        self.status_message = format!(
            "Generated formula-safe CSV ({} rows, {} bytes)",
            rows.len(),
            csv.len()
        );
        csv
    }

    fn toggle_menu(&mut self, menu: TopMenu) {
        self.open_menu = if self.open_menu == Some(menu) {
            None
        } else {
            Some(menu)
        };
    }

    fn close_menu(&mut self) {
        self.open_menu = None;
    }

    fn select_theme(&mut self, id: &str) {
        id.clone_into(&mut self.theme_id);
        // Reset explicit material/overlay overrides when picking a new preset so its native physics shine
        self.aesthetics.material_override = None;
        self.aesthetics.overlay_override = None;
        self.aesthetics.slab_radius_override = None;
        let theme = self.theme();
        self.status_message = format!(
            "Theme: {} ({} • {})",
            theme.display_name,
            theme.material.as_str(),
            theme.fx.wallpaper_overlay.as_str()
        );
    }

    fn cycle_material(&mut self) {
        let current = self.theme().material;
        let idx = ThemeMaterial::ALL
            .iter()
            .position(|m| *m == current)
            .unwrap_or(0);
        let next = ThemeMaterial::ALL[(idx + 1) % ThemeMaterial::ALL.len()];
        self.aesthetics.material_override = Some(next);
        self.status_message = format!("Material archetype: {}", next.as_str());
    }

    fn cycle_overlay(&mut self) {
        let current = self.theme().fx.wallpaper_overlay;
        let idx = ThemeOverlayFx::ALL
            .iter()
            .position(|o| *o == current)
            .unwrap_or(0);
        let next = ThemeOverlayFx::ALL[(idx + 1) % ThemeOverlayFx::ALL.len()];
        self.aesthetics.overlay_override = Some(next);
        self.aesthetics.overlay_opacity = if next == ThemeOverlayFx::None {
            0.0
        } else {
            next.default_opacity().max(0.14)
        };
        self.status_message = format!("Wallpaper FX overlay: {}", next.as_str());
    }

    fn adjust_overlay_opacity(&mut self, delta: f32) {
        let next = clamp_overlay_opacity(self.aesthetics.overlay_opacity + delta);
        self.aesthetics.overlay_opacity = next;
        if self.aesthetics.overlay_override.is_none() {
            let current = self.theme().fx.wallpaper_overlay;
            self.aesthetics.overlay_override = Some(if current == ThemeOverlayFx::None {
                ThemeOverlayFx::CyberGrid
            } else {
                current
            });
        }
        self.status_message = format!("FX overlay opacity: {:.0}%", next * 100.0);
    }

    fn visible_tasks(&self) -> Vec<&TaskItem> {
        self.tasks
            .iter()
            .filter(|task| match self.filter {
                TaskFilter::All => true,
                TaskFilter::Active => !task.completed,
                TaskFilter::Completed => task.completed,
                TaskFilter::HighPriority => task.priority == Priority::High && !task.completed,
            })
            .collect()
    }

    fn count_for_filter(&self, filter: TaskFilter) -> usize {
        self.tasks
            .iter()
            .filter(|task| match filter {
                TaskFilter::All => true,
                TaskFilter::Active => !task.completed,
                TaskFilter::Completed => task.completed,
                TaskFilter::HighPriority => task.priority == Priority::High && !task.completed,
            })
            .count()
    }

    fn completed_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.completed).count()
    }

    fn active_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.completed).count()
    }

    fn high_priority_open_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|t| !t.completed && t.priority == Priority::High)
            .count()
    }

    fn completion_pct(&self) -> f32 {
        if self.tasks.is_empty() {
            return 0.0;
        }
        let pct = (self.completed_count() as f32 / self.tasks.len() as f32) * 100.0;
        clamp_f32(pct, 0.0, 100.0, 0.0)
    }
}

/// GPUI root view for the Refractor Task Manager desktop window.
struct TaskManagerApp {
    state: TaskManagerState,
    focus_handle: FocusHandle,
    focused_once: bool,
}

impl TaskManagerApp {
    fn new(cx: &mut Context<'_, Self>) -> Self {
        Self {
            state: TaskManagerState::new(),
            focus_handle: cx.focus_handle(),
            focused_once: false,
        }
    }

    fn sync_theme(&self, cx: &mut Context<'_, Self>) {
        set_theme(cx, self.state.theme());
        cx.notify();
    }

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<'_, Self>,
    ) {
        let ks = &event.keystroke;
        if ks.modifiers.control || ks.modifiers.platform || ks.modifiers.alt {
            return;
        }

        match ks.key.as_str() {
            "enter" => {
                if should_handle_key(KeyboardEvent::new(KeyAction::Enter)) {
                    let _ = self.state.add_draft_task();
                    cx.notify();
                }
            }
            "backspace" => {
                if should_handle_key(KeyboardEvent::new(KeyAction::Backspace)) {
                    self.state.pop_draft_char();
                    cx.notify();
                }
            }
            "escape" => {
                if should_handle_key(KeyboardEvent::new(KeyAction::Escape)) {
                    if self.state.open_menu.is_some() {
                        self.state.close_menu();
                    } else {
                        self.state.clear_draft();
                    }
                    cx.notify();
                }
            }
            "space" => {
                self.state.push_draft_char(" ");
                cx.notify();
            }
            _ => {
                if let Some(ref ch) = ks.key_char
                    && !ch.is_empty()
                    && !ch.chars().any(char::is_control)
                {
                    self.state.push_draft_char(ch);
                    cx.notify();
                }
            }
        }
    }
    fn render_menu_bar(&self, theme: &PrismTheme, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let fg_muted =
            color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let contrast = color_str_to_hsla(
            &theme.palette.primary.contrast_text,
            RgbColor::new(255, 255, 255),
        );

        let app_badge = optical_surface(theme, SurfaceVariant::PrimaryButton)
            .w(px(22.0))
            .h(px(22.0))
            .rounded(px(6.0))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(12.0))
            .font_weight(FontWeight::BOLD)
            .text_color(contrast)
            .child("✓");

        let mut left_group = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(div().pr(px(6.0)).child(app_badge));

        for (idx, menu) in TopMenu::ALL.iter().enumerate() {
            let active = self.state.open_menu == Some(*menu);
            let target = *menu;
            left_group = left_group.child(
                div()
                    .id(("top-menu-trigger", idx))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.toggle_menu(target);
                        cx.notify();
                    }))
                    .child(MenuBarTrigger::new(menu.label(), active)),
            );
        }

        optical_surface(theme, SurfaceVariant::Chrome)
            .h(px(36.0))
            .px(px(12.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(left_group)
            .child(
                div()
                    .text_size(px(11.5))
                    .text_color(fg_muted)
                    .child(format!(
                        "{} • {} • {}",
                        theme.display_name,
                        theme.material.as_str(),
                        theme.fx.wallpaper_overlay.as_str()
                    )),
            )
    }

    fn render_open_menu_overlay(
        &self,
        menu: TopMenu,
        theme: &PrismTheme,
        cx: &mut Context<'_, Self>,
    ) -> impl IntoElement {
        let divider_col =
            color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));
        let sep = || div().w_full().h(px(1.0)).my(px(4.0)).bg(divider_col);

        let mut panel = optical_surface(theme, SurfaceVariant::Slab)
            .w(px(236.0))
            .p(px(6.0))
            .flex()
            .flex_col()
            .gap(px(2.0));

        match menu {
            TopMenu::File => {
                panel = panel
                    .child(
                        div()
                            .id("menu-file-new")
                            .on_click(cx.listener(|this, _ev, win, cx| {
                                this.state.close_menu();
                                win.focus(&this.focus_handle);
                                "Type a task title and press Enter to save"
                                    .clone_into(&mut this.state.status_message);
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("New Task").shortcut("Enter")),
                    )
                    .child(
                        div()
                            .id("menu-file-export")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                let _ = this.state.export_tasks_csv();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Export Safe CSV").shortcut("OWASP")),
                    )
                    .child(sep())
                    .child(
                        div()
                            .id("menu-file-clear-done")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.clear_completed();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Clear Completed Tasks")),
                    )
                    .child(
                        div()
                            .id("menu-file-reset")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.reset_sample_tasks();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Reset Sample Tasks")),
                    );
            }
            TopMenu::Edit => {
                panel = panel
                    .child(
                        div()
                            .id("menu-edit-complete-all")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.mark_all_completed();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Mark All Completed")),
                    )
                    .child(
                        div()
                            .id("menu-edit-reopen-all")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.reopen_all_tasks();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Reopen All Tasks")),
                    )
                    .child(sep())
                    .child(
                        div()
                            .id("menu-edit-clear-draft")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.clear_draft();
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Clear Draft Input").shortcut("Esc")),
                    );
            }
            TopMenu::View => {
                for (idx, filter) in TaskFilter::ALL.iter().enumerate() {
                    let target = *filter;
                    let checked = self.state.filter == *filter;
                    panel = panel.child(
                        div()
                            .id(("menu-view-filter", idx))
                            .on_click(cx.listener(move |this, _ev, _win, cx| {
                                this.state.filter = target;
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new(filter.label()).checked(checked)),
                    );
                }
                panel = panel
                    .child(sep())
                    .child(
                        div()
                            .id("menu-view-fx-studio")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.aesthetics.fx_studio_open =
                                    !this.state.aesthetics.fx_studio_open;
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(
                                MenuDropdownItem::new("Visual FX Studio")
                                    .checked(self.state.aesthetics.fx_studio_open),
                            ),
                    )
                    .child(
                        div()
                            .id("menu-view-specular")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.aesthetics.specular_highlight =
                                    !this.state.aesthetics.specular_highlight;
                                this.state.close_menu();
                                this.sync_theme(cx);
                            }))
                            .child(
                                MenuDropdownItem::new("Specular Highlights")
                                    .checked(self.state.aesthetics.specular_highlight),
                            ),
                    )
                    .child(
                        div()
                            .id("menu-view-caustics")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.optics_enabled = !this.state.optics_enabled;
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(
                                MenuDropdownItem::new("Ambient Caustic Blooms")
                                    .checked(self.state.optics_enabled),
                            ),
                    );
            }
            TopMenu::Theme => {
                let presets: [(&str, &str); 7] = [
                    ("alabaster", "Alabaster Light"),
                    ("obsidian", "Obsidian Dark"),
                    ("amethyst", "Amethyst Vivid"),
                    ("windows95", "Windows 95 Bevel"),
                    ("neumorphism", "Neumorphism"),
                    ("tokyo-night", "Tokyo Night IDE"),
                    ("cyberpunk", "Cyberpunk Neon"),
                ];
                for (idx, (id, label)) in presets.iter().enumerate() {
                    let target = *id;
                    let checked = self.state.theme_id == *id;
                    panel = panel.child(
                        div()
                            .id(("menu-theme-preset", idx))
                            .on_click(cx.listener(move |this, _ev, _win, cx| {
                                this.state.select_theme(target);
                                this.state.close_menu();
                                this.sync_theme(cx);
                            }))
                            .child(MenuDropdownItem::new(*label).checked(checked)),
                    );
                }
                panel = panel
                    .child(sep())
                    .child(
                        div()
                            .id("menu-theme-material")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.cycle_material();
                                this.state.close_menu();
                                this.sync_theme(cx);
                            }))
                            .child(MenuDropdownItem::new("Cycle Material (13)")),
                    )
                    .child(
                        div()
                            .id("menu-theme-overlay")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                this.state.cycle_overlay();
                                this.state.close_menu();
                                this.sync_theme(cx);
                            }))
                            .child(MenuDropdownItem::new("Cycle FX Overlay (9)")),
                    );
            }
            TopMenu::Help => {
                panel = panel
                    .child(
                        div()
                            .id("menu-help-shortcuts")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                "Shortcuts: Type anywhere to compose • Enter = Add • Esc = Close/Clear"
                                    .clone_into(&mut this.state.status_message);
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("Keyboard Shortcuts")),
                    )
                    .child(
                        div()
                            .id("menu-help-about")
                            .on_click(cx.listener(|this, _ev, _win, cx| {
                                "Refractor Tasks v1.0.1 • Native GPUI + 13 Materials + 9 GPU FX"
                                    .clone_into(&mut this.state.status_message);
                                this.state.close_menu();
                                cx.notify();
                            }))
                            .child(MenuDropdownItem::new("About Refractor Tasks")),
                    );
            }
        }

        div()
            .id("menu-dismiss-scrim")
            .absolute()
            .top(px(36.0))
            .left_0()
            .right_0()
            .bottom_0()
            .on_click(cx.listener(|this, _ev, _win, cx| {
                this.state.close_menu();
                cx.notify();
            }))
            .child(
                div()
                    .absolute()
                    .top(px(4.0))
                    .left(px(menu.left_offset_px()))
                    .child(panel),
            )
    }
}

impl Render for TaskManagerApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        if !self.focused_once {
            window.focus(&self.focus_handle);
            self.focused_once = true;
        }

        let theme = self.state.theme();
        let text_primary =
            color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
        let text_muted =
            color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
        let is_focused = self.focus_handle.is_focused(window);

        let theme_presets: [(&str, &str); 7] = [
            ("alabaster", "Alabaster"),
            ("obsidian", "Obsidian"),
            ("amethyst", "Amethyst"),
            ("windows95", "Win95 Bevel"),
            ("neumorphism", "Neumorphic"),
            ("tokyo-night", "Tokyo Night"),
            ("cyberpunk", "Cyberpunk"),
        ];

        let mut theme_row = div().flex().flex_row().flex_wrap().items_center().gap(px(6.0));
        for (idx, (id, label)) in theme_presets.iter().enumerate() {
            let active = self.state.theme_id == *id;
            let btn = if active {
                PrimaryButton::new(*label)
                    .size(ButtonSize::Sm)
                    .into_any_element()
            } else {
                SecondaryButton::new(*label)
                    .size(ButtonSize::Sm)
                    .into_any_element()
            };
            let target = *id;
            theme_row = theme_row.child(
                div()
                    .id(("theme-preset", idx))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.select_theme(target);
                        this.sync_theme(cx);
                    }))
                    .child(btn),
            );
        }

        let fx_studio_toggle = div()
            .id("toggle-fx-studio")
            .on_click(cx.listener(|this, _ev, _win, cx| {
                this.state.aesthetics.fx_studio_open = !this.state.aesthetics.fx_studio_open;
                cx.notify();
            }))
            .child(
                SecondaryButton::new(if self.state.aesthetics.fx_studio_open {
                    "Hide FX Studio ▴"
                } else {
                    "Visual FX Studio ▾"
                })
                .cobalt(self.state.aesthetics.fx_studio_open)
                .size(ButtonSize::Sm),
            );

        // Live Visual FX & Aesthetic Controls panel (mirrors glass-ui-v1 ThemeCustomizerDrawer)
        let fx_studio_panel = if self.state.aesthetics.fx_studio_open {
            let mut refraction_row = div().flex().flex_row().items_center().gap(px(6.0)).child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_muted)
                    .child("Refraction:"),
            );
            for (idx, (level, label)) in [
                (RefractionIntensity::Subtle, "Subtle"),
                (RefractionIntensity::Standard, "Standard"),
                (RefractionIntensity::Vivid, "Vivid"),
            ]
            .iter()
            .enumerate()
            {
                let selected = self.state.aesthetics.refraction == *level;
                let btn = if selected {
                    PrimaryButton::new(*label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                } else {
                    SecondaryButton::new(*label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                };
                let target_level = *level;
                refraction_row = refraction_row.child(
                    div()
                        .id(("refraction-lvl", idx))
                        .on_click(cx.listener(move |this, _ev, _win, cx| {
                            this.state.aesthetics.refraction = target_level;
                            this.state.status_message = format!("Refraction set to {label}");
                            this.sync_theme(cx);
                        }))
                        .child(btn),
                );
            }

            let mut radius_row = div().flex().flex_row().items_center().gap(px(6.0)).child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_muted)
                    .child("Corners:"),
            );
            for (idx, (r, label)) in [
                (0.0_f32, "0px"),
                (12.0_f32, "12px"),
                (24.0_f32, "24px"),
                (32.0_f32, "32px"),
            ]
            .iter()
            .enumerate()
            {
                let selected = (theme.shape.slab_radius_px - *r).abs() < 1.0;
                let btn = if selected {
                    PrimaryButton::new(*label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                } else {
                    SecondaryButton::new(*label)
                        .size(ButtonSize::Sm)
                        .into_any_element()
                };
                let target_r = *r;
                radius_row = radius_row.child(
                    div()
                        .id(("radius-opt", idx))
                        .on_click(cx.listener(move |this, _ev, _win, cx| {
                            this.state.aesthetics.slab_radius_override = Some(target_r);
                            this.state.status_message = format!("Slab radius set to {label}");
                            this.sync_theme(cx);
                        }))
                        .child(btn),
                );
            }

            let specular_toggle = div()
                .id("toggle-specular")
                .cursor_pointer()
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.aesthetics.specular_highlight =
                        !this.state.aesthetics.specular_highlight;
                    this.state.status_message = if this.state.aesthetics.specular_highlight {
                        "Specular glass arcs enabled".to_owned()
                    } else {
                        "Specular glass arcs disabled".to_owned()
                    };
                    this.sync_theme(cx);
                }))
                .child(ToggleSwitch::new(self.state.aesthetics.specular_highlight).label("Specular"));

            let ambient_toggle = div()
                .id("toggle-ambient-fx")
                .cursor_pointer()
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.optics_enabled = !this.state.optics_enabled;
                    cx.notify();
                }))
                .child(ToggleSwitch::new(self.state.optics_enabled).label("Caustic Blooms"));

            let material_btn = div()
                .id("cycle-material-btn")
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.cycle_material();
                    this.sync_theme(cx);
                }))
                .child(
                    SecondaryButton::new(format!(
                        "Material: {} →",
                        theme.material.as_str()
                    ))
                    .size(ButtonSize::Sm),
                );

            let overlay_btn = div()
                .id("cycle-overlay-btn")
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.cycle_overlay();
                    this.sync_theme(cx);
                }))
                .child(
                    SecondaryButton::new(format!(
                        "FX Overlay: {} →",
                        theme.fx.wallpaper_overlay.as_str()
                    ))
                    .size(ButtonSize::Sm),
                );

            let op_minus = div()
                .id("fx-op-minus")
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.adjust_overlay_opacity(-0.05);
                    this.sync_theme(cx);
                }))
                .child(SecondaryButton::new("−").size(ButtonSize::Sm));

            let op_plus = div()
                .id("fx-op-plus")
                .on_click(cx.listener(|this, _ev, _win, cx| {
                    this.state.adjust_overlay_opacity(0.05);
                    this.sync_theme(cx);
                }))
                .child(SecondaryButton::new("+").size(ButtonSize::Sm));

            let opacity_pct = theme.fx.overlay_opacity * 100.0;

            Some(
                PrismCard::new()
                    .kicker("OPTICAL GLASS VISUAL FX & AESTHETIC CONTROLS")
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .justify_between()
                            .gap(px(10.0))
                            .child(refraction_row)
                            .child(radius_row)
                            .child(specular_toggle)
                            .child(ambient_toggle),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .items_center()
                            .justify_between()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(material_btn)
                                    .child(overlay_btn),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(op_minus)
                                    .child(
                                        RangeSlider::new(opacity_pct, 0.0, 40.0).label(format!(
                                            "Overlay Intensity ({opacity_pct:.0}%)"
                                        )),
                                    )
                                    .child(op_plus),
                            ),
                    ),
            )
        } else {
            None
        };

        // Priority selector pills for new task composer
        let mut priority_row = div().flex().flex_row().items_center().gap(px(6.0)).child(
            div()
                .text_size(px(12.0))
                .text_color(text_muted)
                .child("Priority:"),
        );
        for (idx, prio) in [Priority::Low, Priority::Medium, Priority::High]
            .iter()
            .enumerate()
        {
            let selected = self.state.draft_priority == *prio;
            let btn = if selected {
                PrimaryButton::new(prio.label())
                    .size(ButtonSize::Sm)
                    .into_any_element()
            } else {
                SecondaryButton::new(prio.label())
                    .size(ButtonSize::Sm)
                    .into_any_element()
            };
            let target_prio = *prio;
            priority_row = priority_row.child(
                div()
                    .id(("draft-prio", idx))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.draft_priority = target_prio;
                        cx.notify();
                    }))
                    .child(btn),
            );
        }

        // Category selector pills for new task composer
        let mut category_row = div().flex().flex_row().items_center().gap(px(6.0)).child(
            div()
                .text_size(px(12.0))
                .text_color(text_muted)
                .child("Category:"),
        );
        for (idx, cat) in Category::ALL.iter().enumerate() {
            let selected = self.state.draft_category == *cat;
            let btn = if selected {
                PrimaryButton::new(cat.label())
                    .size(ButtonSize::Sm)
                    .into_any_element()
            } else {
                SecondaryButton::new(cat.label())
                    .size(ButtonSize::Sm)
                    .into_any_element()
            };
            let target_cat = *cat;
            category_row = category_row.child(
                div()
                    .id(("draft-cat", idx))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.draft_category = target_cat;
                        cx.notify();
                    }))
                    .child(btn),
            );
        }

        let input_display = if self.state.draft_title.is_empty() {
            String::new()
        } else if is_focused {
            format!("{}|", self.state.draft_title)
        } else {
            self.state.draft_title.clone()
        };

        let placeholder = if is_focused {
            "Type a new task and press Enter..."
        } else {
            "Click here to type a new task..."
        };

        let draft_input = div()
            .id("task-draft-input")
            .flex_1()
            .cursor_text()
            .on_click(cx.listener(|this, _ev, win, cx| {
                win.focus(&this.focus_handle);
                cx.notify();
            }))
            .child(TextInput::new(input_display).placeholder(placeholder));

        let add_btn = div()
            .id("add-task-btn")
            .on_click(cx.listener(|this, _ev, win, cx| {
                let _ = this.state.add_draft_task();
                win.focus(&this.focus_handle);
                cx.notify();
            }))
            .child(PrimaryButton::new("+ Add Task").size(ButtonSize::Md));

        // Filter bar
        let mut filter_row = div().flex().flex_row().items_center().gap(px(8.0));
        for (idx, filter) in TaskFilter::ALL.iter().enumerate() {
            let count = self.state.count_for_filter(*filter);
            let label = format!("{} ({count})", filter.label());
            let selected = self.state.filter == *filter;
            let btn = if selected {
                PrimaryButton::new(label)
                    .size(ButtonSize::Sm)
                    .into_any_element()
            } else {
                SecondaryButton::new(label)
                    .size(ButtonSize::Sm)
                    .into_any_element()
            };
            let target_filter = *filter;
            filter_row = filter_row.child(
                div()
                    .id(("filter-tab", idx))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.filter = target_filter;
                        cx.notify();
                    }))
                    .child(btn),
            );
        }

        let completed_count = self.state.completed_count();
        let clear_completed_btn = div()
            .id("clear-completed-btn")
            .on_click(cx.listener(|this, _ev, _win, cx| {
                this.state.clear_completed();
                cx.notify();
            }))
            .child(
                SecondaryButton::new(format!("Clear Completed ({completed_count})"))
                    .size(ButtonSize::Sm),
            );

        // Task list rows
        let visible = self.state.visible_tasks();
        let task_list_content = if visible.is_empty() {
            div()
                .pt(px(8.0))
                .child(EmptyState::new(
                    "No tasks in this view",
                    "Type a new task above and press Enter, or switch filters to view other items.",
                ))
                .into_any_element()
        } else {
            let mut list = div().flex().flex_col().gap(px(8.0));
            for task in visible {
                let task_id = task.id;
                let title_text = if task.completed {
                    format!("{}  (Done)", task.title)
                } else {
                    task.title.clone()
                };
                let checkbox_row = div()
                    .id(("task-toggle", task_id))
                    .flex_1()
                    .min_w_0()
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.toggle_task(task_id);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .opacity(if task.completed { 0.6 } else { 1.0 })
                            .child(Checkbox::new(task.completed, SharedString::from(title_text))),
                    );

                let priority_badge = div()
                    .id(("task-prio", task_id))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.cycle_task_priority(task_id);
                        cx.notify();
                    }))
                    .child(BadgeStatus::new(task.priority.label(), task.priority.tone()));

                let delete_btn = div()
                    .id(("task-delete", task_id))
                    .on_click(cx.listener(move |this, _ev, _win, cx| {
                        this.state.delete_task(task_id);
                        cx.notify();
                    }))
                    .child(SecondaryButton::new("Delete").size(ButtonSize::Sm));

                let row = optical_surface(&theme, SurfaceVariant::Crystal)
                    .px(px(14.0))
                    .py(px(10.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(12.0))
                    .child(checkbox_row)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.0))
                            .child(TagChip::new(task.category.label()))
                            .child(priority_badge)
                            .child(delete_btn),
                    );

                list = list.child(row);
            }
            list.into_any_element()
        };

        let completion_pct = self.state.completion_pct();
        let total_tasks = self.state.tasks.len();
        let active_tasks = self.state.active_count();
        let high_open = self.state.high_priority_open_count();

        let mut content_col = div()
            .flex_1()
            .min_h_0()
            .px(px(22.0))
            .pb(px(10.0))
            .flex()
            .flex_col()
            .gap(px(12.0))
            .child(
                TopNavBar::new("Refractor Task Manager").subtitle(
                    "Optical Glass Productivity • 13 Material Archetypes • 9 GPU FX Overlays",
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(10.0))
                    .child(theme_row)
                    .child(fx_studio_toggle),
            );

        if let Some(panel) = fx_studio_panel {
            content_col = content_col.child(panel);
        }

        content_col = content_col
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(12.0))
                    .child(
                        div().flex_1().child(
                            KpiMetricCard::new("Total Tasks", format!("{total_tasks}"))
                                .delta(format!("{active_tasks} remaining")),
                        ),
                    )
                    .child(
                        div().flex_1().child(
                            KpiMetricCard::new("Completed", format!("{completed_count}"))
                                .delta(format!("{completion_pct:.0}% done")),
                        ),
                    )
                    .child(
                        div().flex_1().child(
                            KpiMetricCard::new("High Priority", format!("{high_open}"))
                                .delta("Open urgent items"),
                        ),
                    )
                    .child(
                        optical_surface(&theme, SurfaceVariant::Slab)
                            .p(px(16.0))
                            .flex()
                            .flex_col()
                            .justify_center()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(text_primary)
                                    .child("Overall Progress"),
                            )
                            .child(ProgressAndSpinner::new(completion_pct).label(format!(
                                "{completed_count}/{total_tasks} tasks ({completion_pct:.0}%)"
                            ))),
                    ),
            )
            .child(
                PrismCard::new()
                    .kicker("QUICK ADD TASK")
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(10.0))
                            .child(draft_input)
                            .child(add_btn),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .flex_wrap()
                            .gap(px(12.0))
                            .child(priority_row)
                            .child(category_row),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(px(12.0))
                    .child(filter_row)
                    .child(clear_completed_btn),
            )
            .child(
                div()
                    .id("task-list-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(task_list_content),
            );

        let mut root = div()
            .id("task-manager-root")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
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
                    .child(OpticalBackdrop::new().reduced_motion(!self.state.optics_enabled)),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(self.render_menu_bar(&theme, cx))
                    .child(content_col),
            )
            .child(WorkspaceStatusBar::new(
                self.state.status_message.clone(),
                format!(
                    "Theme: {} | Material: {} | {active_tasks} Active • {completed_count} Done",
                    theme.display_name,
                    theme.material.as_str()
                ),
            ));

        if let Some(active_menu) = self.state.open_menu {
            root = root.child(self.render_open_menu_overlay(active_menu, &theme, cx));
        }

        root
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let initial_state = TaskManagerState::new();
        init_theme(cx, initial_state.theme());

        let bounds = Bounds::centered(None, size(px(1240.0), px(860.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from("Refractor Task Manager")),
                ..TitlebarOptions::default()
            }),
            focus: true,
            show: true,
            ..WindowOptions::default()
        };

        if let Err(err) = cx.open_window(options, |_window, cx| cx.new(TaskManagerApp::new)) {
            eprintln!("Failed to open Refractor Task Manager window: {err:#}");
        }
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_and_toggles_tasks() {
        let mut state = TaskManagerState::new();
        let initial_len = state.tasks.len();

        state.push_draft_char("Ship Refractor task manager");
        state.draft_priority = Priority::High;
        state.draft_category = Category::Engineering;
        assert!(state.add_draft_task());
        assert_eq!(state.tasks.len(), initial_len + 1);
        assert_eq!(state.draft_title, "");

        let created_id = state.tasks[0].id;
        assert!(!state.tasks[0].completed);
        state.toggle_task(created_id);
        assert!(state.tasks[0].completed);
    }

    #[test]
    fn rejects_blank_task_titles() {
        let mut state = TaskManagerState::new();
        let initial_len = state.tasks.len();
        state.push_draft_char("   ");
        assert!(!state.add_draft_task());
        assert_eq!(state.tasks.len(), initial_len);
    }

    #[test]
    fn cycles_priority_filters_and_clears_completed() {
        let mut state = TaskManagerState::new();
        let first_id = state.tasks[0].id;
        assert_eq!(state.tasks[0].priority, Priority::High);

        state.cycle_task_priority(first_id);
        assert_eq!(state.tasks[0].priority, Priority::Low);

        state.filter = TaskFilter::Completed;
        assert_eq!(state.visible_tasks().len(), 1);

        state.clear_completed();
        assert_eq!(state.completed_count(), 0);
        assert_eq!(state.visible_tasks().len(), 0);
    }

    #[test]
    fn aesthetic_controls_override_material_overlay_and_refraction() {
        let mut state = TaskManagerState::new();
        state.aesthetics.refraction = RefractionIntensity::Vivid;
        state.aesthetics.specular_highlight = false;
        state.aesthetics.slab_radius_override = Some(0.0);
        state.cycle_material();
        state.cycle_overlay();

        let resolved = state.theme();
        assert_eq!(resolved.optics.refraction, RefractionIntensity::Vivid);
        assert!(!resolved.optics.specular_highlight);
        assert!(resolved.fx.sharp_corners);
        assert_ne!(resolved.material, ThemeMaterial::Glass);
        assert_ne!(resolved.fx.wallpaper_overlay, ThemeOverlayFx::None);
    }

    #[test]
    fn horizontal_menu_actions_work_cleanly() {
        let mut state = TaskManagerState::new();
        assert_eq!(state.open_menu, None);
        state.toggle_menu(TopMenu::File);
        assert_eq!(state.open_menu, Some(TopMenu::File));
        state.toggle_menu(TopMenu::File);
        assert_eq!(state.open_menu, None);

        state.mark_all_completed();
        assert_eq!(state.active_count(), 0);

        state.reopen_all_tasks();
        assert_eq!(state.completed_count(), 0);

        let csv = state.export_tasks_csv();
        assert!(csv.contains("ID,Title,Status,Priority,Category"));
    }
}
