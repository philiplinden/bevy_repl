use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;

/// Compute the bottom-aligned bar area with the given height inside the total frame area.
pub fn bottom_bar_area(total: Rect, height: u16) -> Rect {
    if total.height == 0 || height == 0 || total.height < height {
        return Rect {
            x: total.x,
            y: total.y,
            width: total.width,
            height: 0,
        };
    }
    Rect {
        x: total.x,
        y: total.y + total.height - height,
        width: total.width,
        height,
    }
}

/// Computes a visible window (substring) of the input buffer such that the cursor remains visible within the given
/// display width (columns), accounting for Unicode grapheme clusters and wide/combining characters, using ratatui APIs.
///
/// The function ensures that the returned slice of the buffer is at most `visible_width` characters wide and contains the character at the `cursor` position (or as close as possible if the cursor is near the start or end).
///
/// # Arguments
/// * `buffer` - The full input string to window.
/// * `cursor` - The current cursor position (as a byte index into `buffer`).
/// * `visible_width` - The maximum number of display columns to show in the window.
///
/// # Returns
/// A tuple containing:
/// * `String` - The visible slice of the buffer to display.
/// * `usize` - The starting byte index in `buffer` of the visible slice.
///
/// # Algorithm
/// The function calculates a window of up to `visible_width` display columns that ends at or includes the cursor
/// grapheme, ensuring the cursor is always visible. If the buffer's display width is shorter than `visible_width`,
/// the entire buffer is shown.
pub fn buffer_window(buffer: &str, cursor: usize, visible_width: u16) -> (String, usize) {
    if visible_width == 0 {
        return (String::new(), 0);
    }
    let vis_cols = visible_width as usize;
    let cursor = cursor.min(buffer.len());

    // Iterate graphemes and compute byte offsets and widths via ratatui
    let span = Span::raw(buffer);
    let mut graphemes: Vec<(usize, &str, usize)> = Vec::new(); // (start_byte, symbol, width)
    let mut byte_off = 0usize;
    for g in span.styled_graphemes(Style::default()) {
        let sym: &str = g.symbol.as_ref();
        let w = Span::raw(sym.to_string()).width();
        graphemes.push((byte_off, sym, w));
        byte_off += sym.len();
    }

    // Fast path: whole buffer fits
    let total_cols: usize = graphemes.iter().map(|(_, _, w)| *w).sum();
    if total_cols <= vis_cols {
        return (buffer.to_string(), 0);
    }

    // Find the grapheme index that contains the cursor (or the nearest before it)
    let mut cur_g_idx = 0usize;
    for (i, (b, sym, _)) in graphemes.iter().enumerate() {
        let end = *b + sym.len();
        if cursor < end {
            cur_g_idx = i;
            break;
        }
        if i == graphemes.len() - 1 {
            cur_g_idx = i;
        }
    }

    // Stage 1: walk left from the cursor to find a start index such that the cursor remains visible
    let mut used_cols = 0usize;
    let mut start_idx = cur_g_idx + 1; // exclusive end while walking left
    let mut i = cur_g_idx as isize;
    while i >= 0 {
        let w = graphemes[i as usize].2;
        if used_cols + w > vis_cols {
            break;
        }
        used_cols += w;
        start_idx = i as usize; // inclusive start
        i -= 1;
    }

    // Stage 2: extend the window to the right from the cursor to fill remaining columns
    let mut end_idx = cur_g_idx; // inclusive; we'll grow rightwards
    let mut j = cur_g_idx + 1;
    while j < graphemes.len() {
        let w = graphemes[j].2;
        if used_cols + w > vis_cols {
            break;
        }
        used_cols += w;
        end_idx = j;
        j += 1;
    }

    // Slice bytes from start to end_idx (inclusive)
    let start_byte = graphemes[start_idx].0;
    let end_byte = {
        let (b, sym, _w) = graphemes[end_idx];
        b + sym.len()
    };
    let slice = &buffer[start_byte..end_byte];
    (slice.to_string(), start_byte)
}

/// Compute clamped cursor position inside `left` area where `left` contains the prompt symbol
/// followed by the visible buffer slice. Computes display columns using Unicode widths.
pub fn cursor_position(
    left: Rect,
    prompt_width: u16,
    buffer: &str,
    start: usize,
    cursor: usize,
) -> (u16, u16) {
    let start = start.min(buffer.len());
    let cursor = cursor.min(buffer.len());
    let slice = &buffer[start..cursor];
    let cursor_cols: u16 = Span::raw(slice.to_string()).width() as u16;
    let unclamped_x = left.x + prompt_width + cursor_cols;
    let max_x = left.x + left.width.saturating_sub(1);
    let x = unclamped_x.min(max_x);
    (x, left.y)
}
