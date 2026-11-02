# Bevy REPL

A command prompt for Bevy apps that lives in the host terminal, in both headless and windowed apps. Add `ReplPlugins` and the bottom line of the terminal becomes a prompt for `clap` commands, while Bevy's logs and `println!` output keep printing above it as usual.

## Language

### Core REPL & Lifecycle

**REPL**:
The central resource that manages the interactive session in the host terminal, holding the line buffer, cursor position, prompt symbol, registered command parsers, and the helpers that set up and restore the terminal.
_Avoid_: In-game overlay console, graphical console, shell instance, ReplContext, ReplTerminal, ReplPrompt

**Raw Mode**:
The terminal configuration where input characters are forwarded immediately without OS line buffering or local echo, enabling instant keystroke capture. Raw mode differs by platform: on Unix (Linux and macOS) it also turns off output processing, so a bare `\n` no longer returns the cursor to column 0; on Windows output is unaffected.
_Avoid_: Unbuffered mode, direct mode

**Additive**:
The REPL adds a prompt without changing anything else about the app. Bevy's logging, `println!`, and `DefaultPlugins` work as they would without the REPL.
_Avoid_: Integrated logging, log capture

**Terminal Restore**:
Returning the terminal to its original state (raw mode off, scroll region reset, cursor visible) whenever the app exits, including on Ctrl+C and on panic.
_Avoid_: Cleanup, teardown

### Commands & Execution

**Command**:
A `clap`-backed type, built with the builder API or `#[derive(ReplCommand)]`, that parses the input line into a value of that type. The parsed value is the event: it is triggered in the world and handled by observers added with `.add_observer`.
_Avoid_: Action, verb, CLI handler

**Built-in Command**:
A command that ships with the plugin (`quit`, `help`, `clear`), each behind its own feature flag.
_Avoid_: Default command, system command

**Command Parser**:
The adapter that splits the input line into shell-style argv tokens and tries to build a typed `ReplCommand` from them.
_Avoid_: Command deserializer, argument reader

**Submission**:
Committing an input line (usually with Enter), which drains the buffer and routes it to the command parsers.
_Avoid_: Execution, line commit, dispatch event

### Prompt & Viewport

**Prompt**:
The input line and its prefix (e.g. `> `), drawn on the bottom row of the terminal.
_Avoid_: Input bar, command bar, cursor line

**Scroll Region**:
The part of the terminal above the prompt, set with the DECSTBM escape sequence, where stdout and log output scroll without overwriting the prompt.
_Avoid_: Window region, log viewport, split screen

**Keymap**:
The fixed mapping from crossterm key events to buffer actions.
_Avoid_: Keybindings table, hotkey router

**Buffer Action**:
An atomic operation on the line buffer, such as inserting or deleting a character or moving the cursor.
_Avoid_: Editing event, buffer mutation
