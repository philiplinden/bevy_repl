pub mod stdout;
pub mod helpers;
pub mod alt_screen;

use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use ratatui::layout::Rect;
use std::sync::Arc;

use crate::repl::{Repl, ReplSet};
use crate::prompt::ReplPromptConfig;
use crate::log_ecs::{LogBuffer, LogLine, CaptureSubscriberPlugin, print_log_events_system};
use self::stdout::StdoutTerminalContext;

pub struct PromptRenderPlugin {
    pub renderer: Arc<dyn PromptRenderer>,
}

/// Strategy interface for prompt rendering
pub trait PromptRenderer: Send + Sync + 'static {
    fn render(&self, _f: &mut ratatui::Frame<'_>, _ctx: &RenderCtx) {}
    /// Configure logging for this renderer. Default: stdout engine
    fn configure_logging(&self, app: &mut App) {
        app.add_plugins(CaptureSubscriberPlugin::default());
        app.add_systems(Update, print_log_events_system);
    }
    fn configure_context(&self, _app: &mut App) {}
}

/// Active renderer resource; apps can override this to customize styling
#[derive(Resource, Clone)]
pub struct ActiveRenderer(pub Arc<dyn PromptRenderer>);

/// Rendering context passed to renderers
pub struct RenderCtx<'a> {
    pub repl: &'a Repl,
    pub cfg: &'a ReplPromptConfig,
    pub area: Rect,
    /// Optional snapshot of recent logs to render inside the frame (top-aligned)
    pub logs: Option<Vec<LogLine>>,
}

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
    }
}

/// Render entrypoint: delegates to the active renderer strategy
pub(super) fn display_prompt(
    repl: Res<Repl>,
    cfg: Option<Res<ReplPromptConfig>>,
    logs_buf: Option<Res<LogBuffer>>,
    active: Res<ActiveRenderer>,
    // Prefer ratatui's default terminal context when present (alternate screen)
    term_ratatui: Option<ResMut<RatatuiContext>>,
    // Fallback to our stdout terminal context when present
    term_stdout: Option<ResMut<StdoutTerminalContext>>,
) {
    let cfg = cfg.map(|v| v.clone()).unwrap_or_default();
    // Take a cheap snapshot of recent logs if present
    let logs_snapshot: Option<Vec<LogLine>> = logs_buf
        .as_ref()
        .map(|b| b.lines.iter().cloned().collect());
    let _ = logs_snapshot; // kept for future in-frame renderers

    if let Some(mut term) = term_ratatui {
        let _ = term.draw(|f| {
            let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
            let ctx = RenderCtx { repl: &repl, cfg: &cfg, area, logs: logs_snapshot.clone() };
            active.0.render(f, &ctx);
        });
        return;
    }

    if let Some(mut term) = term_stdout {
        let _ = term.draw(|f| {
            let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
            let ctx = RenderCtx { repl: &repl, cfg: &cfg, area, logs: logs_snapshot.clone() };
            active.0.render(f, &ctx);
        });
    }
}
