//! Application-level integration validation & deep performance benchmark suite verifying:
//! - All **339 components** across **58 canonical categories** (`refractor-ui` v1.52.0 parity)
//! - All **67 themes**, **13 materials**, **9 FX overlays**, **10 optical shaders**, and **3 GPU performance profiles**
//! - Positive, negative, and adversarial assertions across all 25 new FX/Shader/Loader components
//! - Deep interactive keyboard/IME/security controllers & throughput benchmarks

use std::time::Instant;

use gpui::SharedString;
use refractor_components::{
    Accordion, AcousticActivityBar, AcousticTone, AdvancedDataGrid, AiFocusPalette, AiFocusRing,
    AiOrbState, AiThinkingOrb, AlertBanner, AmbientAuraBleed, AppDockBar, AuraElevation, AuraTone,
    Avatar, BadgeStatus, Breadcrumbs, ButtonSize, CausticLightLayer, ChatMessageStream, Checkbox,
    ChromaticDispersionEdge, ColorPickerState, CommandPalette, CommandPaletteState,
    DataGridRowShimmer, DataGridState, DataTable, DropdownSelect, EmptyState, EnergyRibbonState,
    FloatingActionButton, FluidMeshCanvas, FluidMeshMood, FrostedGrainOverlay,
    GradientSelectionMode, GradientSelectionPalette, GradientTextSelection, GrainSize,
    HaloSpectrum, HolographicFoilSheen, HolographicPreset, IconButton, IridescentRimSurface,
    IridescentSpectrum, KanbanBoardState, KpiMetricCard, LiquidRippleTrigger,
    MetaballFluidBackdrop, ModalDialog, MultiTrackTimeline, MultiTrackTimelineState,
    NodeGraphCanvas, NodeGraphState, OpticalDepthVignette, OpticalDivider, OrbitalRingSpinner,
    Pagination, PipelinePulseLoader, PipelineStageItem, PipelineStageStatus, PrimaryButton,
    PrismCard, PrismTabs, PrismaticCursorHalo, ProgressAndSpinner, ProgressTone, RadarSweepMode,
    RadarSweepTone, RadioGroup, RangeSlider, ReactiveInputShimmer, RefractiveLensContainer,
    RefractiveSkeleton, RibbonPosition, RichTextEditorState, ScanlineRadarSweep, SecondaryButton,
    SegmentedControl, SegmentedGlassProgress, SidebarDrawer, SkeletonPreset, SpinnerSize,
    SplitButton, StatefulEnergyRibbon, StatusTone, Stepper, StreamingTokenReveal, TagChip,
    TerminalEmulator, TextArea, TextInput, ToastSnackbar, ToggleSwitch, Tooltip, TopNavBar,
    WindowTitlebar, WorkspaceStatusBar, all_component_groups, all_component_specs,
    components_in_group, find_component_spec, search_components,
};
use refractor_core::{
    BaseProps, BlurHostContext, FocusRingSpec, FxOverlaySpec, KeyAction, KeyboardEvent,
    MAX_SHADER_DPR, OpticalShaderKind, OpticalShaderRegistry, PointerCoalescer, ShaderBudget,
    ShaderUniforms, SurfaceVariant, TelemetryRingBuffer, optical_surface,
};
use refractor_themes::{
    all_themes, resolve_theme_by_id, search_themes, theme_categories, themes_by_material,
};
use refractor_tokens::{
    ThemeMaterial, ThemeOverlayFx, ThemePerformanceMode, contrast_ratio,
};

#[test]
fn validates_all_339_components_and_58_categories_metadata() {
    let specs = all_component_specs();
    assert_eq!(specs.len(), 339);

    let groups = all_component_groups();
    assert_eq!(groups.len(), 58);

    let mut total_in_groups = 0;
    for group in &groups {
        let members = components_in_group(group);
        assert!(!members.is_empty(), "group {group} must not be empty");
        total_in_groups += members.len();
    }
    assert_eq!(total_in_groups, 339);

    // Verify the 3 new v1.50-v1.52 groups have exact counts (8 + 8 + 9 = 25)
    assert_eq!(components_in_group("Dynamic Gradient & AI-Glow FX").len(), 8);
    assert_eq!(components_in_group("Shader & Optical FX").len(), 8);
    assert_eq!(
        components_in_group("Loaders, Skeletons & Data Transitions").len(),
        9
    );

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
    let _b2 = SecondaryButton::new("Cancel")
        .size(ButtonSize::Sm)
        .cobalt(true)
        .disabled(true);
    let _b3 = IconButton::new("⚙").size_px(40.0);
    let _b4 = SegmentedControl::new(
        vec![SharedString::from("Light"), SharedString::from("Dark")],
        1,
    );
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

    let _n1 = TopNavBar::new("Refractor").subtitle("v1.1.0");
    let _n2 = PrismTabs::new(vec![SharedString::from("Tab 1")], 0);
    let _n3 = Breadcrumbs::new(vec![
        SharedString::from("Home"),
        SharedString::from("Studio"),
    ]);
    let _n4 = Stepper::new(
        vec![SharedString::from("One"), SharedString::from("Two")],
        1,
    );
    let _n5 = Pagination::new(2, 10);
    let _n6 = SidebarDrawer::new("Menu", vec![SharedString::from("Dashboard")], 0);
    let _n7 = CommandPalette::new("theme", vec![SharedString::from("Switch to Cyberpunk")]);

    let _d1 = PrismCard::new()
        .kicker("KICKER")
        .title("Title")
        .child(OpticalDivider::new());
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
    let _os3 = WorkspaceStatusBar::new("Ready", "v1.1.0");
    let _os4 = TerminalEmulator::new(vec![SharedString::from("$ cargo test")]);
    let _os5 = ChatMessageStream::new(vec![(
        SharedString::from("AI"),
        SharedString::from("Ready"),
    )]);

    let _cat1 = AdvancedDataGrid::new()
        .title("Grid")
        .subtitle("Live")
        .value("42 rows")
        .badge("PRO");
    let _cat2 = NodeGraphCanvas::new().title("Shader Graph");
    let _cat3 = MultiTrackTimeline::new().title("Timeline");
}

#[test]
fn validates_all_25_new_fx_shader_and_loader_components_positive_negative_and_adversarial() {
    // --- Suite 8: Dynamic Gradient & AI-Glow FX (8 components) ---
    let halo = PrismaticCursorHalo::new()
        .radius_px(f32::NAN)
        .intensity(f32::INFINITY)
        .pointer(-5.0, 99.0)
        .spectrum(HaloSpectrum::Aurora)
        .border_glow(true);
    assert!((halo.radius_px - 240.0).abs() < f32::EPSILON);
    assert!((halo.intensity - 0.78).abs() < f32::EPSILON);
    assert!((halo.pointer_x - 0.0).abs() < f32::EPSILON);
    assert!((halo.pointer_y - 1.0).abs() < f32::EPSILON);

    let ring = AiFocusRing::new("Agent Prompt")
        .active(true)
        .processing(true)
        .palette(AiFocusPalette::EmeraldCyan)
        .thickness_px(999.0)
        .speed_seconds(-2.0);
    assert!((ring.thickness_px - 8.0).abs() < f32::EPSILON);
    assert!((ring.speed_seconds - 0.4).abs() < f32::EPSILON);

    let mesh = FluidMeshCanvas::new()
        .mood(FluidMeshMood::Thinking)
        .cadence(f32::NAN)
        .intensity(2.5)
        .animated(true);
    assert!((mesh.cadence - 0.35).abs() < f32::EPSILON);
    assert!((mesh.intensity - 1.0).abs() < f32::EPSILON);

    let _gts = GradientTextSelection::new("Optical text")
        .mode(GradientSelectionMode::Prose)
        .palette(GradientSelectionPalette::Solar)
        .selected_phrase("Optical");

    let aura = AmbientAuraBleed::new("Card")
        .tone(AuraTone::AiViolet)
        .elevation(AuraElevation::Lg)
        .intensity(f32::NEG_INFINITY)
        .breathing(false);
    assert!((aura.intensity - 0.78).abs() < f32::EPSILON);

    let _shimmer = ReactiveInputShimmer::new()
        .value("Query")
        .shimmer_trigger(3)
        .suggestions(vec!["One".into(), "Two".into()]);

    let rim = IridescentRimSurface::new("Card")
        .spectrum(IridescentSpectrum::AmberRose)
        .angle_deg(f32::INFINITY)
        .enable_tilt(true);
    assert!((rim.angle_deg - 132.0).abs() < f32::EPSILON);

    let ribbon = StatefulEnergyRibbon::new(EnergyRibbonState::Streaming)
        .position(RibbonPosition::Top)
        .thickness_px(0.0)
        .show_label(true);
    assert!((ribbon.thickness_px - 1.0).abs() < f32::EPSILON);

    // --- Suite 9: Shader & Optical FX (8 components) ---
    let lens = RefractiveLensContainer::new("Lens")
        .ior(9.9)
        .lens_radius(f32::NAN)
        .magnification(0.1)
        .focus(f32::INFINITY, -1.0);
    assert!((lens.ior - 2.4).abs() < f32::EPSILON);
    assert!((lens.lens_radius - 0.52).abs() < f32::EPSILON);
    assert!((lens.magnification - 0.9).abs() < f32::EPSILON);

    let edge = ChromaticDispersionEdge::new("Prism").dispersion(-5.0);
    assert!((edge.dispersion - 0.0).abs() < f32::EPSILON);

    let caustic = CausticLightLayer::new().scale(100.0).speed(f32::NAN);
    assert!((caustic.scale - 4.0).abs() < f32::EPSILON);
    assert!((caustic.speed - 1.0).abs() < f32::EPSILON);

    let grain = FrostedGrainOverlay::new()
        .grain_size(GrainSize::Coarse)
        .opacity(0.99);
    assert!((grain.opacity - 0.4).abs() < f32::EPSILON);

    // Positive + negative + adversarial ripple pool checks
    let mut ripple = LiquidRippleTrigger::new("Tap").max_ripples(99).duration_ms(10);
    assert_eq!(ripple.max_ripples, 8);
    assert_eq!(ripple.duration_ms, 150);
    for i in 0..20_u64 {
        ripple.push_ripple(f32::NAN, 0.5, i * 10);
    }
    assert_eq!(ripple.ripples.len(), 8);
    let mut disabled_ripple = LiquidRippleTrigger::new("Disabled").disabled(true);
    disabled_ripple.push_ripple(0.5, 0.5, 100);
    assert!(disabled_ripple.ripples.is_empty());

    let foil = HolographicFoilSheen::new("Card")
        .preset(HolographicPreset::PrismGold)
        .tilt_degrees(90.0);
    assert!((foil.tilt_degrees - 20.0).abs() < f32::EPSILON);

    let meta = MetaballFluidBackdrop::new().orb_count(0).speed(-1.0);
    assert_eq!(meta.orb_count, 1);
    assert!((meta.speed - 0.0).abs() < f32::EPSILON);

    let vig = OpticalDepthVignette::new()
        .aperture(0.0)
        .blur_px(999.0)
        .dim_opacity(2.0);
    assert!((vig.aperture - 0.08).abs() < f32::EPSILON);
    assert!((vig.blur_px - 32.0).abs() < f32::EPSILON);
    assert!((vig.dim_opacity - 0.92).abs() < f32::EPSILON);

    // --- Suite 10: Loaders, Skeletons & Data Transitions (9 components) ---
    let skel = RefractiveSkeleton::new()
        .preset(SkeletonPreset::Profile)
        .lines(0);
    assert_eq!(skel.lines, 1);

    let spin = OrbitalRingSpinner::new()
        .size(SpinnerSize::Xl)
        .stroke_width(f32::NAN)
        .show_label(true);
    assert!((spin.stroke_width - 4.5).abs() < f32::EPSILON);

    let orb = AiThinkingOrb::new(AiOrbState::Generating).size_px(5.0);
    assert!((orb.size_px - 24.0).abs() < f32::EPSILON);

    let seg = SegmentedGlassProgress::new(50.0)
        .max(100.0)
        .segments(10)
        .tone(ProgressTone::Emerald);
    assert_eq!(seg.filled_segments(), 5);
    let seg_adv = SegmentedGlassProgress::new(f32::NAN).segments(0);
    assert_eq!(seg_adv.segments, 1);
    assert_eq!(seg_adv.filled_segments(), 0);

    let reveal = StreamingTokenReveal::new("one two three four")
        .streaming(true)
        .revealed_tokens(2);
    assert_eq!(reveal.visible_text(), "one two");
    let reveal_skip = reveal.skip_animation(true);
    assert_eq!(reveal_skip.visible_text(), "one two three four");

    let grid_shimmer = DataGridRowShimmer::new().rows(999).columns(Vec::new());
    assert_eq!(grid_shimmer.rows, 24);
    assert_eq!(grid_shimmer.columns.len(), 4); // empty columns rejected

    let eq = AcousticActivityBar::new()
        .bar_count(5)
        .tone(AcousticTone::Cobalt)
        .levels(vec![f32::NAN, 5.0, -2.0, 0.5, 0.9]);
    let resolved = eq.resolved_levels();
    assert_eq!(resolved.len(), 5);
    for lv in resolved {
        assert!((0.08..=1.0).contains(&lv));
    }

    let pipe = PipelinePulseLoader::new()
        .stages(vec![PipelineStageItem::new(
            "s1",
            "Build",
            PipelineStageStatus::Active,
        )])
        .pulse_speed_seconds(f32::INFINITY);
    assert!((pipe.pulse_speed_seconds - 1.5).abs() < f32::EPSILON);

    let radar = ScanlineRadarSweep::new()
        .mode(RadarSweepMode::Radar)
        .tone(RadarSweepTone::Emerald)
        .speed_seconds(0.01);
    assert!((radar.speed_seconds - 0.4).abs() < f32::EPSILON);
}

#[test]
fn validates_shader_engine_disposal_pointer_coalescer_ring_buffer_and_performance_modes() {
    // 1. All 10 shaders mount and dispose with 0 leaks
    let mut reg = OpticalShaderRegistry::new();
    let handles: Vec<u64> = OpticalShaderKind::ALL
        .iter()
        .map(|&kind| {
            assert_ne!(kind.as_str(), "");
            assert_ne!(kind.wgsl_entry_point(), "");
            reg.mount(kind)
        })
        .collect();
    assert_eq!(reg.active_count(), 10);
    for h in handles {
        assert!(reg.dispose(h));
    }
    assert_eq!(reg.active_count(), 0);
    assert_eq!(reg.created_total(), 10);
    assert_eq!(reg.disposed_total(), 10);
    assert!(!reg.dispose(999_999)); // negative: unknown handle

    // Context-loss dispose_all
    let _ = reg.mount(OpticalShaderKind::FluidMesh);
    let _ = reg.mount(OpticalShaderKind::ThinkingOrb);
    assert_eq!(reg.dispose_all(), 2);
    assert_eq!(reg.active_count(), 0);

    // 2. ShaderBudget DPR capping, 8K clamping, visibility & performance-mode guards
    let mut theme = resolve_theme_by_id("alabaster");
    let balanced = ShaderBudget::from_theme(&theme, false);
    assert!(balanced.should_animate());
    let (w, h) = balanced.effective_surface_size(3840.0, 2160.0, 4.0);
    assert!(w <= 768 && h <= 768);
    let (nan_w, nan_h) = balanced.effective_surface_size(f32::NAN, f32::INFINITY, f32::NAN);
    assert!(nan_w >= 1 && nan_h >= 1);
    assert!((balanced.max_dpr - MAX_SHADER_DPR).abs() < f32::EPSILON);

    // Reduced motion and Max performance mode disable animation & overlays
    assert!(!ShaderBudget::from_theme(&theme, true).should_animate());
    theme.optics.performance_mode = ThemePerformanceMode::Max;
    assert!(!ShaderBudget::from_theme(&theme, false).should_animate());
    assert!((theme.optics.effective_blur_px() - 0.0).abs() < f32::EPSILON);
    let fx_max = FxOverlaySpec::from_theme(&theme, false);
    assert!(!fx_max.active);

    // Uniforms NaN defense
    let u = ShaderUniforms::from_theme(&theme, f32::NAN)
        .with_pointer(f32::INFINITY, f32::NEG_INFINITY)
        .with_time(f32::NAN)
        .with_params(f32::NAN, f32::INFINITY, -9999.0);
    assert!(u.pointer_x.is_finite() && u.pointer_y.is_finite());
    assert!(u.time_s.is_finite() && u.param_a.is_finite());

    // 3. PointerCoalescer 200x high-frequency 1000Hz burst test
    let mut coalescer = PointerCoalescer::new(60);
    let mut emitted = 0_u32;
    for i in 0..200_u32 {
        if coalescer
            .push(i as f32 * 0.005, 0.5, f64::from(i))
            .is_some()
        {
            emitted += 1;
        }
    }
    assert!(emitted <= 15, "60fps coalescer should emit <= 15 times in 200ms, got {emitted}");
    assert!(coalescer.coalesced_count() >= 185);
    assert!(coalescer.push(f32::NAN, 0.5, 300.0).is_none());
    assert!(coalescer.flush(205.0).is_some());
    coalescer.cancel();

    // 4. TelemetryRingBuffer bounded memory guarantee
    let mut ring: TelemetryRingBuffer<u32, 8> = TelemetryRingBuffer::new();
    for i in 0..1000_u32 {
        ring.push(i);
    }
    assert_eq!(ring.len(), 8);
    assert_eq!(ring.ordered(), vec![992, 993, 994, 995, 996, 997, 998, 999]);

    // 5. BlurHostContext nested blur elision
    let root_ctx = BlurHostContext::root();
    assert!(!root_ctx.should_elide_blur());
    let child_ctx = root_ctx.enter_blurred_host();
    assert!(child_ctx.should_elide_blur());
}

#[test]
fn validates_deep_interactive_keyboard_ime_and_security_controllers() {
    let mut palette = CommandPaletteState::new(vec![
        "Switch Theme: Alabaster".to_owned(),
        "Switch Theme: Cyberpunk".to_owned(),
        "Open Workbench".to_owned(),
    ]);
    palette.set_query("theme");
    assert_eq!(palette.filtered().len(), 2);

    palette.handle_key(KeyboardEvent::new(KeyAction::Enter).composing(true));
    assert!(palette.open);
    assert!(palette.last_executed.is_none());

    palette.handle_key(KeyboardEvent::new(KeyAction::ArrowDown));
    palette.handle_key(KeyboardEvent::new(KeyAction::Enter));
    assert!(!palette.open);
    assert_eq!(
        palette.last_executed.as_deref(),
        Some("Switch Theme: Cyberpunk")
    );

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

    let mut kanban = KanbanBoardState::new(vec![
        ("Backlog".to_owned(), vec!["RUI-1".to_owned()]),
        ("Done".to_owned(), Vec::new()),
    ]);
    assert!(kanban.move_card(0, 0, 1));
    assert!(!kanban.move_card(0, 0, 1));
    assert_eq!(kanban.columns[1].1, vec!["RUI-1".to_owned()]);

    let mut editor = RichTextEditorState::new("<p>Safe</p><script>alert(1)</script>");
    assert!(!editor.sanitized_html.contains("script"));
    editor.set_html("<strong>Updated</strong>");
    assert_eq!(editor.sanitized_html, "<strong>Updated</strong>");
    assert!(editor.undo());
    assert_eq!(editor.sanitized_html, "<p>Safe</p>");

    let mut picker = ColorPickerState::from_hex("#7C3AED");
    assert!(picker.contrast_against("#FFFFFF") >= 4.5);
    picker.set_hsv(0, 0, 100);
    assert_eq!(picker.hex, "#FFFFFF");

    let mut tl = MultiTrackTimelineState::new(120.0, 24.0);
    tl.scrub(f32::NAN);
    assert!((tl.playhead_sec - 0.0).abs() < f32::EPSILON);
    tl.step_frame(24);
    assert!((tl.playhead_sec - 1.0).abs() < 0.01);
    tl.set_zoom(f32::INFINITY);
    assert!((tl.zoom - 1.0).abs() < f32::EPSILON);
    tl.toggle_play();
    assert!(tl.playing);

    let mut graph = NodeGraphState::new();
    assert!(!graph.add_node("__proto__", 0.0, 0.0));
    assert!(graph.add_node("node-a", 10.0, 20.0));
    assert!(graph.add_node("node-b", 50.0, 80.0));
    assert!(graph.connect("node-a", "node-b"));
    assert!(!graph.connect("node-a", "node-a"));
}

#[test]
fn benchmarks_67_themes_339_components_and_srgb_lut_throughput() {
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
    assert!(!search_themes("cyber").is_empty());

    // O(1) component lookup across all 339 components
    for spec in all_component_specs() {
        assert!(find_component_spec(&spec.name).is_some());
    }
    assert!(search_components("shader").len() >= 8);

    // 10,000 WCAG contrast checks powered by 256-entry sRGB LUT
    for _ in 0..10_000 {
        let ratio = contrast_ratio("#E75C24", "#FFFFFF");
        assert!(ratio > 3.0);
    }

    let elapsed_ms = start.elapsed().as_millis();
    assert!(
        elapsed_ms < 2000,
        "67 themes + 339 O(1) lookups + 10k WCAG contrast checks took {elapsed_ms}ms"
    );
}
