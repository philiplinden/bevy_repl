# Bevy REPL

An interactive command-line REPL integrated directly into the Bevy ECS schedule, allowing runtime inspection, command execution, and logging in headless and graphical applications.

## Language

### Core REPL & Lifecycle

**REPL**:
The central resource and controller that manages the interactive session, holding the active line buffer, cursor position, and registered command parsers.
_Avoid_: Console, shell, terminal instance

**Raw Mode**:
The terminal configuration where input characters are forwarded immediately without OS line buffering or local echo, enabling fine-grained cursor navigation and instant keystroke capture.
_Avoid_: Unbuffered mode, direct mode

**Input Suppression**:
The active clearing and resetting of Bevy's `ButtonInput<KeyCode>` and `KeyboardInput` streams while the REPL is active to prevent prompt keystrokes from leaking into game logic.
_Avoid_: Input blocking, key consumption, event swallowing

### Commands & Execution

**Command**:
A strongly-typed, `clap`-backed specification that parses shell tokenized arguments into an event dispatched to Bevy observer systems.
_Avoid_: Action, verb, CLI handler

**Command Parser**:
The parsing adapter that converts raw input strings into argv tokens and attempts to instantiate a typed `ReplCommand`.
_Avoid_: Command deserializer, argument reader

**Submission**:
The event and action triggered when the user commits an input line (usually via Enter), draining the active buffer and routing it to the command parser.
_Avoid_: Execution, line commit, dispatch event

### Prompt & Viewport

**Prompt**:
The interactive text input line and visual prefix (e.g. `> `) rendered at the designated terminal position.
_Avoid_: Input bar, command bar, cursor line

**Scroll Region**:
The partitioned terminal viewport established via DECSTBM ANSI escape sequences that constrains standard stdout and logging to lines above the pinned prompt.
_Avoid_: Window region, log viewport, split screen

**Keymap**:
The declarative registry that maps crossterm key events to atomic buffer modification actions.
_Avoid_: Keybindings table, hotkey router

**Buffer Action**:
An atomic operation on the REPL line buffer, such as character insertion, deletion, backspace, or cursor navigation.
_Avoid_: Editing event, buffer mutation
