use ratatui::layout::Rect;

/// Compute the bottom-aligned bar area with the given height inside the total frame area.
pub fn bottom_bar_area(total: Rect, height: u16) -> Rect {
    if total.height == 0 || height == 0 || total.height < height {
        return Rect { x: total.x, y: total.y, width: total.width, height: 0 };
    }
    Rect {
        x: total.x,
        y: total.y + total.height - height,
        width: total.width,
        height,
    }
}

/// Computes a visible window (substring) of the input buffer such that the cursor remains visible within the given width.
///
/// The function ensures that the returned slice of the buffer is at most `visible_width` characters wide and contains the character at the `cursor` position (or as close as possible if the cursor is near the start or end).
///
/// # Arguments
/// * `buffer` - The full input string to window.
/// * `cursor` - The current cursor position (as a byte index into `buffer`).
/// * `visible_width` - The maximum number of characters to display in the window.
///
/// # Returns
/// A tuple containing:
/// * `String` - The visible slice of the buffer to display.
/// * `usize` - The starting character index in `buffer` of the visible slice.
///
/// # Algorithm
/// The function calculates a window of up to `visible_width` characters that ends at or after the cursor position, ensuring the cursor is always visible. If the buffer is shorter than `visible_width`, the entire buffer is shown. If the cursor is near the end, the window shifts left to keep the cursor visible.
pub fn buffer_window(buffer: &str, cursor: usize, visible_width: u16) -> (String, usize) {
    if visible_width == 0 {
        return (String::new(), 0);
    }
    let cursor = cursor.min(buffer.len());
    let vis = visible_width as usize;
    let start = cursor.saturating_sub(vis);
    let slice: String = buffer.chars().skip(start).take(vis).collect();
    (slice, start)
}

/// Compute clamped cursor position inside `left` area where `left` contains the prompt symbol
/// followed by the visible buffer slice.
pub fn cursor_position(left: Rect, prompt_width: u16, start: usize, cursor: usize) -> (u16, u16) {
    let cursor_col_in_buf = cursor.saturating_sub(start) as u16;
    let unclamped_x = left.x + prompt_width + cursor_col_in_buf;
    let max_x = left.x + left.width.saturating_sub(1);
    let x = unclamped_x.min(max_x);
    (x, left.y)
}
