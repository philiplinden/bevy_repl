# Bevy REPL TUI Testing Strategy and Decisions (2025-08-19)

This note summarizes the decisions made to establish a robust, repeatable testing approach for the terminal UI (TUI) renderers.

## Goals
- Ensure visual correctness of ratatui-based UIs without manual runs.
- Make tests deterministic, CI-friendly, and extensible to future renderers.

## Key Decisions

- **Introduce shared test harness**
  - Added `src/test_support/` with:
    - `RendererUnderTest` trait: adapters register renderer plugins on a Bevy `App`.
    - `TestAppBuilder`: builds a minimal `App` for renderer/system tests.
  - Gated with `#[cfg(any(test, feature = "test"))]` and `features.test = []`.
  - Exposed from `src/lib.rs` only in tests or when `--features test` is used.

- **Layered testing approach**
  - Unit-level widget snapshots using `ratatui::backend::TestBackend`.
    - Example: `tests/tui_prompt_widget.rs` snapshots borders/prompt layout.
  - System-level (planned next): run Bevy systems with a `TestBackend` and assert frame contents / invariants.
  - End-to-end PTY test (ignored by default) for real terminal behavior.
    - Example: `tests/e2e_pretty_pty.rs` (uses `expectrl`).

- **Renderer adapters**
  - Created adapters in `tests/renderer_adapters.rs` for `PromptPlugin::pretty()` and `PromptPlugin::simple()` to standardize testing across current and future renderers.

- **Snapshot testing**
  - Use `insta` for golden snapshots.
  - First run records snapshots; subsequent runs compare for regressions.
  - Command examples:
    - Record/accept: `INSTA_UPDATE=auto cargo test --features test --test tui_prompt_widget`
    - Review: `cargo insta review`

- **Dependency and feature choices**
  - `insta` and `expectrl` added as dev-dependencies in `Cargo.toml`.
  - Avoided adding `vt100` for now due to `unicode-width` version conflicts with `ratatui 0.29`; can revisit under a feature or pin versions if needed.

- **Gating and access**
  - Integration tests must enable the feature to see `bevy_repl::test_support`:
    - `cargo test --features test`
  - Unit tests in `src/` see it via `cfg(test)` automatically.

- **Stubbing missing examples**
  - Added minimal placeholders for missing examples so `cargo test` does not fail compiling examples:
    - `examples/log_custom_layer.rs`
    - `examples/log_pretty.rs`

## Rationale
- Separating harness (“test_support”) from tests keeps helpers reusable and reduces duplication.
- Layered tests balance speed (unit/snapshot) with fidelity (PTY E2E).
- Feature gating prevents test-only code from leaking into normal builds.

## Next Steps
- Add system-level snapshot tests that run full renderer systems against `TestBackend`.
- Add fixtures (prepared states) under `src/test_support/fixtures.rs` for seeded REPL buffers and log events.
- Optional: Reintroduce ANSI parsing (`vt100`) behind a feature once version constraints are resolved.
- Document test running in `CONTRIBUTING.md` and CI configuration.

## Files Added/Modified
- `src/test_support/mod.rs`
- `src/lib.rs` (gated `test_support` module)
- `tests/tui_prompt_widget.rs`
- `tests/renderer_adapters.rs`
- `tests/e2e_pretty_pty.rs` (ignored by default)
- `examples/log_custom_layer.rs` (placeholder)
- `examples/log_pretty.rs` (placeholder)
- `Cargo.toml` (features and dev-dependencies)

## Commands
- Run all tests with test support: `cargo test --features test`
- Accept snapshots on first run: `INSTA_UPDATE=auto cargo test --features test --test tui_prompt_widget`
- Review snapshots: `cargo insta review`
