# Research: Architecture and Implementation of `bevyterm`

*Date: 2026-08-27*  
*Target Repository: [Mimea005/bevyterm (branch: master)](https://github.com/Mimea005/bevyterm)*  
*Comparison Target: [octotep/bevy_crossterm](https://github.com/octotep/bevy_crossterm)*

---

## Executive Summary

`bevyterm` is a minimal Bevy plugin (developed against Bevy 0.10) that integrates terminal I/O using `crossterm` directly into Bevy's ECS schedules. Unlike older terminal integrations such as `bevy_crossterm`, `bevyterm` **does not replace Bevy's app runner** (`app.set_runner(...)`). Instead, it treats the terminal window as a first-class ECS entity and drives input polling, terminal state maintenance, and lifecycle cleanup through regular Bevy systems placed in base schedule sets.

---

## 1. Integration Pattern & Schedule Hooking

`bevyterm` exposes a root plugin `TerminalPlugin` (`src/lib.rs`), which aggregates two internal plugins: `WindowPlugin` (`src/window.rs`) and `EventPlugin` (`src/event.rs`).

```
App
 └── TerminalPlugin
      ├── WindowPlugin
      │    ├── Init Resource: CrosstermWindowSettings
      │    ├── StartupSet::Startup: setup_terminal
      │    ├── StartupSet & CoreSet Flush stages: restore_terminal_on_exit
      │    └── Update: update_title
      └── EventPlugin
           ├── Init Resources: Events<CrosstermEvent>, Events<KeyEvent>, etc.
           ├── CoreSet::First: poll_events
           └── CoreSet::PreUpdate: update_window
```

### Schedule Placements (Bevy 0.10 Base Sets)

1. **Terminal Setup (`StartupSet::Startup`)**:
   - `setup_terminal` initializes terminal raw mode, captures mouse/alternate screen based on settings, and spawns the window entity.
   - Chained via system piping: `.add_startup_system(setup_terminal.pipe(crash_on_err).in_base_set(StartupSet::Startup))`.

2. **Input Polling (`CoreSet::First`)**:
   - `poll_events` runs at the very beginning of each frame tick in `CoreSet::First`, ensuring all input events are buffered before user logic executes.

3. **Window State Sync (`CoreSet::PreUpdate`)**:
   - `update_window` consumes `WindowEvent::Resize` events and synchronizes dimensions into the `CrosstermWindow` component.

4. **Title Updates (`Update` set)**:
   - `update_title` queries `Query<&CrosstermWindow, Changed<CrosstermWindow>>` and updates the terminal window title via crossterm queue commands.

5. **Lifecycle Teardown (`*Flush` sets)**:
   - `restore_terminal_on_exit` is registered across **eight** flush sets to catch `AppExit` at any boundary:
     - `StartupSet::PreStartupFlush`
     - `StartupSet::StartupFlush`
     - `StartupSet::PostStartupFlush`
     - `CoreSet::FirstFlush`
     - `CoreSet::PreUpdateFlush`
     - `CoreSet::UpdateFlush`
     - `CoreSet::PostUpdateFlush`
     - `CoreSet::LastFlush`

### Error Handling via System Piping

`bevyterm` implements custom system adapters (`src/error_handling.rs`) taking advantage of Bevy's `.pipe(...)` combinator:
- `crash_on_err(In<Result<()>>, EventWriter<AppExit>)`: Logs errors via `bevy::log::error!` and triggers app termination via `AppExit`.
- `log_on_err(In<Result<()>>)`: Logs errors without exiting.

---

## 2. Input Handling

Input processing is entirely ECS-driven in `src/event.rs`.

### Non-Blocking Event Drain

In `poll_events` (`CoreSet::First`), `bevyterm` queries `crossterm::event::poll` with `Duration::ZERO` in a non-blocking `while` loop:

```rust
// src/event.rs
while crossterm::event::poll(Duration::ZERO)? {
    let event = crossterm::event::read()?;
    all_events.send(CrosstermEvent(event.clone()));

    match event {
        crossterm::event::Event::FocusGained => window_events.send(WindowEvent::FocusGained),
        crossterm::event::Event::FocusLost => window_events.send(WindowEvent::FocusLost),
        crossterm::event::Event::Resize(col, row) => window_events.send(WindowEvent::Resize(col, row)),
        crossterm::event::Event::Key(k) => key_events.send(KeyEvent(k)),
        crossterm::event::Event::Mouse(m) => mouse_events.send(MouseEvent(m)),
        #[cfg(feature = "bracketed-paste")]
        crossterm::event::Event::Paste(txt) => paste_events.send(PasteEvent(txt)),
        _ => unreachable!("All events should have been exhausted!")
    }
}
```

### Event Stream Multiplexing

`bevyterm` splits incoming crossterm events into distinct, typed Bevy events:
- `CrosstermEvent` — Generic newtype wrapper over `crossterm::event::Event`.
- `KeyEvent` — Newtype wrapper over `crossterm::event::KeyEvent`.
- `MouseEvent` — Newtype wrapper over `crossterm::event::MouseEvent`.
- `WindowEvent` — Enum for `FocusGained`, `FocusLost`, and `Resize(u16, u16)`.
- `PasteEvent` — Optional bracketed-paste string event.

### Buffer Management

At the beginning of each `poll_events` execution, event buffers are cleared manually (`all_events.clear()`, `key_events.clear()`, etc.) before new events are read.

---

## 3. Output and Rendering

`bevyterm` provides **no rendering subsystem or display abstraction**.

As documented in `readme.md`:
```markdown
- [ ] Display
   - How displaying something should look like.
   - Should text be its own primitive? (since the terminal is text based anyway)
```

### How Output Works in Practice

In both `examples/basic.rs` and `examples/events.rs`, systems perform direct ANSI/crossterm writes to `std::io::stdout()`:

```rust
// examples/events.rs
fn log_event(..., window: Query<&CrosstermWindow>, ...) -> Result<()> {
    let window = window.single();
    queue!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print(format!("Counter: {}", *counter)),
        MoveTo(window.width()/2, 0),
        Print(format!("Cols: {:03}, Rows: {:03}", window.width(), window.height()))
    )?;
    stdout().flush()?;
    Ok(())
}
```

There is no frame buffer, cell grid, double-buffering diff engine, or render schedule within `bevyterm`. Output is left entirely to user systems interacting with `std::io::stdout()`.

---

## 4. Lifecycle & Raw Mode Management

Terminal state is configured via the `CrosstermWindowSettings` resource (`src/components.rs`) and tracked on the `CrosstermWindow` component (`src/window.rs`).

```rust
#[derive(Resource)]
pub struct CrosstermWindowSettings {
    pub title: Option<String>,
    pub alternate_screen: bool,
    pub mouse_capture: bool,
}
```

### Initialization (`setup_terminal`)

1. Calls `crossterm::terminal::enable_raw_mode()?`.
2. Queries terminal dimensions: `crossterm::terminal::size()?`.
3. Queues terminal commands according to settings:
   - `stdout().queue(EnableMouseCapture)?`
   - `stdout().queue(EnterAlternateScreen)?`
   - `stdout().queue(SetTitle(title))?`
4. Spawns the primary window entity:
   ```rust
   command.spawn((
       CrosstermWindow {
           title: cross_settings.title.clone(),
           width,
           height,
           mouse_capture: cross_settings.mouse_capture,
           alternate_screen: cross_settings.alternate_screen,
       },
       PrimaryWindow,
   ));
   ```

### Teardown (`restore_terminal_on_exit`)

`restore_terminal_on_exit` listens for `EventReader<AppExit>`:
1. If `exit.is_empty()`, exits immediately (`Ok(())`).
2. If `AppExit` is present:
   - Disables mouse capture if enabled.
   - Leaves alternate screen if enabled.
   - Flushes `stdout()`.
   - Calls `crossterm::terminal::disable_raw_mode()?`.

### Limitations in Error/Panic Handling

- `bevyterm` handles planned exits (`AppExit`) cleanly across schedule flush points.
- It intercepts errors in its own registered systems using `.pipe(crash_on_err)`.
- However, **it does not set a standard panic hook (`std::panic::set_hook`)**. An unhandled panic in user code will unwind without running `restore_terminal_on_exit`, leaving the user's terminal stuck in raw mode unless an external panic handler is installed.

---

## 5. Architectural Comparison: `bevyterm` vs. `bevy_crossterm`

| Dimension | `bevy_crossterm` (octotep) | `bevyterm` (Mimea005) |
| :--- | :--- | :--- |
| **Runner Pattern** | **Custom Runner (`set_runner`)**: Replaces Bevy's main loop with `crossterm_runner`. Manages its own `tick` closure, sleep timing, and explicit `app.update()` calls. | **Schedule-Native (`Plugin`)**: Uses standard Bevy runner (`ScheduleRunnerPlugin` or default loop). Injects systems into standard schedule sets. |
| **Target Era** | Bevy 0.4 (Legacy `AppBuilder`, `SystemStage`, string-named stages). | Bevy 0.10 (`CoreSet`, `StartupSet`, system piping, `PrimaryWindow` component). |
| **Window Abstraction** | World **Resource** (`app.resources.insert(CrosstermWindow)`). | ECS **Component** spawned on an Entity alongside `bevy::window::PrimaryWindow`. |
| **Input Ingestion** | Ingested in the runner loop *outside* the ECS schedule, published to `Events<T>` resources before `app.update()`. | Polled *inside* the ECS schedule (`CoreSet::First`) via `poll_events` system with `poll(Duration::ZERO)`. |
| **Rendering Architecture** | **Full Terminal Renderer**: Custom stages (`PRE_RENDER`, `RENDER`, `POST_RENDER`), sprite/style asset loaders, and double-buffer diffing (`EntitiesToRedraw`, `PreviousEntityDetails`). | **None**: No rendering pipeline. Users write directly to `stdout()` with crossterm commands inside normal ECS systems. |
| **Terminal Teardown** | Executed in runner code after the `while tick(...)` loop exits. | Executed by `restore_terminal_on_exit` systems placed in `StartupSet` and `CoreSet` flush points when `AppExit` is observed. |
| **Coupling / Invasiveness** | High: Assumes ownership of app scheduling, rendering stages, and asset management. | Low: Minimal surface area (raw mode setup, event polling system, cleanup system). |

---

## 6. Takeaways for `bevy_repl`

1. **Schedule-Native Runner Integration is Preferable**:
   `bevyterm` proves that a terminal application in Bevy does not require a custom `app.set_runner(...)`. Using `ScheduleRunnerPlugin::run_loop(Duration::from_millis(N))` combined with non-blocking event polling (`poll(Duration::ZERO)`) in `PreUpdate` (or `First`) is simpler, avoids multi-loop synchronization, and works transparently with modern Bevy versions (0.17–0.19+).

2. **Window as Entity vs. Resource**:
   Representing the terminal state as a component (`CrosstermWindow`) on an entity with `PrimaryWindow` is idiomatic for modern Bevy windowing.

3. **Event Ingestion Strategy**:
   Polling crossterm events in `First`/`PreUpdate` with `Duration::ZERO` drains all pending user keystrokes for the current frame without blocking the ECS tick.

4. **Essential Gaps to Address**:
   - **Panic Safety**: A production terminal REPL needs a panic hook (`std::panic::set_hook` or `color_eyre::install()`) to restore terminal state and leave the alternate screen on panic, which `bevyterm` omitted.
   - **Targeted Output**: While `bevyterm` omitted rendering entirely, a REPL only requires line/prompt management or a lightweight Ratatui/crossterm draw pass during `PostUpdate`/`Last`, avoiding the heavy sprite rendering baggage of `bevy_crossterm`.
