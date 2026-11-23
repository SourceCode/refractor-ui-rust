//! Keyboard navigation, IME composition guards, and keyboard-only focus ring contracts
//! (`RUI-12` / ADR-0002 & ADR-0006).

use refractor_tokens::PrismTheme;

/// Logical keyboard actions handled across Refractor interactive controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyAction {
    Enter,
    Escape,
    Space,
    Tab,
    ShiftTab,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Backspace,
}

/// Raw keyboard event state with IME composition awareness (ADR-0002).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardEvent {
    pub action: KeyAction,
    /// `true` while an Input Method Editor (CJK/dead-key) composition is active.
    pub is_composing: bool,
    pub disabled: bool,
}

impl KeyboardEvent {
    pub const fn new(action: KeyAction) -> Self {
        Self {
            action,
            is_composing: false,
            disabled: false,
        }
    }

    pub const fn composing(mut self, is_composing: bool) -> Self {
        self.is_composing = is_composing;
        self
    }

    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Guards text-input and overlay keyboard handlers against premature firing during IME
/// composition or when a control is disabled (Rust equivalent of `@refractor/no-unguarded-ime-keydown`).
pub const fn should_handle_key(event: KeyboardEvent) -> bool {
    if event.disabled {
        return false;
    }
    if event.is_composing && matches!(event.action, KeyAction::Enter | KeyAction::Escape) {
        return false;
    }
    true
}

/// Computes the next roving-tabindex selection in a list/menu/tab strip with wrap-around.
pub fn roving_next_index(current: usize, len: usize, action: KeyAction) -> usize {
    if len == 0 {
        return 0;
    }
    let clamped = current.min(len - 1);
    match action {
        KeyAction::ArrowDown | KeyAction::ArrowRight => (clamped + 1) % len,
        KeyAction::ArrowUp | KeyAction::ArrowLeft => (clamped + len - 1) % len,
        KeyAction::Home => 0,
        KeyAction::End => len - 1,
        _ => clamped,
    }
}

/// Keyboard-only focus ring specification honoring ADR-0006 (`:focus-visible` contract).
#[derive(Debug, Clone, PartialEq)]
pub struct FocusRingSpec {
    pub visible: bool,
    pub color: String,
    pub width_px: f32,
    pub offset_px: f32,
}

impl FocusRingSpec {
    /// Returns a visible focus ring ONLY when focused via keyboard navigation (never pointer click).
    pub fn from_theme(theme: &PrismTheme, is_focused: bool, via_keyboard: bool) -> Self {
        Self {
            visible: is_focused && via_keyboard,
            color: theme.palette.primary.main.clone(),
            width_px: 2.0,
            offset_px: 2.0,
        }
    }
}
