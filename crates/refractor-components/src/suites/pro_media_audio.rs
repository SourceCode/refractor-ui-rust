//! Domain Suite 3/7 (`RUI-18`): Pro Media, Audio/DSP, NLE Transport, Multi-Track Timeline,
//! Photo Color Grading & Viewport Export Utilities (32 bespoke GPUI components).

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
enum MediaKind {
    Transport,
    Timeline,
    AudioMeter,
    Spectrum,
    ColorWheel,
    Viewport,
    RackChain,
}

macro_rules! bespoke_media_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub badge: SharedString,
            pub value: SharedString,
            pub level: f32,
            pub bands: Vec<f32>,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    badge: SharedString::from($badge),
                    value: SharedString::from($default_val),
                    level: 0.68,
                    bands: vec![0.35, 0.58, 0.82, 0.91, 0.74, 0.61, 0.45, 0.30, 0.52, 0.76],
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

            pub fn level(mut self, level: f32) -> Self {
                self.level = clamp_f32(level, 0.0, 1.0, 0.5);
                self
            }

            pub fn bands(mut self, bands: Vec<f32>) -> Self {
                self.bands = bands;
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

                match MediaKind::$kind {
                    MediaKind::Transport => root
                        .child(pill_strip(
                            &theme,
                            vec![
                                SharedString::from("⏮"),
                                SharedString::from("◀"),
                                SharedString::from("▶ Play"),
                                SharedString::from("⏸"),
                                SharedString::from("● Rec"),
                            ],
                            Some(2),
                        ))
                        .child(
                            optical_surface(&theme, SurfaceVariant::Well)
                                .px(px(10.0))
                                .py(px(5.0))
                                .text_size(px(12.0))
                                .text_color(fg)
                                .child(self.value),
                        ),
                    MediaKind::Timeline => root
                        .child(gpu_tracks_canvas(&theme, Vec::new(), Some(self.level), 64.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    MediaKind::AudioMeter => root
                        .child(gpu_meter(&theme, self.level, true))
                        .child(gpu_meter(&theme, (self.level * 0.92).clamp(0.0, 1.0), false))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    MediaKind::Spectrum => root
                        .child(gpu_bar_canvas(&theme, self.bands, 64.0, true))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    MediaKind::ColorWheel => root
                        .child(gpu_scatter_canvas(
                            &theme,
                            vec![(0.25, 0.55, 16.0), (0.50, 0.52, 16.0), (0.75, 0.48, 16.0)],
                            64.0,
                        ))
                        .child(kv_rows(
                            &theme,
                            vec![(SharedString::from("Grade"), self.value)],
                        )),
                    MediaKind::Viewport => root
                        .child(gpu_matrix_canvas(&theme, 4, 8, self.bands, 56.0))
                        .child(div().text_size(px(11.0)).text_color(fg).child(self.value)),
                    MediaKind::RackChain => root.child(kv_rows(
                        &theme,
                        vec![
                            (SharedString::from("Slot 1"), SharedString::from("Optical Compressor")),
                            (SharedString::from("Slot 2"), self.value),
                        ],
                    )),
                }
            }
        }
    };
}

// Media, Audio & Spatial Controls (6)
bespoke_media_widget!(AudioWaveformPlayer, "Audio Waveform Player", "48 kHz", "01:14 / 03:42", Spectrum);
bespoke_media_widget!(BeforeAfterSlider, "Before / After Slider", "Split", "50% Wipe", Viewport);
bespoke_media_widget!(CustomVideoPlayer, "Custom Video Player", "4K60", "00:08:12:04", Transport);
bespoke_media_widget!(InteractiveMapViewer, "Interactive Map Viewer", "Spatial", "37.7749° N, 122.4194° W", Viewport);
bespoke_media_widget!(MediaLightbox, "Media Lightbox", "Preview", "IMG_4096.RAW • 100%", Viewport);
bespoke_media_widget!(Product360Viewer, "Product 360° Viewer", "360°", "Orbit: 145°", ColorWheel);

// Transport & Playback Controls (4)
bespoke_media_widget!(FrameStepperRateSelector, "Frame Stepper & Rate", "24 fps", "23.976 / 24 / 60 fps", Transport);
bespoke_media_widget!(JogShuttleWheel, "Jog Shuttle Wheel", "Shuttle", "+1.0x Forward", Transport);
bespoke_media_widget!(TimecodeScrubber, "Timecode Scrubber", "SMPTE", "00:01:24:12", Timeline);
bespoke_media_widget!(TransportControlBar, "Transport Control Bar", "DAW/NLE", "Playing • Loop On", Transport);

// Timeline & Sequencing (6)
bespoke_media_widget!(ClipTrimHandle, "Clip Trim Handle", "Ripple", "In +00:04 • Out -00:12", Timeline);
bespoke_media_widget!(KeyframeCurveEditor, "Keyframe Curve Editor", "Bezier", "Ease-Out Cubic", ColorWheel);
bespoke_media_widget!(MultiTrackTimeline, "Multi-Track Timeline", "4 Tracks", "V1 • V2 • A1 • A2", Timeline);
bespoke_media_widget!(PlayheadScrubRuler, "Playhead Scrub Ruler", "Ruler", "00:00:48:00", Timeline);
bespoke_media_widget!(TimelineZoomNavigator, "Timeline Zoom Navigator", "Zoom", "4.0× Timeline Zoom", AudioMeter);
bespoke_media_widget!(TrackHeaderStrip, "Track Header Strip", "V1 / A1", "Solo • Mute • Lock", RackChain);

// Photo & Color Grading Controls (8)
bespoke_media_widget!(BrushHudControl, "Brush HUD Control", "Brush", "Size 42px • Flow 80%", ColorWheel);
bespoke_media_widget!(CropStraightenOverlay, "Crop & Straighten", "Rule of 3rds", "Angle +1.2°", Viewport);
bespoke_media_widget!(EyedropperPixelLoupe, "Eyedropper Pixel Loupe", "16× Loupe", "sRGB #E75C24", ColorWheel);
bespoke_media_widget!(LayerBlendStack, "Layer Blend Stack", "Layers", "Normal • Screen • Multiply", RackChain);
bespoke_media_widget!(LiveColorScopes, "Live Color Scopes", "RGB Parade", "Rec.709 Legal", Spectrum);
bespoke_media_widget!(ThreeWayColorWheels, "Three-Way Color Wheels", "Lift/Gamma/Gain", "Warm +0.08", ColorWheel);
bespoke_media_widget!(ToneCurveEditor, "Tone Curve Editor", "Luma/RGB", "S-Curve Contrast", ColorWheel);
bespoke_media_widget!(WipeCompareOverlay, "Wipe Compare Overlay", "A / B", "Split 50% Vertical", Viewport);

// Audio Mixing & Signal Processing (6)
bespoke_media_widget!(ChannelStripFader, "Channel Strip Fader", "Master", "-3.2 dBFS", AudioMeter);
bespoke_media_widget!(EffectsRackChain, "Effects Rack Chain", "DSP", "Parametric EQ → Limiter", RackChain);
bespoke_media_widget!(ParametricEqGraph, "Parametric EQ Graph", "6-Band", "High-Shelf +2.4 dB @ 8kHz", Spectrum);
bespoke_media_widget!(StereoPanKnob, "Stereo Pan Knob", "Pan", "Center (0.0)", ColorWheel);
bespoke_media_widget!(VuPeakLevelMeter, "VU Peak Level Meter", "LUFS", "-14.0 LUFS • Peak -1.2 dB", AudioMeter);
bespoke_media_widget!(WaveformSpectrogramView, "Waveform Spectrogram", "FFT", "20 Hz – 20 kHz", Spectrum);

// Viewport & Export Utilities (2)
bespoke_media_widget!(CanvasZoomSafeAreaBar, "Canvas Zoom & Safe Area", "Title Safe", "100% • 16:9 Guides", Transport);
bespoke_media_widget!(ExportRenderQueue, "Export Render Queue", "Encoder", "ProRes 422 HQ • 78%", AudioMeter);
