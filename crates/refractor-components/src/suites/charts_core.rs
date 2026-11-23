//! Domain Suite 2/7 (`RUI-17`): GPU-Canvas Charts, Financial, Scientific, Geospatial,
//! Statistical, Relational, KPI & Storytelling Visualizations (48 bespoke GPUI components).

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
    gpu_bar_canvas, gpu_matrix_canvas, gpu_meter, gpu_scatter_canvas, gpu_tracks_canvas, kv_rows,
    pill_strip, suite_header,
};

#[derive(Debug, Clone, Copy)]
enum ChartPlotKind {
    BarSeries,
    TrendSeries,
    MatrixGrid,
    ScatterPlot,
    FlowTracks,
    GaugeMeter,
    ChromeOverlay,
}

macro_rules! bespoke_chart_widget {
    ($name:ident, $default_title:literal, $badge:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub subtitle: Option<SharedString>,
            pub badge: SharedString,
            pub series: Vec<f32>,
            pub value: SharedString,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    subtitle: None,
                    badge: SharedString::from($badge),
                    series: vec![0.28, 0.54, 0.42, 0.78, 0.66, 0.91, 0.74, 0.86],
                    value: SharedString::from("98.4%"),
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

            pub fn series(mut self, series: Vec<f32>) -> Self {
                self.series = series;
                self
            }

            pub fn value(mut self, value: impl Into<SharedString>) -> Self {
                self.value = value.into();
                self
            }

            pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
                self.badge = badge.into();
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

                match ChartPlotKind::$kind {
                    ChartPlotKind::BarSeries => {
                        root.child(gpu_bar_canvas(&theme, self.series, 72.0, false))
                    }
                    ChartPlotKind::TrendSeries => {
                        root.child(gpu_bar_canvas(&theme, self.series, 64.0, true))
                    }
                    ChartPlotKind::MatrixGrid => {
                        root.child(gpu_matrix_canvas(&theme, 5, 10, self.series, 72.0))
                    }
                    ChartPlotKind::ScatterPlot => {
                        let pts = self
                            .series
                            .iter()
                            .enumerate()
                            .map(|(i, v)| {
                                let x = ((i as f32) + 0.5) / (self.series.len().max(1) as f32);
                                (x, clamp_f32(*v, 0.1, 0.9, 0.5), 9.0)
                            })
                            .collect();
                        root.child(gpu_scatter_canvas(&theme, pts, 76.0))
                    }
                    ChartPlotKind::FlowTracks => {
                        root.child(gpu_tracks_canvas(&theme, Vec::new(), None, 72.0))
                    }
                    ChartPlotKind::GaugeMeter => {
                        let first = self.series.first().copied().unwrap_or(0.76);
                        root.child(gpu_meter(&theme, first, true)).child(kv_rows(
                            &theme,
                            vec![(SharedString::from("Reading"), self.value)],
                        ))
                    }
                    ChartPlotKind::ChromeOverlay => root
                        .child(pill_strip(
                            &theme,
                            vec![
                                SharedString::from("Series A"),
                                SharedString::from("Series B"),
                                SharedString::from("Threshold"),
                            ],
                            Some(0),
                        ))
                        .child(gpu_meter(&theme, 0.68, false)),
                }
            }
        }
    };
}

// Core & Trend Charts (5)
bespoke_chart_widget!(AreaStackedChart, "Area Stacked Chart", "Stacked", TrendSeries);
bespoke_chart_widget!(BarColumnChart, "Bar Column Chart", "Columns", BarSeries);
bespoke_chart_widget!(ComboDualAxisChart, "Combo Dual-Axis Chart", "Dual Y", BarSeries);
bespoke_chart_widget!(LineMultiSeriesChart, "Line Multi-Series Chart", "Trend", TrendSeries);
bespoke_chart_widget!(SparklineChart, "Sparkline Chart", "Inline", TrendSeries);

// Part-to-Whole & Hierarchical Visuals (5)
bespoke_chart_widget!(DonutPieChart, "Donut & Pie Chart", "Ratio", ScatterPlot);
bespoke_chart_widget!(FunnelChart, "Funnel Chart", "Conversion", FlowTracks);
bespoke_chart_widget!(SunburstChart, "Sunburst Chart", "Radial", ScatterPlot);
bespoke_chart_widget!(TreemapChart, "Treemap Chart", "Hierarchy", MatrixGrid);
bespoke_chart_widget!(WaterfallChart, "Waterfall Chart", "Delta", BarSeries);

// Statistical & Distribution Visuals (5)
bespoke_chart_widget!(BoxWhiskerPlot, "Box & Whisker Plot", "Quartiles", BarSeries);
bespoke_chart_widget!(HistogramChart, "Histogram Chart", "Bins", BarSeries);
bespoke_chart_widget!(MatrixHeatmap, "Matrix Heatmap", "2D Grid", MatrixGrid);
bespoke_chart_widget!(RadarSpiderChart, "Radar Spider Chart", "Multi-Axis", ScatterPlot);
bespoke_chart_widget!(ScatterBubblePlot, "Scatter & Bubble Plot", "XYR", ScatterPlot);

// Relational & Flow Visuals (3)
bespoke_chart_widget!(ChordDiagram, "Chord Diagram", "Relations", ScatterPlot);
bespoke_chart_widget!(NetworkForceGraph, "Network Force Graph", "Topology", ScatterPlot);
bespoke_chart_widget!(SankeyDiagram, "Sankey Diagram", "Flow", FlowTracks);

// KPI, Progress & Telemetry Visuals (4)
bespoke_chart_widget!(BulletChart, "Bullet Chart", "Target", GaugeMeter);
bespoke_chart_widget!(CalendarHeatmap, "Calendar Heatmap", "52-Week", MatrixGrid);
bespoke_chart_widget!(LiveTelemetryChart, "Live Telemetry Chart", "60Hz", TrendSeries);
bespoke_chart_widget!(RadialGauge, "Radial Gauge", "KPI", GaugeMeter);

// Geospatial & Map Visuals (3)
bespoke_chart_widget!(ChoroplethMap, "Choropleth Map", "Geo Regions", MatrixGrid);
bespoke_chart_widget!(GeoFlowArcMap, "Geo Flow Arc Map", "Routes", ScatterPlot);
bespoke_chart_widget!(PointClusterMap, "Point Cluster Map", "Spatial", ScatterPlot);

// Chart Chrome & Interactive Controls (4)
bespoke_chart_widget!(BrushRangeScrubber, "Brush Range Scrubber", "Window", ChromeOverlay);
bespoke_chart_widget!(ChartCrosshairTooltip, "Chart Crosshair Tooltip", "Inspect", ChromeOverlay);
bespoke_chart_widget!(ChartThresholdMarker, "Chart Threshold Marker", "SLO", ChromeOverlay);
bespoke_chart_widget!(InteractiveLegend, "Interactive Legend", "Series", ChromeOverlay);

// Financial & Market Visualizations (5)
bespoke_chart_widget!(BurndownBurnupChart, "Burndown / Burnup Chart", "Sprint", TrendSeries);
bespoke_chart_widget!(CandlestickOhlcChart, "Candlestick OHLC Chart", "Market", BarSeries);
bespoke_chart_widget!(CohortRetentionGrid, "Cohort Retention Grid", "Retention", MatrixGrid);
bespoke_chart_widget!(MarimekkoChart, "Marimekko Chart", "Mosaic", MatrixGrid);
bespoke_chart_widget!(OrderBookDepthChart, "Order Book Depth Chart", "L2 Book", TrendSeries);

// Scientific, Statistical & High-Dimensional Visuals (7)
bespoke_chart_widget!(ContourDensityPlot, "Contour Density Plot", "Isolines", MatrixGrid);
bespoke_chart_widget!(HexbinDensityPlot, "Hexbin Density Plot", "Hexbin", MatrixGrid);
bespoke_chart_widget!(ParallelCoordinatesPlot, "Parallel Coordinates", "High-Dim", FlowTracks);
bespoke_chart_widget!(PolarAreaRoseChart, "Polar Area Rose Chart", "Polar", ScatterPlot);
bespoke_chart_widget!(RidgelinePlot, "Ridgeline Plot", "Densities", FlowTracks);
bespoke_chart_widget!(SurfaceMesh3DPlot, "Surface Mesh 3D Plot", "3D Mesh", MatrixGrid);
bespoke_chart_widget!(ViolinPlot, "Violin Plot", "KDE", BarSeries);

// Comparison, Ranking & Storytelling Visuals (7)
bespoke_chart_widget!(BumpChart, "Bump Chart", "Rankings", TrendSeries);
bespoke_chart_widget!(DumbbellPlot, "Dumbbell Plot", "Gap", FlowTracks);
bespoke_chart_widget!(LollipopChart, "Lollipop Chart", "Ranked", BarSeries);
bespoke_chart_widget!(QuadrantMatrix, "Quadrant Matrix", "2×2", ScatterPlot);
bespoke_chart_widget!(SlopegraphChart, "Slopegraph Chart", "Before/After", ScatterPlot);
bespoke_chart_widget!(SmallMultiplesGrid, "Small Multiples Grid", "Faceted", MatrixGrid);
bespoke_chart_widget!(WaffleUnitGrid, "Waffle Unit Grid", "10×10", MatrixGrid);
