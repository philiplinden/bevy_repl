//! Test support utilities for renderer testing.
//!
//! This module provides a minimal harness that can be extended to drive
//! different renderers under tests. It intentionally avoids referencing
//! crate-internal renderer types so it can be used from integration tests
//! without tight coupling. As we add more renderers, we can add small
//! adapter types that implement `RendererUnderTest`.

use bevy::prelude::*;

/// A minimal unified interface for renderers under test.
/// Implementors should register the appropriate plugins/systems
/// on the provided `App` so that a frame can be produced.
pub trait RendererUnderTest {
    /// A short stable name for snapshot naming, logs, etc.
    fn name(&self) -> &'static str;

    /// Register all plugins and systems needed for this renderer.
    fn add_to_app(&self, app: &mut App);
}

/// Builder to assemble a Bevy `App` for renderer testing.
pub struct TestAppBuilder<R: RendererUnderTest> {
    renderer: R,
}

impl<R: RendererUnderTest> TestAppBuilder<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    /// Build a minimal Bevy app configured with the renderer under test.
    /// Additional plugins (time, schedule, etc.) can be added by callers
    /// after obtaining the `App` value.
    pub fn build(self) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        self.renderer.add_to_app(&mut app);
        app
    }
}
