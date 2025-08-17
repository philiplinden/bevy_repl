//! Minimal custom renderer example (feature-gated: `pretty`).
//! Run with: `cargo run --example custom_renderer --features pretty`

use std::sync::Arc;
use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_repl::prelude::*;
use bevy_repl::prompt::renderer::{
    PromptRenderer, RenderCtx,
    helpers::{buffer_window, cursor_position},
};
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

struct CustomRenderer;

impl PromptRenderer for CustomRenderer {
    fn render(&self, f: &mut ratatui::Frame<'_>, ctx: &RenderCtx) {
        // Place the prompt at the TOP of the screen (row 0)
        let area = Rect { x: ctx.area.x, y: ctx.area.y, width: ctx.area.width, height: 1 };
        if area.height == 0 {
            return;
        }

        let prompt = ctx.cfg.symbol.as_ref().map(|s| s.text.clone()).unwrap_or_default();
        let prompt_width = Span::raw(prompt.clone()).width() as u16;
        if area.width <= prompt_width {
            return;
        }
        let visible_width = area.width - prompt_width;

        let buffer = &ctx.repl.buffer;
        let cursor = ctx.repl.cursor_pos.min(buffer.len());
        let (visible_buf, start) = buffer_window(buffer, cursor, visible_width);
        let prompt_style = ctx.cfg.symbol.as_ref().map(|s| s.style).unwrap_or_default();

        let spans = vec![Span::styled(prompt.clone(), prompt_style), Span::raw(visible_buf)];
        f.render_widget(Paragraph::new(Line::from(spans)), area);

        let (cx, cy) = cursor_position(area, prompt_width, buffer, start, cursor);
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

#[derive(Debug, Clone, Event, Default)]
struct SayCommand {
    message: String,
    repeat: usize,
    shout: bool,
}

impl ReplCommand for SayCommand {
    fn clap_command() -> clap::Command {
        clap::Command::new("say")
            .about("Say something")
            .arg(
                clap::Arg::new("message")
                    .help("Message to say")
                    .required(true),
            )
            .alias("echo")
            .arg(
                clap::Arg::new("repeat")
                    .short('r')
                    .long("repeat")
                    .help("Number of times to repeat")
                    .default_value("1"),
            )
            .arg(
                clap::Arg::new("shout")
                    .short('s')
                    .long("shout")
                    .help("Shout the message")
                    .action(clap::ArgAction::SetTrue)
                    .num_args(0),
            )
    }

    fn to_event(matches: &clap::ArgMatches) -> ReplResult<Self> {
        let message = matches.get_one::<String>("message").unwrap().clone();
        let repeat = matches
            .get_one::<String>("repeat")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(1);
        let shout = matches.get_flag("shout");

        Ok(SayCommand {
            message,
            repeat,
            shout,
        })
    }
}

fn on_say(trigger: Trigger<SayCommand>) {
    let command = trigger.event();

    let message = if command.shout {
        command.message.to_uppercase()
    } else {
        command.message.clone()
    };
    // Print the main message
    repl_println!("Saying: {}", message);

    // Print repeated messages
    for i in 0..command.repeat {
        repl_println!("{}: {}", i + 1, message);
    }
}

fn instructions() {
    repl_println!();
    repl_println!("Welcome to the Bevy REPL custom renderer example!");
    repl_println!();
    repl_println!("Try typing a command:");
    repl_println!("  `ping`                     - Ping the app");
    repl_println!("  `say <message>`            - Say a message");
    repl_println!("  `say -s <message>`         - Shout the message");
    repl_println!("  `say -r N <message>`       - Repeat N times");
    repl_println!("  `quit`                     - Close the app");
    repl_println!();
    repl_println!("Press CTRL+C to exit any time.");
    repl_println!();
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                60.0_f64.recip(),
            ))),
            bevy::input::InputPlugin::default(),
            ReplPlugins.set(PromptPlugin {
                renderer: Arc::new(CustomRenderer),
                ..default()
            }),
        ))
        .add_repl_command::<PingCommand>()
        .add_observer(on_ping)
        .add_repl_command::<SayCommand>()
        .add_observer(on_say)
        .add_systems(PostStartup, instructions)
        .run();
}
