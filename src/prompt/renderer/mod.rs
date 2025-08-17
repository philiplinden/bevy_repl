pub mod minimal;
pub mod helpers;
#[cfg(feature = "pretty")]
pub mod pretty;

use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::layout::Rect;
use std::sync::Arc;

use crate::repl::{Repl, FallbackTerminalContext, ReplSet};
use crate::prompt::{ReplPrompt, ReplPromptConfig};
use crate::log_ecs::{LogBuffer, LogLine};

pub struct PromptRenderPlugin {
    pub renderer: Arc<dyn PromptRenderer>,
}

/// Rendering context passed to renderers
pub struct RenderCtx<'a> {
    pub repl: &'a Repl,
    pub prompt: &'a ReplPrompt,
    pub visuals: &'a ReplPromptConfig,
    pub area: Rect,
    /// Optional snapshot of recent logs to render inside the frame (top-aligned)
    pub logs: Option<Vec<LogLine>>,
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
        #[cfg(feature = "pretty")]
        app.add_plugins(pretty::ScrollRegionPlugin);
    }
}

/// Render entrypoint: delegates to the active renderer strategy
pub(super) fn display_prompt(
    // Prefer ratatui's default terminal context when present (alternate screen)
    term_ratatui: Option<ResMut<RatatuiContext>>, 
    // Fallback to the crate's custom terminal context to preserve compatibility
    term_fallback: Option<ResMut<FallbackTerminalContext>>, 
    repl: Res<Repl>,
    prompt: Res<ReplPrompt>,
    visuals: Option<Res<ReplPromptConfig>>,
    logs_buf: Option<Res<LogBuffer>>,
    active: Res<ActiveRenderer>,
) {
    let visuals = visuals.map(|v| v.clone()).unwrap_or_default();
    // Take a cheap snapshot of recent logs if present
    let logs_snapshot: Option<Vec<LogLine>> = logs_buf
        .as_ref()
        .map(|b| b.lines.iter().cloned().collect());

    if let Some(mut term) = term_ratatui {
        let _ = term.draw(|f| {
            let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
            let ctx = RenderCtx { repl: &repl, prompt: &prompt, visuals: &visuals, area, logs: logs_snapshot.clone() };
            active.0.render(f, &ctx);
        });
        return;
    }

    let Some(mut term) = term_fallback else { return }; // No terminal context yet
    let _ = term.draw(|f| {
        let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
        let ctx = RenderCtx { repl: &repl, prompt: &prompt, visuals: &visuals, area, logs: logs_snapshot.clone() };
        active.0.render(f, &ctx);
    });
}
