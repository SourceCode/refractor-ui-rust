//! Application-level integration validation & benchmark suite verifying all 314 components
//! across all 55 categories, all 67 themes, all 13 materials, all 9 FX overlays, and all
//! deep interactive keyboard/IME/security controllers.

use std::time::Instant;

use gpui::SharedString;
use refractor_components::{
    Accordion, AdvancedDataGrid, AlertBanner, AppDockBar, Avatar, BadgeStatus, Breadcrumbs,
    ButtonSize, ChatMessageStream, Checkbox, ColorPickerState, CommandPalette, CommandPaletteState,
    DataGridState, DataTable, DropdownSelect, EmptyState, FloatingActionButton, IconButton,
    KanbanBoardState, KpiMetricCard, ModalDialog, MultiTrackTimeline, MultiTrackTimelineState,
    NodeGraphCanvas, NodeGraphState, OpticalDivider, Pagination, PrimaryButton, PrismCard,
    PrismTabs, ProgressAndSpinner, RadioGroup, RangeSlider, RichTextEditorState, SecondaryButton,
    SegmentedControl, SidebarDrawer, SplitButton, StatusTone, Stepper, TagChip, TerminalEmulator,
    TextArea, TextInput, ToastSnackbar, ToggleSwitch, Tooltip, TopNavBar, WindowTitlebar,
    WorkspaceStatusBar, all_component_groups, all_component_specs, components_in_group,
    find_component_spec, search_components,
};
use refractor_core::{
    BaseProps, FocusRingSpec, FxOverlaySpec, KeyAction, KeyboardEvent, SurfaceVariant,
    optical_surface,
};
use refractor_themes::{all_themes, resolve_theme_by_id, theme_categories, themes_by_material};
use refractor_tokens::{ThemeMaterial, ThemeOverlayFx};

#[test]
fn validates_all_314_components_and_55_categories_metadata() {
    let specs = all_component_specs();
    assert_eq!(specs.len(), 314);

    let groups = all_component_groups();
    assert_eq!(groups.len(), 55);

    let mut total_in_groups = 0;
    for group in &groups {
        let members = components_in_group(group);
        assert!(!members.is_empty(), "group {group} must not be empty");
        total_in_groups += members.len();
    }
    assert_eq!(total_in_groups, 314);

    for spec in specs {
        assert_ne!(spec.name, "");
        assert_ne!(spec.display_name, "");
        assert!(find_component_spec(&spec.name).is_some());
    }
}

#[test]
fn validates_all_40_flagship_and_catalog_component_builders() {
    let _b1 = PrimaryButton::new("Deploy")
        .size(ButtonSize::Lg)
        .leading_icon("▶")
        .trailing_icon("→")
        .disabled(false)
        .base(BaseProps::new().with_id("deploy"));
    let _b2 = SecondaryButton::new("Cancel").size(ButtonSize::Sm).cobalt(true).disabled(true);
    let _b3 = IconButton::new("⚙").size_px(40.0);
    let _b4 = SegmentedControl::new(vec![SharedString::from("Light"), SharedString::from("Dark")], 1);
    let _b5 = SplitButton::new("Export");
    let _b6 = FloatingActionButton::new("+").label("New");

    let _i1 = TextInput::new("hello").label("Query").placeholder("Search...");
    let _i2 = TextArea::new("body").label("Notes").rows(6);
    let _i3 = ToggleSwitch::new(true).label("Optics");
    let _i4 = Checkbox::new(true, "Accept");
    let _i5 = RadioGroup::new(vec![SharedString::from("A"), SharedString::from("B")], 0);
    let slider = RangeSlider::new(50.0, 0.0, 100.0).label("Blur");
    assert!((slider.normalized_fraction() - 0.5).abs() < 0.001);
    let _i7 = DropdownSelect::new("Alabaster Light");

    let _n1 = TopNavBar::new("Refractor").subtitle("v1.0.0");
    let _n2 = PrismTabs::new(vec![SharedString::from("Tab 1")], 0);
    let _n3 = Breadcrumbs::new(vec![SharedString::from("Home"), SharedString::from("Studio")]);
    let _n4 = Stepper::new(vec![SharedString::from("One"), SharedString::from("Two")], 1);
    let _n5 = Pagination::new(2, 10);
    let _n6 = SidebarDrawer::new("Menu", vec![SharedString::from("Dashboard")], 0);
    let _n7 = CommandPalette::new("theme", vec![SharedString::from("Switch to Cyberpunk")]);

    let _d1 = PrismCard::new().kicker("KICKER").title("Title").child(OpticalDivider::new());
    let table = DataTable::new(
        vec!["Col".to_owned()],
        vec![vec!["+1+1".to_owned()], vec!["safe".to_owned()]],
    );
    assert!(table.export_safe_csv().contains("'+1+1"));
    let _d3 = BadgeStatus::new("Live", StatusTone::Success);
    let _d4 = TagChip::new("GPUI");
    let _d5 = Avatar::new("RR").size_px(44.0);
    let _d6 = Accordion::new("Details", "Content", true);
    let _d7 = KpiMetricCard::new("FPS", "120").delta("+0ms");

    let _f1 = AlertBanner::new("Notice", "All systems nominal", StatusTone::Primary);
    let _f2 = ToastSnackbar::new("Saved");
    let _f3 = ProgressAndSpinner::new(80.0).label("Build");
    let _f4 = EmptyState::new("No items", "Create your first item");
    let _f5 = ModalDialog::new("Confirm", "Proceed with release?");
    let _f6 = Tooltip::new("Shortcut: Ctrl+K");

    let _os1 = WindowTitlebar::new("Workbench");
    let _os2 = AppDockBar::new(vec![SharedString::from("Studio")]);
    let _os3 = WorkspaceStatusBar::new("Ready", "v1.0.0");
    let _os4 = TerminalEmulator::new(vec![SharedString::from("$ cargo test")]);
    let _os5 = ChatMessageStream::new(vec![(SharedString::from("AI"), SharedString::from("Ready"))]);

    let _cat1 = AdvancedDataGrid::new().title("Grid").subtitle("Live").value("42 rows").badge("PRO");
    let _cat2 = NodeGraphCanvas::new().title("Shader Graph");
    let _cat3 = MultiTrackTimeline::new().title("Timeline");
}

#[test]
fn validates_deep_interactive_keyboard_ime_and_security_controllers() {
    // 1. CommandPalette IME guard + roving selection + Enter execution
    let mut palette = CommandPaletteState::new(vec![
        "Switch Theme: Alabaster".to_owned(),
        "Switch Theme: Cyberpunk".to_owned(),
        "Open Workbench".to_owned(),
    ]);
    palette.set_query("theme");
    assert_eq!(palette.filtered().len(), 2);

    // IME composing Enter must NOT execute prematurely (ADR-0002)
    palette.handle_key(KeyboardEvent::new(KeyAction::Enter).composing(true));
    assert!(palette.open);
    assert!(palette.last_executed.is_none());

    palette.handle_key(KeyboardEvent::new(KeyAction::ArrowDown));
    palette.handle_key(KeyboardEvent::new(KeyAction::Enter));
    assert!(!palette.open);
    assert_eq!(palette.last_executed.as_deref(), Some("Switch Theme: Cyberpunk"));

    // 2. DataGrid sorting + keyboard row navigation + OWASP CSV
    let mut grid = DataGridState::new(
        vec!["Name".to_owned(), "Payload".to_owned()],
        vec![
            vec!["Zed".to_owned(), "=SUM(1,2)".to_owned()],
            vec!["Alpha".to_owned(), "clean".to_owned()],
        ],
    );
    grid.sort_by(0);
    assert_eq!(grid.rows[0][0], "Alpha");
    grid.handle_key(KeyboardEvent::new(KeyAction::ArrowDown));
    assert_eq!(grid.selected_row, 1);
    assert!(grid.export_safe_csv().contains("'=SUM(1,2)"));

    // 3. Kanban keyboard card move
    let mut kanban = KanbanBoardState::new(vec![
        ("Backlog".to_owned(), vec!["RUI-1".to_owned()]),
        ("Done".to_owned(), Vec::new()),
    ]);
    assert!(kanban.move_card(0, 0, 1));
    assert!(!kanban.move_card(0, 0, 1)); // now empty
    assert_eq!(kanban.columns[1].1, vec!["RUI-1".to_owned()]);

    // 4. RichTextEditor XSS sanitization + undo
    let mut editor = RichTextEditorState::new("<p>Safe</p><script>alert(1)</script>");
    assert!(!editor.sanitized_html.contains("script"));
    editor.set_html("<strong>Updated</strong>");
    assert_eq!(editor.sanitized_html, "<strong>Updated</strong>");
    assert!(editor.undo());
    assert_eq!(editor.sanitized_html, "<p>Safe</p>");

    // 5. ColorPicker WCAG contrast
    let mut picker = ColorPickerState::from_hex("#7C3AED");
    assert!(picker.contrast_against("#FFFFFF") >= 4.5);
    picker.set_hsv(0, 0, 100);
    assert_eq!(picker.hex, "#FFFFFF");

    // 6. MultiTrackTimeline NaN-safe scrubbing & frame step
    let mut tl = MultiTrackTimelineState::new(120.0, 24.0);
    tl.scrub(f32::NAN);
    assert!((tl.playhead_sec - 0.0).abs() < f32::EPSILON);
    tl.step_frame(24);
    assert!((tl.playhead_sec - 1.0).abs() < 0.01);
    tl.set_zoom(f32::INFINITY);
    assert!((tl.zoom - 1.0).abs() < f32::EPSILON);
    tl.toggle_play();
    assert!(tl.playing);

    // 7. NodeGraph prototype-pollution rejection
    let mut graph = NodeGraphState::new();
    assert!(!graph.add_node("__proto__", 0.0, 0.0));
    assert!(graph.add_node("node-a", 10.0, 20.0));
    assert!(graph.add_node("node-b", 50.0, 80.0));
    assert!(graph.connect("node-a", "node-b"));
    assert!(!graph.connect("node-a", "node-a"));
}

#[test]
fn benchmarks_67_themes_and_314_components_throughput() {
    let start = Instant::now();
    assert_eq!(theme_categories().len(), 9);
    for entry in all_themes() {
        let resolved = resolve_theme_by_id(&entry.id);
        let fx_spec = FxOverlaySpec::from_theme(&resolved, false);
        assert!(fx_spec.opacity <= 0.4 + f32::EPSILON);
        let focus = FocusRingSpec::from_theme(&resolved, true, true);
        assert!(focus.visible);
        let _surface = optical_surface(&resolved, SurfaceVariant::Slab);
    }
    for mat in ThemeMaterial::ALL {
        let _matching = themes_by_material(mat);
    }
    assert_eq!(ThemeOverlayFx::ALL.len(), 9);
    let matches = search_components("button");
    assert!(!matches.is_empty(), "expected button search matches");

    let elapsed_ms = start.elapsed().as_millis();
    assert!(
        elapsed_ms < 2000,
        "resolving 67 themes + optical surfaces + catalog search took {elapsed_ms}ms"
    );
}
