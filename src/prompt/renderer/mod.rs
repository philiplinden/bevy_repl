pub mod minimal;
pub mod helpers;
pub mod scroll;

use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use crate::repl::{Repl, ReplSet};

use crate::prompt::{ReplPrompt, ReplPromptConfig};
use ratatui::layout::Rect;
use std::sync::Arc;

/// Public label: "scroll region ready". Always available, even in minimal mode.
pub struct PromptRenderPlugin {
    pub renderer: Arc<dyn PromptRenderer>,
}

/// Rendering context passed to renderers
pub struct RenderCtx<'a> {
    pub repl: &'a Repl,
    pub prompt: &'a ReplPrompt,
    pub visuals: &'a ReplPromptConfig,
    pub area: Rect,
}

/// Strategy interface for prompt rendering
pub trait PromptRenderer: Send + Sync + 'static {
    fn render(&self, _f: &mut ratatui::Frame<'_>, _ctx: &RenderCtx) {}
}

/// Active renderer resource; apps can override this to customize styling
#[derive(Resource, Clone)]
pub struct ActiveRenderer(pub Arc<dyn PromptRenderer>);

impl Plugin for PromptRenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveRenderer(self.renderer.clone()));
        app.add_systems(
            Update,
            (
                // When enabled, capture terminal input
                display_prompt
                    .in_set(ReplSet::Render)
                    .in_set(ReplSet::All)
                    .after(ReplSet::Buffer),
            ),
        );
        app.add_plugins(
            scroll::ScrollRegionPlugin,
        );
    }
}

/// Render entrypoint: delegates to the active renderer strategy
pub(super) fn display_prompt(
    // Prefer ratatui's default terminal context when present (alternate screen)
    term_ratatui: Option<ResMut<RatatuiContext>>,
    repl: Res<Repl>,
    prompt: Res<ReplPrompt>,
    visuals: Option<Res<ReplPromptConfig>>,
    active: Res<ActiveRenderer>,
) {
    let visuals = visuals.map(|v| v.clone()).unwrap_or_default();

    if let Some(mut term) = term_ratatui {
        let _ = term.draw(|f| {
            let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
            let ctx = RenderCtx { repl: &repl, prompt: &prompt, visuals: &visuals, area };
            active.0.render(f, &ctx);
        });
    } else { return }; // No terminal context yet
}
