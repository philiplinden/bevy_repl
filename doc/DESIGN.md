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

### Why use `clap` and `rustyline`?

The REPL uses two key libraries for handling user interaction:

`clap` handles command parsing by providing a robust, well-documented argument
parser with features like:

- Help message generation
- Subcommand support
- Argument validation
- Strong community support

`rustyline` manages terminal input with capabilities including:

- Command history
- Tab completion
- Syntax highlighting
- Wide community adoption

### Why use a three-thread architecture with channels?

**Problem:** The REPL needs to handle user input without blocking the main game loop,
while also managing output display and command processing.

**Solution:** Use a `ReplThreadManager` with three coordinated threads communicating
via channels.

**Thread Architecture:**

```text
[Main Bevy Thread] ←→ [Rustyline Input Thread] ←→ [Output Thread]
```

**Channel Communication:**

**Input Channel (`input_tx`/`input_rx`):**

- **Rustyline thread** → **Main thread**: Sends user commands
- `input_tx.send(line)` - Rustyline sends typed commands
- `input_rx.try_recv()` - Main thread receives commands

**Output Channel (`output_tx`/`output_rx`):**

- **Main thread** → **Output thread**: Sends command results
- `output_tx.send(result)` - Main thread sends command output
- `output_rx.recv()` - Output thread prints results

**How Each Thread Works:**

**Rustyline Input Thread:**

```rust
while !quit_flag.load(Ordering::Relaxed) {
    match rl.readline(&prompt) {
        Ok(line) => {
            rl.add_history_entry(&line).ok();
            input_tx.send(line).ok(); // Send to main thread
        }
    }
}
```

**Main Bevy Thread:**

```rust
// Receives commands from rustyline thread
while let Some(input) = self.try_recv_input() {
    // Process command
    let result = registry.parse_and_execute(&input, &mut world);
    self.send_output(result); // Send to output thread
}
```

**Output Thread:**

```rust
while let Ok(output) = output_rx.recv() {
    println!("{}", output); // Print to terminal
}
```

**Benefits:**

- **Non-blocking**: Main thread never blocks on I/O
- **Responsive**: Game continues running while waiting for input
- **Thread-safe**: Channels handle synchronization
- **Clean shutdown**: Quit flag coordinates all threads
- **Dynamic lifecycle**: Threads can be spawned/killed when REPL is enabled/disabled

**Flow:**

1. User types command → Rustyline thread captures it
2. Command sent via channel → Main thread processes it
3. Result sent via channel → Output thread prints it
4. All threads coordinate via quit flag for shutdown

This creates a **fully asynchronous REPL** that doesn't interfere with the game loop.

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
