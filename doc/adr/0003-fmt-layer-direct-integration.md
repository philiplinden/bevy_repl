# Route REPL Log Output via `LogPlugin.fmt_layer`, Not a Disable-and-Replace Wrapper

**Status:** Proposed — not yet implemented. This document is for review; no source, examples, or existing docs have been changed.

We stop disabling Bevy's `LogPlugin` and standing up a second, independent `tracing_subscriber` registry. Instead, `bevy_repl` exports a single free function shaped to slot directly into `LogPlugin.fmt_layer`, and the app author assigns it themselves in a plain struct literal — matching Bevy's own official pattern for this exact problem.

## Context

The current wiring is disconnected and was flagged by the app author as not fully understood, which is why it was built this way rather than fixed at the time.

Concretely, as of this proposal:

- `ReplDefaultPluginsExt::adapt_for_repl()` (`src/lib.rs:71-84`) calls `.disable::<bevy::log::LogPlugin>()`. This means `LogPlugin::build()` never runs, so its `EnvFilter` (built from the user's `filter`/`level`) is never constructed.
- `ReplPlugin::build()` (`src/repl.rs:37-40`) detects `LogPlugin` is absent and installs its own, separate `tracing_subscriber::registry().with(make_repl_layer()).try_init()`. This stack has **no `EnvFilter` layer at all**, so log level filtering and `RUST_LOG` do nothing on this path.
- `repl_tracing_layer()` (`src/tracing.rs:56`), exported from the prelude and documented as "ready to attach to Bevy's `LogPlugin`," is shaped to return `Option<BoxedLayer>` — the type `LogPlugin.custom_layer` expects. It is never actually assigned to any `LogPlugin` anywhere in this repo (`grep -rn "repl_tracing_layer" --include="*.rs" .` finds only its definition and its prelude re-export). It is dead code.
- There is no path by which `ReplPlugin::build()`'s fallback registry could honor a user's configured `filter`/`level` even if it wanted to: once `LogPlugin` is disabled via `PluginGroupBuilder::disable::<T>()`, there is no API to read the field values of the `LogPlugin` value the app author originally constructed. `App::get_added_plugins::<T>()` only returns plugins that have already built — by which point `LogPlugin::build()`'s own `set_global_default` call has already run (or, since it's disabled, never runs at all). This was confirmed against `bevy_app-0.19.1`'s public `PluginGroupBuilder` API and is recorded in `doc/notes/2026-08-30-logging-sink-research.md` §1.7.

Root cause, restated: `custom_layer` and `fmt_layer` are not symmetric. `fmt_layer` fully replaces Bevy's default stderr formatter (exactly one is ever composed into the subscriber); `custom_layer` only adds alongside whatever `fmt_layer` resolves to. `make_repl_layer()` — `fmt::layer().with_ansi(true).with_writer(ReplMakeWriter)` — does the job of a `fmt_layer`: it formats and redirects where the human-readable line goes. It is not doing the job `custom_layer` exists for (an additional side-channel sink alongside normal output). The current code disables `LogPlugin` entirely instead of handing it a `fmt_layer`, which is why none of `LogPlugin`'s own machinery — including its filter — is available to the REPL's output today.

### Why not a convenience wrapper instead

Before landing on the proposal below, a `LogPlugin`-targeted extension trait was considered as a middle ground:

```rust
pub trait ReplLogPluginExt { fn adapt_for_repl(self) -> Self; }
impl ReplLogPluginExt for LogPlugin {
    fn adapt_for_repl(self) -> Self { LogPlugin { fmt_layer: repl_fmt_layer, ..self } }
}
```

This was checked against two primary sources and rejected:

- Bevy's own official examples for configuring `LogPlugin.custom_layer`/`fmt_layer` (`examples/app/log_layers.rs` and `examples/app/log_layers_ecs.rs`, shipped inside the published `bevy` crate at the pinned `0.19.1`) use a plain struct literal at the call site in every case. No wrapper trait, no builder-extension method exists anywhere in Bevy's own documentation of this feature.
- `bevy_ratatui` — the closest real-world analog, a crate whose entire purpose is embedding Bevy inside a raw-mode/alternate-screen terminal, the same problem class as `bevy_repl` — has **zero** references to `LogPlugin`, `fmt_layer`, `custom_layer`, or `tracing` anywhere in its source, examples, or README. It does not attempt to solve this problem at all, so there is no ecosystem convention to match here either, beyond what Bevy's own docs already teach.

Given neither Bevy itself nor the nearest sibling crate in the ecosystem wraps this in any abstraction, adding one here would be inventing a convention rather than following one.

## Decision

1. Export a free function matching `LogPlugin.fmt_layer`'s exact signature:

   ```rust
   pub fn repl_fmt_layer(_app: &mut App) -> Option<BoxedFmtLayer> {
       Some(Box::new(
           tracing_subscriber::fmt::Layer::default()
               .with_ansi(true)
               .with_writer(ReplMakeWriter),
       ))
   }
   ```

   This replaces `repl_tracing_layer()`'s role. `make_repl_layer()`'s existing construction is reused; only the wrapping function's return type and the field it targets change.

2. Document — and expect app authors to write — the plain, explicit construction, matching Bevy's own example style exactly:

   ```rust
   App::new().add_plugins(
       DefaultPlugins.set(LogPlugin {
           fmt_layer: bevy_repl::repl_fmt_layer,
           ..default()
       }),
   )
   ```

   An author who wants a non-default filter still writes it directly in the same literal (`LogPlugin { filter: "wgpu=warn".into(), fmt_layer: bevy_repl::repl_fmt_layer, ..default() }`), and it is honored automatically: `fmt_layer` is composed into the same subscriber stack as the `EnvFilter` `LogPlugin::build()` constructs from that literal, so REPL output is gated by it for free (per `tracing-subscriber`'s shared-`enabled()` gate across the whole `Layer` stack, confirmed in `doc/notes/2026-08-30-logging-sink-research.md` §1.2).

3. Remove `ReplDefaultPluginsExt` and `.adapt_for_repl()` (`src/lib.rs:69-84`) entirely. Disabling `LogPlugin` is no longer part of this crate's integration story.

4. In `ReplPlugin::build()` (`src/repl.rs:37-40`), narrow the fallback rather than delete it outright: keep installing a bare registry only for app authors who genuinely never add `LogPlugin` at all (e.g. `bevy` without the `bevy_log` feature, or a hand-assembled plugin set). In that narrower case there is no user `filter`/`level` to preserve — there is no `LogPlugin` value in existence to read one from — so this is not a config-loss regression, but the fallback should still chain a default `EnvFilter` (mirroring `bevy_log`'s own `DEFAULT_FILTER`/`Level::INFO` default) instead of installing an entirely unfiltered layer as it does today. Document this path explicitly as the no-`LogPlugin` degraded mode, not the primary integration story.

5. Delete `repl_tracing_layer()` and its `custom_layer`-shaped export. It is unused dead code today, and its only defined purpose (`custom_layer`: an additional side-channel sink running alongside normal formatted output — e.g. mirroring raw events to a file, or into the ECS as `examples/app/log_layers_ecs.rs` demonstrates via `mpsc` + `NonSend` resource) is not a use case this crate currently implements or has a scoped feature for. Reintroduce it, shaped to the actual requirement, if and when such a feature is built — `log_layers_ecs.rs` is the reference pattern to start from.

## Consequences

- App authors' `filter`/`level` configuration is honored by REPL output automatically, with no code in `bevy_repl` needing to read or reconstruct it.
- `ReplPlugin::build()` no longer competes with `LogPlugin` to install the global `tracing` dispatcher; there is exactly one subscriber-construction path in the primary (has-`LogPlugin`) case.
- The public API surface shrinks: `ReplDefaultPluginsExt`, `.adapt_for_repl()` on `DefaultPlugins`/`PluginGroupBuilder`, and `repl_tracing_layer()` are all removed. `repl_fmt_layer()` is the one new/renamed export doing this job.
- This is a breaking API change for any existing caller of `.adapt_for_repl()` on `DefaultPlugins`/`PluginGroupBuilder`. Per the app author, example-code compatibility is not a constraint on this decision.
- `doc/src/design/logging.md` becomes fully stale under this proposal (it currently documents the pre-migration `bevy_ratatui` + `mpsc`-channel approach and calls the DECSTBM path "experimental"). Rewriting it, and updating the examples that currently call `.adapt_for_repl()`, is follow-up work once this proposal is accepted — intentionally not done as part of this document.
- The open technical gap flagged in `doc/notes/2026-08-30-logging-sink-research.md` §1.4/§204 — whether a `fmt::Layer`-returning closure actually satisfies `fmt_layer`'s stricter `Layer<PreFmtSubscriber>` bound, as opposed to the looser `Layer<Registry>` bound `custom_layer` accepts — is exercised directly by implementing step 1 above, since `bevy`'s own `log_layers.rs` example uses the identical `tracing_subscriber::fmt::Layer::default().with_writer(...)` construction against the same field. It should compile; this proposal does not treat it as a remaining risk, but it is the first thing to verify when implementing.
