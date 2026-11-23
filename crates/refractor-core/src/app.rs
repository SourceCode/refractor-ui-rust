//! Iced-inspired declarative application architecture (`RefractorApp`, `Task`, `Subscription`)
//! layered cleanly on top of Zed's GPUI (`Application`, `Window`, `Entity<T>`).
//!
//! Developers can build complex desktop applications using Elm-style `State + Message + update + view`
//! while retaining full access to GPUI's GPU-accelerated rendering and `Entity<T>` reactivity.

use gpui::{
    AnyElement, App, AppContext, Application, Bounds, Context, IntoElement, Render, Window,
    WindowBounds, WindowOptions, px, size,
};
use refractor_tokens::PrismTheme;

use crate::gpui_bridge::{init_theme, set_theme};

/// Declarative side-effect task returned from [`RefractorApp::update`] (inspired by Iced `Task<M>`).
#[derive(Debug, Default)]
pub enum Task<M> {
    #[default]
    None,
    /// Dispatches a follow-up message synchronously on the next update pass.
    Done(M),
    /// Batches multiple tasks in order.
    Batch(Vec<Self>),
}

impl<M> Task<M> {
    pub const fn none() -> Self {
        Self::None
    }

    pub const fn done(message: M) -> Self {
        Self::Done(message)
    }

    pub fn batch(tasks: impl IntoIterator<Item = Self>) -> Self {
        Self::Batch(tasks.into_iter().collect())
    }

    fn drain_messages(self, out: &mut Vec<M>) {
        match self {
            Self::None => {}
            Self::Done(m) => out.push(m),
            Self::Batch(list) => {
                for task in list {
                    task.drain_messages(out);
                }
            }
        }
    }
}

/// Declarative external event subscription descriptor (inspired by Iced `Subscription<M>`).
#[derive(Debug, Default)]
pub enum Subscription<M> {
    #[default]
    None,
    /// Periodic tick interval in milliseconds producing a message.
    Tick {
        interval_ms: u64,
        on_tick: fn() -> M,
    },
    /// Combines multiple subscriptions.
    Batch(Vec<Self>),
}

impl<M> Subscription<M> {
    pub const fn none() -> Self {
        Self::None
    }

    pub fn batch(subs: impl IntoIterator<Item = Self>) -> Self {
        Self::Batch(subs.into_iter().collect())
    }
}

/// Iced-inspired declarative desktop application trait running natively on GPUI.
pub trait RefractorApp: Sized + 'static {
    /// Strongly typed user/system message enum.
    type Message: Clone + Send + 'static;

    /// Constructs initial application state and optional startup task.
    fn new() -> (Self, Task<Self::Message>);

    /// Window title displayed by the native OS window manager.
    fn title(&self) -> String;

    /// Initial window size in logical pixels (`width`, `height`).
    fn window_size(&self) -> (f32, f32) {
        (1280.0, 820.0)
    }

    /// Active [`PrismTheme`] derived from application state.
    fn theme(&self) -> PrismTheme {
        refractor_themes::alabaster_theme()
    }

    /// Handles a state transition message and returns a follow-up [`Task`].
    fn update(&mut self, message: Self::Message) -> Task<Self::Message>;

    /// Declares active subscriptions (keyboard shortcuts, timers, streams).
    fn subscriptions(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    /// Renders the application UI tree into a GPU-accelerated GPUI [`AnyElement`].
    fn view(
        &self,
        theme: &PrismTheme,
        window: &mut Window,
        cx: &mut Context<'_, AppHost<Self>>,
    ) -> AnyElement;
}

/// GPUI [`Render`] host bridging any [`RefractorApp`] into a native GPUI [`Window`].
pub struct AppHost<A: RefractorApp> {
    state: A,
}

impl<A: RefractorApp> std::fmt::Debug for AppHost<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppHost")
            .field("title", &self.state.title())
            .finish()
    }
}

impl<A: RefractorApp> AppHost<A> {
    /// Dispatches a message to the underlying [`RefractorApp`], executes any chained
    /// [`Task`] messages, synchronizes [`crate::ThemeGlobal`], and triggers a GPUI re-render.
    pub fn dispatch(&mut self, message: A::Message, cx: &mut Context<'_, Self>) {
        let mut queue = vec![message];
        let mut guard = 0usize;
        while let Some(msg) = queue.first().cloned() {
            queue.remove(0);
            let task = self.state.update(msg);
            task.drain_messages(&mut queue);
            guard += 1;
            if guard >= 64 {
                break;
            }
        }
        set_theme(cx, self.state.theme());
        cx.notify();
    }

    /// Read-only access to the current application state.
    pub const fn state(&self) -> &A {
        &self.state
    }
}

impl<A: RefractorApp> Render for AppHost<A> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = self.state.theme();
        self.state.view(&theme, window, cx)
    }
}

/// Launches a [`RefractorApp`] in a native cross-platform GPUI desktop window.
pub fn run_app<A: RefractorApp>() {
    Application::new().run(|cx: &mut App| {
        let (mut state, startup_task) = A::new();
        let mut startup_msgs = Vec::new();
        startup_task.drain_messages(&mut startup_msgs);
        for msg in startup_msgs {
            let _ = state.update(msg);
        }

        let initial_theme = state.theme();
        let (w, h) = state.window_size();
        init_theme(cx, initial_theme);

        let bounds = Bounds::centered(None, size(px(w), px(h)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..WindowOptions::default()
        };

        let _ = cx.open_window(options, |_window, cx| cx.new(|_cx| AppHost { state }));
        cx.activate(true);
    });
}
