//! Minimal custom renderer example
//!
//! This example shows how you can create your own prompt renderer for the REPL
//! and configure the plugin to use it instead of the built-in renderer.

use std::sync::Arc;

use bevy::prelude::*;
use bevy_repl::prelude::*;
use bevy_repl::prompt::renderer::helpers::{bottom_bar_area, buffer_window, cursor_position};
use bevy_repl::prompt::renderer::{PromptRenderer, RenderCtx};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

struct CustomRenderer;

impl PromptRenderer for CustomRenderer {
    fn render(&self, f: &mut ratatui::Frame<'_>, ctx: &RenderCtx) {
        let area = bottom_bar_area(ctx.area, 1);
        if area.height == 0 {
            return;
        }

        let prompt = ctx.prompt.symbol.clone().unwrap_or_default();
        let prompt_width = prompt.len() as u16;
        if area.width <= prompt_width {
            return;
        }
        let visible_width = area.width - prompt_width;

        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);

        // Simple style: cyan prompt when visuals.color is Some, otherwise default
        let prompt_style = Style::default();

        let spans = vec![Span::styled(prompt, prompt_style), Span::raw(visible_buf)];
        f.render_widget(Paragraph::new(Line::from(spans)), area);

        let (cx, cy) = cursor_position(area, prompt_width, start, cursor);
        f.set_cursor_position((cx, cy));
    }
}

#[derive(Debug, Clone, Event, Default)]
struct PingCommand;

impl ReplCommand for PingCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("ping").about("Output pong")
    }
}

fn on_ping(_: Trigger<PingCommand>) {
    repl_println!("Pong");
}

fn instructions() {
    repl_println!();
    repl_println!("Bevy REPL custom renderer example");
    repl_println!();
    repl_println!("Try typing in the REPL:");
    repl_println!("  ping");
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            )),
            ReplPlugins.set(PromptPlugin {
                renderer: Arc::new(CustomRenderer),
                ..default()
            }),
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_systems(PostStartup, instructions)
        .run();
}
