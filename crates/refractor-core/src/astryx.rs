//! Astryx Open-Internals core contracts: `BaseProps`, `ComponentDoc`, all 55
//! `ComponentCategory` variants, `ThemeProps`, and `ControllableState`.

use std::collections::BTreeMap;

use refractor_tokens::ComponentStyleOverride;
use serde::{Deserialize, Serialize};

/// Shared base properties embedded by every Refractor component (Astryx Tier 4).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BaseProps {
    /// Stable semantic identifier (`refractor-<kebab>`).
    pub id: Option<String>,
    /// Accessible label for screen readers and keyboard focus narration.
    pub aria_label: Option<String>,
    /// Optional explicit style override merged on top of theme overrides.
    pub style_override: Option<ComponentStyleOverride>,
    /// Semantic `data-*` state attributes for diagnostics and testing.
    pub data_attrs: BTreeMap<String, String>,
    /// Disables interactive state without misleading hover affordances.
    pub disabled: bool,
}

impl BaseProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_style_override(mut self, style: ComponentStyleOverride) -> Self {
        self.style_override = Some(style);
        self
    }

    pub fn with_data_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data_attrs.insert(key.into(), value.into());
        self
    }
}

/// Deterministic `refractor-*` class and `data-*` state metadata (ported from `themeProps()`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeProps {
    pub class_name: String,
    pub state_attrs: BTreeMap<String, String>,
}

impl ThemeProps {
    pub fn new(kebab_name: &str) -> Self {
        Self {
            class_name: format!("refractor-{kebab_name}"),
            state_attrs: BTreeMap::new(),
        }
    }

    pub fn with_state(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.state_attrs
            .insert(format!("data-{}", key.into()), value.into());
        self
    }
}

/// Controlled or uncontrolled state helper (Rust equivalent of `useControllableState`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControllableState<T> {
    controlled: Option<T>,
    internal: T,
}

impl<T: Clone> ControllableState<T> {
    pub fn uncontrolled(default_value: T) -> Self {
        Self {
            controlled: None,
            internal: default_value,
        }
    }

    pub fn controlled(value: T) -> Self {
        Self {
            controlled: Some(value.clone()),
            internal: value,
        }
    }

    pub fn get(&self) -> &T {
        self.controlled.as_ref().unwrap_or(&self.internal)
    }

    pub fn set(&mut self, next: T) {
        if self.controlled.is_none() {
            self.internal = next;
        }
    }

    pub fn is_controlled(&self) -> bool {
        self.controlled.is_some()
    }
}

/// Best practice guidance item in `ComponentDoc`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BestPractice {
    /// `true` = Do, `false` = Don't.
    pub guidance: bool,
    pub description: &'static str,
}

/// WCAG 2.2 AA accessibility contract entry in `ComponentDoc`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct A11yContract {
    pub name: &'static str,
    pub criterion: &'static str,
    pub requirement: &'static str,
    pub description: &'static str,
}

/// Typed prop documentation entry in `ComponentDoc`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropDoc {
    pub name: &'static str,
    pub ty: &'static str,
    pub default_value: &'static str,
    pub description: &'static str,
}

/// Machine-readable documentation block consumed by `refractor-cli` and docs tooling
/// (Rust equivalent of `<ComponentName>.doc.mjs` from ADR-0001).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub display_name: &'static str,
    pub group: &'static str,
    pub category: ComponentCategory,
    pub keywords: &'static [&'static str],
    pub description: &'static str,
    pub best_practices: &'static [BestPractice],
    pub accessibility: &'static [A11yContract],
    pub props: &'static [PropDoc],
    pub example: &'static str,
}

/// All 58 component categories from `refractor-ui` v1.52.0 spanning all 339 components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentCategory {
    ButtonsActions,
    FormInputsSelection,
    Navigation,
    DataDisplayContent,
    FeedbackSystemStatus,
    OverlaysPopups,
    LayoutWorkspace,
    AdvancedDataVisualization,
    AdvancedSearchFiltering,
    RichInputContentCreation,
    InteractiveDragDrop,
    CollaborationGuidance,
    AiConversational,
    MediaAudioSpatial,
    SecurityAuthAccount,
    CommerceBillingConversion,
    MobileTouchSystem,
    CliTerminalSurfaces,
    AiAgentExecutionControl,
    AiContextMemoryConfig,
    MultiAgentOrchestration,
    DateTimeScheduling,
    NumberCurrencyFormatted,
    ComplexButtonActions,
    ComplexControlVariations,
    CoreTrendCharts,
    PartToWholeCharts,
    StatisticalDistributionCharts,
    RelationalFlowCharts,
    KpiProgressTelemetryCharts,
    GeospatialMapCharts,
    ChartChromeControls,
    TransportPlayback,
    TimelineSequencing,
    PhotoColorGrading,
    AudioMixingDsp,
    ViewportExportUtilities,
    FinancialMarketCharts,
    ScientificHighDimCharts,
    SystemDevopsObservability,
    ComparisonStorytellingCharts,
    FoundationalPrimitives,
    DesktopOsWindowing,
    DocumentKnowledgeSpreadsheet,
    Design3dShaderInspectors,
    EnterpriseWorkflowsDataOps,
    HardwareIotTactile,
    FileExplorerNavigation,
    FileManagementOperations,
    WindowAnatomyControls,
    WindowManagementTiling,
    MenusLaunchers,
    SystemTrayControlCenter,
    HardwareHudsOverlays,
    DesktopSurfacesSystemDialogs,
    GradientAiGlowFx,
    ShaderOpticalFx,
    LoadersSkeletonsTransitions,
}

impl ComponentCategory {
    /// All 58 categories in canonical catalog order.
    pub const ALL: [Self; 58] = [
        Self::ButtonsActions,
        Self::FormInputsSelection,
        Self::Navigation,
        Self::DataDisplayContent,
        Self::FeedbackSystemStatus,
        Self::OverlaysPopups,
        Self::LayoutWorkspace,
        Self::AdvancedDataVisualization,
        Self::AdvancedSearchFiltering,
        Self::RichInputContentCreation,
        Self::InteractiveDragDrop,
        Self::CollaborationGuidance,
        Self::AiConversational,
        Self::MediaAudioSpatial,
        Self::SecurityAuthAccount,
        Self::CommerceBillingConversion,
        Self::MobileTouchSystem,
        Self::CliTerminalSurfaces,
        Self::AiAgentExecutionControl,
        Self::AiContextMemoryConfig,
        Self::MultiAgentOrchestration,
        Self::DateTimeScheduling,
        Self::NumberCurrencyFormatted,
        Self::ComplexButtonActions,
        Self::ComplexControlVariations,
        Self::CoreTrendCharts,
        Self::PartToWholeCharts,
        Self::StatisticalDistributionCharts,
        Self::RelationalFlowCharts,
        Self::KpiProgressTelemetryCharts,
        Self::GeospatialMapCharts,
        Self::ChartChromeControls,
        Self::TransportPlayback,
        Self::TimelineSequencing,
        Self::PhotoColorGrading,
        Self::AudioMixingDsp,
        Self::ViewportExportUtilities,
        Self::FinancialMarketCharts,
        Self::ScientificHighDimCharts,
        Self::SystemDevopsObservability,
        Self::ComparisonStorytellingCharts,
        Self::FoundationalPrimitives,
        Self::DesktopOsWindowing,
        Self::DocumentKnowledgeSpreadsheet,
        Self::Design3dShaderInspectors,
        Self::EnterpriseWorkflowsDataOps,
        Self::HardwareIotTactile,
        Self::FileExplorerNavigation,
        Self::FileManagementOperations,
        Self::WindowAnatomyControls,
        Self::WindowManagementTiling,
        Self::MenusLaunchers,
        Self::SystemTrayControlCenter,
        Self::HardwareHudsOverlays,
        Self::DesktopSurfacesSystemDialogs,
        Self::GradientAiGlowFx,
        Self::ShaderOpticalFx,
        Self::LoadersSkeletonsTransitions,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ButtonsActions => "Buttons & Actions",
            Self::FormInputsSelection => "Form Inputs & Selection Controls",
            Self::Navigation => "Navigation",
            Self::DataDisplayContent => "Data Display & Content",
            Self::FeedbackSystemStatus => "Feedback & System Status",
            Self::OverlaysPopups => "Overlays & Popups",
            Self::LayoutWorkspace => "Layout & Workspace Controls",
            Self::AdvancedDataVisualization => "Advanced Data & Visualization",
            Self::AdvancedSearchFiltering => "Advanced Search & Filtering",
            Self::RichInputContentCreation => "Rich Input & Content Creation",
            Self::InteractiveDragDrop => "Interactive & Drag-and-Drop",
            Self::CollaborationGuidance => "Collaboration & Guidance",
            Self::AiConversational => "AI & Conversational Interfaces",
            Self::MediaAudioSpatial => "Media, Audio & Spatial Controls",
            Self::SecurityAuthAccount => "Security, Auth & Account Management",
            Self::CommerceBillingConversion => "Commerce, Billing & Conversion",
            Self::MobileTouchSystem => "Mobile, Touch & System Utilities",
            Self::CliTerminalSurfaces => "CLI & Terminal Surfaces",
            Self::AiAgentExecutionControl => "AI Agent Execution & Control",
            Self::AiContextMemoryConfig => "AI Context, Memory & Configuration",
            Self::MultiAgentOrchestration => "Multi-Agent & Orchestration",
            Self::DateTimeScheduling => "Date, Time & Scheduling Fields",
            Self::NumberCurrencyFormatted => "Number, Currency & Formatted Type Fields",
            Self::ComplexButtonActions => "Complex Button & Action Variations",
            Self::ComplexControlVariations => "Complex Variations of Common Controls",
            Self::CoreTrendCharts => "Core & Trend Charts",
            Self::PartToWholeCharts => "Part-to-Whole & Hierarchical Visuals",
            Self::StatisticalDistributionCharts => "Statistical & Distribution Visuals",
            Self::RelationalFlowCharts => "Relational & Flow Visuals",
            Self::KpiProgressTelemetryCharts => "KPI, Progress & Telemetry Visuals",
            Self::GeospatialMapCharts => "Geospatial & Map Visuals",
            Self::ChartChromeControls => "Chart Chrome & Interactive Controls",
            Self::TransportPlayback => "Transport & Playback Controls",
            Self::TimelineSequencing => "Timeline & Sequencing",
            Self::PhotoColorGrading => "Photo & Color Grading Controls",
            Self::AudioMixingDsp => "Audio Mixing & Signal Processing",
            Self::ViewportExportUtilities => "Viewport & Export Utilities",
            Self::FinancialMarketCharts => "Financial & Market Visualizations",
            Self::ScientificHighDimCharts => "Scientific, Statistical & High-Dimensional Visuals",
            Self::SystemDevopsObservability => "System, DevOps & Observability Widgets",
            Self::ComparisonStorytellingCharts => "Comparison, Ranking & Storytelling Visuals",
            Self::FoundationalPrimitives => "Foundational Primitives & Micro-Utilities",
            Self::DesktopOsWindowing => "Desktop OS & Windowing Metaphors",
            Self::DocumentKnowledgeSpreadsheet => "Document, Knowledge & Spreadsheet Tools",
            Self::Design3dShaderInspectors => "Design, 3D & Shader Inspector Controls",
            Self::EnterpriseWorkflowsDataOps => "Enterprise Workflows, Data Ops & API Tooling",
            Self::HardwareIotTactile => "Hardware, IoT & Tactile Surfaces",
            Self::FileExplorerNavigation => "File Explorer Navigation & Views",
            Self::FileManagementOperations => "File Management & Operations",
            Self::WindowAnatomyControls => "Window Anatomy & Controls",
            Self::WindowManagementTiling => "Window Management & Desktop Tiling",
            Self::MenusLaunchers => "Menus & Launchers",
            Self::SystemTrayControlCenter => "System Tray & Control Center",
            Self::HardwareHudsOverlays => "Hardware HUDs & System Overlays",
            Self::DesktopSurfacesSystemDialogs => "Desktop Surfaces & Native System Dialogs",
            Self::GradientAiGlowFx => "Dynamic Gradient & AI-Glow FX",
            Self::ShaderOpticalFx => "Shader & Optical FX",
            Self::LoadersSkeletonsTransitions => "Loaders, Skeletons & Data Transitions",
        }
    }
}
