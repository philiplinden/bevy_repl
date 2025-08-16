//! ECS log capture from tracing layer to Bevy `Event<LogEvent>`.
//! Based on bevy's `log_layers_ecs.rs` example, adapted to print via the REPL.

use std::sync::mpsc;

use bevy::prelude::*;
use bevy::log::{
    tracing::{self, Subscriber},
    tracing_subscriber::{self as ts, Layer},
    BoxedLayer,
};

/// Event emitted into the ECS for each tracing log event captured by the layer.
#[derive(Event, Clone)]
pub struct LogEvent {
    pub message: String,
    pub level: tracing::Level,
}

/// Non-send resource that holds the mpsc receiver (Receiver is !Sync).
pub struct CapturedLogEvents(pub mpsc::Receiver<LogEvent>);

/// Transfer all currently available items from the mpsc receiver into the ECS event queue.
pub fn transfer_log_events(
    mut log_events: EventWriter<LogEvent>,
    receiver: NonSend<CapturedLogEvents>,
) {
    log_events.write_batch(receiver.0.try_iter());
}

/// Tracing subscriber Layer that forwards events through an mpsc Sender to ECS.
struct CaptureLayer {
    sender: mpsc::Sender<LogEvent>,
}

impl<S: Subscriber> Layer<S> for CaptureLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: ts::layer::Context<'_, S>) {
        // Extract the formatted message from the event fields via a visitor
        let mut message = None;
        event.record(&mut CaptureLayerVisitor(&mut message));
        if let Some(message) = message {
            let metadata = event.metadata();
            let _ = self.sender.send(LogEvent { message, level: *metadata.level() });
        }
    }
}

/// Visitor that records the `message` field from tracing events as a String.
struct CaptureLayerVisitor<'a>(&'a mut Option<String>);
impl tracing::field::Visit for CaptureLayerVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            *self.0 = Some(format!("{value:?}"));
        }
    }
}

/// Create a CaptureLayer and register the plumbing in the provided `App`.
/// Returns the boxed Layer to be attached to Bevy's `LogPlugin` via its `custom_layer`.
pub fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    let (sender, receiver) = mpsc::channel();
    let layer = CaptureLayer { sender };

    app.insert_non_send_resource(CapturedLogEvents(receiver));
    app.add_event::<LogEvent>();
    app.add_systems(Update, transfer_log_events);

    Some(layer.boxed())
}

/// Convenience system that prints captured `LogEvent`s via the REPL printer so they appear
/// correctly above the prompt.
pub fn print_log_events_system(mut events: EventReader<LogEvent>) {
    use crate::repl_println;
    for ev in events.read() {
        repl_println!("{:5} {}", ev.level, ev.message);
    }
}

// --- Direct REPL formatting path: install a fmt layer that writes to REPL ---

/// A MakeWriter that produces writers which forward bytes to `repl_print` line-by-line,
/// preserving ANSI escapes produced by tracing's formatter.
struct ReplMakeWriter;

impl ts::fmt::MakeWriter<'_> for ReplMakeWriter {
    type Writer = ReplWriter;
    fn make_writer(&self) -> Self::Writer { ReplWriter::default() }
}

#[derive(Default)]
struct ReplWriter { buf: String }

impl std::io::Write for ReplWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for ch in s.chars() {
            if ch == '\n' {
                let line = std::mem::take(&mut self.buf);
                let _ = crate::print::repl_print(format_args!("{}", line));
            } else if ch != '\r' {
                use std::fmt::Write as _;
                let _ = self.buf.write_char(ch);
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if !self.buf.is_empty() {
            let line = std::mem::take(&mut self.buf);
            let _ = crate::print::repl_print(format_args!("{}", line));
        }
        Ok(())
    }
}

/// Install a global tracing subscriber with a fmt layer that writes to the REPL printer.
/// Call this BEFORE adding `DefaultPlugins`, and disable `LogPlugin` to avoid duplicate stdout.
pub fn tracing_to_repl_fmt() {
    tracing_to_repl_fmt_with_level(bevy::log::Level::INFO);
}

/// Same as `install_tracing_to_repl_fmt`, but lets you choose the max log level
/// (to mirror the `level` used by Bevy's `LogPlugin`).
pub fn tracing_to_repl_fmt_with_level(level: bevy::log::Level) {
    use ts::{fmt, prelude::*, registry::Registry};
    use ts::filter::LevelFilter;

    let lf = match level {
        bevy::log::Level::ERROR => LevelFilter::ERROR,
        bevy::log::Level::WARN => LevelFilter::WARN,
        bevy::log::Level::INFO => LevelFilter::INFO,
        bevy::log::Level::DEBUG => LevelFilter::DEBUG,
        bevy::log::Level::TRACE => LevelFilter::TRACE,
    };

    let layer = fmt::layer()
        .compact()
        .with_ansi(true)
        .with_writer(ReplMakeWriter)
        .with_filter(lf);

    let _ = Registry::default().with(layer).try_init();
}
