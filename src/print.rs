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
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(feature = "pretty")]
use once_cell::sync::OnceCell;
#[cfg(feature = "pretty")]
use std::sync::RwLock;

use bevy_ratatui::crossterm::{
    cursor::{MoveToColumn, MoveTo},
    queue,
};

// Shared scroll-region info: (terminal_height, reserved_bottom_lines)
#[cfg(feature = "pretty")]
static SCROLL_REGION_INFO: OnceCell<RwLock<Option<(u16, u16)>>> = OnceCell::new();

#[cfg(feature = "pretty")]
fn scroll_region_info() -> &'static RwLock<Option<(u16, u16)>> {
    SCROLL_REGION_INFO.get_or_init(|| RwLock::new(None))
}

/// Set current scroll-region info; pretty renderer calls this when it updates the region.
#[cfg(feature = "pretty")]
pub fn set_scroll_region_info(height: u16, reserved_bottom: u16) {
    if let Ok(mut guard) = scroll_region_info().write() {
        if reserved_bottom == 0 { *guard = None; } else { *guard = Some((height, reserved_bottom)); }
    }
}

/// Read current scroll-region info.
#[cfg(feature = "pretty")]
pub fn get_scroll_region_info() -> Option<(u16, u16)> {
    scroll_region_info().read().ok().and_then(|g| *g)
}

// No-op stubs when pretty is disabled
#[cfg(not(feature = "pretty"))]
#[inline]
pub fn set_scroll_region_info(_: u16, _: u16) {}

#[cfg(not(feature = "pretty"))]
#[inline]
pub fn get_scroll_region_info() -> Option<(u16, u16)> { None }

// Track how many lines have been printed
static PRINT_COUNT: AtomicU64 = AtomicU64::new(0);

#[inline]
pub fn printed_lines() -> u64 { PRINT_COUNT.load(Ordering::Relaxed) }

/// Low-level function used by `repl_println!` to print a formatted line
/// with CRLF and flush, robust for raw/alt screen.
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
