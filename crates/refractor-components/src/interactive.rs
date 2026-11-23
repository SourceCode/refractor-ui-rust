//! Deep interactive keyboard, IME-safe, and state controllers for flagship complex desktop
//! components across the 55-category catalog (`RUI-12` / ADR-0005 & ADR-0006).

use std::collections::BTreeMap;

use refractor_core::{KeyAction, KeyboardEvent, roving_next_index, should_handle_key};
use refractor_security::{
    HighlightSegment, build_safe_csv, clamp_f32, is_safe_object_key, sanitize_html,
    split_literal_highlight,
};
use refractor_tokens::{HsvColor, contrast_ratio, hex_to_hsv, hsv_to_hex};

/// Interactive keyboard & IME-safe controller for `CommandPalette`.
#[derive(Debug, Clone)]
pub struct CommandPaletteState {
    pub open: bool,
    pub query: String,
    pub commands: Vec<String>,
    pub active_index: usize,
    pub last_executed: Option<String>,
}

impl CommandPaletteState {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            open: true,
            query: String::new(),
            commands,
            active_index: 0,
            last_executed: None,
        }
    }

    pub fn filtered(&self) -> Vec<(&str, Vec<HighlightSegment>)> {
        let q = self.query.trim().to_ascii_lowercase();
        self.commands
            .iter()
            .filter(|c| q.is_empty() || c.to_ascii_lowercase().contains(&q))
            .map(|c| (c.as_str(), split_literal_highlight(c, &self.query)))
            .collect()
    }

    pub fn set_query(&mut self, next: impl Into<String>) {
        self.query = next.into();
        self.active_index = 0;
    }

    pub fn handle_key(&mut self, event: KeyboardEvent) {
        if !should_handle_key(event) {
            return;
        }
        let matches = self.filtered();
        match event.action {
            KeyAction::Escape => self.open = false,
            KeyAction::ArrowDown | KeyAction::ArrowUp | KeyAction::Home | KeyAction::End => {
                self.active_index = roving_next_index(self.active_index, matches.len(), event.action);
            }
            KeyAction::Enter => {
                if let Some((cmd, _)) = matches.get(self.active_index) {
                    self.last_executed = Some((*cmd).to_owned());
                    self.open = false;
                }
            }
            _ => {}
        }
    }
}

/// Interactive keyboard & OWASP-CSV-safe state controller for `AdvancedDataGrid`.
#[derive(Debug, Clone)]
pub struct DataGridState {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub selected_row: usize,
    pub sort_col: Option<usize>,
    pub ascending: bool,
}

impl DataGridState {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self {
            headers,
            rows,
            selected_row: 0,
            sort_col: None,
            ascending: true,
        }
    }

    pub fn sort_by(&mut self, col_idx: usize) {
        if col_idx >= self.headers.len() {
            return;
        }
        if self.sort_col == Some(col_idx) {
            self.ascending = !self.ascending;
        } else {
            self.sort_col = Some(col_idx);
            self.ascending = true;
        }
        let asc = self.ascending;
        self.rows.sort_by(|a, b| {
            let va = a.get(col_idx).map_or("", String::as_str);
            let vb = b.get(col_idx).map_or("", String::as_str);
            if asc { va.cmp(vb) } else { vb.cmp(va) }
        });
    }

    pub fn handle_key(&mut self, event: KeyboardEvent) {
        if !should_handle_key(event) {
            return;
        }
        self.selected_row = roving_next_index(self.selected_row, self.rows.len(), event.action);
    }

    pub fn export_safe_csv(&self) -> String {
        build_safe_csv(&self.headers, &self.rows)
    }
}

/// Interactive keyboard-accessible state controller for `KanbanBoard`.
#[derive(Debug, Clone)]
pub struct KanbanBoardState {
    pub columns: Vec<(String, Vec<String>)>,
}

impl KanbanBoardState {
    pub fn new(columns: Vec<(String, Vec<String>)>) -> Self {
        Self { columns }
    }

    /// Moves a card between columns (keyboard parity for drag-and-drop per ADR-0006).
    pub fn move_card(&mut self, from_col: usize, card_idx: usize, to_col: usize) -> bool {
        if from_col >= self.columns.len() || to_col >= self.columns.len() || from_col == to_col {
            return false;
        }
        if card_idx >= self.columns[from_col].1.len() {
            return false;
        }
        let card = self.columns[from_col].1.remove(card_idx);
        self.columns[to_col].1.push(card);
        true
    }
}

/// XSS-safe rich text editor state controller for `RichTextEditor` (ADR-0005).
#[derive(Debug, Clone)]
pub struct RichTextEditorState {
    pub sanitized_html: String,
    pub history: Vec<String>,
}

impl RichTextEditorState {
    pub fn new(initial_html: &str) -> Self {
        Self {
            sanitized_html: sanitize_html(initial_html),
            history: Vec::new(),
        }
    }

    pub fn set_html(&mut self, raw_html: &str) {
        self.history.push(self.sanitized_html.clone());
        self.sanitized_html = sanitize_html(raw_html);
    }

    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.sanitized_html = prev;
            true
        } else {
            false
        }
    }
}

/// Deterministic HSV/Hex color picker controller with live WCAG 2.2 AA contrast calculation.
#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub hex: String,
    pub hsv: HsvColor,
}

impl ColorPickerState {
    pub fn from_hex(hex: &str) -> Self {
        let hsv = hex_to_hsv(hex);
        Self {
            hex: hsv_to_hex(hsv),
            hsv,
        }
    }

    pub fn set_hsv(&mut self, h: u16, s: u8, v: u8) {
        self.hsv = HsvColor {
            h: h % 360,
            s: s.min(100),
            v: v.min(100),
        };
        self.hex = hsv_to_hex(self.hsv);
    }

    pub fn contrast_against(&self, other_hex: &str) -> f64 {
        contrast_ratio(&self.hex, other_hex)
    }
}

/// NLE/DAW timeline & transport controller for `MultiTrackTimeline` and `TransportControlBar`.
#[derive(Debug, Clone)]
pub struct MultiTrackTimelineState {
    pub playhead_sec: f32,
    pub duration_sec: f32,
    pub zoom: f32,
    pub playing: bool,
    pub fps: f32,
}

impl MultiTrackTimelineState {
    pub fn new(duration_sec: f32, fps: f32) -> Self {
        Self {
            playhead_sec: 0.0,
            duration_sec: clamp_f32(duration_sec, 1.0, 86_400.0, 60.0),
            zoom: 1.0,
            playing: false,
            fps: clamp_f32(fps, 1.0, 240.0, 24.0),
        }
    }

    pub fn scrub(&mut self, sec: f32) {
        self.playhead_sec = clamp_f32(sec, 0.0, self.duration_sec, self.playhead_sec);
    }

    pub fn step_frame(&mut self, delta_frames: i32) {
        let step = (delta_frames as f32) / self.fps;
        self.scrub(self.playhead_sec + step);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = clamp_f32(zoom, 0.25, 16.0, 1.0);
    }

    pub fn toggle_play(&mut self) {
        self.playing = !self.playing;
    }
}

/// Prototype-pollution-safe node graph controller for `NodeGraphCanvas` (ADR-0005).
#[derive(Debug, Clone, Default)]
pub struct NodeGraphState {
    pub nodes: BTreeMap<String, (f32, f32)>,
    pub edges: Vec<(String, String)>,
}

impl NodeGraphState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, id: &str, x: f32, y: f32) -> bool {
        if id.trim().is_empty() || !is_safe_object_key(id) {
            return false;
        }
        self.nodes.insert(
            id.to_owned(),
            (
                clamp_f32(x, -10_000.0, 10_000.0, 0.0),
                clamp_f32(y, -10_000.0, 10_000.0, 0.0),
            ),
        );
        true
    }

    pub fn connect(&mut self, from: &str, to: &str) -> bool {
        if !self.nodes.contains_key(from) || !self.nodes.contains_key(to) || from == to {
            return false;
        }
        self.edges.push((from.to_owned(), to.to_owned()));
        true
    }
}
