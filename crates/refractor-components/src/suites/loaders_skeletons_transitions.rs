//! Suite 10: Loaders, Skeletons & Data Transitions (9 components — `refractor-ui` v1.52.0 parity).
//!
//! Components:
//! 1. [`RefractiveSkeleton`]
//! 2. [`OrbitalRingSpinner`]
//! 3. [`AiThinkingOrb`]
//! 4. [`SegmentedGlassProgress`]
//! 5. [`StreamingTokenReveal`]
//! 6. [`DataGridRowShimmer`]
//! 7. [`AcousticActivityBar`]
//! 8. [`PipelinePulseLoader`]
//! 9. [`ScanlineRadarSweep`]

use gpui::{
    App, BorderStyle, Bounds, BoxShadow, Corners, Edges, FontWeight, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, fill, linear_color_stop,
    linear_gradient, point, px, quad, relative, size,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface, rgb_to_hsla,
};
use refractor_security::clamp_f32;
use refractor_tokens::RgbColor;

// ─────────────────────────────────────────────────────────────────────────────
// 1. RefractiveSkeleton
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonPreset {
    #[default]
    Card,
    Profile,
    Article,
    Media,
    Metrics,
}

/// Layout-matching frosted glass skeleton placeholder swept by an angled prismatic light wave.
#[derive(Debug, Clone, IntoElement)]
pub struct RefractiveSkeleton {
    pub loading: bool,
    pub preset: SkeletonPreset,
    pub lines: usize,
    pub animated: bool,
    pub speed_seconds: f32,
    pub base: BaseProps,
}

impl Default for RefractiveSkeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl RefractiveSkeleton {
    pub const fn component_name() -> &'static str {
        "RefractiveSkeleton"
    }

    pub fn new() -> Self {
        Self {
            loading: true,
            preset: SkeletonPreset::Card,
            lines: 3,
            animated: true,
            speed_seconds: 1.85,
            base: BaseProps::new(),
        }
    }

    pub const fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub const fn preset(mut self, preset: SkeletonPreset) -> Self {
        self.preset = preset;
        self
    }

    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = lines.clamp(1, 12);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn speed_seconds(mut self, sec: f32) -> Self {
        self.speed_seconds = clamp_f32(sec, 0.4, 10.0, 1.85);
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for RefractiveSkeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let shimmer_a = rgb_to_hsla(theme.palette.primary.rgb, 0.14);
        let shimmer_b = rgb_to_hsla(theme.palette.secondary.rgb, 0.18);

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .p(px(14.0))
            .flex()
            .flex_col()
            .gap(px(8.0));

        if !self.loading {
            return root.child(
                div()
                    .text_size(px(12.0))
                    .text_color(color_str_to_hsla(
                        &theme.palette.text.secondary,
                        RgbColor::new(71, 85, 105),
                    ))
                    .child("Content loaded"),
            );
        }

        if matches!(self.preset, SkeletonPreset::Profile | SkeletonPreset::Media) {
            root = root.child(
                div()
                    .w(if self.preset == SkeletonPreset::Profile {
                        px(40.0)
                    } else {
                        px(180.0)
                    })
                    .h(if self.preset == SkeletonPreset::Profile {
                        px(40.0)
                    } else {
                        px(72.0)
                    })
                    .rounded(px(if self.preset == SkeletonPreset::Profile {
                        9999.0
                    } else {
                        10.0
                    }))
                    .bg(linear_gradient(
                        115.0,
                        linear_color_stop(shimmer_a, 0.0),
                        linear_color_stop(shimmer_b, 1.0),
                    )),
            );
        }

        for idx in 0..self.lines {
            let width_frac = match idx % 3 {
                0 => 0.92,
                1 => 0.76,
                _ => 0.58,
            };
            root = root.child(
                div()
                    .w(relative(width_frac))
                    .h(px(10.0))
                    .rounded(px(9999.0))
                    .bg(linear_gradient(
                        110.0,
                        linear_color_stop(shimmer_a, 0.0),
                        linear_color_stop(shimmer_b, 1.0),
                    )),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. OrbitalRingSpinner
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl SpinnerSize {
    pub const fn diameter_px(self) -> f32 {
        match self {
            Self::Sm => 20.0,
            Self::Md => 32.0,
            Self::Lg => 48.0,
            Self::Xl => 64.0,
        }
    }
}

/// Variable-stroke circular orbital spinner with multi-hue gradient tail and leading bead.
#[derive(Debug, Clone, IntoElement)]
pub struct OrbitalRingSpinner {
    pub size: SpinnerSize,
    pub stroke_width: f32,
    pub speed_seconds: f32,
    pub label: SharedString,
    pub show_label: bool,
    pub paused: bool,
    pub base: BaseProps,
}

impl Default for OrbitalRingSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl OrbitalRingSpinner {
    pub const fn component_name() -> &'static str {
        "OrbitalRingSpinner"
    }

    pub fn new() -> Self {
        Self {
            size: SpinnerSize::Md,
            stroke_width: 4.5,
            speed_seconds: 1.1,
            label: "Loading...".into(),
            show_label: false,
            paused: false,
            base: BaseProps::new(),
        }
    }

    pub const fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }

    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = clamp_f32(width, 1.5, 10.0, 4.5);
        self
    }

    pub fn speed_seconds(mut self, sec: f32) -> Self {
        self.speed_seconds = clamp_f32(sec, 0.25, 10.0, 1.1);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub const fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub const fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for OrbitalRingSpinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let d = self.size.diameter_px();
        let sw = self.stroke_width;
        let primary = theme.palette.primary.rgb;
        let secondary = theme.palette.secondary.rgb;
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));

        let ring = div()
            .w(px(d))
            .h(px(d))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        window.paint_quad(quad(
                            bounds,
                            Corners::all(px(d * 0.5)),
                            rgb_to_hsla(primary, 0.0),
                            Edges::all(px(sw)),
                            rgb_to_hsla(secondary, 0.35),
                            BorderStyle::Solid,
                        ));
                        let bead_d = (sw * 1.45).max(5.0);
                        let bead = Bounds::new(
                            point(
                                bounds.origin.x + px(d * 0.5 - bead_d * 0.5),
                                bounds.origin.y,
                            ),
                            size(px(bead_d), px(bead_d)),
                        );
                        window.paint_quad(quad(
                            bead,
                            Corners::all(px(bead_d * 0.5)),
                            rgb_to_hsla(primary, 0.98),
                            Edges::all(px(0.0)),
                            rgb_to_hsla(primary, 0.0),
                            BorderStyle::Solid,
                        ));
                    },
                )
                .size_full(),
            );

        let mut root = div().flex().flex_row().items_center().gap(px(8.0)).child(ring);
        if self.show_label {
            root = root.child(div().text_size(px(12.0)).text_color(muted).child(self.label));
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. AiThinkingOrb
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AiOrbState {
    Idle,
    Listening,
    #[default]
    Thinking,
    Generating,
    Complete,
}

impl AiOrbState {
    pub const fn default_label(self) -> &'static str {
        match self {
            Self::Idle => "Ready",
            Self::Listening => "Listening...",
            Self::Thinking => "Thinking...",
            Self::Generating => "Generating...",
            Self::Complete => "Complete",
        }
    }

    pub const fn colors(self) -> (RgbColor, RgbColor) {
        match self {
            Self::Idle => (RgbColor::new(148, 163, 184), RgbColor::new(56, 189, 248)),
            Self::Listening => (RgbColor::new(56, 189, 248), RgbColor::new(34, 211, 238)),
            Self::Thinking => (RgbColor::new(139, 92, 246), RgbColor::new(56, 189, 248)),
            Self::Generating => (RgbColor::new(231, 92, 36), RgbColor::new(245, 158, 11)),
            Self::Complete => (RgbColor::new(16, 185, 129), RgbColor::new(52, 211, 153)),
        }
    }
}

/// Organic liquid morphing AI sphere pulsing and shifting semantic hue across agent states.
#[derive(Debug, Clone, IntoElement)]
pub struct AiThinkingOrb {
    pub state: AiOrbState,
    pub size_px: f32,
    pub label: Option<SharedString>,
    pub show_label: bool,
    pub disable_shader: bool,
    pub base: BaseProps,
}

impl Default for AiThinkingOrb {
    fn default() -> Self {
        Self::new(AiOrbState::Thinking)
    }
}

impl AiThinkingOrb {
    pub const fn component_name() -> &'static str {
        "AiThinkingOrb"
    }

    pub fn new(state: AiOrbState) -> Self {
        Self {
            state,
            size_px: 68.0,
            label: None,
            show_label: true,
            disable_shader: false,
            base: BaseProps::new(),
        }
    }

    pub fn size_px(mut self, size: f32) -> Self {
        self.size_px = clamp_f32(size, 24.0, 220.0, 68.0);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub const fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    pub const fn disable_shader(mut self, disable: bool) -> Self {
        self.disable_shader = disable;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for AiThinkingOrb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let (c1, c2) = self.state.colors();
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let d = self.size_px;

        let mut orb = div()
            .w(px(d))
            .h(px(d))
            .rounded(px(d * 0.5))
            .bg(linear_gradient(
                135.0,
                linear_color_stop(rgb_to_hsla(c1, 0.92), 0.0),
                linear_color_stop(rgb_to_hsla(c2, 0.88), 1.0),
            ));

        if !self.disable_shader && theme.optics.performance_mode.allows_fx_overlays() {
            orb = orb.shadow(vec![BoxShadow {
                color: rgb_to_hsla(c1, 0.42),
                offset: point(px(0.0), px(6.0)),
                blur_radius: px(18.0),
                spread_radius: px(-2.0),
            }]);
        }

        let mut root = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(8.0))
            .child(orb);

        if self.show_label {
            let txt: SharedString = self
                .label
                .unwrap_or_else(|| self.state.default_label().into());
            root = root.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(fg)
                    .child(txt),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. SegmentedGlassProgress
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressTone {
    #[default]
    Prism,
    Emerald,
    Cobalt,
    Amber,
    Critical,
}

/// Recessed glass pill track filling with a glowing internal liquid gradient across 1–20 segments.
#[derive(Debug, Clone, IntoElement)]
pub struct SegmentedGlassProgress {
    pub value: f32,
    pub max: f32,
    pub segments: usize,
    pub tone: ProgressTone,
    pub show_percentage: bool,
    pub indeterminate: bool,
    pub base: BaseProps,
}

impl Default for SegmentedGlassProgress {
    fn default() -> Self {
        Self::new(64.0)
    }
}

impl SegmentedGlassProgress {
    pub const fn component_name() -> &'static str {
        "SegmentedGlassProgress"
    }

    pub fn new(value: f32) -> Self {
        Self {
            value: clamp_f32(value, 0.0, 100.0, 0.0),
            max: 100.0,
            segments: 10,
            tone: ProgressTone::Prism,
            show_percentage: true,
            indeterminate: false,
            base: BaseProps::new(),
        }
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = clamp_f32(max, 1.0, 1_000_000.0, 100.0);
        self.value = clamp_f32(self.value, 0.0, self.max, 0.0);
        self
    }

    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments.clamp(1, 20);
        self
    }

    pub const fn tone(mut self, tone: ProgressTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn show_percentage(mut self, show: bool) -> Self {
        self.show_percentage = show;
        self
    }

    pub const fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn percentage(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        clamp_f32((self.value / self.max) * 100.0, 0.0, 100.0, 0.0)
    }

    pub fn filled_segments(&self) -> usize {
        if self.indeterminate {
            return (self.segments / 3).max(1);
        }
        let ratio = self.percentage() / 100.0;
        ((ratio * (self.segments as f32)).round() as usize).min(self.segments)
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for SegmentedGlassProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let fill_rgb = match self.tone {
            ProgressTone::Prism => theme.palette.primary.rgb,
            ProgressTone::Emerald => theme.palette.success.rgb,
            ProgressTone::Cobalt => theme.palette.info.rgb,
            ProgressTone::Amber => theme.palette.warning.rgb,
            ProgressTone::Critical => theme.palette.error.rgb,
        };
        let filled = self.filled_segments();
        let pct = self.percentage();

        let mut track = optical_surface(&theme, SurfaceVariant::Well)
            .w_full()
            .p(px(4.0))
            .rounded(px(9999.0))
            .flex()
            .flex_row()
            .gap(px(4.0));

        for idx in 0..self.segments {
            let active = idx < filled;
            track = track.child(
                div()
                    .flex_1()
                    .h(px(10.0))
                    .rounded(px(9999.0))
                    .bg(if active {
                        rgb_to_hsla(fill_rgb, 0.92)
                    } else {
                        rgb_to_hsla(fill_rgb, 0.12)
                    }),
            );
        }

        let mut root = div().w_full().flex().flex_col().gap(px(4.0)).child(track);
        if self.show_percentage {
            root = root.child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(fg)
                    .child(if self.indeterminate {
                        "Streaming...".to_owned()
                    } else {
                        format!("{pct:.0}%")
                    }),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. StreamingTokenReveal
// ─────────────────────────────────────────────────────────────────────────────

/// Progressive token/typewriter reveal wrapper with leading spectrum caret & instant skip.
#[derive(Debug, Clone, IntoElement)]
pub struct StreamingTokenReveal {
    pub text: SharedString,
    pub streaming: bool,
    pub reveal_rate_ms: u32,
    pub chunk_size: usize,
    pub skip_animation: bool,
    pub revealed_tokens: Option<usize>,
    pub base: BaseProps,
}

impl Default for StreamingTokenReveal {
    fn default() -> Self {
        Self::new("Refractor streams tokens at 60fps with optical caret luminance.")
    }
}

impl StreamingTokenReveal {
    pub const fn component_name() -> &'static str {
        "StreamingTokenReveal"
    }

    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            streaming: false,
            reveal_rate_ms: 45,
            chunk_size: 1,
            skip_animation: false,
            revealed_tokens: None,
            base: BaseProps::new(),
        }
    }

    pub const fn streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    pub fn reveal_rate_ms(mut self, ms: u32) -> Self {
        self.reveal_rate_ms = ms.clamp(10, 1000);
        self
    }

    pub fn chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size.clamp(1, 32);
        self
    }

    pub const fn skip_animation(mut self, skip: bool) -> Self {
        self.skip_animation = skip;
        self
    }

    pub const fn revealed_tokens(mut self, count: usize) -> Self {
        self.revealed_tokens = Some(count);
        self
    }

    /// Computes the currently visible token prefix (respecting `skip_animation`).
    pub fn visible_text(&self) -> String {
        if self.skip_animation {
            return self.text.to_string();
        }
        match self.revealed_tokens {
            None => self.text.to_string(),
            Some(n) => self
                .text
                .split_whitespace()
                .take(n)
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for StreamingTokenReveal {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let visible = self.visible_text();

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .px(px(12.0))
            .py(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .child(div().text_size(px(13.0)).text_color(fg).child(visible));

        if self.streaming && !self.skip_animation {
            root = root.child(
                div()
                    .w(px(3.0))
                    .h(px(15.0))
                    .rounded(px(2.0))
                    .bg(linear_gradient(
                        180.0,
                        linear_color_stop(rgb_to_hsla(theme.palette.primary.rgb, 0.98), 0.0),
                        linear_color_stop(rgb_to_hsla(theme.palette.secondary.rgb, 0.98), 1.0),
                    )),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. DataGridRowShimmer
// ─────────────────────────────────────────────────────────────────────────────

/// Table-specific optical skeleton animating rows in a staggered top-to-bottom prismatic wave.
#[derive(Debug, Clone, IntoElement)]
pub struct DataGridRowShimmer {
    pub loading: bool,
    pub rows: usize,
    pub columns: Vec<SharedString>,
    pub stagger_ms: u32,
    pub animated: bool,
    pub base: BaseProps,
}

impl Default for DataGridRowShimmer {
    fn default() -> Self {
        Self::new()
    }
}

impl DataGridRowShimmer {
    pub const fn component_name() -> &'static str {
        "DataGridRowShimmer"
    }

    pub fn new() -> Self {
        Self {
            loading: true,
            rows: 5,
            columns: vec![
                "Artifact".into(),
                "Status".into(),
                "Latency".into(),
                "Updated".into(),
            ],
            stagger_ms: 90,
            animated: true,
            base: BaseProps::new(),
        }
    }

    pub const fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.clamp(1, 24);
        self
    }

    pub fn columns(mut self, cols: Vec<SharedString>) -> Self {
        if !cols.is_empty() {
            self.columns = cols;
        }
        self
    }

    pub fn stagger_ms(mut self, ms: u32) -> Self {
        self.stagger_ms = ms.clamp(0, 500);
        self
    }

    pub const fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for DataGridRowShimmer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let col_count = self.columns.len().max(1);

        let mut header = div().flex().flex_row().gap(px(8.0)).pb(px(6.0));
        for col in self.columns {
            header = header.child(
                div()
                    .flex_1()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(muted)
                    .child(col),
            );
        }

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .p(px(12.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(header);

        if self.loading {
            for r in 0..self.rows {
                let alpha = (0.14 + (r % 3) as f32 * 0.04).min(0.28);
                let mut row = div().flex().flex_row().gap(px(8.0));
                for _ in 0..col_count {
                    row = row.child(
                        div()
                            .flex_1()
                            .h(px(12.0))
                            .rounded(px(6.0))
                            .bg(rgb_to_hsla(theme.palette.secondary.rgb, alpha)),
                    );
                }
                root = root.child(row);
            }
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. AcousticActivityBar
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcousticTone {
    #[default]
    Prism,
    Emerald,
    Cobalt,
    Amber,
}

/// Compact row of animated vertical glass equalizer bars for voice input, audio DSP, or telemetry.
#[derive(Debug, Clone, IntoElement)]
pub struct AcousticActivityBar {
    pub active: bool,
    pub bar_count: usize,
    pub levels: Option<Vec<f32>>,
    pub height_px: f32,
    pub tone: AcousticTone,
    pub show_label: bool,
    pub base: BaseProps,
}

impl Default for AcousticActivityBar {
    fn default() -> Self {
        Self::new()
    }
}

impl AcousticActivityBar {
    pub const fn component_name() -> &'static str {
        "AcousticActivityBar"
    }

    pub fn new() -> Self {
        Self {
            active: true,
            bar_count: 9,
            levels: None,
            height_px: 32.0,
            tone: AcousticTone::Prism,
            show_label: false,
            base: BaseProps::new(),
        }
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn bar_count(mut self, count: usize) -> Self {
        self.bar_count = count.clamp(3, 32);
        self
    }

    pub fn levels(mut self, levels: Vec<f32>) -> Self {
        self.levels = Some(levels);
        self
    }

    pub fn height_px(mut self, height: f32) -> Self {
        self.height_px = clamp_f32(height, 12.0, 128.0, 32.0);
        self
    }

    pub const fn tone(mut self, tone: AcousticTone) -> Self {
        self.tone = tone;
        self
    }

    pub const fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Returns sanitized normalized bar levels in `0.08..=1.0`.
    pub fn resolved_levels(&self) -> Vec<f32> {
        if let Some(custom) = &self.levels {
            return (0..self.bar_count)
                .map(|i| {
                    let v = custom.get(i).copied().unwrap_or(0.2);
                    clamp_f32(v, 0.08, 1.0, 0.2)
                })
                .collect();
        }
        (0..self.bar_count)
            .map(|i| {
                if self.active {
                    ((i as f32 * 0.85).sin().abs() * 0.72 + 0.24).clamp(0.12, 1.0)
                } else {
                    0.16
                }
            })
            .collect()
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for AcousticActivityBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let levels = self.resolved_levels();
        let bar_rgb = match self.tone {
            AcousticTone::Prism => theme.palette.primary.rgb,
            AcousticTone::Emerald => theme.palette.success.rgb,
            AcousticTone::Cobalt => theme.palette.info.rgb,
            AcousticTone::Amber => theme.palette.warning.rgb,
        };
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));
        let h_px = self.height_px;

        let mut bars = optical_surface(&theme, SurfaceVariant::Well)
            .px(px(10.0))
            .py(px(6.0))
            .flex()
            .flex_row()
            .items_end()
            .gap(px(4.0))
            .h(px(h_px + 12.0));

        for lv in levels {
            bars = bars.child(
                div()
                    .w(px(5.0))
                    .h(px((h_px * lv).max(4.0)))
                    .rounded(px(9999.0))
                    .bg(rgb_to_hsla(bar_rgb, if self.active { 0.92 } else { 0.35 })),
            );
        }

        let mut root = div().flex().flex_row().items_center().gap(px(8.0)).child(bars);
        if self.show_label {
            root = root.child(
                div()
                    .text_size(px(11.0))
                    .text_color(muted)
                    .child(if self.active { "Live Audio" } else { "Muted" }),
            );
        }

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. PipelinePulseLoader
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PipelineStageStatus {
    #[default]
    Pending,
    Active,
    Complete,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStageItem {
    pub id: SharedString,
    pub label: SharedString,
    pub status: PipelineStageStatus,
}

impl PipelineStageItem {
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        status: PipelineStageStatus,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status,
        }
    }
}

/// Connected node-and-track multi-stage loader where a glowing optical light packet travels between stages.
#[derive(Debug, Clone, IntoElement)]
pub struct PipelinePulseLoader {
    pub stages: Vec<PipelineStageItem>,
    pub pulse_speed_seconds: f32,
    pub label: SharedString,
    pub base: BaseProps,
}

impl Default for PipelinePulseLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelinePulseLoader {
    pub const fn component_name() -> &'static str {
        "PipelinePulseLoader"
    }

    pub fn default_stages() -> Vec<PipelineStageItem> {
        vec![
            PipelineStageItem::new("ingest", "Ingest", PipelineStageStatus::Complete),
            PipelineStageItem::new("compile", "Compile", PipelineStageStatus::Complete),
            PipelineStageItem::new("verify", "Verify", PipelineStageStatus::Active),
            PipelineStageItem::new("publish", "Publish", PipelineStageStatus::Pending),
        ]
    }

    pub fn new() -> Self {
        Self {
            stages: Self::default_stages(),
            pulse_speed_seconds: 1.5,
            label: "Pipeline execution status".into(),
            base: BaseProps::new(),
        }
    }

    pub fn stages(mut self, stages: Vec<PipelineStageItem>) -> Self {
        if !stages.is_empty() {
            self.stages = stages;
        }
        self
    }

    pub fn pulse_speed_seconds(mut self, sec: f32) -> Self {
        self.pulse_speed_seconds = clamp_f32(sec, 0.3, 10.0, 1.5);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for PipelinePulseLoader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(71, 85, 105));

        let mut track = div().flex().flex_row().items_center().gap(px(8.0));
        let total = self.stages.len();
        for (idx, st) in self.stages.into_iter().enumerate() {
            let dot_rgb = match st.status {
                PipelineStageStatus::Complete => theme.palette.success.rgb,
                PipelineStageStatus::Active => theme.palette.primary.rgb,
                PipelineStageStatus::Error => theme.palette.error.rgb,
                PipelineStageStatus::Pending => RgbColor::new(148, 163, 184),
            };
            track = track.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(
                        div()
                            .w(px(10.0))
                            .h(px(10.0))
                            .rounded(px(9999.0))
                            .bg(rgb_to_hsla(dot_rgb, 0.95)),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(if st.status == PipelineStageStatus::Active {
                                FontWeight::SEMIBOLD
                            } else {
                                FontWeight::NORMAL
                            })
                            .text_color(if st.status == PipelineStageStatus::Pending {
                                muted
                            } else {
                                fg
                            })
                            .child(st.label),
                    ),
            );
            if idx + 1 < total {
                track = track.child(
                    div()
                        .w(px(22.0))
                        .h(px(2.0))
                        .rounded(px(9999.0))
                        .bg(rgb_to_hsla(dot_rgb, 0.45)),
                );
            }
        }

        let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
            .px(px(14.0))
            .py(px(10.0))
            .flex()
            .flex_col()
            .gap(px(6.0))
            .child(div().text_size(px(11.0)).text_color(muted).child(self.label))
            .child(track);

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. ScanlineRadarSweep
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadarSweepMode {
    #[default]
    Vertical,
    Horizontal,
    Radar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadarSweepTone {
    #[default]
    Cyan,
    Prism,
    Emerald,
    Amber,
}

/// Thin luminous optical beam sweeping across preview canvases or code blocks during active scanning.
#[derive(Debug, Clone, IntoElement)]
pub struct ScanlineRadarSweep {
    pub active: bool,
    pub mode: RadarSweepMode,
    pub tone: RadarSweepTone,
    pub speed_seconds: f32,
    pub status_label: SharedString,
    pub base: BaseProps,
}

impl Default for ScanlineRadarSweep {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanlineRadarSweep {
    pub const fn component_name() -> &'static str {
        "ScanlineRadarSweep"
    }

    pub fn new() -> Self {
        Self {
            active: true,
            mode: RadarSweepMode::Vertical,
            tone: RadarSweepTone::Cyan,
            speed_seconds: 2.2,
            status_label: "Scanning optical surface...".into(),
            base: BaseProps::new(),
        }
    }

    pub const fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub const fn mode(mut self, mode: RadarSweepMode) -> Self {
        self.mode = mode;
        self
    }

    pub const fn tone(mut self, tone: RadarSweepTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn speed_seconds(mut self, sec: f32) -> Self {
        self.speed_seconds = clamp_f32(sec, 0.4, 12.0, 2.2);
        self
    }

    pub fn status_label(mut self, label: impl Into<SharedString>) -> Self {
        self.status_label = label.into();
        self
    }

    pub fn base(mut self, base: BaseProps) -> Self {
        self.base = base;
        self
    }
}

impl RenderOnce for ScanlineRadarSweep {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = active_theme(cx);
        let beam_rgb = match self.tone {
            RadarSweepTone::Cyan => RgbColor::new(34, 211, 238),
            RadarSweepTone::Prism => theme.palette.primary.rgb,
            RadarSweepTone::Emerald => theme.palette.success.rgb,
            RadarSweepTone::Amber => theme.palette.warning.rgb,
        };
        let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(15, 23, 42));
        let active = self.active && theme.optics.performance_mode.allows_fx_overlays();
        let mode = self.mode;

        let mut root = optical_surface(&theme, SurfaceVariant::Slab)
            .relative()
            .overflow_hidden()
            .min_h(px(72.0))
            .p(px(14.0))
            .child(
                canvas(
                    |_b, _w, _cx| {},
                    move |bounds: Bounds<Pixels>, (), window, _cx| {
                        if !active {
                            return;
                        }
                        let w: f32 = bounds.size.width.into();
                        let h: f32 = bounds.size.height.into();
                        let beam = match mode {
                            RadarSweepMode::Vertical => Bounds::new(
                                point(bounds.origin.x, bounds.origin.y + px(h * 0.46)),
                                size(bounds.size.width, px(2.5)),
                            ),
                            RadarSweepMode::Horizontal => Bounds::new(
                                point(bounds.origin.x + px(w * 0.46), bounds.origin.y),
                                size(px(2.5), bounds.size.height),
                            ),
                            RadarSweepMode::Radar => {
                                let d = w.min(h).max(36.0) * 0.8;
                                Bounds::new(
                                    point(
                                        bounds.origin.x + px(w * 0.5 - d * 0.5),
                                        bounds.origin.y + px(h * 0.5 - d * 0.5),
                                    ),
                                    size(px(d), px(d)),
                                )
                            }
                        };
                        if mode == RadarSweepMode::Radar {
                            window.paint_quad(quad(
                                beam,
                                Corners::all(px(999.0)),
                                rgb_to_hsla(beam_rgb, 0.10),
                                Edges::all(px(1.5)),
                                rgb_to_hsla(beam_rgb, 0.82),
                                BorderStyle::Solid,
                            ));
                        } else {
                            window.paint_quad(fill(beam, rgb_to_hsla(beam_rgb, 0.85)));
                        }
                    },
                )
                .absolute()
                .inset_0(),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(fg)
                    .child(self.status_label),
            );

        if let Some(ov) = theme.components.get(Self::component_name()) {
            root = apply_component_override(root, ov);
        }
        if let Some(ov) = &self.base.style_override {
            root = apply_component_override(root, ov);
        }
        root
    }
}
