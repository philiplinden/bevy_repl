use bevy::prelude::*;
use bevy_ratatui::RatatuiContext;
use crate::repl::{Repl, ReplContext, ReplSet};
use crate::prompt::{ReplPrompt, ReplPromptConfig};
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use super::helpers::{bottom_bar_area, buffer_window, cursor_position};

pub struct PromptRenderPlugin;

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
#[derive(Resource)]
pub struct ActiveRenderer(pub Box<dyn PromptRenderer>);


impl Plugin for PromptRenderPlugin {
    fn build(&self, app: &mut App) {
        // Default renderer; apps can override by inserting their own ActiveRenderer
        #[cfg(feature = "pretty")]
        {
            // Use Pretty renderer by default when feature is enabled
            app.insert_resource(ActiveRenderer(Box::new(super::pretty::PrettyRenderer)));
        }
        #[cfg(not(feature = "pretty"))]
        {
            app.insert_resource(ActiveRenderer(Box::new(MinimalRenderer)));
        }
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
    // Prefer ratatui's default terminal context when present (alternate screen)
    term_ratatui: Option<ResMut<RatatuiContext>>, 
    // Fallback to the crate's custom terminal context to preserve compatibility
    term_custom: Option<ResMut<ReplContext>>, 
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
        return;
    }

    let Some(mut term) = term_custom else { return }; // No terminal context yet
    let _ = term.draw(|f| {
        let area = Rect { x: 0, y: 0, width: f.area().width, height: f.area().height };
        let ctx = RenderCtx { repl: &repl, prompt: &prompt, visuals: &visuals, area };
        active.0.render(f, &ctx);
    });
}

/// Minimal renderer: single line, no borders, no colors, no hints
struct MinimalRenderer;
impl PromptRenderer for MinimalRenderer {
    fn render(&self, f: &mut ratatui::Frame<'_>, ctx: &RenderCtx) {
        // Always one line
        if ctx.area.height == 0 { return; }
        let area = bottom_bar_area(ctx.area, 1);

        // Layout
        let left_area = area;
        let prompt_symbol = ctx.prompt.symbol.clone().unwrap_or_default();
        let prompt_width = prompt_symbol.len() as u16;
        if left_area.width <= prompt_width { return; }
        let visible_width = left_area.width - prompt_width;

        // Buffer windowing
        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Render text
        let mut spans = Vec::with_capacity(2);
        if !prompt_symbol.is_empty() { spans.push(Span::raw(prompt_symbol)); }
        spans.push(Span::raw(visible_buf));
        f.render_widget(Paragraph::new(Line::from(spans)), left_area);

        // Cursor position
        let (cursor_x, cursor_y) = cursor_position(left_area, prompt_width, start, cursor);
        f.set_cursor_position((cursor_x, cursor_y));
    }
}