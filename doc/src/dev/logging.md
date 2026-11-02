# Logging

The REPL is purely additive: Bevy's logs and anything printed with `println!`
should keep printing the way they would without the REPL, above the prompt. You
shouldn't need to change `LogPlugin`, disable it, or modify `DefaultPlugins`.

The REPL reserves the bottom row of the terminal for the prompt and sets a
scroll region (DECSTBM) over the rows above it. Output scrolls inside that
region, so it doesn't overwrite the prompt. Because nothing is captured or
redirected, output stays in the terminal after the app exits.

Use `repl_println!` to print from a command's observer. It prints into the
scroll region like `println!` does and handles raw mode on every platform.

> [!WARNING]
> This is the target behavior for 1.0, and the code doesn't fully match it yet.
> Most examples still call `DefaultPlugins.adapt_for_repl()`, which disables
> `LogPlugin` so the REPL can install its own log writer, and log lines written
> while the user is typing can garble the prompt. See the
> [2026-11-01 devlog](../devlog/03-crossterm-rewrite/2026-11-01.md) for the plan.
