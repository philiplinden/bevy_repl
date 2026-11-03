# Design Approach

> [!WARNING]
> **Draft, not reviewed.** Drafted from the
> [2026-11-02 devlog](../devlog/03-crossterm-rewrite/2026-11-02.md) and not yet
> checked by the author. The devlog is the source until this page is reviewed.

This page holds the decisions that stand whichever way the console ends up
being built. How it's built is still being studied: two prototypes, a console
inside the app's terminal and a companion client over the Bevy Remote Protocol
(BRP), get compared side by side before one is taken further. See
[Lessons learned](lessons.md) for how the design got here, and
[Thoughts about vanilla headless Bevy](vanilla.md) for how Bevy behaves in a
terminal without BevyREPL.

<!-- toc -->

## Intent

A Bevy app is started from a terminal, with `cargo run` or the Bevy CLI.
Usually it opens a window and its logs stream to the terminal. It can also run
headless, with no renderer: just the schedule, the game logic and the event
system, with the logs still streaming to the terminal.

BevyREPL gives that app the experience of a quake-style console, including
when it runs headless and there's nothing to draw an in-game console with.
The console is a manual control panel for debugging and the like: a layer that
reaches into the app, maybe instruments it, and doesn't interfere with how the
app runs. The app runs as natively as possible, and using the console
shouldn't change its state or performance except through the commands
themselves.

Windowed, the window shows what the app is doing, so the console can take more
of the terminal. Headless, the logs are the only view of the app, so the
console can't take them away.

## Assumptions

1. A person is at the terminal that started the app, either at the machine
   itself or over SSH.
2. The app runs without the console too, detached or with no terminal at all.
   When there's no terminal to interact with, the console doesn't start, and
   the app carries on without it.
3. There are two run modes, windowed and headless, and the console behaves the
   same in both.
4. The console doesn't change how the app runs. It changes the app's state
   only when a command does so on purpose, and it costs no noticeable
   performance.
5. Log messages are kept somewhere they can be reviewed after the app exits.

## Scope for 1.0

- **A log file is the app author's job.** If they want one, they add a
  `custom_layer` to `LogPlugin` the normal way, as the Bevy examples do. The
  console never takes logs away from where they normally go.
- **Setup changes are allowed where they're needed.** If the console needs the
  app's logs, it asks the author for one `custom_layer` on `LogPlugin`. That's
  a reasonable thing to ask, and avoiding it isn't worth the trouble.
- **The console can use the alternate screen and a full TUI** if it runs in the
  app's terminal, as long as the terminal is restored on every exit.
- **The console can be switched on and off at runtime.**
- **Not considered for 1.0:** logs left in the terminal's scrollback after
  exit, piped input and output, command history, tab completion, custom
  keybinds, and access from anywhere but the local machine. These are set
  aside, not ruled out.

## Console behavior

What the console has to do, described by what the person sees, not how it's
built. Both prototypes are measured against this list.

### Commands

1. A typed line runs against the live app when Enter is pressed.
2. The line can be edited before Enter: Backspace and the Left and Right arrows
   at least.
3. Each response appears next to the command that caused it, readable, and
   distinct from the app's logs.
4. The same response also goes into the app's logs as its own log message, so
   the log holds a record of what was typed and what came back.
5. A bad command or bad arguments show clap's error, and the app keeps running.
6. `help` lists the commands, and `<command> --help` shows one command's usage.
7. `quit` exits the app cleanly.

### Logs

8. No log line is lost while the console is in use. The lines that arrived
   during typing can be read back.
9. Logs never garble the line being typed.

### Lifecycle

10. Every terminal the REPL touches is left exactly as it was, whether the app
    exits normally, through `quit`, through Ctrl+C or by panicking.
11. Ctrl+C exits whatever program it's pressed in. In the app's terminal,
    including a console there, it exits the app. In a separate client it exits
    only the client, and the app keeps running.
12. The app runs normally when the console is missing, fails or is closed.
13. A separate client doesn't crash when there's no app to connect to. It says
    no game is running, and Enter (or `retry`) tries again until there is one.

### Where it works

14. The same behavior with the app windowed or headless.
15. Windows, Linux, and over SSH.
16. Windowed, typing in the console never reaches the app's input, and keys
    pressed in the app window never reach the console.

### Cost

17. No noticeable cost to the app while nobody is typing.
18. The app author adds as little as possible.

## Still open

- Whether the console lives in the app's terminal or in a separate client over
  BRP. The prototypes decide.
- What the app author's setup looks like, which follows from that.
