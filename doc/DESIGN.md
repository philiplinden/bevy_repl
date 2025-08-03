# Design

The REPL is implemented as a Bevy plugin. It is responsible for:

- Receiving input from the user via the terminal
- Parsing the input into a command using `clap`
- Executing the command using the Bevy ECS
- Displaying the output in the terminal with other Bevy log messages

The REPL is meant to be an alternative to [makspll/bevy-console] for Bevy apps
that don't need a GUI but still want a console for debugging and development.

[makspll/bevy-console]: https://github.com/makspll/bevy-console

## User Experience

A developer adds the REPL plugin to their Bevy app and configures it with a
config resource. Custom commands can be added to the REPL by implementing the
`ReplCommand` trait, which allows you to register a `clap` command with the
REPL.

```rust
fn main() {
    let config = ReplConfig::new()
        .with_prompt("game> ")

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ReplPlugin::with_config(config))
        .add_repl_command::<CustomGameCommand>()
        .run();
}
```

The REPL will then be available in the terminal as a prompt
shown below the game's log messages.

```shell
INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL
game>
```

The developer can then type commands to interact with the game. The REPL will
display the output of the command in the terminal.

```shell
game> spawn-player Bob
```

```shell
INFO: 2025-07-28T12:00:00.000Z: bevy_repl: Starting REPL
game> spawn-player Bob
Spawned player: Bob
game>
```

To add or remove features from `clap` or `rustyline`, you can enable or disable
features in your `Cargo.toml` file alongside the `bevy_repl` dependency.

```toml
[dependencies]
bevy_repl = "0.1.0"
clap = { version = "4.5", features = ["derive", "suggestions", "color"] }
rustyline = { version = "16.0", features = ["with-file-history", "with-dirs"] }
```

## Design Decisions

### Why a separate thread for input handling?

**Problem:** Bevy's main thread runs the game loop and ECS systems. If we tried to
read user input directly in the main thread, it would:

- Block the entire game when waiting for user input
- Prevent the game from running at consistent frame rates
- Create a poor user experience

**Solution:** Move input handling to a separate thread that can block safely while
the main game continues running.

### Why use a resource queue instead of events for command processing?

**Problem:** Commands need to be processed across multiple frames and may require
complex state management, retry logic, and error handling.

**Solution:** Use a `ReplCommandQueue` resource instead of Bevy's event system.

**Benefits of Resource Queue over Events:**

- **Cross-frame processing**: Commands can be processed across multiple frames if needed
- **Error handling**: Can implement retry logic for failed commands
- **Queue inspection**: Can show pending commands to users or implement queue management
- **Batching**: Can process multiple commands in one frame if desired
- **State management**: Can track command execution state and metadata
- **Manual control**: Full control over when commands are added/removed from the queue

**Example advantages:**

```rust
// Can implement retry logic
if command_failed {
    command_queue.commands.push_front(failed_command); // Retry
}

// Can show queue status
repl.send_output(format!("{} commands pending", command_queue.commands.len()));

// Can batch process commands
let batch_size = 5;
for _ in 0..batch_size {
    if let Some(cmd) = command_queue.commands.pop_front() {
        // Process command
    }
}
```

**Events would be limiting because:**

- Events are automatically cleared after being read
- Events are designed for immediate, same-frame processing
- Events don't support complex state management
- Events can't be inspected or modified after being sent

### Why use `clap` and `crossterm`?

The REPL uses two key libraries for handling user interaction:

`clap` handles command parsing by providing a robust, well-documented argument
parser with features like:

- Help message generation
- Subcommand support
- Argument validation
- Strong community support

`crossterm` manages terminal input with capabilities including:

- Non-blocking event-driven input
- Cross-platform terminal manipulation
- Raw mode support
- Command history
- Line editing capabilities

### Why re-implement clap derive instead of using clap's derive macros?

**Problem:** We need to provide a clean, Bevy-specific API for command definition while leveraging clap's robust argument parsing capabilities.

**Solution:** Re-implement clap derive functionality in our own `bevy_repl_derive` macro rather than using clap's derive macros directly.

**Benefits of Re-implementation:**

- **Bevy Integration**: Our macro generates Bevy-specific code (access to `Commands`, ECS patterns)
- **Simplified API**: Users only need `#[derive(ReplCommand)]` instead of implementing traits manually
- **Custom Execution Flow**: We control how arguments flow into the `run()` method with direct field access
- **No Additional Dependencies**: Doesn't require clap's `derive` feature, keeping dependencies minimal

**Example of our approach:**

```rust
#[derive(ReplCommand)]
#[command(name = "spawn", about = "Spawn an entity")]
pub struct SpawnCommand {
    #[arg(help = "The name of the entity")]
    name: String,
    
    #[arg(short, long, default_value = "100")]
    health: i32,
}

impl SpawnCommand {
    fn run(&self, commands: &mut Commands) -> ReplResult<String> {
        // Direct field access with automatic parsing!
        commands.spawn(Health { value: self.health });
        Ok(format!("Spawned {} with {} health", self.name, self.health))
    }
}
```

**Alternative approach using clap derive:**

```rust
#[derive(Parser)]
#[command(name = "spawn", about = "Spawn an entity")]
pub struct SpawnCommand {
    #[arg(help = "The name of the entity")]
    name: String,
    
    #[arg(short, long, default_value = "100")]
    health: i32,
}

impl ReplCommand for SpawnCommand {
    fn command(&self) -> clap::Command {
        SpawnCommand::command()
    }
    
    fn execute(&self, commands: &mut Commands, matches: &clap::ArgMatches) -> ReplResult<String> {
        let args = SpawnCommand::from_arg_matches(matches)?;
        // More boilerplate to access fields
        commands.spawn(Health { value: args.health });
        Ok(format!("Spawned {} with {} health", args.name, args.health))
    }
}
```

**Trade-offs:**

**Re-implementation Pros:**

- Cleaner user experience with less boilerplate
- Full control over generated code
- Bevy-specific optimizations
- No dependency on clap's derive feature

**Re-implementation Cons:**

- Duplicating clap's attribute parsing logic
- Need to maintain compatibility with clap's API
- More complex macro implementation
- Risk of falling behind clap's features

**Decision:** The re-implementation approach was chosen to prioritize user experience and Bevy integration over leveraging clap's derive macros directly. This aligns with the project's goal of providing a seamless, Bevy-native REPL experience.

### Why use a single-thread architecture with crossterm?

**Problem:** The REPL needs to handle user input without blocking the main game loop,
while also managing output display and command processing.

**Solution:** Use a `CrosstermTerminal` with event-driven input processing integrated
directly into Bevy's main thread.

**Architecture:**

```text
[Main Bevy Thread with Crossterm Terminal]
```

**Event-Driven Input Processing:**

**Non-blocking Event Polling:**

- **Main thread** polls for terminal events every frame
- `event::poll(Duration::from_millis(0))` - Non-blocking event check
- `event::read()` - Read available events immediately

**Integrated Output Handling:**

- **Main thread** directly prints to terminal
- Direct terminal manipulation via crossterm
- Synchronized with game loop execution

**How It Works:**

**Event Polling in Main Thread:**

```rust
fn repl_input_system(mut terminal: ResMut<CrosstermTerminal>) {
    if let Ok(Some(event)) = terminal.poll_event() {
        match event {
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                let command = terminal.get_current_line();
                // Process command immediately
            }
            Event::Key(KeyEvent { code, .. }) => {
                terminal.handle_key(code); // Update prompt buffer
            }
        }
    }
}
```

**Direct Terminal Output:**

```rust
fn repl_output_system(mut terminal: ResMut<CrosstermTerminal>) {
    terminal.print_output(&result); // Direct terminal printing
}
```

**Benefits:**

- **Simpler architecture**: No thread coordination or channels needed
- **Lower overhead**: Single thread resource usage
- **Better integration**: Natural fit with Bevy's event-driven systems
- **Easier debugging**: All code runs in main thread
- **Non-blocking**: Event polling never blocks the game loop
- **Responsive**: Game continues running at consistent frame rates

**Flow:**

1. User types command → Event polled in main thread
2. Command processed immediately → Main thread handles execution
3. Result printed directly → Terminal output synchronized with game loop

This creates a **fully integrated REPL** that operates seamlessly within Bevy's event loop.

## Current Limitations

### World Access Not Supported

**Issue:** Commands that need to read from the Bevy `World` (like inspecting
entities, components, or resources) are not currently supported due to a
fundamental Bevy ECS constraint.

**Technical Problem:** The command execution system requires both:

- `Commands` parameter for mutable world access (spawning entities, sending events)
- `&World` parameter for immutable world access (reading entities, components, resources)

**Bevy ECS Conflict:** Having both `Commands` and `&World` in the same system
violates Rust's borrowing rules, causing a runtime panic: `&World` conflicts
with a previous mutable system parameter. Allowing this would break Rust's
mutability rules

**Impact:** This prevents implementing commands like:

- `help` - Cannot read the command registry from world resources
- `sysinfo` - Cannot read diagnostics or entity counts
- `tree` - Cannot inspect entities and their components
- Custom commands that need to query the world state

**Current Workaround:** Only commands that work with `Commands` (spawning, events, basic operations) are supported.

**Future Solutions:** Potential approaches to resolve this:

1. **Exclusive Systems** - Use `&mut World` instead of individual parameters
2. **Split Command Types** - Separate systems for read-only vs write commands  
3. **Deferred Execution** - Queue world-reading operations for later execution
4. **Command Buffer Pattern** - Collect world data in one frame, execute commands in another

This is a known architectural limitation that will be addressed in future versions.

## Built-in Commands

### `quit`

Gracefully shuts down the Bevy application by sending an `AppExit::Success` event.

### `close`

Disables the REPL but keeps the application running. The REPL can be re-enabled via toggle key (if configured) or programmatically.

## Future Features

### High Priority

- [ ] **Resolve World Access Limitation** - Implement one of the proposed solutions to enable commands that read from the Bevy `World`
- [ ] **Restore Built-in Commands** - Re-implement `help`, `sysinfo`, and `tree` commands once world access is resolved

### Enhancement Features  

- [ ] Add command suggestions with `trie-rs` similar to the implementation in `bevy-console`
- [ ] Add a `clear` command to clear the terminal
- [ ] Add a `history` command to show the command history
- [ ] Add a `clear-history` command to clear the command history
- [ ] Add tab completion for command names and arguments
- [ ] Add command aliases and shortcuts
