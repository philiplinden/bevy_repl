//! ECS log capture from tracing layer to Bevy `Event<LogEvent>`.
//! Based on bevy's `log_layers_ecs.rs` example, adapted to print via the REPL.

use std::sync::mpsc;
use std::collections::VecDeque;

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

/// Configuration for in-frame logging.
#[derive(Resource, Debug, Clone)]
pub struct LogCaptureConfig {
    pub level: bevy::log::Level,
    pub capacity: usize,
    /// If true, this plugin will install a global tracing subscriber with the CaptureLayer.
    /// Set to false if you use Bevy's LogPlugin with `custom_layer()` instead.
    pub init_subscriber: bool,
}

impl Default for LogCaptureConfig {
    fn default() -> Self {
        Self { level: bevy::log::Level::INFO, capacity: 512, init_subscriber: true }
    }
}

/// Plugin that wires tracing capture -> ECS events -> LogBuffer for in-ratatui rendering.
pub struct InFrameLogPlugin;

impl Plugin for InFrameLogPlugin {
    fn build(&self, app: &mut App) {
        // Config
        app.init_resource::<LogCaptureConfig>();
        let cfg = app.world().get_resource::<LogCaptureConfig>().cloned().unwrap_or_default();

        // Events and transfer system
        app.add_event::<LogEvent>();
        app.add_systems(Update, transfer_log_events);

        // Install NonSend receiver if missing
        if !app.world().contains_non_send::<CapturedLogEvents>() {
            let (sender, receiver) = mpsc::channel();
            app.insert_non_send_resource(CapturedLogEvents(receiver));

            if cfg.init_subscriber {
                use ts::{prelude::*, registry::Registry};
                use ts::filter::LevelFilter;
                let lf = match cfg.level {
                    bevy::log::Level::ERROR => LevelFilter::ERROR,
                    bevy::log::Level::WARN => LevelFilter::WARN,
                    bevy::log::Level::INFO => LevelFilter::INFO,
                    bevy::log::Level::DEBUG => LevelFilter::DEBUG,
                    bevy::log::Level::TRACE => LevelFilter::TRACE,
                };
                let layer = CaptureLayer { sender };
                let _ = Registry::default().with(layer.with_filter(lf)).try_init();
            } else {
                // If not installing a subscriber here, drop the sender to avoid leaks.
                drop(sender);
            }
        }

        // Buffer + drain system
        if !app.world().contains_resource::<LogBuffer>() {
            app.insert_resource(LogBuffer::with_capacity(cfg.capacity));
        }
        app.add_systems(Update, drain_events_into_buffer);
    }
}

/// Non-send resource that holds the mpsc receiver (Receiver is !Sync).
pub struct CapturedLogEvents(pub mpsc::Receiver<LogEvent>);

/// Send side used by the Capture layer; stored so `custom_layer(&mut World)` can read it.
#[derive(Resource, Clone)]
pub struct CaptureLogSender(pub mpsc::Sender<LogEvent>);

/// Plugin that sets up the channel plumbing and ECS wiring required for capture.
/// Add this BEFORE `DefaultPlugins` if you want to use `bevy::log::LogPlugin::custom_layer`.
pub struct CapturePlumbingPlugin;

impl Plugin for CapturePlumbingPlugin {
    fn build(&self, app: &mut App) {
        // Ensure events and transfer system exist
        app.add_event::<LogEvent>();
        app.add_systems(Update, transfer_log_events);

        // Install NonSend receiver and store a Sender resource (if missing)
        if !app.world().contains_non_send::<CapturedLogEvents>() {
            let (sender, receiver) = mpsc::channel();
            app.insert_non_send_resource(CapturedLogEvents(receiver));
            app.insert_resource(CaptureLogSender(sender));
        }
    }
}

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

/// Build a capture Layer using the sender stored by `CapturePlumbingPlugin`.
/// Attach this to Bevy's `LogPlugin` via its `custom_layer` field.
///
/// Usage:
///   .add_plugins(CapturePlumbingPlugin)
///   .add_plugins(DefaultPlugins.set(LogPlugin { custom_layer: |app| custom_layer(app), ..Default::default() }))
pub fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
    let sender = app.world().get_resource::<CaptureLogSender>()?.0.clone();
    let layer = CaptureLayer { sender };
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

/// A single log line to display inside the ratatui frame (minimal fields for now).
#[derive(Debug, Clone)]
pub struct LogLine {
    pub level: tracing::Level,
    pub message: String,
}

/// Circular buffer of recent log lines for in-frame rendering.
#[derive(Resource, Debug)]
pub struct LogBuffer {
    pub lines: VecDeque<LogLine>,
    pub capacity: usize,
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self { lines: VecDeque::with_capacity(256), capacity: 256 }
    }
}

impl LogBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { lines: VecDeque::with_capacity(capacity), capacity }
    }
    pub fn push(&mut self, line: LogLine) {
        if self.lines.len() >= self.capacity { let _ = self.lines.pop_front(); }
        self.lines.push_back(line);
    }
}

/// Drain captured `LogEvent`s into the in-memory `LogBuffer` used by the renderer.
pub fn drain_events_into_buffer(
    mut events: EventReader<LogEvent>,
    mut buffer: ResMut<LogBuffer>,
) {
    for ev in events.read() {
        buffer.push(LogLine { level: ev.level, message: ev.message.clone() });
    }
}

/// Plugin that ensures `LogBuffer` exists and drains events into it.
pub struct LogBufferPlugin {
    pub level: bevy::log::Level,
    pub capacity: usize,
}

impl Default for LogBufferPlugin {
    fn default() -> Self { Self { level: bevy::log::Level::INFO, capacity: 512 } }
}

impl Plugin for LogBufferPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<LogBuffer>() {
            app.insert_resource(LogBuffer::with_capacity(self.capacity));
        }
        app.add_systems(Update, drain_events_into_buffer);
        app.add_plugins((
            InFrameLogPlugin,
            CaptureSubscriberPlugin { level: self.level },
        ));
    }
}

/// Plugin that installs the capture subscriber and wires captured events into ECS.
pub struct CaptureSubscriberPlugin {
    pub level: bevy::log::Level,
}

impl Default for CaptureSubscriberPlugin {
    fn default() -> Self { Self { level: bevy::log::Level::INFO } }
}

impl Plugin for CaptureSubscriberPlugin {
    fn build(&self, app: &mut App) {
        use ts::{prelude::*, registry::Registry};
        use ts::filter::LevelFilter;

        app.add_event::<LogEvent>();
        app.add_systems(Update, transfer_log_events);

        if !app.world().contains_non_send::<CapturedLogEvents>() {
            let (sender, receiver) = mpsc::channel();
            app.insert_non_send_resource(CapturedLogEvents(receiver));

            let lf = match self.level {
                bevy::log::Level::ERROR => LevelFilter::ERROR,
                bevy::log::Level::WARN => LevelFilter::WARN,
                bevy::log::Level::INFO => LevelFilter::INFO,
                bevy::log::Level::DEBUG => LevelFilter::DEBUG,
                bevy::log::Level::TRACE => LevelFilter::TRACE,
            };
            let layer = CaptureLayer { sender };
            let _ = Registry::default().with(layer.with_filter(lf)).try_init();
        }
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
