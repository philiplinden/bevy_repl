# Bevy REPL – Design Review (2025-08-09)

This note summarizes the current crate design, input/terminal handling, command flow, and open issues. It captures decisions made today and recommendations for next steps.

## Overview

- __Crate goal__: Provide an in-game REPL in a Bevy app with a terminal-style prompt and command dispatch.
- __Key modules__:
  - `src/repl.rs`: REPL resource (`Repl`), plugin (`ReplPlugin`), raw mode/terminal context management (`ReplContext` via bevy_ratatui `TerminalContext`), toggle event observer.
  - `src/prompt.rs`: Prompt UI + input systems (crossterm event consumption, REPL buffer update, prompt rendering, blocking input forwarding), Bevy-keyboard-based toggle.
  - `src/command.rs`: Command trait, Clap-based parsing and dispatch, shell-style tokenization.

## Input and terminal handling

- __Raw mode lifecycle__ (`src/repl.rs`):
  - Raw mode enabled when REPL is enabled (on `ReplToggleEvent::Enable`) by inserting `ReplContext` (which internally enables raw mode via crossterm/ratatui).
  - Raw mode restored/disabled when REPL is disabled (on `ReplToggleEvent::Disable`) by calling `ReplContext::restore()` and removing the resource.
- __Input sources__:
  - When REPL is disabled, Bevy input runs normally.
  - When REPL is enabled (raw mode on), we read crossterm `KeyEvent`s for prompt input.
- __Scheduling__ (high level):
  - Bevy toggle detector (`on_toggle_key_bevy`) is scheduled in `InputSet::Post`.
  - Prompt capture + buffer update (`capture_repl_input`, `update_repl_buffer`) are also scheduled (currently chained) in `InputSet::Post` and gated by `run_if(repl_is_enabled)`.
  - Prompt display + input forwarding block are in `InputSet::Post` when REPL is enabled.

## Toggle key design

- __Storage__: `Repl.toggle_key: Option<KeyCode>` (Bevy’s `KeyCode`), default `Some(KeyCode::Backquote)`.
- __Detection__ (both states):
  - Uses Bevy `ButtonInput<KeyCode>` to detect `just_pressed(KeyCode::Backquote)` in `on_toggle_key_bevy()`.
- __Filtering from REPL input__:
  - While enabled, crossterm events drive the buffer. We must prevent the toggle key from being inserted as text.
  - Correct approach: map Bevy `KeyCode` → crossterm `KeyCode` and skip that event in `capture_repl_input()`.
  - Today’s code regressed to directly comparing `Some(event.code) == repl.toggle_key`, which mixes `CrosstermKeyCode` with `KeyCode` and will not compile. We should restore the mapper or store both representations.

## Command parsing and dispatch

- __Tokenization__: `shell-words` splits input into argv respecting quotes/escapes.
- __Parsing__: Each command implements a `clap` definition; unknown subcommands/args are handled with error reporting.
- __Dispatch__: Commands are registered by primary name and aliases; lookup by first token; event(s) triggered on success.

## Event flow and gating

- __Gating__: Prompt-related systems run only when `repl_is_enabled()`.
- __Forwarding block__: When enabled, keyboard events should be consumed to avoid leaking to game logic; disabled state forwards normally.
- __Observers__: `manage_context()` observes `ReplToggleEvent` to enable/restore raw mode.

## Current issues / risks

- __Type mismatch in toggle filtering__ (`src/prompt.rs::capture_repl_input`):
  - Code compares `Some(event.code)` (crossterm) with `repl.toggle_key` (Bevy). Needs Bevy→crossterm mapping or dual storage.
- __System ordering for toggle detection__:
  - `on_toggle_key_bevy` currently in `InputSet::Post` alongside `block_keyboard_input_forwarding`. Ensure it runs before we block/clear input. Recommendation: explicitly order it `.before(block_keyboard_input_forwarding)` or place it in an earlier set that still sees Bevy input.
- __Lint warnings__ (prompt.rs): placeholders like `height`, `prompt_text` are unused; they can be prefixed with `_` or implemented in rendering.
- __Configurable toggle keys__:
  - Only `Backquote` is mapped/detected today. If we expose a public API for arbitrary toggle keys, we need a complete mapper and modifier handling (e.g., Ctrl+`, etc.).
- __Dual input path complexity__:
  - We intentionally simplified: Bevy detects toggle in both states; crossterm handles prompt characters only when enabled. Keep this boundary clean.

## Recommendations

- __Fix toggle filtering__:
  - Reintroduce a small Bevy→crossterm mapper (cover printable ASCII, arrows, Enter, etc.), and use it in `capture_repl_input()` to `continue` on the toggle key.
  - Alternatively store both `toggle_key_bevy: KeyCode` and `toggle_key_ct: KeyCode` (crossterm) when configuring, but mapping is lighter.
- __Clarify system order__:
  - Ensure `on_toggle_key_bevy` runs before `block_keyboard_input_forwarding`. Add explicit ordering to avoid future regressions.
- __Prompt rendering__:
  - Implement a minimal prompt renderer to consume the placeholder variables and remove lints.
- __Keybind API__:
  - Provide a `ReplPlugin::toggle_key(KeyCode)` builder method. Document which keys are supported and how modifiers are handled.
- __Testing__:
  - Add an example or integration test that simulates toggle on/off and verifies no stray backtick is inserted into the buffer after toggling.

## Open questions

- Should we allow modifier-based toggles (e.g., Ctrl+Backquote)? If so, we need a policy to keep Bevy/crossterm modifiers consistent and filter appropriately.
- Do we want a fallback toggle detector using crossterm when raw mode is on? Current design sticks to Bevy-only for simplicity.

## Action items

1. Restore Bevy→crossterm mapper and use it in `capture_repl_input()` for filtering.
2. Add `.before(block_keyboard_input_forwarding)` (or move set) for `on_toggle_key_bevy`.
3. Implement minimal prompt rendering to eliminate lints.
4. Expose `ReplPlugin::toggle_key()` and document supported keys.
5. Add example coverage for toggling and input filtering.
