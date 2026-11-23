//! Domain Suite 4/7 (`RUI-19`): AI Conversational, AI Agent Execution & Control,
//! AI Context & Memory, Multi-Agent Orchestration, CLI/Terminal & DevOps Observability
//! (34 bespoke GPUI components).

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
    gpu_bar_canvas, gpu_meter, gpu_scatter_canvas, gpu_tracks_canvas, kv_rows, pill_strip,
    suite_header,
};

#[derive(Debug, Clone, Copy)]
enum AiObsKind {
    AgentCard,
    TokenMeter,
    TraceWaterfall,
    TerminalBlock,
    StatusStrip,
    Topology,
    ApprovalGate,
}

macro_rules! bespoke_ai_obs_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub badge: SharedString,
            pub value: SharedString,
            pub ratio: f32,
            pub items: Vec<SharedString>,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    badge: SharedString::from($badge),
                    value: SharedString::from($default_val),
                    ratio: 0.72,
                    items: Vec::new(),
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

            pub fn ratio(mut self, ratio: f32) -> Self {
                self.ratio = clamp_f32(ratio, 0.0, 1.0, 0.5);
                self
            }

            pub fn items(mut self, items: Vec<SharedString>) -> Self {
                self.items = items;
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

                match AiObsKind::$kind {
                    AiObsKind::AgentCard => root.child(kv_rows(
                        &theme,
                        vec![
                            (SharedString::from("State"), self.value),
                            (SharedString::from("Confidence"), SharedString::from("98.2%")),
                        ],
                    )),
                    AiObsKind::TokenMeter => root
                        .child(gpu_meter(&theme, self.ratio, true))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    AiObsKind::TraceWaterfall => root
                        .child(gpu_tracks_canvas(&theme, Vec::new(), None, 64.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    AiObsKind::TerminalBlock => {
                        let bg = color_str_to_hsla(&theme.editor.background, RgbColor::new(15, 23, 42));
                        let code_fg = color_str_to_hsla(&theme.editor.text, RgbColor::new(226, 232, 240));
                        root.child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .bg(bg)
                                .p(px(10.0))
                                .text_size(px(12.0))
                                .text_color(code_fg)
                                .child(self.value),
                        )
                    }
                    AiObsKind::StatusStrip => root
                        .child(gpu_bar_canvas(
                            &theme,
                            vec![0.98, 0.99, 1.0, 0.96, 1.0, 1.0, 0.94, 1.0, 1.0, 0.99],
                            36.0,
                            true,
                        ))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    AiObsKind::Topology => root
                        .child(gpu_scatter_canvas(
                            &theme,
                            vec![(0.2, 0.5, 13.0), (0.5, 0.78, 15.0), (0.8, 0.45, 13.0)],
                            64.0,
                        ))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    AiObsKind::ApprovalGate => root
                        .child(div().text_size(px(12.0)).text_color(fg).child(self.value))
                        .child(pill_strip(
                            &theme,
                            vec![
                                SharedString::from("✓ Approve"),
                                SharedString::from("✕ Reject"),
                                SharedString::from("Inspect Diff"),
                            ],
                            Some(0),
                        )),
                }
            }
        }
    };
}

// AI & Conversational Interfaces (5)
bespoke_ai_obs_widget!(CitationChip, "Citation Chip", "Source", "[1] ADR-0003 §2.1", AgentCard);
bespoke_ai_obs_widget!(PromptComposer, "Prompt Composer", "Ctrl+Enter", "Ask Refractor agent...", ApprovalGate);
bespoke_ai_obs_widget!(ResponseActionBar, "Response Action Bar", "Actions", "Copy • Retry • Branch", ApprovalGate);
bespoke_ai_obs_widget!(SplitArtifactCanvas, "Split Artifact Canvas", "Artifact", "Live Rust Preview", TerminalBlock);
bespoke_ai_obs_widget!(ThinkingAccordion, "Thinking Accordion", "Reasoning", "Verified 314 components in 1.2s", AgentCard);

// AI Agent Execution & Control (7)
bespoke_ai_obs_widget!(AgentControlBar, "Agent Control Bar", "Running", "Pause • Step • Stop", ApprovalGate);
bespoke_ai_obs_widget!(AgentPlanList, "Agent Plan List", "7/7 Done", "All domain suites verified", AgentCard);
bespoke_ai_obs_widget!(ApprovalGateCard, "Approval Gate Card", "Gate", "Allow cargo publish --dry-run?", ApprovalGate);
bespoke_ai_obs_widget!(CheckpointRollbackStrip, "Checkpoint Rollback Strip", "Git", "cp-04 • v2.0.0-rc1", StatusStrip);
bespoke_ai_obs_widget!(InlineDiffReviewer, "Inline Diff Reviewer", "+42 -8", "crates/refractor-core/src/optical.rs", TerminalBlock);
bespoke_ai_obs_widget!(RecoveryRoutingCard, "Recovery Routing Card", "Fallback", "Auto-retry with strict diagnostics", AgentCard);
bespoke_ai_obs_widget!(ToolCallCard, "Tool Call Card", "MCP", "pinnacle_create_ticket -> 200 OK", TerminalBlock);

// AI Context, Memory & Configuration (6)
bespoke_ai_obs_widget!(AttachedContextBar, "Attached Context Bar", "4 Files", "Cargo.toml • ADR-0001..0006", ApprovalGate);
bespoke_ai_obs_widget!(ConfidenceIndicator, "Confidence Indicator", "Score", "98.6% Grounded", TokenMeter);
bespoke_ai_obs_widget!(ConnectedToolsDrawer, "Connected Tools Drawer", "MCP", "ai-memory • pinnacle • cargo", AgentCard);
bespoke_ai_obs_widget!(ContextTokenMeter, "Context Token Meter", "Tokens", "42,800 / 128,000 tokens", TokenMeter);
bespoke_ai_obs_widget!(MemoryPersonaInspector, "Memory & Persona Inspector", "Memory", "Principal Rust Architect", AgentCard);
bespoke_ai_obs_widget!(ModelParameterSwitcher, "Model Parameter Switcher", "Config", "temp=0.2 • top_p=0.95", AgentCard);

// Multi-Agent & Orchestration (4)
bespoke_ai_obs_widget!(AgentExecutionTrace, "Agent Execution Trace", "Spans", "6 spans • 142ms total", TraceWaterfall);
bespoke_ai_obs_widget!(AgentRosterRail, "Agent Roster Rail", "3 Active", "Architect • Coder • Reviewer", Topology);
bespoke_ai_obs_widget!(HandoffDivider, "Handoff Divider", "Handoff", "Architect → Rust Synthesizer", AgentCard);
bespoke_ai_obs_widget!(ParallelComparisonView, "Parallel Comparison View", "A / B", "Variant A vs Variant B", ApprovalGate);

// CLI & Terminal Surfaces (6)
bespoke_ai_obs_widget!(CommandInputBar, "Command Input Bar", "Shell", "$ cargo test --workspace", TerminalBlock);
bespoke_ai_obs_widget!(CopyableCommandBlock, "Copyable Command Block", "Copy", "cargo run -p refractor-cli -- theme list", TerminalBlock);
bespoke_ai_obs_widget!(LiveLogStreamer, "Live Log Streamer", "INFO", "[20:54:01] 39/39 tests passed", TerminalBlock);
bespoke_ai_obs_widget!(ProcessStatusPill, "Process Status Pill", "PID 4820", "Running • 0.8% CPU", TokenMeter);
bespoke_ai_obs_widget!(StructuredOutputViewer, "Structured Output Viewer", "JSON", "{ \"components\": 314, \"themes\": 68 }", TerminalBlock);
bespoke_ai_obs_widget!(TerminalMultiplexer, "Terminal Multiplexer", "2 Panes", "Pane 1: cargo watch • Pane 2: git", TerminalBlock);

// System, DevOps & Observability Widgets (6)
bespoke_ai_obs_widget!(AnomalyForecastChart, "Anomaly Forecast Chart", "ML Ops", "0 anomalies in last 24h", StatusStrip);
bespoke_ai_obs_widget!(CapacityAllocationStrip, "Capacity Allocation Strip", "Cluster", "CPU 64% • RAM 48% • GPU 22%", TokenMeter);
bespoke_ai_obs_widget!(DistributedTraceWaterfall, "Distributed Trace Waterfall", "OTel", "POST /render 11.4ms", TraceWaterfall);
bespoke_ai_obs_widget!(FlameGraphChart, "Flame Graph Chart", "Profiler", "optical_surface 1.2% self", TraceWaterfall);
bespoke_ai_obs_widget!(ServiceTopologyMap, "Service Topology Map", "Mesh", "4 services • 0 errors", Topology);
bespoke_ai_obs_widget!(UptimeStatusBar, "Uptime Status Bar", "99.99%", "90-Day SLA Operational", StatusStrip);
