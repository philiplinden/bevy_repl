//! Direct tracing subscriber integration for Bevy REPL.

use bevy::log::BoxedLayer;
use bevy::prelude::*;
use tracing_subscriber::fmt;

/// Custom writer that forwards formatted tracing bytes line-by-line to `repl_print`,
/// ensuring all tracing logs format cleanly in the DECSTBM scroll region.
#[derive(Default)]
pub struct ReplWriter {
    buf: String,
}

impl std::io::Write for ReplWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for ch in s.chars() {
            if ch == '\n' {
                let line = std::mem::take(&mut self.buf);
                let _ = crate::print::repl_print(format_args!("{}", line));
            } else if ch != '\r' {
                self.buf.push(ch);
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

/// A MakeWriter factory that creates `ReplWriter` instances.
pub struct ReplMakeWriter;

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for ReplMakeWriter {
    type Writer = ReplWriter;
    fn make_writer(&self) -> Self::Writer {
        ReplWriter::default()
    }
}

/// Returns a boxed tracing layer ready to attach to Bevy's `LogPlugin`.
pub fn repl_tracing_layer(_app: &mut App) -> Option<BoxedLayer> {
    let layer = fmt::layer().with_ansi(true).with_writer(ReplMakeWriter);

    Some(Box::new(layer))
}
