# Summary

[Quickstart](index.md)

---

- [Custom Commands](commands/index.md)
  - [Builder Pattern](commands/builder_pattern.md)
  - [Derive Pattern](commands/derive_pattern.md)
  - [Built-in Commands](commands/default_commands.md)
<!--- [Configuration](_archive/config/index.md)
  - [Keybinds](_archive/config/keybinds.md)
  - [Prompt Styling](_archive/config/prompt_styling.md)-->

---

- [Design Approach](design/index.md)
  - [Thoughts about vanilla headless Bevy](design/vanilla.md)
  - [Lessons learned](design/lessons.md)
- [Implementation Details](dev/index.md)
  - [Known Issues & Limitations](dev/known_issues.md)
  - [Build Guide](dev/build_guide.md)
  - [Devlog](devlog/index.md)
    - [Ratatui Prototype](devlog/01-ratatui-prototype/index.md)
      - [2025-08-02](devlog/01-ratatui-prototype/2025-08-02.md)
      - [2025-08-03: Design](devlog/01-ratatui-prototype/2025-08-03.md)
      - [2025-08-04: Learning Clap](devlog/01-ratatui-prototype/2025-08-04.md)
      - [2025-08-05: REPL Frame Sequence Analysis](devlog/01-ratatui-prototype/2025-08-05.md)
      - [2025-08-06: Summary of Changes](devlog/01-ratatui-prototype/2025-08-06.md)
      - [2025-08-07: Prompt Display Investigation](devlog/01-ratatui-prototype/2025-08-07.md)
      - [2025-08-09: Design Review](devlog/01-ratatui-prototype/2025-08-09-design-review.md)
      - [2025-08-09: REPL Toggle and Raw Mode Behavior](devlog/01-ratatui-prototype/2025-08-09.md)
      - [2025-08-16: Logging + Prompt Rendering Rework](devlog/01-ratatui-prototype/2025-08-16-repl-logging-design.md)
      - [2025-08-16: Unicode-safe Prompt Rendering Fixes](devlog/01-ratatui-prototype/2025-08-16.md)
      - [2025-08-17: Logging Strategies](devlog/01-ratatui-prototype/2025-08-17-logging-strategies.md)
      - [2025-08-19: TUI Testing Strategy and Decisions](devlog/01-ratatui-prototype/2025-08-19-testing-strategy.md)
      - [2025-08-23: Retro: Renderer and Logging Rework](devlog/01-ratatui-prototype/2025-08-23-retro.md)
    - [Keeping Up with Bevy](devlog/02-keeping-up-with-bevy/index.md)
      - [2025-10-06: Bevy 0.16 → 0.17 Migration Impact](devlog/02-keeping-up-with-bevy/2025-10-06-bevy-0.17-migration-notes.md)
      - [2025-10-06: Bevy 0.17 Migration Spike](devlog/02-keeping-up-with-bevy/2025-10-06-bevy-0.17-migration-spike.md)
      - [2025-10-06: Recommendations Without `bevy_ratatui`](devlog/02-keeping-up-with-bevy/2025-10-06-recommendations-without-bevy_ratatui.md)
      - [2026-03-15: Bevy 0.17 → 0.18 Migration Impact](devlog/02-keeping-up-with-bevy/2026-03-15-bevy-0.18-migration.md)
    - [Crossterm Rewrite](devlog/03-crossterm-rewrite/index.md)
      - [2026-08-27: Research: Architecture and Implementation of `bevyterm`](devlog/03-crossterm-rewrite/2026-08-27-bevyterm_research.md)
      - [2026-08-27](devlog/03-crossterm-rewrite/2026-08-27.md)
      - [2026-08-28: Native Crossterm Migration & Architecture Deepening](devlog/03-crossterm-rewrite/2026-08-28.md)
      - [2026-08-30: Research: `bevy_log`/`tracing` Internals and Terminal Scroll-Region Prior Art](devlog/03-crossterm-rewrite/2026-08-30-logging-sink-research.md)
      - [2026-08-31: Handoff: Implement ADR 0003 (`fmt_layer` direct integration)](devlog/03-crossterm-rewrite/2026-08-31-fmt-layer-integration-handoff.md)
      - [2026-09-02: Reconsidering Tracing & The Clean-Slate Architecture](devlog/03-crossterm-rewrite/2026-09-02.md)
      - [2026-11-01: Picking Back Up: Bevy 0.20 and Finishing the Crossterm Migration](devlog/03-crossterm-rewrite/2026-11-01.md)
      - [2026-11-02: Stepping Back: Intent, Assumptions, Questions](devlog/03-crossterm-rewrite/2026-11-02.md)
<!--- Archived 2026-11-02, kept out of the book:
- [Vision for a better terminal experience](_archive/design/vision.md)
- [Design Approach (old)](_archive/design/index.md)
- [Preserving normal Logging system](_archive/logging.md)
- [Prototypes](_archive/prototypes/index.md)-->
