# Design

The REPL is implemented as a Bevy plugin using `bevy_crossterm` to create a virtual terminal interface within the game window. It provides:

- A terminal emulation layer with input at the bottom and scrollable logs above
- Command parsing and execution using the Bevy ECS
- Integration with Bevy's logging system for unified output display
- Full ECS access for both read and write operations

The REPL is designed as an alternative to [makspll/bevy-console] for Bevy apps that want a terminal-like interface without external dependencies.

[makspll/bevy-console]: https://github.com/makspll/bevy-console

## User Experience

A developer adds the REPL plugin to their Bevy app and configures it with a config resource. Custom commands can be added by implementing the `ReplCommand` trait.

```rust
fn main() {
    let config = ReplConfig::new()
        .with_prompt("game> ")
        .with_terminal_size(80, 25);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .add_repl_command::<CustomGameCommand>()
        .run();
}
```

The REPL appears as a terminal interface within the game window:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL                    │
│ INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Type 'help' for commands        │
│                                                                              │
│ [Game logs and command output appear here...]                               │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│                                                                              │
│ game> spawn-player Bob                                                       │
└──────────────────────────────────────────────────────────────────────────────┘
```

The developer can type commands to interact with the game, with output displayed in the log area above the input line.

## Architecture Overview

### Terminal Emulation Layer

The REPL uses `bevy_crossterm` to create a virtual terminal interface:

```rust
struct TerminalEmulation {
    layout: TerminalLayout,
    input: ReplInput,
    log_capture: LogCapture,
    renderer: TerminalRenderer,
}
```

**Terminal Layout:**
- **Input Area**: Fixed bottom line for command input with cursor
- **Log Area**: Scrollable upper area for logs and command output
- **Border**: Optional terminal-style border around the interface

**Input Handling:**
- Real-time character input with cursor positioning
- Command history navigation (up/down arrows)
- Tab completion for commands and arguments
- Special key handling (Ctrl+C, Ctrl+D, etc.)

**Log Integration:**
- Captures Bevy logs via custom logger implementation
- Displays logs in scrollable buffer
- Supports log filtering and search
- Maintains log history across sessions

### Command System

Commands are processed using the Bevy ECS with full world access:

```rust
trait ReplCommand: Send + Sync + 'static {
    fn command(&self) -> clap::Command;
    fn execute(&self, world: &mut World) -> ReplResult<String>;
}
```

**Benefits of Full ECS Access:**
- Commands can read from and write to the world
- No threading complications or borrowing conflicts
- Access to all Bevy systems, components, and resources
- Support for complex game state queries and modifications

### Log Capture System

The REPL integrates with Bevy's logging system to capture and display logs:

```rust
struct LogCapture {
    log_buffer: VecDeque<LogEntry>,
    max_logs: usize,
    scroll_offset: usize,
    filter_level: log::LevelFilter,
}

struct LogEntry {
    level: log::Level,
    target: String,
    message: String,
    timestamp: Instant,
}
```

**Features:**
- Automatic capture of all Bevy log messages
- Configurable log levels and filtering
- Scrollable log history
- Timestamp and source tracking
- Color-coded log levels

## Design Decisions

### Why Terminal Emulation Instead of System Terminal?

**Problem:** System terminal integration has several limitations:
- Platform-specific behavior differences
- Threading complications and race conditions
- Limited control over appearance and behavior
- Poor integration with game UI and styling

**Solution:** Use `bevy_crossterm` to create a virtual terminal within the game window.

**Benefits:**
- **Cross-platform consistency**: Same behavior on all platforms
- **Full control**: Complete control over appearance and behavior
- **Better integration**: Seamless integration with game UI
- **No threading issues**: Everything runs in the main thread
- **Styling flexibility**: Can be themed and styled like other game elements

### Why Full ECS Access Instead of Limited Commands?

**Problem:** Previous approaches limited commands to only `Commands` parameter, preventing:
- World state inspection commands (`help`, `sysinfo`, `tree`)
- Commands that need to read components or resources
- Complex game state queries and modifications

**Solution:** Use `&mut World` parameter for all commands.

**Benefits:**
- **Complete access**: Commands can read and write to the world
- **No limitations**: Support for all types of game operations
- **Simpler API**: Single parameter for all world access needs
- **Future-proof**: Supports any future Bevy ECS features

### Why Integrated Log Capture Instead of Separate Output?

**Problem:** Separate output systems create:
- Disconnected user experience
- Difficulty tracking command context
- Inconsistent formatting and styling
- Complex output synchronization

**Solution:** Integrate log capture directly into the terminal emulation.

**Benefits:**
- **Unified experience**: All output appears in the same interface
- **Context preservation**: Logs and commands are visually connected
- **Consistent styling**: Uniform appearance across all output
- **Better debugging**: Easy to correlate commands with their effects

### Why bevy_crossterm Instead of Custom Terminal Implementation?

**Problem:** Custom terminal implementation would require:
- Extensive cross-platform terminal manipulation code
- Complex input handling and event processing
- Terminal state management and restoration
- Significant maintenance burden

**Solution:** Leverage `bevy_crossterm`'s mature terminal emulation.

**Benefits:**
- **Proven reliability**: Well-tested cross-platform terminal library
- **Rich features**: Built-in support for colors, styling, input handling
- **Active maintenance**: Regular updates and bug fixes
- **Bevy integration**: Designed specifically for Bevy applications

## Implementation Details

### Terminal Layout Management

The terminal layout is managed through a dedicated system:

```rust
fn terminal_layout_system(
    mut terminal: ResMut<TerminalEmulation>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    terminal.layout.update_size(window.width(), window.height());
}
```

**Layout Features:**
- Responsive sizing based on window dimensions
- Fixed input area height with flexible log area
- Automatic text wrapping and overflow handling
- Support for different terminal themes and styles

### Input Processing Pipeline

Input is processed through a multi-stage pipeline:

1. **Raw Input Capture**: `bevy_crossterm` captures keyboard events
2. **Input Processing**: Convert events to text input and special commands
3. **Command Parsing**: Use `clap` to parse commands and arguments
4. **Command Execution**: Execute commands with full world access
5. **Output Display**: Display results in the log area

### Log Integration

Logs are captured using a custom Bevy logger:

```rust
struct ReplLogger {
    log_capture: Arc<Mutex<LogCapture>>,
    inner_logger: Box<dyn log::Log>,
}
```

**Integration Points:**
- Intercepts all Bevy log messages
- Formats logs for terminal display
- Maintains log history and scroll state
- Provides filtering and search capabilities

## Built-in Commands

### `help`

Displays available commands and their descriptions. This command can now read from the command registry since it has full world access.

### `quit`

Gracefully shuts down the Bevy application by sending an `AppExit::Success` event.

### `close`

Disables the REPL but keeps the application running. The REPL can be re-enabled via toggle key or programmatically.

### `clear`

Clears the log area while preserving the input line.

### `sysinfo`

Displays system information including entity counts, resource usage, and performance metrics.

### `tree`

Shows a tree view of entities and their components, demonstrating full world access capabilities.

## Configuration Options

```rust
pub struct ReplConfig {
    pub prompt: String,
    pub terminal_size: (u16, u16),
    pub toggle_key: Option<KeyCode>,
    pub enabled_on_startup: bool,
    pub history_file: Option<String>,
    pub max_logs: usize,
    pub log_level: log::LevelFilter,
    pub theme: TerminalTheme,
}
```

**Configuration Features:**
- Customizable prompt and terminal size
- Keyboard toggle for enabling/disabling
- Command history persistence
- Log buffer size and filtering
- Terminal styling and themes

## Future Features

### High Priority

- [ ] **Tab Completion**: Intelligent completion for commands and arguments
- [ ] **Command Suggestions**: Similar command suggestions on typos
- [ ] **Log Search**: Search functionality within log history
- [ ] **Custom Themes**: Additional terminal themes and styling options

### Enhancement Features

- [ ] **Split Views**: Multiple terminal windows for different purposes
- [ ] **Scripting**: Support for running command scripts
- [ ] **Macros**: User-defined command macros and shortcuts
- [ ] **Export**: Export log history and command sessions
- [ ] **Remote Access**: Network-based REPL access for debugging
