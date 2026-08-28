# Spec: Native Crossterm REPL Integration

## Destination

Remove `bevy_ratatui` and `ratatui`. Implement a native `crossterm` execution loop within Bevy's schedule, with a sticky prompt, panic-safe terminal lifecycle, tracing log routing, and macro-driven command dispatch.

## Architecture

### Core Modules
- `src/repl.rs` — `Repl` resource (enabled flag, input buffer, submit buffer)
- `src/prompt/` — Input capture, keymap, rendering (sticky prompt)
- `src/command/` — Parser (shell-words + clap), registration, observer dispatch
- `src/print.rs` — Scroll-region-aware log printer
- `src/log_ecs.rs` — Tracing layer
- `src/built_ins/` — Default commands (help, clear, quit)
- `src/prompt/keymap.rs` — `PromptKeymap` resource
- `src/prompt/scroll.rs` — DECSTBM scroll region management
- `src/prompt/input.rs` — Event capture + input suppression

### Schedule Sets (`ReplSet`)
| Set | Phase | Responsibility |
|-----|-------|---------------|
| `Capture` | `PreUpdate` | Non-blocking `crossterm::event::poll(Duration::ZERO)`; emit `CrosstermKeyEvent`; filter key kinds |
| `Buffer` | `Update` | Map key events → `ReplBufferEvent` (Insert, Backspace, Move, etc.) via `PromptKeymap` |
| `Parse` | `Update` (after Buffer) | Drain buffer on Submit → write `ReplSubmitEvent` |
| `Render` | `PostUpdate` | Draw sticky prompt at `H-1`; render buffer with cursor |
| `Post` | `PostUpdate` (after Render) | Suppress game keyboard input when REPL enabled |

### Key Data Structures

#### `Repl` (Bevy Resource)
```rust
pub struct Repl {
    pub enabled: bool,
    pub buffer: String,
    pub cursor: usize,
    pub history: Vec<String>,        // future work
    pub history_cursor: Option<usize>,
}
```

#### `PromptKeymap` (Bevy Resource)
- Maps `(KeyCode, KeyModifiers)` → `ReplBufferEvent`
- Default bindings installed by `ReplDefaultKeymapPlugin`

#### `CrosstermKeyEvent` (Bevy Message)
```rust
pub struct CrosstermKeyEvent(pub crossterm::event::KeyEvent);
```

### Input Pipeline

1. **Capture** (`PreUpdate` / `ReplSet::Capture`)
   - Only when `repl.enabled`:
   - `crossterm::event::poll(Duration::ZERO)` in a while loop
   - Read `crossterm::event::read()`
   - Filter: `KeyEventKind::Press | KeyEventKind::Repeat`
   - Send `CrosstermKeyEvent` message

2. **Buffer** (`Update` / `ReplSet::Buffer`)
   - `capture_terminal_input` reads `CrosstermKeyEvent` messages
   - Check for toggle key (default: backtick) → emit `ReplLifecycleEvent::Disable`
   - Use `PromptKeymap` to map key → `ReplBufferEvent`
   - Special cases:
     - Shift+Enter → `ReplBufferEvent::Insert('\n')` (multiline)
     - Ctrl+C → `ReplBufferEvent::Clear` (clear line)
     - Ctrl+L → `ReplBufferEvent::ClearScreen`
     - Ctrl+U → `ReplBufferEvent::ClearToStart`
   - Write `ReplBufferEvent` messages

3. **Update Buffer** (`Update` / `ReplSet::Buffer` — separate system)
   - `update_repl_buffer` reads `ReplBufferEvent` messages
   - Mutates `Repl` resource (insert/backspace/delete/move)
   - On Submit → write `ReplSubmitEvent` message

4. **Parse** (`Update` / `ReplSet::Parse`)
   - `parse_buffer_commands` (TODO — currently a stub)
   - Will: receive `ReplSubmitEvent`, tokenize buffer string

5. **Suppress Game Input** (`PostUpdate` / `ReplSet::Post`)
   - `block_keyboard_input_forwarding`
   - Clear `Messages<KeyboardInput>` and `ButtonInput<KeyCode>`
   - Unconditionally (no `enabled` check yet — may revisit)

### Command Pipeline

1. **Capture command string** (via `ReplBufferEvent::Submit`)
2. **Parse**: `shell-words` tokenizer → `Vec<String>`
3. **Dispatch to `clap::Command`** (registered via builder pattern)
4. **Trigger observer**: `Commands::trigger(ReplEventName)` on matched command
5. **Handler system** processes the triggered event

#### Command Registration
- `ReplCommand` derive (or `#[repl_command]` attribute macro)
- Auto-injects `clap::Parser`, `Event`, `Clone` derives
- Must implement `to_event: Fn(&Args) -> Option<M>` — explicit, no silent failures
- `bevy_repl_derive` is optional (`features = ["derive"]`)

### Terminal Lifecycle

#### `ReplTerminal` Resource (RAII)
- Constructed lazily when REPL is first enabled
- Holds `OriginalAlternateScreen` guard
- On `Drop` → restores terminal (raw mode off, cursor visible, alternate screen off)
- `ReplSet::Enable` / `ReplSet::Disable` schedule sets toggle `repl.enabled` + raw mode

#### Panic Safety
- Custom panic hook (`install_terminal_safety_nets`)
- Resets terminal before forwarding to default hook
- Uses `ctrlc` crate to handle SIGINT gracefully

### Sticky Prompt Rendering

#### DECSTBM Approach
- Use ANSI escape `\x1b[salvage;H-1r` to set scroll region on main screen
- Logs write to full terminal; prompt line reserved at `H-1`
- Cursor saved/restored around log writes
- On resize → recalculates and re-emits scroll region

#### Rendering (in `ReplSet::Render`)
- Change-detection on `Repl` resource
- Draw prompt (`>> ` or similar) + buffer at the reserved bottom line
- Position cursor within the rendered text

#### Log Routing (`repl_tracing_layer`)
- Integrates with Bevy's `LogPlugin`
- Writes logs at row `H-2` and above
- Restores cursor to prompt line after write
- Uses `ReplTerminal` guard for terminal state management

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `derive` | Yes | Enables `bevy_repl_derive` + `clap/derive` |
| `log` | No | Enables `tracing-subscriber` integration |
| `default_commands` | Yes | Built-in `quit`, `help`, `clear` commands |

### Dependencies (post-pruning)
- `bevy = { version = "0.19", default-features = false }`
- `crossterm = "0.28"`
- `clap = "4.6"`
- `shell-words = "1.1"`
- `ctrlc = "3"`
- `tracing-subscriber` *(optional, behind `log`)*

## Open Questions (Fog from Map)

- Multi-line / word-wrapped prompt reserving N bottom rows
- Status bar / gutter line for diagnostics
- Command history buffer + arrow-key navigation
- Comprehensive key modifier chord mapping table
- Whether unconditional input suppression is acceptable

## Implementation Notes for Human Developer

1. **Start with `ReplSet` ordering** — the schedule sets are the backbone
2. **Verify terminal state on every toggle** — panic hooks must run
3. **DECSTBM is terminal-dependent** — test on target platforms (Linux primary)
4. **Input suppression currently unconditional** — consider checking `repl.enabled` to avoid blocking game input during play
5. **Use `.cargo/config.toml` for fast builds** — linkers and flags already configured
6. **The `parse_buffer_commands` system is a stub** — implement tokenization + dispatch there
