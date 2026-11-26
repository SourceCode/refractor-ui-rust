//! Native GPUI Desktop Application (`apps/studio-showcase`) validating live GPU rendering
//! across all 7 bespoke domain suites:
//! - Suite 1: Forms, Rich Content & Controls (`RatingControl`, `CodeEditor`, `ColorPicker`, `RotaryDial`)
//! - Suite 2: GPU-Canvas Charts (`CandlestickOhlcChart`, `MatrixHeatmap`, `ScatterBubblePlot`, `RadialGauge`)
//! - Suite 3: Pro NLE/DAW/Audio (`MultiTrackTimeline`, `ParametricEqGraph`, `VuPeakLevelMeter`, `ThreeWayColorWheels`)
//! - Suite 4: AI Agents & DevOps (`ContextTokenMeter`, `DistributedTraceWaterfall`, `ApprovalGateCard`, `UptimeStatusBar`)
//! - Suite 5: Enterprise & 3D (`AdvancedDataGrid`, `OpticalMaterialInspector`, `PasswordStrengthMeter`)
//! - Suite 6: Desktop OS & Files (`DualPaneCommander`, `ControlCenterGrid`, `FileTransferQueue`)
//! - Suite 7: Hardware & Micro-Utilities (`DigitalLedReadout`, `StepSequencerMatrix`, `ThermostatDial`)

use std::env;

use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement,
    Styled, Window, div, px,
};
use refractor_components::{
    AcousticActivityBar, AdvancedDataGrid, AiFocusRing, AiOrbState, AiThinkingOrb,
    ApprovalGateCard, ButtonSize, CandlestickOhlcChart, CodeEditor, ColorPicker,
    ContextTokenMeter, ControlCenterGrid, DigitalLedReadout, DistributedTraceWaterfall,
    DualPaneCommander, EnergyRibbonState, FileTransferQueue, HolographicFoilSheen, MatrixHeatmap,
    MultiTrackTimeline, OpticalMaterialInspector, ParametricEqGraph, PasswordStrengthMeter,
    PipelinePulseLoader, PrimaryButton, PrismaticCursorHalo, RadialGauge, RatingControl,
    RefractiveLensContainer, RotaryDial, ScatterBubblePlot, SecondaryButton,
    SegmentedGlassProgress, StatefulEnergyRibbon, StepSequencerMatrix, ThermostatDial,
    ThreeWayColorWheels, TopNavBar, UptimeStatusBar, VuPeakLevelMeter, WindowTitlebar,
    WorkspaceStatusBar,
};
use refractor_core::{AppHost, OpticalBackdrop, RefractorApp, Task, run_app};
use refractor_themes::resolve_theme_by_id;
use refractor_tokens::PrismTheme;

#[derive(Debug, Clone)]
enum ShowcaseMessage {
    SelectTheme(&'static str),
}

struct StudioShowcase {
    theme_id: String,
}

impl RefractorApp for StudioShowcase {
    type Message = ShowcaseMessage;

    fn new() -> (Self, Task<Self::Message>) {
        let initial = env::args()
            .skip(1)
            .collect::<Vec<_>>()
            .windows(2)
            .find(|w| w[0] == "--theme")
            .map_or_else(|| "alabaster".to_owned(), |w| w[1].clone());
        (Self { theme_id: initial }, Task::none())
    }

    fn title(&self) -> String {
        format!(
            "Refractor Studio Showcase — 7 Bespoke Suites ({})",
            self.theme_id
        )
    }

    fn window_size(&self) -> (f32, f32) {
        (1280.0, 820.0)
    }

    fn theme(&self) -> PrismTheme {
        resolve_theme_by_id(&self.theme_id)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            ShowcaseMessage::SelectTheme(id) => {
                id.clone_into(&mut self.theme_id);
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
        let presets: [(&str, &str); 6] = [
            ("alabaster", "Alabaster Glass"),
            ("obsidian", "Obsidian Dark"),
            ("amethyst", "Amethyst Vivid"),
            ("cyberpunk", "Cyberpunk Neon"),
            ("windows95", "Windows 95 Bevel"),
            ("tokyo-night", "Tokyo Night IDE"),
        ];

        let mut theme_row = div().flex().flex_row().gap(px(8.0));
        for (idx, (id, label)) in presets.iter().enumerate() {
            let active = self.theme_id == *id;
            let btn = if active {
                PrimaryButton::new(*label).size(ButtonSize::Sm).into_any_element()
            } else {
                SecondaryButton::new(*label).size(ButtonSize::Sm).into_any_element()
            };
            let target_id = *id;
            theme_row = theme_row.child(
                div()
                    .id(("preset-btn", idx))
                    .on_click(cx.listener(move |host, _ev, _win, cx| {
                        host.dispatch(ShowcaseMessage::SelectTheme(target_id), cx);
                    }))
                    .child(btn),
            );
        }

        div()
            .relative()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .justify_between()
            .child(div().absolute().inset_0().child(OpticalBackdrop::new()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(10.0))
                    .child(WindowTitlebar::new(format!(
                        "Refractor UI for Rust — 339 Bespoke Native GPUI Components ({})",
                        theme.display_name
                    )))
                    .child(
                        div()
                            .px(px(18.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(
                                TopNavBar::new("Refractor Studio Showcase")
                                    .subtitle("All 10 Bespoke Domain Suites • 339 Native GPUI Components • GPUI 0.2.2"),
                            )
                            .child(theme_row)
                            // Row 1: GPU-Canvas Charts (Suite 2)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(CandlestickOhlcChart::new()))
                                    .child(div().flex_1().min_w_0().child(MatrixHeatmap::new()))
                                    .child(div().flex_1().min_w_0().child(ScatterBubblePlot::new()))
                                    .child(div().flex_1().min_w_0().child(RadialGauge::new().value("97.4% Cov"))),
                            )
                            // Row 2: Pro NLE/DAW/Audio (Suite 3)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(MultiTrackTimeline::new()))
                                    .child(div().flex_1().min_w_0().child(ParametricEqGraph::new()))
                                    .child(div().flex_1().min_w_0().child(VuPeakLevelMeter::new()))
                                    .child(div().flex_1().min_w_0().child(ThreeWayColorWheels::new())),
                            )
                            // Row 3: AI Agents, CLI & DevOps Observability (Suite 4)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(ContextTokenMeter::new()))
                                    .child(div().flex_1().min_w_0().child(DistributedTraceWaterfall::new()))
                                    .child(div().flex_1().min_w_0().child(ApprovalGateCard::new()))
                                    .child(div().flex_1().min_w_0().child(UptimeStatusBar::new())),
                            )
                            // Row 4: Forms, Enterprise, Desktop OS & Hardware (Suites 1, 5, 6, 7)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(CodeEditor::new()))
                                    .child(div().flex_1().min_w_0().child(OpticalMaterialInspector::new()))
                                    .child(div().flex_1().min_w_0().child(DualPaneCommander::new()))
                                    .child(div().flex_1().min_w_0().child(DigitalLedReadout::new())),
                            )
                            // Row 5: Additional Enterprise, OS & Hardware Tactile Surfaces
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(AdvancedDataGrid::new()))
                                    .child(div().flex_1().min_w_0().child(ColorPicker::new()))
                                    .child(div().flex_1().min_w_0().child(PasswordStrengthMeter::new()))
                                    .child(div().flex_1().min_w_0().child(ControlCenterGrid::new()))
                                    .child(div().flex_1().min_w_0().child(FileTransferQueue::new()))
                                    .child(div().flex_1().min_w_0().child(StepSequencerMatrix::new())),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(RatingControl::new()))
                                    .child(div().flex_1().min_w_0().child(RotaryDial::new()))
                                    .child(div().flex_1().min_w_0().child(ThermostatDial::new())),
                            )
                            // Row 6: Dynamic Gradient & AI-Glow FX, Shader FX & Loaders (Suites 8-10)
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .gap(px(10.0))
                                    .child(div().flex_1().min_w_0().child(PrismaticCursorHalo::new()))
                                    .child(div().flex_1().min_w_0().child(AiFocusRing::new("Optical AI Prompt").active(true).processing(true)))
                                    .child(div().flex_1().min_w_0().child(RefractiveLensContainer::new("Optical Lens")))
                                    .child(div().flex_1().min_w_0().child(HolographicFoilSheen::new("Dichroic Foil"))),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(10.0))
                                    .child(AiThinkingOrb::new(AiOrbState::Thinking).size_px(44.0))
                                    .child(div().flex_1().min_w_0().child(SegmentedGlassProgress::new(72.0)))
                                    .child(AcousticActivityBar::new())
                                    .child(div().flex_1().min_w_0().child(PipelinePulseLoader::new()))
                                    .child(div().w(px(140.0)).child(StatefulEnergyRibbon::new(EnergyRibbonState::Streaming).show_label(true))),
                            ),
                    ),
            )
            .child(WorkspaceStatusBar::new(
                format!(
                    "Theme: {} | Material: {} | Overlay: {}",
                    theme.display_name,
                    theme.material.as_str(),
                    theme.fx.wallpaper_overlay.as_str()
                ),
                "Refractor UI v1.1.0 • 339 Bespoke Native GPUI Components",
            ))
            .into_any_element()
    }
}

fn main() {
    run_app::<StudioShowcase>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_showcase_switches_themes_cleanly() {
        let (mut app, _) = StudioShowcase::new();
        let _ = app.update(ShowcaseMessage::SelectTheme("cyberpunk"));
        assert_eq!(app.theme().name, "cyberpunk");
        let _ = app.update(ShowcaseMessage::SelectTheme("windows95"));
        assert_eq!(app.theme().name, "windows95");
    }
}
