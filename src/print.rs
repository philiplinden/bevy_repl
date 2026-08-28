//! Terminal output, scroll region management, and safe printing helpers.

use bevy::prelude::*;
use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    queue, terminal,
};
use std::io::{Write, stdout};

use crate::repl::Repl;

/// System that ensures the terminal scroll region reserves the bottom prompt area
/// so that stdout/logs scroll above the active prompt line.
pub fn manage_scroll_region(repl: Res<Repl>, mut last_state: Local<Option<(bool, u16)>>) {
    let Ok((_w, h)) = terminal::size() else {
        return;
    };
    let current_state = (repl.enabled, h);

    if last_state.as_ref() == Some(&current_state) {
        return;
    }

    let mut out = stdout();
    if repl.enabled {
        let bottom = h.saturating_sub(1);
        let _ = write!(out, "\x1B[1;{}r", bottom);
    } else if last_state.is_some() {
        let _ = write!(out, "\x1B[r");
    }
    let _ = out.flush();

    *last_state = Some(current_state);
}

/// Low-level function used by [`repl_println!`] to print a formatted line with
/// explicit CRLF (`\r\n`) and cursor coordination within the scroll region.
pub fn repl_print(args: std::fmt::Arguments) -> std::io::Result<()> {
    let mut out = stdout();

    if let Ok((_cols, rows)) = terminal::size() {
        // Move to the bottom of the scroll region (row H-1 in 0-based indexing)
        let target_row = rows.saturating_sub(2);
        let prompt_row = rows.saturating_sub(1);

        let _ = queue!(out, MoveTo(0, target_row));
        write!(out, "{}", args)?;
        write!(out, "\r\n")?;
        // Move back down toward prompt row
        let _ = queue!(out, MoveTo(0, prompt_row));
    } else {
        let _ = queue!(out, MoveToColumn(0));
        write!(out, "{}", args)?;
        write!(out, "\r\n")?;
    }

    out.flush()
}

/// Print a line that behaves well in raw/alternate screen contexts.
///
/// Ensures a carriage return is sent (CRLF) and stdout is flushed.
#[macro_export]
macro_rules! repl_println {
    () => {{
        let _ = $crate::print::repl_print(format_args!(""));
    }};
    ($($arg:tt)*) => {{
        let _ = $crate::print::repl_print(format_args!($($arg)*));
    }};
}
