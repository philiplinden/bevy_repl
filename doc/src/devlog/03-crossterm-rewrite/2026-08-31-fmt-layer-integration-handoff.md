# Handoff: Implement ADR 0003 (`fmt_layer` direct integration)

*Date: 2026-08-31*
*For: an agent picking up implementation after design/validation planning, without the full prior conversation*
*Read first:* `doc/adr/0003-fmt-layer-direct-integration.md` (the accepted decision this handoff executes)
*Background:* `doc/notes/2026-08-30-logging-sink-research.md` (bevy_log/tracing internals research this ADR is built on)

## What this is

ADR 0003 is **accepted and written**, but **not yet implemented**. This doc is the execution plan: a sequence of small, independently-verifiable steps, each scoped to one skill/workflow, so the ADR's one unverified technical assumption gets tested before any refactor happens, and every step leaves the repo in a compiling, working state.

Do not skip straight to deleting code. Follow the step order below — step 1 exists specifically to de-risk everything after it.

## The decision, in one paragraph

`bevy_repl` currently disables `bevy::log::LogPlugin` entirely (`ReplDefaultPluginsExt::adapt_for_repl()`, `src/lib.rs:71-84`) and stands up its own, separate, **unfiltered** `tracing_subscriber::registry()` as a fallback (`ReplPlugin::build()`, `src/repl.rs:37-40`). This means a user's `RUST_LOG`/`filter`/`level` config is silently ignored for REPL output today. ADR 0003 fixes this by exporting a plain function, `repl_fmt_layer`, shaped to slot directly into `LogPlugin.fmt_layer` (not `custom_layer` — the two are asymmetric; see ADR §"Why not a convenience wrapper instead" and the research note §1.2-1.3 for why). The app author assigns it themselves in a normal struct literal, matching Bevy's own `examples/app/log_layers.rs` convention — no wrapper trait, no builder extension method. `LogPlugin` is never disabled. `repl_tracing_layer()` (the current, unused, `custom_layer`-shaped export) and `ReplDefaultPluginsExt`/`.adapt_for_repl()` are deleted.

## Current file state (as of this handoff — verify against `git log` before trusting line numbers)

- `src/tracing.rs` — has `ReplWriter`, `ReplMakeWriter`, `make_repl_layer()`, and `repl_tracing_layer()` (to be deleted, see step 3). `make_repl_layer()`'s construction (`fmt::layer().with_ansi(true).with_writer(ReplMakeWriter)`) is reused as-is inside the new `repl_fmt_layer()`.
- `src/lib.rs:71-84` — `ReplDefaultPluginsExt` / `.adapt_for_repl()`, to be deleted (step 3).
- `src/repl.rs:37-40` — `ReplPlugin::build()`'s fallback branch that stands up a bare, unfiltered registry when `LogPlugin` is absent. This is *narrowed*, not deleted (step 4) — it should still exist for apps that never add `LogPlugin` at all, but it should chain a default `EnvFilter` instead of installing an unfiltered layer.
- Every example under `examples/` currently calls `.adapt_for_repl()` (confirmed via scout: `examples/log.rs:65`, `examples/custom_log_layer.rs:59`, `examples/default.rs:99`, `examples/demo.rs:571`, etc.) — all need updating in step 5.
- `doc/src/design/logging.md` — already stale (documents a pre-ratatui-removal mpsc-channel design). ADR 0003 explicitly defers rewriting it; just don't let it get more wrong.

## Workflow (do these in order; each step should leave `cargo build`/`cargo test` green before moving on)

### Step 1 — Spike: compile-check the one unverified assumption, in isolation

**Skill:** `brainstorming` (spike path) — throwaway code, 2-3 sentence probe, report finding, no permanent edits to `src/`.

The ADR's own "Consequences" section flags this explicitly as the first thing to verify: does `tracing_subscriber::fmt::Layer::default().with_writer(ReplMakeWriter)` actually satisfy the `Layer<PreFmtSubscriber>` bound required by `LogPlugin.fmt_layer` (`BoxedFmtLayer = Box<dyn Layer<PreFmtSubscriber> + Send + Sync>`), as opposed to the looser `Layer<Registry>` bound `custom_layer` accepts? `PreFmtSubscriber` is an internal, feature-flag-dependent `bevy_log` type alias — this has never been independently compiled against in this repo (see research note §1.4/"Open gap", and ADR §Consequences last bullet).

Write a standalone throwaway example (mirror the shape of Bevy's own `examples/app/log_layers.rs`, shipped in the pinned `bevy` crate at `0.19.1`) that:

1. Constructs a real `LogPlugin { fmt_layer: repl_fmt_layer, ..default() }` (a temporary copy of the function shown in ADR §Decision item 1, doesn't need to live in `src/tracing.rs` yet).
2. Builds a minimal `App`, emits one `tracing::info!(...)`.
3. Confirms it compiles and the line reaches the writer.

If this doesn't compile, stop and report back — the ADR's central bet is wrong and needs revisiting before anything else happens. If it compiles, this de-risks every step after it and confirms the exact construction to use in step 2/3.

### Step 2 — TDD: prove the filter is actually honored end-to-end

**Skill:** `test-driven-development` — write the failing/absent assertion first.

This is the actual payoff ADR 0003 promises: today, `RUST_LOG`/`filter`/`level` does nothing for REPL output (confirmed: the fallback registry in `src/repl.rs:37-40` has no `EnvFilter` layer at all). Write a test that:

1. Constructs `LogPlugin { filter: "warn".into(), level: Level::INFO, fmt_layer: repl_fmt_layer, ..default() }` for real (not disabled).
2. Emits both an `info!` and a `warn!` through it.
3. Asserts only the `warn!` line reaches `ReplWriter`/the REPL's line sink.

Get this test written and failing (or passing against the step-1 throwaway) before touching `src/tracing.rs` for real. This is the regression guard that protects the whole rationale of the ADR going forward — without it, nothing stops a future change from silently reintroducing the disconnected-filter bug.

### Step 3 — Safe-refactor: delete the confirmed-dead/disconnected code

**Skill:** `safe-refactor` — verify before, verify after, pure removal.

Once steps 1-2 are green:

- Add `repl_fmt_layer(_app: &mut App) -> Option<BoxedFmtLayer>` to `src/tracing.rs` (the exact construction from ADR §Decision item 1, now proven by step 1).
- Delete `repl_tracing_layer()` from `src/tracing.rs` and its prelude re-export (ADR §Decision item 5 — confirmed unused via `grep -rn "repl_tracing_layer" --include="*.rs" .`, only its own definition and re-export match).
- Delete `ReplDefaultPluginsExt` and `.adapt_for_repl()` from `src/lib.rs:71-84` (ADR §Decision item 3).

Nothing here should be a behavior change beyond removing dead code and adding the new function — confirm via `cargo build` that nothing outside `examples/` still references the deleted items before moving to step 5 (examples are handled deliberately in step 5, not incidentally here).

### Step 4 — Surgical-patch: narrow (don't delete) the no-`LogPlugin` fallback

**Skill:** `surgical-patch` — narrowest responsible layer, regression-provable.

`src/repl.rs:37-40`'s fallback branch should still exist, but only for apps that genuinely never add `LogPlugin` at all (e.g. `bevy` built without the `bevy_log` feature). In that case there is no user `filter`/`level` to preserve — there's no `LogPlugin` value in existence to read one from, so this isn't a config-loss regression. But today this fallback installs a layer with **no `EnvFilter` at all**, which is worse than necessary. Change it to chain a default `EnvFilter` (mirror `bevy_log`'s own `DEFAULT_FILTER`/`Level::INFO` default, per ADR §Decision item 4) instead of an unfiltered layer. Document this explicitly in a comment as the degraded/no-`LogPlugin` mode, not the primary integration path.

### Step 5 — Migration: update every example call site

**Skill:** `migration` — one call site at a time, compile-checked, reversible.

Every example currently calling `.adapt_for_repl()` needs to move to the plain struct-literal pattern from ADR §Decision item 2:

```rust
App::new().add_plugins(
    DefaultPlugins.set(LogPlugin {
        fmt_layer: bevy_repl::repl_fmt_layer,
        ..default()
    }),
)
```

This is a breaking API change — per the ADR, example-code compatibility was explicitly *not* a constraint on the decision, so don't hesitate on that account, but do update every call site so the crate's own examples still build and demonstrate the new pattern correctly. Known call sites as of this handoff (re-verify with `grep -rn "adapt_for_repl" examples/` since this list may drift): `examples/log.rs`, `examples/custom_log_layer.rs`, `examples/default.rs`, `examples/demo.rs`.

### Step 6 — Close the loop

**Skill:** `verification-before-completion`

Before calling this done:

- Re-run every example (`cargo run --example <name>` for each touched one) and confirm they build and run.
- Run `lens_diagnostics` (mode=all) on every changed file.
- Confirm `doc/src/design/logging.md`'s staleness is explicitly flagged as follow-up debt in a commit/PR note or a new `doc/.scratch/` ticket — the ADR defers rewriting it, but that deferral should be visible, not silent.
- Update `CHANGELOG.md` per repo convention (check `cliff.toml`/existing entries for format).

## Explicitly out of scope for this handoff

Two adjacent, real risks were surfaced during the research/brainstorming that led to ADR 0003, but ADR 0003 does **not** address them — do not fold them into this implementation pass:

1. **Write-path reentrancy/deadlock risk.** `ReplWriter::write()` currently calls `crate::print::repl_print()` synchronously, inline, on whatever thread/system the `tracing::info!`/etc. macro fired from — this is structurally the same shape as a documented real deadlock class (`tokio-rs/tracing#3269`: a `Drop`-triggered log call deadlocking during another internal tracing operation). Best-practice mitigation is the `tracing-appender`-style channel-decoupling pattern (buffer to newline, push complete lines to a channel, drain on a separate stack/schedule — never do sink work on the tracing call's own stack). Not part of this ADR; worth its own ADR next.
2. **DECSTBM cursor-restore/scroll-region synchronization gap.** The most recent commit before this ADR (`d729608`, message "idek") removed the post-log-write cursor-restore-to-prompt-row step in `repl_print` and changed `render_prompt` to run only `.run_if(resource_changed::<Repl>)` instead of every frame. There is currently no coordination primitive between tracing-triggered `repl_print` calls (fire from arbitrary systems, any schedule) and the scheduled `render_prompt`/`manage_scroll_region` — correctness rests on "whichever draws last this frame wins the cursor." Separately, Windows Terminal/ConPTY has an open, unprioritized DECSTBM bug (`microsoft/terminal#19016`) whose maintainer explicitly recommends *not* mixing OS-level line input with VT scroll margins — `bevy_repl`'s own crossterm-raw-mode line editor is already aligned with that guidance, but the performance pathology under heavy scroll-region output (`microsoft/terminal#7019`, closed *not planned* — an accepted, permanent limitation, not a pending fix) is unaddressed. Not part of this ADR; flagged for a follow-up ADR once this one is validated and landed.

## Acceptance criteria for this handoff (done when)

- [ ] Step 1 spike confirms (or refutes, with a report) the `fmt_layer` trait-bound compile
- [ ] Step 2 test exists, passes, and specifically proves filter-honoring behavior that did not exist before
- [ ] `repl_tracing_layer`, `ReplDefaultPluginsExt`, `.adapt_for_repl()` are deleted; `repl_fmt_layer` exists and is exported
- [ ] No-`LogPlugin` fallback still exists but is filtered (default `EnvFilter`), not unfiltered
- [ ] All examples build and run using the new struct-literal pattern
- [ ] `lens_diagnostics` clean on changed files; CHANGELOG updated
- [ ] The two out-of-scope risks above are recorded somewhere durable (a new `doc/.scratch/` ticket or ADR stub) rather than dropped
