# Keybinds

This page explains the default keybinds and how to customize them via
`PromptKeymap`.


See `examples/keybinds.rs` for a runnable setup that configures `PromptKeymap`.

```bash
cargo run --example keybinds
```

## Default keybinds

The following keys control the REPL input buffer by default:

| Key         | Action                 |
|-------------|------------------------|
| Enter       | Submit command         |
| Esc         | Clear input buffer     |
| Left/Right  | Move cursor            |
| Home/End    | Jump to start/end      |
| Backspace   | Delete before cursor   |
| Delete      | Delete at cursor       |
| Ctrl+C      | Terminate app (signal) |

> [!WARNING]
> Ctrl+C behaves like a normal terminal interrupt because Bevy REPL
> installs a safety hook to handle `SIGINT` (Ctrl+C) and restore the terminal
> (disable raw mode) on exit. This works even if a quit command is disabled but
> also does not allow to use Ctrl+C to be mapped to other actions.

## Customizing keybinds

Keybinds are configured with the `PromptKeymap` resource in `bevy_repl::prompt::keymap`.
Each action maps to an exact `(KeyCode, KeyModifiers)` pair as a `ReplKeybind`.

> [!IMPORTANT]
> The REPL uses Crossterm keycodes and modifiers to capture input, NOT Bevy
> keycodes and modifiers.
> ```rust
> use bevy_ratatui::crossterm::event::{KeyCode as CrosstermKeyCode, KeyModifiers};
> ```

### Examples of combinations

- v: `ReplKeybind { code: CrosstermKeyCode::Char('v'), mods: KeyModifiers::NONE }`
- Shift+v: `ReplKeybind { code: CrosstermKeyCode::Char('V'), mods: KeyModifiers::SHIFT }`
- Ctrl+v: `ReplKeybind { code: CrosstermKeyCode::Char('v'), mods: KeyModifiers::CONTROL }`
- Ctrl+Shift+v: `ReplKeybind { code: CrosstermKeyCode::Char('V'), mods: KeyModifiers::CONTROL | KeyModifiers::SHIFT }`
- Ctrl+Alt+Shift+v: `ReplKeybind { code: CrosstermKeyCode::Char('V'), mods: KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SHIFT }`

### Capital letters and Shift

Terminals often report Shifted letters as uppercase `KeyCode::Char('V')` and may also set `SHIFT`.
Match both `code` and `mods` exactly in your binding.

By default, the fallback “insert printable char” only fires for unmodified keys (no modifiers).
If you want Shift-only typing (e.g., `Shift+v` -> `V`) to insert without an explicit binding,
you can relax the fallback policy inside `PromptKeymap::map`:

```rust
// inside PromptKeymap::map fallback
use bevy_ratatui::crossterm::event::KeyModifiers as M;
if self.allow_plain_char_insert {
    if let KeyCode::Char(c) = event.code {
        if event.modifiers.is_empty() || event.modifiers == M::SHIFT {
            return Some(ReplBufferEvent::Insert(c));
        }
    }
}
```

### Advanced mappings & Kitty protocol
Ratatui uses Kitty protocol by default, which is necessary for advanced keybinds
like Ctrl+Enter. For now, this is not supported in the REPL natively, but you
can use the REPL together with `bevy_ratatui` and may have better results.

See `examples/alt_screen.rs` for a runnable setup that uses `bevy_ratatui`.
