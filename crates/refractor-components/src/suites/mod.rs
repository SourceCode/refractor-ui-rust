//! Bespoke native Rust/GPUI domain suites replacing all 274 former catalog shells.
//!
//! Together with the 40 flagship components in `buttons`, `inputs`, `navigation`, `display`,
//! `feedback`, and `desktop`, all **314 components across all 55 categories** are bespoke
//! native GPUI `RenderOnce` implementations.

pub mod ai_cli_observability;
pub mod charts_core;
pub mod core_utilities_hardware;
pub mod desktop_os_files;
pub mod enterprise_data_commerce;
pub mod forms_inputs;
pub mod pro_media_audio;

use gpui::{
    AnyElement, Bounds, Div, IntoElement, ParentElement, Pixels, SharedString, Styled, canvas, div,
    fill, point, px, size,
};
use refractor_core::{SurfaceVariant, color_str_to_hsla, optical_surface, rgb_to_hsla};
use refractor_security::clamp_f32;
use refractor_tokens::{PrismTheme, RgbColor};

/// Shared optical card header used by complex multi-part domain components.
pub(crate) fn suite_header(
    theme: &PrismTheme,
    title: impl Into<SharedString>,
    badge: Option<SharedString>,
) -> Div {
    let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
    let accent = color_str_to_hsla(&theme.palette.primary.main, theme.palette.primary.rgb);
    let mut row = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(px(8.0))
        .child(div().text_size(px(13.0)).text_color(fg).child(title.into()));
    if let Some(b) = badge {
        row = row.child(
            optical_surface(theme, SurfaceVariant::Badge)
                .px(px(8.0))
                .py(px(2.0))
                .text_size(px(10.0))
                .text_color(accent)
                .child(b),
        );
    }
    row
}

/// Renders a GPU-accelerated vertical bar / sparkline / histogram / EQ strip on a GPUI `canvas`.
pub(crate) fn gpu_bar_canvas(
    theme: &PrismTheme,
    values: Vec<f32>,
    height_px: f32,
    use_success_palette: bool,
) -> AnyElement {
    let safe_vals: Vec<f32> = if values.is_empty() {
        vec![0.35, 0.62, 0.48, 0.84, 0.71, 0.92, 0.55]
    } else {
        values
            .into_iter()
            .map(|v| clamp_f32(v, 0.0, 1.0, 0.0))
            .collect()
    };
    let primary = if use_success_palette {
        rgb_to_hsla(theme.palette.success.rgb, 0.92)
    } else {
        rgb_to_hsla(theme.palette.primary.rgb, 0.92)
    };
    let secondary = rgb_to_hsla(theme.palette.secondary.rgb, 0.85);
    let grid_line = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));

    optical_surface(theme, SurfaceVariant::Well)
        .w_full()
        .h(px(height_px.max(24.0)))
        .p(px(6.0))
        .child(
            canvas(
                |_b, _w, _cx| {},
                move |bounds: Bounds<Pixels>, (), window, _cx| {
                    let w: f32 = bounds.size.width.into();
                    let h: f32 = bounds.size.height.into();
                    if w <= 2.0 || h <= 2.0 || safe_vals.is_empty() {
                        return;
                    }
                    // Subtle horizontal baseline
                    let base = Bounds::new(
                        point(bounds.origin.x, bounds.origin.y + px(h - 1.0)),
                        size(bounds.size.width, px(1.0)),
                    );
                    window.paint_quad(fill(base, grid_line));

                    let count = safe_vals.len() as f32;
                    let slot = w / count;
                    let bar_w = (slot * 0.72).max(2.0);
                    for (idx, val) in safe_vals.iter().enumerate() {
                        let bh = (h * val.clamp(0.05, 1.0)).max(2.0);
                        let x = (idx as f32) * slot + (slot - bar_w) * 0.5;
                        let y = h - bh;
                        let rect = Bounds::new(
                            point(bounds.origin.x + px(x), bounds.origin.y + px(y)),
                            size(px(bar_w), px(bh)),
                        );
                        let color = if idx % 2 == 0 { primary } else { secondary };
                        window.paint_quad(fill(rect, color));
                    }
                },
            )
            .size_full(),
        )
        .into_any_element()
}

/// Renders a GPU-accelerated 2D matrix / heatmap / calendar / step-sequencer grid on GPUI `canvas`.
pub(crate) fn gpu_matrix_canvas(
    theme: &PrismTheme,
    rows: usize,
    cols: usize,
    cells: Vec<f32>,
    height_px: f32,
) -> AnyElement {
    let r_count = rows.clamp(1, 16);
    let c_count = cols.clamp(1, 24);
    let base_rgb = theme.palette.primary.rgb;
    let well_bg = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));

    optical_surface(theme, SurfaceVariant::Well)
        .w_full()
        .h(px(height_px.max(36.0)))
        .p(px(6.0))
        .child(
            canvas(
                |_b, _w, _cx| {},
                move |bounds: Bounds<Pixels>, (), window, _cx| {
                    let w: f32 = bounds.size.width.into();
                    let h: f32 = bounds.size.height.into();
                    if w <= 4.0 || h <= 4.0 {
                        return;
                    }
                    let cw = w / (c_count as f32);
                    let ch = h / (r_count as f32);
                    for r in 0..r_count {
                        for c in 0..c_count {
                            let idx = r * c_count + c;
                            let intensity = cells
                                .get(idx)
                                .copied()
                                .map_or(((idx * 37 % 100) as f32) / 100.0, |v| {
                                    clamp_f32(v, 0.0, 1.0, 0.0)
                                });
                            let rect = Bounds::new(
                                point(
                                    bounds.origin.x + px((c as f32) * cw + 1.0),
                                    bounds.origin.y + px((r as f32) * ch + 1.0),
                                ),
                                size(px((cw - 2.0).max(2.0)), px((ch - 2.0).max(2.0))),
                            );
                            let color = if intensity < 0.08 {
                                well_bg
                            } else {
                                rgb_to_hsla(base_rgb, (0.18 + intensity * 0.82).clamp(0.18, 1.0))
                            };
                            window.paint_quad(fill(rect, color));
                        }
                    }
                },
            )
            .size_full(),
        )
        .into_any_element()
}

/// Renders a GPU-accelerated 2D scatter / bubble / node / geospatial / radar canvas.
pub(crate) fn gpu_scatter_canvas(
    theme: &PrismTheme,
    points_norm: Vec<(f32, f32, f32)>,
    height_px: f32,
) -> AnyElement {
    let pts: Vec<(f32, f32, f32)> = if points_norm.is_empty() {
        vec![
            (0.15, 0.72, 8.0),
            (0.32, 0.45, 11.0),
            (0.54, 0.28, 13.0),
            (0.71, 0.60, 9.0),
            (0.86, 0.22, 12.0),
        ]
    } else {
        points_norm
            .into_iter()
            .map(|(x, y, r)| {
                (
                    clamp_f32(x, 0.05, 0.95, 0.5),
                    clamp_f32(y, 0.05, 0.95, 0.5),
                    clamp_f32(r, 4.0, 22.0, 8.0),
                )
            })
            .collect()
    };
    let primary = rgb_to_hsla(theme.palette.primary.rgb, 0.88);
    let secondary = rgb_to_hsla(theme.palette.secondary.rgb, 0.85);
    let axis = color_str_to_hsla(&theme.palette.divider, RgbColor::new(148, 163, 184));

    optical_surface(theme, SurfaceVariant::Well)
        .w_full()
        .h(px(height_px.max(48.0)))
        .p(px(6.0))
        .child(
            canvas(
                |_b, _w, _cx| {},
                move |bounds: Bounds<Pixels>, (), window, _cx| {
                    let w: f32 = bounds.size.width.into();
                    let h: f32 = bounds.size.height.into();
                    if w <= 4.0 || h <= 4.0 {
                        return;
                    }
                    // Crosshair grid lines
                    window.paint_quad(fill(
                        Bounds::new(
                            point(bounds.origin.x, bounds.origin.y + px(h * 0.5)),
                            size(bounds.size.width, px(1.0)),
                        ),
                        axis,
                    ));
                    window.paint_quad(fill(
                        Bounds::new(
                            point(bounds.origin.x + px(w * 0.5), bounds.origin.y),
                            size(px(1.0), bounds.size.height),
                        ),
                        axis,
                    ));
                    for (idx, (nx, ny, radius)) in pts.iter().enumerate() {
                        let px_x = nx * w - radius * 0.5;
                        let px_y = (1.0 - ny) * h - radius * 0.5;
                        let dot = Bounds::new(
                            point(bounds.origin.x + px(px_x), bounds.origin.y + px(px_y)),
                            size(px(*radius), px(*radius)),
                        );
                        window.paint_quad(fill(dot, if idx % 2 == 0 { primary } else { secondary }));
                    }
                },
            )
            .size_full(),
        )
        .into_any_element()
}

/// Renders a GPU-accelerated horizontal multi-track / Gantt / trace waterfall / flamegraph canvas.
pub(crate) fn gpu_tracks_canvas(
    theme: &PrismTheme,
    spans: Vec<(usize, f32, f32)>,
    playhead_pct: Option<f32>,
    height_px: f32,
) -> AnyElement {
    let safe_spans: Vec<(usize, f32, f32)> = if spans.is_empty() {
        vec![
            (0, 0.02, 0.38),
            (0, 0.44, 0.30),
            (1, 0.14, 0.52),
            (2, 0.25, 0.62),
        ]
    } else {
        spans
            .into_iter()
            .map(|(row, start, width)| {
                (
                    row.min(7),
                    clamp_f32(start, 0.0, 0.95, 0.0),
                    clamp_f32(width, 0.04, 1.0, 0.2),
                )
            })
            .collect()
    };
    let max_row = safe_spans.iter().map(|(r, _, _)| *r).max().unwrap_or(0) + 1;
    let c1 = rgb_to_hsla(theme.palette.primary.rgb, 0.88);
    let c2 = rgb_to_hsla(theme.palette.secondary.rgb, 0.86);
    let c3 = rgb_to_hsla(theme.palette.success.rgb, 0.86);
    let playhead_color = rgb_to_hsla(theme.palette.error.rgb, 0.95);

    optical_surface(theme, SurfaceVariant::Well)
        .w_full()
        .h(px(height_px.max(48.0)))
        .p(px(6.0))
        .child(
            canvas(
                |_b, _w, _cx| {},
                move |bounds: Bounds<Pixels>, (), window, _cx| {
                    let w: f32 = bounds.size.width.into();
                    let h: f32 = bounds.size.height.into();
                    if w <= 4.0 || h <= 4.0 {
                        return;
                    }
                    let row_h = (h / (max_row as f32)).max(6.0);
                    for (idx, (row, start, width)) in safe_spans.iter().enumerate() {
                        let rx = start * w;
                        let rw = (width * w).min(w - rx).max(4.0);
                        let ry = (*row as f32) * row_h + 2.0;
                        let rh = (row_h - 4.0).max(4.0);
                        let rect = Bounds::new(
                            point(bounds.origin.x + px(rx), bounds.origin.y + px(ry)),
                            size(px(rw), px(rh)),
                        );
                        let color = match idx % 3 {
                            0 => c1,
                            1 => c2,
                            _ => c3,
                        };
                        window.paint_quad(fill(rect, color));
                    }
                    if let Some(ph) = playhead_pct {
                        let px_x = clamp_f32(ph, 0.0, 1.0, 0.0) * w;
                        window.paint_quad(fill(
                            Bounds::new(
                                point(bounds.origin.x + px(px_x), bounds.origin.y),
                                size(px(2.0), bounds.size.height),
                            ),
                            playhead_color,
                        ));
                    }
                },
            )
            .size_full(),
        )
        .into_any_element()
}

/// Renders a GPU-accelerated horizontal meter / gauge / VU / token bar.
pub(crate) fn gpu_meter(theme: &PrismTheme, fraction: f32, use_success: bool) -> AnyElement {
    let pct = clamp_f32(fraction, 0.0, 1.0, 0.0);
    let fill_color = if use_success {
        rgb_to_hsla(theme.palette.success.rgb, 0.94)
    } else {
        rgb_to_hsla(theme.palette.primary.rgb, 0.94)
    };
    optical_surface(theme, SurfaceVariant::Well)
        .w_full()
        .h(px(12.0))
        .rounded(px(999.0))
        .p(px(2.0))
        .child(
            canvas(
                |_b, _w, _cx| {},
                move |bounds: Bounds<Pixels>, (), window, _cx| {
                    let w: f32 = bounds.size.width.into();
                    let fill_w = (w * pct).max(2.0);
                    window.paint_quad(fill(
                        Bounds::new(bounds.origin, size(px(fill_w), bounds.size.height)),
                        fill_color,
                    ));
                },
            )
            .size_full(),
        )
        .into_any_element()
}

/// Compact key-value metadata rows for inspectors, drawers, and property sheets.
pub(crate) fn kv_rows(theme: &PrismTheme, pairs: Vec<(SharedString, SharedString)>) -> Div {
    let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
    let muted = color_str_to_hsla(&theme.palette.text.secondary, RgbColor::new(100, 116, 139));
    let mut col = div().flex().flex_col().gap(px(4.0));
    for (k, v) in pairs {
        col = col.child(
            optical_surface(theme, SurfaceVariant::Well)
                .px(px(10.0))
                .py(px(5.0))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(div().text_size(px(11.0)).text_color(muted).child(k))
                .child(div().text_size(px(12.0)).text_color(fg).child(v)),
        );
    }
    col
}

/// Horizontal row of optical pills/tags with optional active highlight.
pub(crate) fn pill_strip(
    theme: &PrismTheme,
    items: Vec<SharedString>,
    active_index: Option<usize>,
) -> Div {
    let fg = color_str_to_hsla(&theme.palette.text.primary, RgbColor::new(21, 30, 44));
    let active_fg = color_str_to_hsla(
        &theme.palette.primary.contrast_text,
        RgbColor::new(255, 255, 255),
    );
    let mut row = div().flex().flex_row().flex_wrap().gap(px(6.0));
    for (idx, item) in items.into_iter().enumerate() {
        let is_active = active_index == Some(idx);
        let pill = if is_active {
            optical_surface(theme, SurfaceVariant::PrimaryButton).text_color(active_fg)
        } else {
            optical_surface(theme, SurfaceVariant::Well).text_color(fg)
        };
        row = row.child(pill.px(px(10.0)).py(px(4.0)).text_size(px(11.0)).child(item));
    }
    row
}
