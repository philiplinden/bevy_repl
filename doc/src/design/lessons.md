# Lessons Learned

> [!WARNING]
> **Draft, not reviewed.** Drafted from the
> [2026-11-02 devlog](../devlog/03-crossterm-rewrite/2026-11-02.md) and not yet
> checked by the author.

What went wrong in the design process before 2026-11-02, and what to do
differently. The earlier documents are kept in `doc/src/_archive/`.

## Check imagined rules against reality early

The first vision set rules that were never tested: no TUI, no changes to
`LogPlugin` or `DefaultPlugins`, and logs left in the terminal's scrollback
after the app exits. Each one gave way once the real constraints were laid
out:

- A console and live logs sharing one headless terminal needs some kind of
  TUI.
- A plugin can't see Bevy's logs unless the app author routes them to it, and
  asking for one `custom_layer` is a reasonable price.
- What mattered about scrollback was not losing the logs, and a log file
  covers that.

Rules like these read as requirements but are guesses. Test them against how
Bevy, `tracing` and terminals work before building on them.

## Don't over-constrain the prototypes

The first prototype plan had rules that worked against it. Prototypes could
not depend on each other, yet they were numbered like a sequence. The list
mixed four kinds of question (reading input, where output goes, parsing
commands, cost), and several prototypes overlapped. It also tested
assumptions about a plugin whose intent and run conditions had never been
written down.

Start from the intent and the assumptions, list the questions that are
actually open, and design a prototype per question.

## Know why you're dropping a dependency

ratatui was removed because it felt too heavy, and the plan was a lower-level
crossterm loop. Once the two-pane console was described, ratatui's UI was
clearly needed again. What the removal was meant to achieve was avoiding a
separate UI system to maintain, and sparing the app author from setting one
up. A dependency contained inside the plugin meets that goal.

## Look for existing solutions before designing

Checking for prior art (bevyterm, bevy_term, the Bevy Remote Protocol) came
late. It found no crate to reuse, but it turned up BRP, a different way to
build the whole thing, after the design had already gone some distance. That
check is cheapest at the start.
