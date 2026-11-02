# Design Approach

The Bevy engine logs its behavior and other messages at runtime that communicate what's happening to the developer under the hood.
User interaction is usually handled by the app logic, leaving the console to be read-only or emulated with a TUI.
Since TUIs are renderers, the commands and logs are discarded as soon as the app closes.
Normal (read-only) logging sends messages directly to the console, so messages can still be read after closing the app.

> 1. I want the normal headless logging experience **and** an
>    interactive CLI console while running my app, even without a
>    renderer, but
> 2. I do **not** want to maintain a second input system for
>    headless mode, and
> 3. I want the console to behave the same way **with and without**
>    an active renderer.

- [Thoughts about vanilla headless Bevy](vanilla.md): how Bevy
  behaves in a terminal without the REPL.
- [Vision for a better terminal experience](vision.md): what
  BevyREPL 1.0 should do, and what it won't.
