use blitz_shell::{BlitzApplication, View};
use dioxus_core::{provide_context, ScopeId};
use dioxus_history::{History, MemoryHistory};
use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::{MouseScrollDelta, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::WindowId;

use crate::DioxusNativeWindowRenderer;
use crate::{contexts::DioxusNativeDocument, BlitzShellEvent, DioxusDocument, WindowConfig};

/// Dioxus-native specific event type
pub enum DioxusNativeEvent {
    /// A hotreload event, basically telling us to update our templates.
    #[cfg(all(
        feature = "hot-reload",
        debug_assertions,
        not(target_os = "android"),
        not(target_os = "ios")
    ))]
    DevserverEvent(dioxus_devtools::DevserverMsg),

    /// Create a new head element from the Link and Title elements
    ///
    /// todo(jon): these should probabkly be synchronous somehow
    CreateHeadElement {
        window: WindowId,
        name: String,
        attributes: Vec<(String, String)>,
        contents: Option<String>,
    },
}

const SCROLL_LINE_PX: f64 = 56.0;
const SCROLL_EASE: f64 = 0.2;

#[derive(Default)]
struct SmoothScroll {
    window: Option<WindowId>,
    pending: (f64, f64),
}

pub struct DioxusNativeApplication {
    smooth_scroll: SmoothScroll,
    pending_window: Option<WindowConfig<DioxusNativeWindowRenderer>>,
    inner: BlitzApplication<DioxusNativeWindowRenderer>,
    proxy: EventLoopProxy<BlitzShellEvent>,
}

impl DioxusNativeApplication {
    pub fn new(
        proxy: EventLoopProxy<BlitzShellEvent>,
        config: WindowConfig<DioxusNativeWindowRenderer>,
    ) -> Self {
        Self {
            smooth_scroll: SmoothScroll::default(),
            pending_window: Some(config),
            inner: BlitzApplication::new(proxy.clone()),
            proxy,
        }
    }

    fn step_smooth_scroll(&mut self, window_id: WindowId) {
        if self.smooth_scroll.window != Some(window_id) {
            return;
        }

        let (pending_x, pending_y) = self.smooth_scroll.pending;
        let settled = pending_x.abs() < 0.5 && pending_y.abs() < 0.5;
        let (step_x, step_y) = if settled {
            (pending_x, pending_y)
        } else {
            (pending_x * SCROLL_EASE, pending_y * SCROLL_EASE)
        };

        self.smooth_scroll.pending = if settled {
            (0.0, 0.0)
        } else {
            (pending_x - step_x, pending_y - step_y)
        };

        let Some(view) = self.inner.windows.get_mut(&window_id) else {
            return;
        };

        let changed = match view.doc.get_hover_node_id() {
            Some(node_id) => view.doc.scroll_node_by_has_changed(node_id, step_x, step_y),
            None => view.doc.scroll_viewport_by_has_changed(step_x, step_y),
        };

        if !changed {
            self.smooth_scroll.pending = (0.0, 0.0);
        }
    }

    pub fn add_window(&mut self, window_config: WindowConfig<DioxusNativeWindowRenderer>) {
        self.inner.add_window(window_config);
    }

    fn handle_blitz_shell_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: &DioxusNativeEvent,
    ) {
        match event {
            #[cfg(all(
                feature = "hot-reload",
                debug_assertions,
                not(target_os = "android"),
                not(target_os = "ios")
            ))]
            DioxusNativeEvent::DevserverEvent(event) => match event {
                dioxus_devtools::DevserverMsg::HotReload(hotreload_message) => {
                    for window in self.inner.windows.values_mut() {
                        let doc = window.downcast_doc_mut::<DioxusDocument>();

                        // Apply changes to vdom
                        dioxus_devtools::apply_changes(&doc.vdom, hotreload_message);

                        // Reload changed assets
                        for asset_path in &hotreload_message.assets {
                            if let Some(url) = asset_path.to_str() {
                                doc.reload_resource_by_href(url);
                            }
                        }

                        window.poll();
                    }
                }
                dioxus_devtools::DevserverMsg::Shutdown => event_loop.exit(),
                dioxus_devtools::DevserverMsg::FullReloadStart => {}
                dioxus_devtools::DevserverMsg::FullReloadFailed => {}
                dioxus_devtools::DevserverMsg::FullReloadCommand => {}
                _ => {}
            },

            DioxusNativeEvent::CreateHeadElement {
                name,
                attributes,
                contents,
                window,
            } => {
                if let Some(window) = self.inner.windows.get_mut(window) {
                    let doc = window.downcast_doc_mut::<DioxusDocument>();
                    doc.create_head_element(name, attributes, contents);
                    window.poll();
                }
            }

            // Suppress unused variable warning
            #[cfg(not(all(
                feature = "hot-reload",
                debug_assertions,
                not(target_os = "android"),
                not(target_os = "ios")
            )))]
            #[allow(unreachable_patterns)]
            _ => {
                let _ = event_loop;
                let _ = event;
            }
        }
    }
}

impl ApplicationHandler<BlitzShellEvent> for DioxusNativeApplication {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(feature = "tracing")]
        tracing::debug!("Injecting document provider into all windows");

        if let Some(config) = self.pending_window.take() {
            let mut window = View::init(config, event_loop, &self.proxy);
            let renderer = window.renderer.clone();
            let handle = window.window.clone();
            let window_id = window.window_id();
            let doc = window.downcast_doc_mut::<DioxusDocument>();

            doc.vdom.in_scope(ScopeId::ROOT, || {
                let shared: Rc<dyn dioxus_document::Document> =
                    Rc::new(DioxusNativeDocument::new(self.proxy.clone(), window_id));
                provide_context(shared);
            });

            // Add history
            let history_provider: Rc<dyn History> = Rc::new(MemoryHistory::default());
            doc.vdom
                .in_scope(ScopeId::ROOT, move || provide_context(history_provider));

            // Add renderer
            doc.vdom
                .in_scope(ScopeId::ROOT, move || provide_context(renderer));

            // Expose the winit window so the app can draw its own window controls
            doc.vdom
                .in_scope(ScopeId::ROOT, move || provide_context(handle));

            // Queue rebuild
            doc.initial_build();

            // And then request redraw
            window.request_redraw();

            // todo(jon): we should actually mess with the pending windows instead of passing along the contexts
            self.inner.windows.insert(window_id, window);
        }

        self.inner.resumed(event_loop);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.inner.suspended(event_loop);
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        self.inner.new_events(event_loop, cause);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match &event {
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(x, y),
                ..
            } => {
                self.smooth_scroll.window = Some(window_id);
                self.smooth_scroll.pending.0 += *x as f64 * SCROLL_LINE_PX;
                self.smooth_scroll.pending.1 += *y as f64 * SCROLL_LINE_PX;

                if let Some(view) = self.inner.windows.get(&window_id) {
                    view.request_redraw();
                }

                return;
            }
            WindowEvent::RedrawRequested => self.step_smooth_scroll(window_id),
            _ => {}
        }

        self.inner.window_event(event_loop, window_id, event);

        let scrolling = self.smooth_scroll.window == Some(window_id)
            && (self.smooth_scroll.pending.0 != 0.0 || self.smooth_scroll.pending.1 != 0.0);

        if scrolling {
            if let Some(view) = self.inner.windows.get(&window_id) {
                view.request_redraw();
            }
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: BlitzShellEvent) {
        match event {
            BlitzShellEvent::Embedder(event) => {
                if let Some(event) = event.downcast_ref::<DioxusNativeEvent>() {
                    self.handle_blitz_shell_event(event_loop, event);
                }
            }
            event => self.inner.user_event(event_loop, event),
        }
    }
}
