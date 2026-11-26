//! Optical Shader Engine & High-Performance Frame Primitives (`refractor-ui` v1.48–v1.52 parity + Rust extensions).
//!
//! Provides:
//! - [`OpticalShaderKind`]: All 10 precision-bounded optical shaders (`FluidMesh`, `RefractiveLens`,
//!   `ChromaticDispersion`, `CausticLight`, `FrostedGrain`, `LiquidRipple`, `HolographicFoil`,
//!   `MetaballFluid`, `OpticalVignette`, `ThinkingOrb`) with both GLSL and native WGSL fragment source metadata.
//! - [`ShaderBudget`]: Enforces DPR capping (`<= 1.5`), dimension clamping (`<= 768px`, hard cap `1024px`),
//!   offscreen / hidden-window pause, `reduced_motion`, and `ThemePerformanceMode::Max` guards.
//! - [`ShaderUniforms`]: NaN/Infinity-safe normalized uniforms for deterministic GPU/CPU evaluation.
//! - [`OpticalShaderRegistry`]: Deterministic lifecycle tracker guaranteeing zero leaked shader handles.
//! - [`PointerCoalescer`]: Frame-budget pointer-event coalescer (Rust equivalent of `createCoalescedPointerMove`).
//! - [`TelemetryRingBuffer`]: Fixed-capacity, zero-reallocation bounded ring buffer for ripples/telemetry/audio.
//! - [`BlurHostContext`]: Nested blur-pass elision tracker matching `[data-refractor-blur-host="true"]`.

use std::collections::BTreeSet;

use refractor_security::clamp_f32;
use refractor_tokens::{PrismTheme, RgbColor, ThemeMode, ThemePerformanceMode};
use serde::{Deserialize, Serialize};

/// Maximum DPR allowed for optical shader backing surfaces (prevents 4K quadratic stalls).
pub const MAX_SHADER_DPR: f32 = 1.5;

/// Default maximum edge dimension in pixels for optical shader backing surfaces.
pub const DEFAULT_MAX_SHADER_DIMENSION_PX: u32 = 768;

/// Hard upper bound for optical shader edge dimension in pixels.
pub const HARD_MAX_SHADER_DIMENSION_PX: u32 = 1024;

/// All 10 precision-bounded optical shaders matching `refractor-ui` v1.51.0–v1.52.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OpticalShaderKind {
    FluidMesh,
    RefractiveLens,
    ChromaticDispersion,
    CausticLight,
    FrostedGrain,
    LiquidRipple,
    HolographicFoil,
    MetaballFluid,
    OpticalVignette,
    ThinkingOrb,
}

impl OpticalShaderKind {
    /// All 10 supported optical shader presets.
    pub const ALL: [Self; 10] = [
        Self::FluidMesh,
        Self::RefractiveLens,
        Self::ChromaticDispersion,
        Self::CausticLight,
        Self::FrostedGrain,
        Self::LiquidRipple,
        Self::HolographicFoil,
        Self::MetaballFluid,
        Self::OpticalVignette,
        Self::ThinkingOrb,
    ];

    /// Canonical kebab-case shader preset identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FluidMesh => "fluid-mesh",
            Self::RefractiveLens => "refractive-lens",
            Self::ChromaticDispersion => "chromatic-dispersion",
            Self::CausticLight => "caustic-light",
            Self::FrostedGrain => "frosted-grain",
            Self::LiquidRipple => "liquid-ripple",
            Self::HolographicFoil => "holographic-foil",
            Self::MetaballFluid => "metaball-fluid",
            Self::OpticalVignette => "optical-vignette",
            Self::ThinkingOrb => "thinking-orb",
        }
    }

    /// Default visual intensity in `0.0..=1.0`.
    pub const fn default_intensity(self) -> f32 {
        match self {
            Self::FluidMesh | Self::HolographicFoil | Self::MetaballFluid => 0.85,
            Self::RefractiveLens | Self::LiquidRipple => 0.90,
            Self::ChromaticDispersion => 0.88,
            Self::CausticLight => 0.72,
            Self::FrostedGrain => 0.14,
            Self::OpticalVignette => 0.58,
            Self::ThinkingOrb => 0.82,
        }
    }

    /// Returns a precision-bounded WGSL fragment shader entry snippet for native Rust GPU pipelines (wgpu/Metal/DX12).
    pub const fn wgsl_entry_point(self) -> &'static str {
        match self {
            Self::FluidMesh => "fs_fluid_mesh",
            Self::RefractiveLens => "fs_refractive_lens",
            Self::ChromaticDispersion => "fs_chromatic_dispersion",
            Self::CausticLight => "fs_caustic_light",
            Self::FrostedGrain => "fs_frosted_grain",
            Self::LiquidRipple => "fs_liquid_ripple",
            Self::HolographicFoil => "fs_holographic_foil",
            Self::MetaballFluid => "fs_metaball_fluid",
            Self::OpticalVignette => "fs_optical_vignette",
            Self::ThinkingOrb => "fs_thinking_orb",
        }
    }
}

/// DPR-capped, visibility-aware shader execution budget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderBudget {
    pub max_dpr: f32,
    pub max_dimension_px: u32,
    pub target_fps: u16,
    pub reduced_motion: bool,
    pub performance_mode: ThemePerformanceMode,
    pub visible: bool,
    pub window_active: bool,
    pub disabled: bool,
}

impl Default for ShaderBudget {
    fn default() -> Self {
        Self {
            max_dpr: MAX_SHADER_DPR,
            max_dimension_px: DEFAULT_MAX_SHADER_DIMENSION_PX,
            target_fps: 60,
            reduced_motion: false,
            performance_mode: ThemePerformanceMode::Balanced,
            visible: true,
            window_active: true,
            disabled: false,
        }
    }
}

impl ShaderBudget {
    /// Derives a safe [`ShaderBudget`] from the active [`PrismTheme`].
    pub fn from_theme(theme: &PrismTheme, reduced_motion: bool) -> Self {
        let performance_mode = theme.optics.performance_mode;
        let max_dimension_px = match performance_mode {
            ThemePerformanceMode::Balanced => DEFAULT_MAX_SHADER_DIMENSION_PX,
            ThemePerformanceMode::High => 512,
            ThemePerformanceMode::Max => 256,
        };
        let target_fps = match performance_mode {
            ThemePerformanceMode::Balanced => 60,
            ThemePerformanceMode::High => 30,
            ThemePerformanceMode::Max => 0,
        };
        Self {
            max_dpr: MAX_SHADER_DPR,
            max_dimension_px,
            target_fps,
            reduced_motion,
            performance_mode,
            visible: true,
            window_active: true,
            disabled: false,
        }
    }

    /// Returns `true` only when the shader is visible, window is active, motion is allowed,
    /// not disabled, and `performance_mode` permits animations.
    pub const fn should_animate(&self) -> bool {
        !self.disabled
            && self.visible
            && self.window_active
            && !self.reduced_motion
            && self.performance_mode.allows_animations()
            && self.target_fps > 0
    }

    /// Computes DPR-capped and dimension-clamped backing pixel dimensions `(width, height)`.
    /// Defends against NaN, Infinity, zero, negative, and 8K/16K adversarial inputs.
    pub fn effective_surface_size(&self, logical_width: f32, logical_height: f32, dpr: f32) -> (u32, u32) {
        let safe_w = clamp_f32(logical_width, 1.0, 16_384.0, 1.0);
        let safe_h = clamp_f32(logical_height, 1.0, 16_384.0, 1.0);
        let cap_dpr = clamp_f32(self.max_dpr, 0.5, MAX_SHADER_DPR, MAX_SHADER_DPR);
        let safe_dpr = clamp_f32(dpr, 0.5, cap_dpr, 1.0);
        let max_dim = self.max_dimension_px.clamp(16, HARD_MAX_SHADER_DIMENSION_PX) as f32;

        let mut px_w = (safe_w * safe_dpr).round().max(1.0);
        let mut px_h = (safe_h * safe_dpr).round().max(1.0);
        let longest = px_w.max(px_h);
        if longest > max_dim {
            let scale = max_dim / longest;
            px_w = (px_w * scale).round().max(1.0);
            px_h = (px_h * scale).round().max(1.0);
        }
        (px_w as u32, px_h as u32)
    }
}

/// Sanitized shader uniforms shared across all 10 optical shaders.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaderUniforms {
    pub pointer_x: f32,
    pub pointer_y: f32,
    pub time_s: f32,
    pub intensity: f32,
    pub param_a: f32,
    pub param_b: f32,
    pub param_c: f32,
    pub primary: RgbColor,
    pub secondary: RgbColor,
    pub accent: RgbColor,
    pub dark_mode: bool,
}

impl ShaderUniforms {
    /// Builds theme-coupled shader uniforms with NaN/Infinity defense on all scalar parameters.
    pub fn from_theme(theme: &PrismTheme, intensity: f32) -> Self {
        Self {
            pointer_x: 0.5,
            pointer_y: 0.5,
            time_s: 0.0,
            intensity: clamp_f32(intensity, 0.0, 1.0, 0.8),
            param_a: 1.0,
            param_b: 0.5,
            param_c: 0.0,
            primary: theme.palette.primary.rgb,
            secondary: theme.palette.secondary.rgb,
            accent: theme.palette.info.rgb,
            dark_mode: theme.palette.mode == ThemeMode::Dark,
        }
    }

    /// Updates normalized pointer coordinates clamped to `0.0..=1.0`.
    pub fn with_pointer(mut self, x: f32, y: f32) -> Self {
        self.pointer_x = clamp_f32(x, 0.0, 1.0, 0.5);
        self.pointer_y = clamp_f32(y, 0.0, 1.0, 0.5);
        self
    }

    /// Updates elapsed animation time in seconds (wrapped safely to avoid float precision drift).
    pub fn with_time(mut self, seconds: f32) -> Self {
        let safe = clamp_f32(seconds, 0.0, 86_400.0, 0.0);
        self.time_s = safe % 3600.0;
        self
    }

    /// Sets custom shader parameters (`param_a`, `param_b`, `param_c`) with NaN/Infinity clamping.
    pub fn with_params(mut self, a: f32, b: f32, c: f32) -> Self {
        self.param_a = clamp_f32(a, -100.0, 100.0, 1.0);
        self.param_b = clamp_f32(b, -100.0, 100.0, 0.5);
        self.param_c = clamp_f32(c, -100.0, 100.0, 0.0);
        self
    }
}

/// Deterministic lifecycle tracker for mounted optical shader instances.
/// Guarantees that every created shader program/buffer handle is cleanly disposed (zero leaks).
#[derive(Debug, Default)]
pub struct OpticalShaderRegistry {
    next_id: u64,
    active: BTreeSet<u64>,
    created_total: u64,
    disposed_total: u64,
}

impl OpticalShaderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allocates a new shader instance handle.
    pub fn mount(&mut self, _kind: OpticalShaderKind) -> u64 {
        self.next_id = self.next_id.saturating_add(1);
        let id = self.next_id;
        self.active.insert(id);
        self.created_total = self.created_total.saturating_add(1);
        id
    }

    /// Disposes a shader instance handle. Returns `true` if the handle was active.
    pub fn dispose(&mut self, handle: u64) -> bool {
        if self.active.remove(&handle) {
            self.disposed_total = self.disposed_total.saturating_add(1);
            true
        } else {
            false
        }
    }

    /// Disposes all active shader handles (e.g., on GPU context loss or window teardown).
    pub fn dispose_all(&mut self) -> usize {
        let count = self.active.len();
        self.active.clear();
        self.disposed_total = self.disposed_total.saturating_add(count as u64);
        count
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    pub const fn created_total(&self) -> u64 {
        self.created_total
    }

    pub const fn disposed_total(&self) -> u64 {
        self.disposed_total
    }
}

/// Frame-budget pointer event coalescer (Rust equivalent of `createCoalescedPointerMove`).
/// Coalesces 120Hz–1000Hz pointermove bursts into at most one update per frame interval (`16.67ms` at 60fps).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerCoalescer {
    min_interval_ms: f64,
    last_emit_ms: f64,
    pending: Option<(f32, f32)>,
    emitted_count: u64,
    coalesced_count: u64,
}

impl PointerCoalescer {
    /// Creates a coalescer targeting `target_fps` (clamped to `15..=240`).
    pub fn new(target_fps: u16) -> Self {
        let fps = f64::from(target_fps.clamp(15, 240));
        Self {
            min_interval_ms: 1000.0 / fps,
            last_emit_ms: f64::NEG_INFINITY,
            pending: None,
            emitted_count: 0,
            coalesced_count: 0,
        }
    }

    /// Pushes a pointer coordinate `(x, y)` at `timestamp_ms`.
    /// Returns `Some((x, y))` when the frame budget has elapsed, or `None` when coalesced.
    pub fn push(&mut self, x: f32, y: f32, timestamp_ms: f64) -> Option<(f32, f32)> {
        if !x.is_finite() || !y.is_finite() || !timestamp_ms.is_finite() {
            return None;
        }
        if timestamp_ms - self.last_emit_ms >= self.min_interval_ms {
            self.last_emit_ms = timestamp_ms;
            self.pending = None;
            self.emitted_count = self.emitted_count.saturating_add(1);
            Some((x, y))
        } else {
            self.pending = Some((x, y));
            self.coalesced_count = self.coalesced_count.saturating_add(1);
            None
        }
    }

    /// Flushes any trailing coalesced pointer position on frame tick or pointerup.
    pub fn flush(&mut self, timestamp_ms: f64) -> Option<(f32, f32)> {
        let pt = self.pending.take()?;
        if timestamp_ms.is_finite() {
            self.last_emit_ms = timestamp_ms;
        }
        self.emitted_count = self.emitted_count.saturating_add(1);
        Some(pt)
    }

    /// Cancels any pending pointer event (called on unmount / pointer cancel).
    pub fn cancel(&mut self) {
        self.pending = None;
    }

    pub const fn emitted_count(&self) -> u64 {
        self.emitted_count
    }

    pub const fn coalesced_count(&self) -> u64 {
        self.coalesced_count
    }
}

/// Fixed-capacity bounded ring buffer for live telemetry, audio levels, and ripple wave pools.
/// Never grows past `N` and allocates once.
#[derive(Debug, Clone)]
pub struct TelemetryRingBuffer<T, const N: usize> {
    buf: Vec<T>,
    head: usize,
}

impl<T: Clone, const N: usize> Default for TelemetryRingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> TelemetryRingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(N),
            head: 0,
        }
    }

    /// Appends a sample, overwriting the oldest sample once capacity `N` is reached.
    pub fn push(&mut self, value: T) {
        if N == 0 {
            return;
        }
        if self.buf.len() < N {
            self.buf.push(value);
        } else {
            self.buf[self.head] = value;
            self.head = (self.head + 1) % N;
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns items in chronological order (oldest -> newest).
    pub fn ordered(&self) -> Vec<T> {
        if self.buf.len() < N || self.head == 0 {
            return self.buf.clone();
        }
        let mut out = Vec::with_capacity(self.buf.len());
        out.extend_from_slice(&self.buf[self.head..]);
        out.extend_from_slice(&self.buf[..self.head]);
        out
    }

    pub fn clear(&mut self) {
        self.buf.clear();
        self.head = 0;
    }
}

/// Nested blur-host elision context (Rust equivalent of `[data-refractor-blur-host="true"]`).
/// Automatically elides redundant depth-2+ backdrop blur passes inside already-blurred parent panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlurHostContext {
    pub depth: u8,
    pub host_blurred: bool,
}

impl BlurHostContext {
    pub const fn root() -> Self {
        Self {
            depth: 0,
            host_blurred: false,
        }
    }

    pub const fn enter_blurred_host(self) -> Self {
        Self {
            depth: self.depth.saturating_add(1),
            host_blurred: true,
        }
    }

    /// Returns `true` if a child surface should elide its own backdrop blur pass.
    pub const fn should_elide_blur(self) -> bool {
        self.host_blurred && self.depth >= 1
    }
}
