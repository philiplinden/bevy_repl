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
use std::sync::atomic::{AtomicU64, AtomicU16, Ordering};

use bevy_ratatui::crossterm::{
    cursor::{MoveToColumn, MoveTo},
    queue,
};

// Track scroll region info (terminal height and reserved bottom lines) so printers can
// position output above the prompt area when using ratatui's alternate screen.
static SCROLL_H: AtomicU16 = AtomicU16::new(0);
static SCROLL_RESERVED: AtomicU16 = AtomicU16::new(0);

#[inline]
pub fn set_scroll_region_info(h: u16, reserved: u16) {
    SCROLL_H.store(h, Ordering::Relaxed);
    SCROLL_RESERVED.store(reserved, Ordering::Relaxed);
}

#[inline]
pub fn get_scroll_region_info() -> Option<(u16, u16)> {
    let h = SCROLL_H.load(Ordering::Relaxed);
    if h == 0 { return None; }
    let r = SCROLL_RESERVED.load(Ordering::Relaxed);
    Some((h, r))
}

// Track how many lines have been printed
static PRINT_COUNT: AtomicU64 = AtomicU64::new(0);

#[inline]
pub fn printed_lines() -> usize { PRINT_COUNT.load(Ordering::Relaxed).try_into().unwrap() }

/// Low-level function used by [`repl_println!`] to print a formatted line.
///
/// # Scroll Region Behavior
/// If a scroll region is active (as set by the pretty renderer), this function moves the cursor
/// to the last scrollable line (just above any reserved bottom lines) before printing. This ensures
/// that output scrolls above the prompt or status area, rather than overwriting it. If no scroll region
/// is active, the cursor is simply moved to column 0 for robustness.
///
/// # CRLF Handling
/// The function always appends a carriage return and line feed (`\r\n`) after the formatted content,
/// regardless of platform. This ensures correct line endings and cursor positioning in raw or alternate
/// screen modes, where standard `\n` may not behave as expected.
///
/// # When to Use
/// Use this function (or, preferably, the [`repl_println!`] macro) when printing output in raw or
/// alternate screen contexts, or when robust cursor and line handling is required. It is a drop-in
/// replacement for `println!` in these scenarios. For standard terminal output outside of raw/alt
/// screen modes, regular printing macros may suffice.
///
/// This function is typically not called directly; prefer using [`repl_println!`] for convenience.
pub fn repl_print(args: std::fmt::Arguments) -> std::io::Result<()> {
    let mut out = stdout();
    // If a scroll region is active (pretty mode), move to the last scrollable line
    // so output scrolls ABOVE the prompt area. When we position the cursor explicitly,
    // we skip MoveToColumn and rely on a simple '\n' for newline to avoid CR issues.
    let mut used_explicit_position = false;
    if let Some((h, reserved)) = get_scroll_region_info() {
        if reserved > 0 {
            let target_row = h.saturating_sub(reserved).saturating_sub(1); // 0-based row index
            queue!(out, MoveTo(0, target_row))?;
            used_explicit_position = true;
        }
    }
    if !used_explicit_position {
        // Minimal/normal case: ensure we start at column 0 for robustness
        queue!(out, MoveToColumn(0))?;
    }
    write!(out, "{}", args)?;
    write!(out, "\r\n")?;
    out.flush()
        .map(|_| { PRINT_COUNT.fetch_add(1, Ordering::Relaxed); () })
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
