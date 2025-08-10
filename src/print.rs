//! Simple, robust printing helpers suitable for raw/alternate screen.
//! 
//! Use the `repl_println!` macro to print a formatted line that:
//! - moves the cursor to column 0
//! - writes the formatted content
//! - appends a CRLF ("\r\n")
//! - flushes stdout
//!
//! This avoids newline/cursor issues that can happen in raw or alternate screen modes.

use std::io::{stdout, Write};

use bevy_ratatui::crossterm::{
    cursor::MoveToColumn,
    queue,
};

/// Low-level function used by `repl_println!` to print a formatted line
/// with CRLF and flush, robust for raw/alt screen.
pub fn repl_print(args: std::fmt::Arguments) -> std::io::Result<()> {
    let mut out = stdout();
    // Ensure we start at column 0, then print, add CRLF, and flush.
    queue!(out, MoveToColumn(0))?;
    write!(out, "{}", args)?;
    write!(out, "\r\n")?;
    out.flush()
}

/// Print a line that behaves well in raw/alternate screen contexts.
///
/// This is a drop-in replacement where you'd use `println!`, but it ensures
/// a carriage return is sent (CRLF) and stdout is flushed.
///
/// Example:
/// ```ignore
/// repl_println!("Hello {}", name);
/// ```
#[macro_export]
macro_rules! repl_println {
    () => {{
        let _ = $crate::print::repl_print(format_args!(""));
    }};
    ($($arg:tt)*) => {{
        let _ = $crate::print::repl_print(format_args!($($arg)*));
    }};
}
