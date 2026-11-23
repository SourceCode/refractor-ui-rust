//! Core GPUI + Iced-inspired UI framework engine for **Refractor UI for Rust**.
//!
//! Combines:
//! - **Refractor Optical Glass (`glass-ui-v1`)** multi-material token & Astryx Open-Internals architecture
//! - **Zed GPUI (`gpui`)** GPU-accelerated rendering, flexbox layout, and entity reactivity
//! - **Iced-inspired** declarative `RefractorApp` (`State` + `Message` + `update` + `view` + `Task`) ergonomics

pub mod app;
pub mod astryx;
pub mod fx_overlay;
pub mod gpui_bridge;
pub mod interaction;
pub mod optical;

pub use app::{AppHost, RefractorApp, Subscription, Task, run_app};
pub use astryx::{
    A11yContract, BaseProps, BestPractice, ComponentCategory, ComponentDoc, ControllableState,
    PropDoc, ThemeProps,
};
pub use fx_overlay::{
    FxOverlay, FxOverlaySpec, MAX_OVERLAY_OPACITY, OpticalBackdrop, clamp_overlay_opacity,
};
pub use gpui_bridge::{
    ThemeGlobal, active_theme, color_str_to_hsla, hex_to_hsla, init_theme, rgb_to_hsla,
    rgba_to_hsla, set_theme, set_theme_by_id,
};
pub use interaction::{
    FocusRingSpec, KeyAction, KeyboardEvent, roving_next_index, should_handle_key,
};
pub use optical::{SurfaceVariant, apply_component_override, optical_surface};

#[cfg(test)]
mod tests {
    use super::*;
    use refractor_tokens::{ComponentStyleOverride, RgbColor, ThemeMaterial};

    #[test]
    fn base_props_and_theme_props_builders_work() {
        let base = BaseProps::new()
            .with_id("primary-cta")
            .with_aria_label("Start project")
            .with_disabled(false)
            .with_data_attr("size", "md")
            .with_style_override(ComponentStyleOverride {
                opacity: Some(0.95),
                ..ComponentStyleOverride::default()
            });
        assert_eq!(base.id.as_deref(), Some("primary-cta"));
        assert_eq!(base.aria_label.as_deref(), Some("Start project"));
        assert!(!base.disabled);
        assert_eq!(base.data_attrs.get("size").map(String::as_str), Some("md"));

        let tp = ThemeProps::new("primary-button").with_state("size", "md");
        assert_eq!(tp.class_name, "refractor-primary-button");
        assert_eq!(tp.state_attrs.get("data-size").map(String::as_str), Some("md"));
    }

    #[test]
    fn controllable_state_respects_controlled_and_uncontrolled_modes() {
        let mut unc = ControllableState::uncontrolled(false);
        assert!(!unc.is_controlled());
        assert!(!*unc.get());
        unc.set(true);
        assert!(*unc.get());

        let mut ctrl = ControllableState::controlled(42_u32);
        assert!(ctrl.is_controlled());
        ctrl.set(99);
        assert_eq!(*ctrl.get(), 42);
    }

    #[test]
    fn all_55_component_categories_have_non_empty_labels() {
        assert_eq!(ComponentCategory::ALL.len(), 55);
        for cat in ComponentCategory::ALL {
            assert_ne!(cat.label(), "");
        }
    }

    #[test]
    fn color_bridge_and_optical_surface_render_all_13_materials() {
        let hsla = rgb_to_hsla(RgbColor::new(231, 92, 36), 0.9);
        assert!((hsla.a - 0.9).abs() < 0.01);

        let fallback = hex_to_hsla("not-valid");
        assert!(fallback.a > 0.99);

        for material in ThemeMaterial::ALL {
            let mut theme = refractor_themes::alabaster_theme();
            theme.material = material;
            let _btn = optical_surface(&theme, SurfaceVariant::PrimaryButton);
            let _slab = optical_surface(&theme, SurfaceVariant::Slab);
            let _well = optical_surface(&theme, SurfaceVariant::Well);
            let _emerald = optical_surface(&theme, SurfaceVariant::Emerald);
            let ov = ComponentStyleOverride {
                background: Some("#111827".to_owned()),
                foreground: Some("#F8FAFC".to_owned()),
                border_color: Some("#38BDF8".to_owned()),
                border_radius_px: Some(12.0),
                padding_x_px: Some(16.0),
                padding_y_px: Some(8.0),
                opacity: Some(0.9),
            };
            let _applied = apply_component_override(optical_surface(&theme, SurfaceVariant::Crystal), &ov);
        }
    }

    #[test]
    fn fx_overlay_spec_clamps_opacity_and_respects_reduced_motion() {
        assert!((clamp_overlay_opacity(f32::NAN) - 0.0).abs() < f32::EPSILON);
        assert!((clamp_overlay_opacity(0.95) - MAX_OVERLAY_OPACITY).abs() < f32::EPSILON);

        let cyberpunk = refractor_themes::resolve_theme_by_id("cyberpunk");
        let full = FxOverlaySpec::from_theme(&cyberpunk, false);
        let reduced = FxOverlaySpec::from_theme(&cyberpunk, true);
        assert!(full.active);
        assert!(reduced.opacity <= full.opacity);

        let alabaster = refractor_themes::alabaster_theme();
        let none = FxOverlaySpec::from_theme(&alabaster, false);
        assert!(!none.active);
    }

    #[test]
    fn keyboard_ime_guard_roving_index_and_focus_ring_work() {
        assert!(!should_handle_key(KeyboardEvent::new(KeyAction::Enter).composing(true)));
        assert!(!should_handle_key(KeyboardEvent::new(KeyAction::Escape).composing(true)));
        assert!(!should_handle_key(KeyboardEvent::new(KeyAction::ArrowDown).disabled(true)));
        assert!(should_handle_key(KeyboardEvent::new(KeyAction::Enter)));

        assert_eq!(roving_next_index(0, 4, KeyAction::ArrowDown), 1);
        assert_eq!(roving_next_index(0, 4, KeyAction::ArrowUp), 3);
        assert_eq!(roving_next_index(2, 4, KeyAction::Home), 0);
        assert_eq!(roving_next_index(1, 4, KeyAction::End), 3);
        assert_eq!(roving_next_index(0, 0, KeyAction::ArrowDown), 0);

        let theme = refractor_themes::alabaster_theme();
        assert!(FocusRingSpec::from_theme(&theme, true, true).visible);
        assert!(!FocusRingSpec::from_theme(&theme, true, false).visible);
        assert!(!FocusRingSpec::from_theme(&theme, false, true).visible);
    }

    #[test]
    fn task_and_subscription_builders_construct_cleanly() {
        let _t: Task<u32> = Task::batch([Task::none(), Task::done(1), Task::done(2)]);
        let _s: Subscription<u32> = Subscription::batch([
            Subscription::none(),
            Subscription::Tick {
                interval_ms: 16,
                on_tick: || 1,
            },
        ]);
    }
}
