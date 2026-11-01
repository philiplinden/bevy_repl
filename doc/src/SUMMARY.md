# Summary

[Introduction](index.md)

---

- [Writing Commands](commands/index.md)
  - [Builder Pattern](commands/builder_pattern.md)
  - [Derive Pattern](commands/derive_pattern.md)
- [Built-in Commands](features/default_commands.md)
- [Feature Flags](features/index.md)
- [Logging](how_it_works/logging.md)
- [Known Issues & Limitations](dev/known_issues.md)
<!--- [Configuration](_archive/config/index.md)
  - [Keybinds](_archive/config/keybinds.md)
  - [Prompt Styling](_archive/config/prompt_styling.md)-->

---

- [Vision for BevyREPL](design/index.md)
- [How It Works](how_it_works/index.md)
- [Development](dev/index.md)

---

- [Devlog](devlog/index.md)
  - [Ratatui Prototype](devlog/ratatui-prototype/index.md)
    - [2025-08-02](devlog/ratatui-prototype/2025-08-02.md)
    - [2025-08-03: Design](devlog/ratatui-prototype/2025-08-03.md)
    - [2025-08-04: Learning Clap](devlog/ratatui-prototype/2025-08-04.md)
    - [2025-08-05: REPL Frame Sequence Analysis](devlog/ratatui-prototype/2025-08-05.md)
    - [2025-08-06: Summary of Changes](devlog/ratatui-prototype/2025-08-06.md)
    - [2025-08-07: Prompt Display Investigation](devlog/ratatui-prototype/2025-08-07.md)
    - [2025-08-09: Design Review](devlog/ratatui-prototype/2025-08-09-design-review.md)
    - [2025-08-09: REPL Toggle and Raw Mode Behavior](devlog/ratatui-prototype/2025-08-09.md)
    - [2025-08-16: Logging + Prompt Rendering Rework](devlog/ratatui-prototype/2025-08-16-repl-logging-design.md)
    - [2025-08-16: Unicode-safe Prompt Rendering Fixes](devlog/ratatui-prototype/2025-08-16.md)
    - [2025-08-17: Logging Strategies](devlog/ratatui-prototype/2025-08-17-logging-strategies.md)
    - [2025-08-19: TUI Testing Strategy and Decisions](devlog/ratatui-prototype/2025-08-19-testing-strategy.md)
    - [2025-08-23: Retro: Renderer and Logging Rework](devlog/ratatui-prototype/2025-08-23-retro.md)
  - [Keeping Up with Bevy](devlog/keeping-up-with-bevy/index.md)
    - [2025-10-06: Bevy 0.16 → 0.17 Migration Impact](devlog/keeping-up-with-bevy/2025-10-06-bevy-0.17-migration-notes.md)
    - [2025-10-06: Bevy 0.17 Migration Spike](devlog/keeping-up-with-bevy/2025-10-06-bevy-0.17-migration-spike.md)
    - [2025-10-06: Recommendations Without `bevy_ratatui`](devlog/keeping-up-with-bevy/2025-10-06-recommendations-without-bevy_ratatui.md)
    - [2026-03-15: Bevy 0.17 → 0.18 Migration Impact](devlog/keeping-up-with-bevy/2026-03-15-bevy-0.18-migration.md)
  - [Crossterm Rewrite](devlog/crossterm-rewrite/index.md)
    - [2026-08-27: Research: Architecture and Implementation of `bevyterm`](devlog/crossterm-rewrite/2026-08-27-bevyterm_research.md)
    - [2026-08-27](devlog/crossterm-rewrite/2026-08-27.md)
    - [2026-08-28: Native Crossterm Migration & Architecture Deepening](devlog/crossterm-rewrite/2026-08-28.md)
    - [2026-08-30: Research: `bevy_log`/`tracing` Internals and Terminal Scroll-Region Prior Art](devlog/crossterm-rewrite/2026-08-30-logging-sink-research.md)
    - [2026-08-31: Handoff: Implement ADR 0003 (`fmt_layer` direct integration)](devlog/crossterm-rewrite/2026-08-31-fmt-layer-integration-handoff.md)
    - [2026-09-02: Reconsidering Tracing & The Clean-Slate Architecture](devlog/crossterm-rewrite/2026-09-02.md)
    - [2026-11-01: Picking Back Up: Bevy 0.20 and Finishing the Crossterm Migration](devlog/crossterm-rewrite/2026-11-01.md)

---

[Changelog](dev/changelog.md)
[License](dev/license.md)
