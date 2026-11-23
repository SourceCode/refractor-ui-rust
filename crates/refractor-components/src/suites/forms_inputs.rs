//! Domain Suite 1/7 (`RUI-16`): Forms, Scheduling, Formatted Inputs, Complex Buttons,
//! Complex Controls, Rich Content Creation & Drag-and-Drop (37 bespoke GPUI components).

use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use refractor_core::{
    BaseProps, SurfaceVariant, active_theme, apply_component_override, color_str_to_hsla,
    optical_surface,
};
use refractor_security::{clamp_f32, sanitize_html};
use refractor_tokens::RgbColor;

use super::{gpu_bar_canvas, gpu_meter, gpu_scatter_canvas, kv_rows, pill_strip, suite_header};

macro_rules! bespoke_input_widget {
    ($name:ident, $default_title:literal, $badge:literal, $default_val:literal, $kind:ident) => {
        #[derive(Debug, Clone, IntoElement)]
        pub struct $name {
            pub title: SharedString,
            pub value: SharedString,
            pub secondary: SharedString,
            pub numeric: f32,
            pub items: Vec<SharedString>,
            pub base: BaseProps,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    title: SharedString::from($default_title),
                    value: SharedString::from($default_val),
                    secondary: SharedString::from($badge),
                    numeric: 0.65,
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

            pub fn secondary(mut self, secondary: impl Into<SharedString>) -> Self {
                self.secondary = secondary.into();
                self
            }

            pub fn numeric(mut self, numeric: f32) -> Self {
                self.numeric = clamp_f32(numeric, 0.0, 1.0, 0.5);
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
                let mut root = optical_surface(&theme, SurfaceVariant::Crystal)
                    .p(px(12.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(suite_header(&theme, self.title, Some(self.secondary.clone())));
                if self.base.disabled {
                    root = root.opacity(0.5);
                }
                if let Some(ov) = theme.components.get(stringify!($name)) {
                    root = apply_component_override(root, ov);
                }
                if let Some(ref ov) = self.base.style_override {
                    root = apply_component_override(root, ov);
                }
                render_form_kind(&theme, root, FormKind::$kind, self.value, self.numeric, self.items)
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
enum FormKind {
    WellField,
    DateSchedule,
    Dropzone,
    Rating,
    NumericSpin,
    ActionCapsule,
    SliderHistogram,
    RotaryControl,
    CodeSurface,
    ColorSurface,
    RichHtmlSurface,
    GraphCanvas,
    DualList,
    SelectionCards,
}

fn render_form_kind(
    theme: &refractor_tokens::PrismTheme,
    root: gpui::Div,
    kind: FormKind,
    value: SharedString,
    numeric: f32,
    items: Vec<SharedString>,
) -> gpui::Div {
    let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
    let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
    let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);

    match kind {
        FormKind::WellField => root.child(
            optical_surface(theme, SurfaceVariant::Well)
                .px(px(12.0))
                .py(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .text_size(px(13.0))
                .text_color(fg)
                .child(value)
                .child(div().text_size(px(11.0)).text_color(muted).child("▾")),
        ),
        FormKind::DateSchedule => {
            let chips = if items.is_empty() {
                vec![
                    SharedString::from("Mon"),
                    SharedString::from("Tue"),
                    SharedString::from("Wed"),
                    SharedString::from("Thu"),
                    SharedString::from("Fri"),
                ]
            } else {
                items
            };
            root.child(
                optical_surface(theme, SurfaceVariant::Well)
                    .px(px(10.0))
                    .py(px(6.0))
                    .text_size(px(13.0))
                    .text_color(fg)
                    .child(value),
            )
            .child(pill_strip(theme, chips, Some(2)))
        }
        FormKind::Dropzone => root.child(
            optical_surface(theme, SurfaceVariant::Well)
                .p(px(16.0))
                .flex()
                .flex_col()
                .items_center()
                .gap(px(4.0))
                .child(div().text_size(px(13.0)).text_color(accent).child("⬆ Drop files or browse"))
                .child(div().text_size(px(11.0)).text_color(muted).child(value)),
        ),
        FormKind::Rating => {
            let filled = (numeric * 5.0).round().clamp(0.0, 5.0) as usize;
            let mut stars = div().flex().flex_row().gap(px(4.0)).text_size(px(16.0));
            for idx in 0..5 {
                stars = stars.child(
                    div()
                        .text_color(if idx < filled { accent } else { muted })
                        .child(if idx < filled { "★" } else { "☆" }),
                );
            }
            root.child(stars.child(div().text_size(px(12.0)).text_color(muted).child(value)))
        }
        FormKind::NumericSpin => root.child(
            optical_surface(theme, SurfaceVariant::Well)
                .px(px(10.0))
                .py(px(6.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .text_size(px(13.0))
                .text_color(fg)
                .child("−")
                .child(value)
                .child("+"),
        ),
        FormKind::ActionCapsule => root.child(
            optical_surface(theme, SurfaceVariant::PrimaryButton)
                .px(px(14.0))
                .py(px(8.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .text_size(px(13.0))
                .text_color(color_str_to_hsla(
                    &theme.palette.primary.contrast_text,
                    RgbColor::new(255, 255, 255),
                ))
                .child(value)
                .child("▸"),
        ),
        FormKind::SliderHistogram => root
            .child(gpu_bar_canvas(
                theme,
                vec![0.2, 0.45, 0.85, 0.95, 0.7, 0.5, 0.3, 0.15],
                42.0,
                false,
            ))
            .child(gpu_meter(theme, numeric, false)),
        FormKind::RotaryControl => root.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(12.0))
                .child(
                    optical_surface(theme, SurfaceVariant::PrimaryButton)
                        .w(px(36.0))
                        .h(px(36.0))
                        .rounded(px(999.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(color_str_to_hsla(
                            &theme.palette.primary.contrast_text,
                            RgbColor::new(255, 255, 255),
                        ))
                        .child("◉"),
                )
                .child(div().text_size(px(13.0)).text_color(fg).child(value)),
        ),
        FormKind::CodeSurface => {
            let bg = color_str_to_hsla(&theme.editor.background, RgbColor::new(15, 23, 42));
            let code_fg = color_str_to_hsla(&theme.editor.text, RgbColor::new(226, 232, 240));
            let kw = color_str_to_hsla(&theme.editor.keyword, theme.palette.secondary.rgb);
            root.child(
                optical_surface(theme, SurfaceVariant::Well)
                    .bg(bg)
                    .p(px(10.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(div().text_size(px(11.0)).text_color(kw).child("fn render(&mut self)"))
                    .child(div().text_size(px(12.0)).text_color(code_fg).child(value)),
            )
        }
        FormKind::ColorSurface => root
            .child(gpu_scatter_canvas(theme, vec![(numeric, 0.72, 12.0)], 48.0))
            .child(kv_rows(
                theme,
                vec![
                    (SharedString::from("Hex"), value),
                    (SharedString::from("WCAG AA"), SharedString::from("7.4:1 Pass")),
                ],
            )),
        FormKind::RichHtmlSurface => {
            let clean = sanitize_html(value.as_ref());
            root.child(pill_strip(
                theme,
                vec![
                    SharedString::from("B"),
                    SharedString::from("I"),
                    SharedString::from("U"),
                    SharedString::from("H1"),
                    SharedString::from("Link"),
                ],
                Some(0),
            ))
            .child(
                optical_surface(theme, SurfaceVariant::Well)
                    .p(px(10.0))
                    .text_size(px(12.0))
                    .text_color(fg)
                    .child(clean),
            )
        }
        FormKind::GraphCanvas => root.child(gpu_scatter_canvas(
            theme,
            vec![(0.2, 0.5, 14.0), (0.5, 0.75, 14.0), (0.8, 0.35, 14.0)],
            64.0,
        )),
        FormKind::DualList => {
            let entries = if items.is_empty() {
                vec![
                    (SharedString::from("Available"), SharedString::from("12 items")),
                    (SharedString::from("Selected"), value),
                ]
            } else {
                items
                    .into_iter()
                    .enumerate()
                    .map(|(i, s)| (SharedString::from(format!("#{}", i + 1)), s))
                    .collect()
            };
            root.child(kv_rows(theme, entries))
        }
        FormKind::SelectionCards => {
            let opts = if items.is_empty() {
                vec![
                    SharedString::from("Standard"),
                    SharedString::from("Pro Studio"),
                    SharedString::from("Enterprise"),
                ]
            } else {
                items
            };
            root.child(pill_strip(theme, opts, Some(1)))
        }
    }
}

// Form Inputs & Selection Controls (4)
bespoke_input_widget!(DateTimePicker, "Date & Time Picker", "ISO-8601", "2026-11-22 14:30", DateSchedule);
bespoke_input_widget!(FileDropzone, "File Dropzone", "Upload", "PNG, PDF, JSON up to 25 MB", Dropzone);
bespoke_input_widget!(RatingControl, "Rating Control", "5-Star", "4.5 / 5.0", Rating);
bespoke_input_widget!(SearchWithSuggestions, "Search With Suggestions", "Autocomplete", "Search 314 components...", WellField);

// Date, Time & Scheduling Fields (6)
bespoke_input_widget!(CombinedDateTimePicker, "Combined Date & Time", "UTC", "2026-11-22T20:00Z", DateSchedule);
bespoke_input_widget!(DateRangePicker, "Date Range Picker", "Range", "Nov 01 – Nov 30, 2026", DateSchedule);
bespoke_input_widget!(DurationInput, "Duration Input", "ISO Duration", "02h 15m 30s", NumericSpin);
bespoke_input_widget!(MonthQuarterYearPicker, "Month / Quarter / Year", "Fiscal", "Q4 2026", DateSchedule);
bespoke_input_widget!(RecurrenceBuilder, "Recurrence Builder", "RRULE", "Every Mon, Wed, Fri", DateSchedule);
bespoke_input_widget!(TimezoneSelector, "Timezone Selector", "IANA", "America/Los_Angeles (UTC-8)", WellField);

// Number, Currency & Formatted Type Fields (6)
bespoke_input_widget!(CurrencyInput, "Currency Input", "USD", "$1,250.00", NumericSpin);
bespoke_input_widget!(InternationalPhoneInput, "International Phone", "E.164", "+1 (415) 555-0199", WellField);
bespoke_input_widget!(MaskedPatternInput, "Masked Pattern Input", "Mask", "94107-2048", WellField);
bespoke_input_widget!(MeasurementUnitInput, "Measurement Unit Input", "SI", "24.0 px", NumericSpin);
bespoke_input_widget!(NumericStepper, "Numeric Stepper", "Step", "42", NumericSpin);
bespoke_input_widget!(ScrubbableNumberField, "Scrubbable Number", "Drag X", "120.0 fps", NumericSpin);

// Complex Button & Action Variations (5)
bespoke_input_widget!(ComboSplitButton, "Combo Split Button", "Action", "Run Build & Test", ActionCapsule);
bespoke_input_widget!(HoldToConfirmButton, "Hold To Confirm", "Safety", "Hold 1.5s to Purge", ActionCapsule);
bespoke_input_widget!(MultiStateCycleButton, "Multi-State Cycle", "Cycle", "Mode: Vivid", ActionCapsule);
bespoke_input_widget!(SpeedDialButton, "Speed Dial Button", "FAB", "+ Quick Create", ActionCapsule);
bespoke_input_widget!(StatefulAsyncButton, "Stateful Async Button", "Async", "Deploying v2.0.0...", ActionCapsule);

// Complex Variations of Common Controls (7)
bespoke_input_widget!(CascadingSelect, "Cascading Select", "Hierarchy", "Themes › Cyber › Cyberpunk", WellField);
bespoke_input_widget!(CreatableCombobox, "Creatable Combobox", "Tags", "+ Create 'optical-glass'", WellField);
bespoke_input_widget!(DualThumbSlider, "Dual Thumb Slider", "Range", "20% – 85%", SliderHistogram);
bespoke_input_widget!(HistogramRangeSlider, "Histogram Range Slider", "Distribution", "P10 – P90", SliderHistogram);
bespoke_input_widget!(RichSelectionCards, "Rich Selection Cards", "Cards", "Pro Studio", SelectionCards);
bespoke_input_widget!(RotaryDial, "Rotary Dial", "Encoder", "72% / +6.0 dB", RotaryControl);
bespoke_input_widget!(TriStateCheckbox, "Tri-State Checkbox", "Indeterminate", "Partial (4/7 selected)", SelectionCards);

// Rich Input & Content Creation (6)
bespoke_input_widget!(CodeEditor, "Code Editor", "Rust", "let theme = alabaster_theme();", CodeSurface);
bespoke_input_widget!(ColorPicker, "Color Picker", "HSV / WCAG", "#E75C24", ColorSurface);
bespoke_input_widget!(ImageCropper, "Image Cropper", "16:9", "1920 × 1080 crop", GraphCanvas);
bespoke_input_widget!(InlineEditableField, "Inline Editable Field", "Inline", "Refractor Optical Workbench", WellField);
bespoke_input_widget!(RichTextEditor, "Rich Text Editor", "Sanitized HTML", "<p>Optical glass typography</p>", RichHtmlSurface);
bespoke_input_widget!(SignaturePad, "Signature Pad", "Vector Ink", "Signed • Verified", GraphCanvas);

// Interactive & Drag-and-Drop (3)
bespoke_input_widget!(NodeGraphCanvas, "Node Graph Canvas", "DAG", "3 nodes • 2 edges", GraphCanvas);
bespoke_input_widget!(SortableList, "Sortable List", "Keyboard DnD", "3 ordered items", DualList);
bespoke_input_widget!(TransferList, "Transfer List", "Dual Pane", "4 assigned", DualList);
