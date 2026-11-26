//! Complete 339-component catalog across 58 categories for **Refractor UI for Rust**.
//!
//! Powered by Zed's GPUI (`gpui`) and governed by Astryx Open-Internals (`ADR-0001`..`ADR-0006`).

pub mod buttons;
pub mod desktop;
pub mod display;
pub mod feedback;
pub mod inputs;
pub mod interactive;
pub mod navigation;
pub mod registry;
pub mod suites;

pub use buttons::{
    ButtonSize, FloatingActionButton, IconButton, PrimaryButton, SecondaryButton, SegmentedControl,
    SplitButton,
};
pub use desktop::{
    AppDockBar, ChatMessageStream, TerminalEmulator, WindowChromeStyle, WindowSplitter,
    WindowTitlebar, WorkspaceStatusBar,
};
pub use display::{
    Accordion, Avatar, BadgeStatus, DataTable, KpiMetricCard, OpticalDivider, PrismCard,
    StatusTone, TagChip,
};
pub use feedback::{
    AlertBanner, EmptyState, ModalDialog, ProgressAndSpinner, ToastSnackbar, Tooltip,
};
pub use inputs::{
    Checkbox, DropdownSelect, RadioGroup, RangeSlider, TextArea, TextInput, ToggleSwitch,
};
pub use interactive::{
    ColorPickerState, CommandPaletteState, DataGridState, KanbanBoardState,
    MultiTrackTimelineState, NodeGraphState, RichTextEditorState,
};
pub use navigation::{
    Breadcrumbs, CommandPalette, MenuBarTrigger, MenuDropdownItem, Pagination, PrismTabs,
    SidebarDrawer, Stepper, TopNavBar,
};
pub use registry::{
    ComponentA11yItem, ComponentBestPractice, ComponentPropSpec, ComponentSpec,
    all_component_groups, all_component_specs, components_in_group, find_component_spec,
    search_components,
};
pub use suites::ai_cli_observability::*;
pub use suites::charts_core::*;
pub use suites::core_utilities_hardware::*;
pub use suites::desktop_os_files::*;
pub use suites::enterprise_data_commerce::*;
pub use suites::forms_inputs::*;
pub use suites::gradient_ai_glow_fx::*;
pub use suites::loaders_skeletons_transitions::*;
pub use suites::pro_media_audio::*;
pub use suites::shader_optical_fx::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_exposes_all_339_components_across_58_categories() {
        let specs = all_component_specs();
        assert_eq!(specs.len(), 339);
        assert_eq!(all_component_groups().len(), 58);
    }

    #[test]
    fn search_and_lookup_handle_positive_negative_and_adversarial_queries() {
        assert!(find_component_spec("PrimaryButton").is_some());
        assert!(find_component_spec("AdvancedDataGrid").is_some());
        assert!(find_component_spec("NodeGraphCanvas").is_some());
        assert!(find_component_spec("MultiTrackTimeline").is_some());
        assert!(find_component_spec("NonExistentWidget").is_none());

        // Adversarial ReDoS query treated strictly literally
        let results = search_components("(a+)+$");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn data_table_exports_owasp_safe_csv() {
        let table = DataTable::new(
            vec!["User".to_owned(), "Formula".to_owned()],
            vec![vec!["Ryan".to_owned(), "=CMD|' /C calc'!A0".to_owned()]],
        );
        let csv = table.export_safe_csv();
        assert!(csv.contains("'=CMD|' /C calc'!A0"));
    }

    #[test]
    fn slider_and_progress_handle_nan_and_infinity_adversarial_inputs() {
        let slider = RangeSlider::new(f32::NAN, 0.0, 100.0);
        assert!((slider.normalized_fraction() - 0.0).abs() < f32::EPSILON);

        let slider_inf = RangeSlider::new(f32::INFINITY, 0.0, 100.0);
        assert!((slider_inf.normalized_fraction() - 0.0).abs() < f32::EPSILON);

        let prog = ProgressAndSpinner::new(f32::NAN);
        assert!((prog.progress_pct - 0.0).abs() < f32::EPSILON);

        let prog_over = ProgressAndSpinner::new(500.0);
        assert!((prog_over.progress_pct - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn bespoke_domain_suite_components_construct_cleanly() {
        let grid = AdvancedDataGrid::new().title("Revenue Grid").badge("314");
        assert_eq!(AdvancedDataGrid::component_name(), "AdvancedDataGrid");
        assert_eq!(grid.title.as_ref(), "Revenue Grid");

        let timeline = MultiTrackTimeline::new().value("00:01:24:12");
        assert_eq!(timeline.value.as_ref(), "00:01:24:12");

        let ohlc = CandlestickOhlcChart::new().series(vec![0.2, 0.8, 0.5, 0.9]);
        assert_eq!(ohlc.series.len(), 4);

        let led = DigitalLedReadout::new().value("144.0 FPS");
        assert_eq!(led.value.as_ref(), "144.0 FPS");
    }
}
