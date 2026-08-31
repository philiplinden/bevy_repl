# Research: `bevy_log`/`tracing` Internals and Terminal Scroll-Region Prior Art

*Date: 2026-08-30*
*Pinned versions (via `Cargo.lock`): `bevy_log 0.19.1`, `bevy_app 0.19.1`, `tracing 0.1.44`, `tracing-core 0.1.36`, `tracing-subscriber 0.3.23`, `crossterm 0.28.1`*
*Context: `doc/adr/0002-direct-crossterm-over-ratatui.md`, `src/lib.rs::adapt_for_repl()`, `src/print.rs`, `src/repl.rs`, `src/tracing.rs`*

---

## Executive Summary

This note answers two independent research questions raised while deciding how `bevy_repl` should integrate with Bevy's logging stack and how it manages the terminal viewport. It makes no design recommendation; it only records what the pinned dependency source code and external prior art actually do.

Headline finding for Section 1: reading `bevy_log-0.19.1/src/lib.rs` directly overturns nothing in the prior handoff doc's central claim — `fmt_layer` **is** substitutive (it fully replaces Bevy's default stderr formatter; the default is only constructed via `unwrap_or_else` when `fmt_layer` returns `None`), while `custom_layer` **is** purely additive (its output always coexists with whichever `fmt_layer` — default or user-supplied — gets composed afterward). This asymmetry is the direct, textual explanation for the "every log line prints twice" symptom recorded from approach 1.

Headline finding for Section 2: `crossterm` has no DECSTBM-specific API; `bevy_repl`'s raw escape-sequence approach in `src/print.rs` is the only way to do this through `crossterm`. DECSTBM scroll-region reliability is a live, currently-open compatibility problem on Windows Terminal/ConPTY specifically (multiple open upstream issues, including one that already bit Claude Code's own `/statusline` feature). Separately, `crossterm::event::Event::Resize` already exists and is already being read (and silently discarded) by `bevy_repl`'s own input-polling loop, so polling `terminal::size()` on every write is not strictly the only mechanism available, even though it is currently the one in use.

---

## Section 1 — `bevy_log` / `tracing` internals

### 1.1 `LogPlugin` struct fields

Source: `bevy_log-0.19.1/src/lib.rs:218-250`.

```rust
pub struct LogPlugin {
    /// Filters logs using the [`EnvFilter`] format
    pub filter: String,

    /// Filters out logs that are "less than" the given level.
    /// This can be further filtered using the `filter` setting.
    pub level: Level,

    /// Optionally add an extra [`Layer`] to the tracing subscriber
    ///
    /// This function is only called once, when the plugin is built.
    ///
    /// Because [`BoxedLayer`] takes a `dyn Layer`, `Vec<Layer>` is also an acceptable return value.
    ///
    /// Access to [`App`] is also provided to allow for communication between the
    /// [`Subscriber`](tracing::Subscriber) and the [`App`].
    ///
    /// Please see the `examples/app/log_layers.rs` for a complete example.
    pub custom_layer: fn(app: &mut App) -> Option<BoxedLayer>,

    /// Override the default [`tracing_subscriber::fmt::Layer`] with a custom one.
    ///
    /// This differs from [`custom_layer`](Self::custom_layer) in that
    /// [`fmt_layer`](Self::fmt_layer) allows you to overwrite the default formatter layer, while
    /// `custom_layer` only allows you to add additional layers (which are unable to modify the
    /// default formatter).
    ///
    /// For example, you can use [`tracing_subscriber::fmt::Layer::without_time`] to remove the
    /// timestamp from the log output.
    ///
    /// Please see the `examples/app/log_layers.rs` for a complete example.
    pub fmt_layer: fn(app: &mut App) -> Option<BoxedFmtLayer>,
}
```

Both `custom_layer` and `fmt_layer` exist simultaneously, and both fields are `pub`. There is **no `#[derive(...)]` attribute at all** above the struct (`bevy_log-0.19.1/src/lib.rs:218`, immediately after the doc-comment block that starts at line 73) — `LogPlugin` does not derive `Clone`, `Debug`, or anything else. It only gets a hand-written `Default` impl at `bevy_log-0.19.1/src/lib.rs:284-293`:

```rust
impl Default for LogPlugin {
    fn default() -> Self {
        Self {
            filter: DEFAULT_FILTER.to_string(),
            level: Level::INFO,
            custom_layer: |_| None,
            fmt_layer: |_| None,
        }
    }
}
```

Types: `custom_layer: fn(app: &mut App) -> Option<BoxedLayer>` where `BoxedLayer = Box<dyn Layer<Registry> + Send + Sync + 'static>` (`bevy_log-0.19.1/src/lib.rs:253`); `fmt_layer: fn(app: &mut App) -> Option<BoxedFmtLayer>` where `BoxedFmtLayer = Box<dyn Layer<PreFmtSubscriber> + Send + Sync + 'static>` (`bevy_log-0.19.1/src/lib.rs:267`). `PreFmtSubscriber` is itself a feature-gated type alias (`bevy_log-0.19.1/src/lib.rs:256-264`) that differs depending on whether the `trace` feature is active (it includes `tracing_error::ErrorLayer` in that case). This means the exact type a `fmt_layer` closure must produce is coupled to `bevy_log`'s internal, feature-dependent type, not a stable standalone type.

### 1.2 `LogPlugin::build()` walkthrough — layer order and filter scope

Source: `bevy_log-0.19.1/src/lib.rs:295-403`, condensed to the ordering that matters:

```
subscriber = Registry::default()                                    // line 310
subscriber = subscriber.with((self.custom_layer)(app))               // line 313
subscriber = subscriber.with(self.build_filter_layer())              // line 315  <- EnvFilter
[ trace feature ] subscriber = subscriber.with(ErrorLayer::default()) // line 318
fmt_layer = (self.fmt_layer)(app).unwrap_or_else(|| default fmt::Layer to stderr) // 349-354
subscriber = subscriber.with(fmt_layer)                               // line 364
[ chrome / tracy / android layers appended after this ]               // 366-371
finished_subscriber = subscriber
tracing::subscriber::set_global_default(finished_subscriber)          // line 389
```

The `EnvFilter` (built from `self.level` + `self.filter` in `build_filter_layer`, `bevy_log-0.19.1/src/lib.rs:406-434`) is added directly via `.with()`, **not** via `.with_filter()`. `tracing-subscriber`'s own module docs describe exactly this distinction (`tracing-subscriber-0.3.23/src/layer/mod.rs:397-416`):

> "A `Layer` that implements a filtering strategy should override the `register_callsite` and/or `enabled` methods... Note that the `Layer::register_callsite` and `Layer::enabled` methods determine whether a span or event is enabled *globally*... The filtering methods on a stack of `Layer`s are evaluated in a top-down order, starting with the outermost `Layer` and ending with the wrapped `Subscriber`. If any layer returns `false` from its `enabled` method... filter evaluation will short-circuit and the span or event will be disabled."

This is confirmed mechanically in the composition impl itself, `tracing-subscriber-0.3.23/src/layer/layered.rs:266-274` (`Layer` impl for `Layered<A, B, S>`):

```rust
fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool {
    if self.layer.enabled(metadata, ctx.clone()) {
        // if the outer subscriber enables the callsite metadata, ask the inner layer.
        self.inner.enabled(metadata, ctx)
    } else {
        // otherwise, the callsite is disabled by this layer
        false
    }
}
```

and the top-level `Subscriber` impl at `tracing-subscriber-0.3.23/src/layer/layered.rs:105-120` (same AND-chain pattern). Because `fmt::Layer` and a typical `custom_layer` closure do not override `enabled` (default returns `true`), the *only* layer in bevy_log's stack that actually returns `false` for anything is the `EnvFilter`. Since the whole chain is an AND across every layer regardless of nesting position, and the *entire* per-callsite gate (`Subscriber::enabled`, called once by `tracing-core` before an event is even constructed) must pass before `on_event` is invoked on *any* layer, the `EnvFilter` added at line 315 gates **the whole registry** — both `custom_layer` (added at line 313, structurally "inside" the filter) and `fmt_layer`/chrome/tracy (added afterward, structurally "outside" it). Position only changes which layer's `enabled()` runs first for short-circuiting, not which layers are ultimately subject to the filter.

Practical corollary: a `custom_layer` does respect the user's configured `filter`/`level`. It is not able to bypass it. (The `tracing-tracy` layer additionally applies its own extra `.with_filter()` on top at `bevy_log-0.19.1/src/lib.rs:358-362`, which is a genuine per-layer filter narrowing tracy's own view further — an example of the *other* filtering mode described in the same tracing-subscriber docs at lines 440-529.)

### 1.3 `fmt_layer` is substitutive; `custom_layer` is additive

Source: `bevy_log-0.19.1/src/lib.rs:313` vs. `:349-364`.

`custom_layer` (line 313): `let subscriber = subscriber.with((self.custom_layer)(app));` — this always runs, and whatever it returns (`Option<BoxedLayer>`) is *added on top of* the registry. Nothing else is skipped as a result.

`fmt_layer` (lines 349-364):
```rust
let fmt_layer = (self.fmt_layer)(app).unwrap_or_else(|| {
    // note: the implementation of `Default` reads from the env var NO_COLOR
    // to decide whether to use ANSI color codes, which is common convention
    // https://no-color.org/
    Box::new(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr))
});
...
let subscriber = subscriber.with(fmt_layer);
```
Exactly one `fmt_layer` value is ever composed into the subscriber: the user's closure result if `Some`, or Bevy's own default `fmt::Layer` to stderr if `None`. There is no code path in which both the user's `fmt_layer` and Bevy's default stderr formatter are installed simultaneously — supplying `Some(...)` fully replaces, not supplements, the default. This directly confirms the prior handoff doc's "fmt_layer is substitutive" claim, verified against `bevy_log 0.19.1` specifically (not an older Bevy version).

This asymmetry explains the double-print symptom from approach 1: a `custom_layer` that itself constructs a `fmt::Layer` writing to stderr (or to the REPL) always coexists with the separately-composed default `fmt_layer` (also writing to stderr by default), so the same event is formatted and written twice by two independent layers, both of which pass the same `EnvFilter` gate from §1.2.

### 1.4 Can `fmt_layer` swap only the writer while keeping default formatting?

The `fmt_layer` closure must return a fully-built `BoxedFmtLayer` (`Box<dyn Layer<PreFmtSubscriber> + Send + Sync>`) — there is no API surface on `LogPlugin` for "keep Bevy's formatting, just change the writer." However, because Bevy's own fallback at line 353 is literally `tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr)`, a caller can reproduce the same effect by writing that same line themselves with a different writer, e.g. `Box::new(tracing_subscriber::fmt::Layer::default().with_writer(MyMakeWriter))`. This inherits `fmt::Layer::default()`'s target names, ANSI/`NO_COLOR` handling, and timestamps, because it's the same constructor Bevy uses — the caller is just not handed a narrower "override-writer-only" method; they must reconstruct the whole `fmt::Layer` value themselves (one line, but full responsibility). `bevy_repl`'s own `src/tracing.rs::make_repl_layer()` already does this pattern (`fmt::layer().with_ansi(true).with_writer(ReplMakeWriter)`), so this technique is already validated as compiling against `bevy_log`'s current `custom_layer` type; whether it satisfies `fmt_layer`'s stricter `Layer<PreFmtSubscriber>` bound was not independently exercised in this research (see gaps, below).

### 1.5 Panic hook handling in `bevy_log` and `bevy_app`

`bevy_log-0.19.1/src/lib.rs:295-307`:
```rust
fn build(&self, app: &mut App) {
    #[cfg(feature = "trace")]
    {
        let old_handler = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |infos| {
            eprintln!("{}", tracing_error::SpanTrace::capture());
            old_handler(infos);
        }));
    }
    ...
```
This block is gated by `bevy_log`'s own `trace` feature (declared at `bevy_log-0.19.1/Cargo.toml`, `trace = ["tracing-error"]`). `bevy_repl`'s own `Cargo.toml` depends on `bevy` with `default-features = false, features = ["keyboard", "bevy_log"]` — the top-level `bevy`/`bevy_internal` `trace` feature (`bevy-0.19.1/Cargo.toml:2867`, `bevy_internal-0.19.1/Cargo.toml:448`) is a separate, non-enabled feature from `bevy_log`'s facade feature `bevy_log = ["bevy_internal/bevy_log"]` (`bevy-0.19.1/Cargo.toml:2688`). So **in `bevy_repl`'s actual current dependency configuration, `bevy_log`'s panic-hook block does not compile in and `LogPlugin::build()` installs no panic hook at all**, unless a downstream app opts into Bevy's `trace` feature independently.

Separately, `bevy_app-0.19.1/src/panic_handler.rs:37-63` defines `PanicHandlerPlugin` (part of `DefaultPlugins`), which on non-wasm, non-`error_panic_hook`-feature targets does nothing (`_ => ()` at line 58) — its doc comment says outright: "Other platforms are currently not setup" (line 15).

`bevy_repl`'s own hook, `src/repl.rs:62-67`:
```rust
fn install_safety_hooks() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = Repl::restore_terminal();
        default_hook(panic_info);
    }));
    ...
```
follows the standard "take-then-wrap" idiom, same as `bevy_log`'s own hook installation. Because `ReplPlugin::build` (`src/repl.rs:33-35`) calls `install_safety_hooks()` and `bevy_log`'s hook block (when it does compile, i.e. `trace` feature on) also takes-then-wraps, the two compose without clobbering each other regardless of registration order — whichever plugin's `build()` runs second wraps whichever ran first, and both eventually delegate to the previous hook. No conflict was found in the source; the two mechanisms are structurally compatible by construction, not accidentally.

### 1.6 `tracing_core::dispatcher::set_global_default` idempotency

Source: `tracing-core-0.1.36/src/dispatcher.rs:299-332`:
```rust
pub fn set_global_default(dispatcher: Dispatch) -> Result<(), SetGlobalDefaultError> {
    if GLOBAL_INIT.compare_exchange(UNINITIALIZED, INITIALIZING, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        ...
        Ok(())
    } else {
        Err(SetGlobalDefaultError { _no_construct: () })
    }
}
```
Confirmed via source: this is an atomic `compare_exchange` guard. A second call after a first success returns `Err(SetGlobalDefaultError)`, not a panic — `SetGlobalDefaultError` is a plain struct implementing `Debug`/`Display`/`std::error::Error` (`tracing-core-0.1.36/src/dispatcher.rs:345-365`), and `bevy_log` itself relies on this non-panicking contract: `bevy_log-0.19.1/src/lib.rs:387-401` calls `set_global_default` and, on `Err`, just logs via `error!(...)` rather than aborting.

Note a discrepancy worth flagging: `LogPlugin`'s own doc comment (`bevy_log-0.19.1/src/lib.rs:199-203`) states "This plugin should not be added multiple times in the same process... rerunning the same initialization multiple times will lead to a panic," but the actual `build()` body it documents (lines 387-401) explicitly matches on both failure combinations and only logs an `error!()` — it does not panic in either failure branch that was read. The doc comment appears stale/inaccurate relative to the current implementation, at least for the "both already set" and "one already set" cases traced through this exact source file.

### 1.7 Reading a `LogPlugin`'s configured values before disabling it

`LogPlugin`'s four fields (`filter: String`, `level: Level`, `custom_layer: fn(...)`, `fmt_layer: fn(...)`) are all `pub` (`bevy_log-0.19.1/src/lib.rs:220-249`), and `Level` derives `Copy, Clone, Debug, PartialEq, Eq, Hash` (`tracing-core-0.1.36/src/metadata.rs:220-221`). `fn` pointers are `Copy`, and `String` is `Clone`. So *if code holds a reference or owned value of type `LogPlugin`*, it can read or manually reconstruct every field via a struct literal (`LogPlugin { filter: x.filter.clone(), level: x.level, custom_layer: x.custom_layer, fmt_layer: x.fmt_layer }`) without an actual `#[derive(Clone)]` being present, since `LogPlugin` derives nothing (§1.1).

However, there is no built-in mechanism to pull those field values automatically out of a `PluginGroupBuilder` before calling `.disable::<LogPlugin>()`. `PluginGroupBuilder`'s public API (`bevy_app-0.19.1/src/plugin_group.rs`) exposes `contains::<T>()`, `enabled::<T>()`, `set`, `add`, `add_before/after`, `enable`, `disable`, `finish` — no getter that returns the already-staged plugin *value* for inspection. The only reader that exists, `App::get_added_plugins<T>() -> Vec<&T>` (`bevy_app-0.19.1/src/app.rs:590-614`, doc: "This can be used to read the settings of any existing plugins"), only returns plugins that have already been **added and built** into the `App` — i.e., by the time you could call it on `LogPlugin`, `LogPlugin::build()` has already run and already claimed the global `tracing` dispatcher slot (§1.6), so there is no "read the settings, then still cleanly disable and replace" path through this particular API. Extracting the user's filter/level ahead of disabling `LogPlugin` would require the caller to hand over the constructed `LogPlugin` value directly (e.g. as a function argument) rather than pulling it back out of `DefaultPlugins`/`PluginGroupBuilder` reflectively.

### Implications for bevy_repl (Section 1)

- Because `EnvFilter` gates the entire layer stack regardless of `.with()` order (§1.2), a `custom_layer` supplied to `LogPlugin` does respect the user's `filter`/`level` configuration; it cannot see events the filter has excluded, and it cannot bypass the filter to see more.
- Because `fmt_layer` fully replaces the default stderr formatter while `custom_layer` only adds alongside whatever `fmt_layer` resolves to (default or custom), any design that installs REPL output via `custom_layer` while `fmt_layer` is left at its default will duplicate every line (default formatter to stderr, plus the custom layer's own output) — this matches the originally observed double-print symptom exactly.
- A `fmt_layer` closure can reproduce Bevy's default formatting (target names, ANSI/`NO_COLOR`, timestamps) while redirecting output, by constructing `tracing_subscriber::fmt::Layer::default().with_writer(...)` itself — but `LogPlugin` gives no narrower "just swap the writer" entry point; the whole `fmt::Layer` must be built by the caller, and its type is coupled to `bevy_log`'s internal, feature-flag-dependent `PreFmtSubscriber` alias.
- In `bevy_repl`'s actual current Cargo feature configuration (`bevy` with `default-features = false, features = ["keyboard", "bevy_log"]`), `bevy_log`'s `trace` feature is not enabled, so `bevy_log::LogPlugin::build()` installs no panic hook at all in this repo as configured; `bevy_repl`'s own panic-hook installation in `ReplPlugin::build` is therefore the only hook actually active from these two crates today, and if `bevy_log`'s hook were ever activated (by a downstream app enabling Bevy's `trace` feature), the two would compose via the standard take-then-wrap pattern without clobbering each other regardless of plugin registration order.
- A second `tracing::subscriber::set_global_default` call after the first succeeded returns a non-panicking `Err`; `bevy_log` itself relies on and handles this by logging an error rather than aborting. Any design where both `LogPlugin` and `bevy_repl` might call `set_global_default`/`try_init` needs to account for whichever one runs first "winning" silently (from `tracing-core`'s perspective) unless the losing caller checks the `Result`.
- `LogPlugin`'s fields are `pub` and individually cloneable/copyable even though the struct itself derives nothing, so preserving a user's filter/level when replacing `LogPlugin` is only possible if the code has direct access to the actual `LogPlugin` value the user constructed — there is no API to retrieve that value back out of a `PluginGroupBuilder` before disabling it, and retrieving it after it has built (`App::get_added_plugins`) is already too late to avoid a redundant/losing `set_global_default` call.

**Open gap**: this research did not compile-check whether `make_repl_layer()`'s return type (currently used as a `custom_layer`-compatible function returning `impl Layer<S> where S: Subscriber + LookupSpan` in `src/tracing.rs:48-53`) actually satisfies the stricter `BoxedFmtLayer = Box<dyn Layer<PreFmtSubscriber> + Send + Sync>` bound required by the `fmt_layer` field signature. The type is plausible but was not verified by an actual build against `bevy_log-0.19.1`.

---

## Section 2 — Terminal drawing / scroll-region prior art

### 2.1 Does crossterm provide a DECSTBM helper?

No. Grepping `crossterm-0.28.1/src/terminal.rs` for scroll-related commands turns up only `ScrollUp(pub u16)` (`crossterm-0.28.1/src/terminal.rs:289-309`) and `ScrollDown(pub u16)` (`:311-329`) — these scroll the *entire visible screen* content by N rows (via `sys::scroll_up`/`sys::scroll_down`, which on Windows call `scroll_up`/`scroll_down` in `crossterm-0.28.1/src/terminal/sys/windows.rs:102-131`), not a DECSTBM top/bottom-margin region. There is no `SetScrollRegion`, `SetMargins`, or any DECSTBM-named type anywhere in `crossterm-0.28.1/src/`. `bevy_repl`'s approach in `src/print.rs:35-46` — hand-writing `\x1B[1;{}r` / `\x1B[r` via `write!` directly to `stdout()` — is confirmed to be the only way to get DECSTBM behavior through this crate; crossterm has no wrapper for it.

### 2.2 DECSTBM reliability across terminals

The escape sequence itself is well-specified and old (VT100-era): `CSI Ps ; Ps r`, default top=1/bottom=screen-height, resets to full-screen if top≥bottom or either parameter is 0 (https://ghostty.org/docs/vt/csi/decstbm). It is broadly implemented, but Windows Terminal/ConPTY specifically has multiple **currently open** (as of this research) upstream bugs in exactly this area:

- microsoft/terminal#19016, "Input on line at end of scrolling region (set with DECSTBM) causes cursor to move down outside the scroll region" — open, reported June 2025, labeled Priority-3, explicitly said to be "blocking LLDB's status line feature on Windows." (https://github.com/microsoft/terminal/issues/19016)
- microsoft/terminal#1849, "Incorrect interpretation of DECSTBM parameters," and PR #1881 attempting a fix for cases where top==bottom or bottom exceeds screen height. (https://github.com/microsoft/terminal/issues/1849, https://github.com/microsoft/terminal/pull/1881)
- microsoft/terminal#3673, "Scrolling within regions that include the top of the screen doesn't push lines into scrollback." (https://github.com/microsoft/terminal/issues/3673)
- microsoft/terminal#7019, "conpty exhibits pathological performance on scrolling region redraw (repaints entire screen)." (https://github.com/microsoft/terminal/issues/7019)
- anthropics/claude-code#14716, "Windows Terminal rendering broken after /statusline - scroll margins not reset" — a real-world case of exactly this class of bug hitting another CLI tool's pinned-bottom-line feature. Root cause as described in the issue: Claude Code's `/statusline` sets `ESC[<top>;<bottom>r` to reserve the bottom line; if the margins are not explicitly reset with `ESC[r` when the feature is disabled or the process errors, ConPTY (which persists ANSI state, unlike legacy `conhost`/CMD) leaves the terminal permanently constrained, and subsequent cursor-manipulation sequences then corrupt visible output. A partial PowerShell-side workaround (`[Console]::Write([char]27 + "[r")`) is documented in the issue but described as incomplete. (https://github.com/anthropics/claude-code/issues/14716)

No equivalent open compatibility bugs specifically about DECSTBM were surfaced in this research for native Linux/macOS terminal emulators (xterm-descended terminals implement it as core VT100 behavior). The concrete, documented gap found is Windows Terminal/ConPTY, not "terminals in general."

### 2.3 crossterm resize events vs. polling `terminal::size()`

`crossterm::event::Event` (`crossterm-0.28.1/src/event.rs:547-563`) has a dedicated variant:
```rust
pub enum Event {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Mouse(MouseEvent),
    #[cfg(feature = "bracketed-paste")]
    Paste(String),
    /// An resize event with new dimensions after resize (columns, rows).
    /// **Note** that resize events can occur in batches.
    Resize(u16, u16),
}
```
On Unix, this is driven by an actual `SIGWINCH` handler crossterm installs internally (`crossterm-0.28.1/src/event/source/unix/tty.rs:72`, `pipe::register(libc::SIGWINCH, sender)?`, and `crossterm-0.28.1/src/event/source/unix/mio.rs:48`, `Signals::new([signal_hook::consts::SIGWINCH])?`). So polling `terminal::size()` is **not** the only mechanism available through crossterm — an actual resize *event* exists and is delivered through the same `event::read()`/`event::poll()` API bevy_repl already uses for keyboard input.

Grepping the actual `bevy_repl` source (not an estimate) for `terminal::size()`/`crossterm::terminal::size()` call sites finds **8 total**, not concentrated only in `src/print.rs`:
- `src/print.rs:58`, `:82`, `:106` (3 sites)
- `src/repl.rs:78`, `:136`, `:146`, `:161`, `:168` (5 sites)

Separately, `bevy_repl`'s own input-polling loop in `src/input.rs:38-39` already calls `event::read()` inside a `while event::poll(...)` loop but only matches on the result:
```rust
while event::poll(Duration::ZERO).unwrap_or(false) {
    if let Ok(Event::Key(key_event)) = event::read() {
```
Any `Event::Resize(..)` (or `FocusGained`/`FocusLost`/`Mouse`) delivered through this same `event::read()` call is silently dropped by the non-matching `if let`, since `event::read()` still consumes the event from crossterm's internal queue even when the pattern doesn't match. So resize notifications are already flowing through code `bevy_repl` runs every frame; they are read and discarded rather than being unavailable.

### 2.4 Prior art: how other tools pin a line while output scrolls above it

**ratatui** (the library `bevy_repl` migrated away from, per `doc/adr/0002-direct-crossterm-over-ratatui.md`): uses an alternate screen buffer with full-frame immediate-mode redraw, not a scroll region. Per ratatui's own docs (https://ratatui.rs/concepts/rendering/under-the-hood/): `Terminal::draw(|frame| ...)` wipes an in-memory `Buffer` before each call, the application re-renders every widget from scratch into that buffer each tick, and ratatui internally keeps a double buffer, diffing the new frame against the previous one so only changed cells are actually written to the terminal. The pinned-vs-scrolling illusion in a ratatui app is produced by the application re-rendering the whole layout (log pane + input line) every frame inside a full-screen alternate-screen buffer, not by a native terminal scroll-region primitive. This matches the ADR's stated reason for leaving ratatui (unneeded layout/widget machinery for a REPL that only needs a sticky prompt line), rather than any claim that ratatui uses DECSTBM.

**indicatif** (`console-rs/indicatif`): the redraw routine `draw_to_term` (`src/draw_target.rs` on the `main` branch, fetched 2026-08-30: https://raw.githubusercontent.com/console-rs/indicatif/main/src/draw_target.rs) uses cursor-movement and line-clearing primitives — `term.move_cursor_up(n)`, `term.write_str("\r")`, `term.clear_line()`, `term.move_cursor_down(1)` — to erase and redraw the previously-drawn bar lines in place each tick. `ProgressBar::println()` (https://docs.rs/indicatif/latest/indicatif/struct.ProgressBar.html) prints a new log line "above the progress bar" by using this same clear/redraw cycle: move up to the bar's position, print the new line plus a fresh newline, then redraw the bar below it. No alternate screen buffer and no DECSTBM scroll region are used.

**reedline** (`nushell/reedline`), external-printer feature: `print_external_message` (`src/painting/painter.rs` on the `main` branch, fetched 2026-08-30: https://raw.githubusercontent.com/nushell/reedline/main/src/painting/painter.rs) similarly uses cursor movement plus ordinary newline-driven scrolling — it issues `MoveUp(buffer_num_lines - 1)`, then queues `Print(line)` followed by `Print("\r\n")` for each external message, relying on the terminal's own newline-triggered scrollback to push prior content up, and re-queries the actual cursor position afterward (via a flush + cursor-position query) to re-anchor the prompt, explicitly because — per a comment in that source — queued-but-unflushed writes mean "a row counted forward from `starting_row` names a position the terminal has not reached" yet. No alternate screen buffer and no DECSTBM scroll region are used here either.

Across all three surveyed tools, **none** use a native DECSTBM scroll region the way `bevy_repl` does. ratatui uses full alternate-screen immediate-mode redraw with double-buffer diffing; indicatif and reedline both use cursor-save/move + clear-line + newline-driven natural scrolling, redrawing the pinned content in place after new lines are printed above it.

### Implications for bevy_repl (Section 2)

- `crossterm` has no built-in DECSTBM/scroll-region API at the pinned version (0.28.1); `bevy_repl`'s hand-rolled `\x1B[1;{}r`/`\x1B[r` writes in `src/print.rs` are not working around an existing crossterm feature — they are filling a genuine gap in crossterm's surface.
- DECSTBM support has open, currently-unfixed correctness and performance bugs specifically on Windows Terminal/ConPTY (cursor escaping the scroll region on the last line, scroll-margin state not resetting between sessions/features, full-screen repaint performance pathology). A comparable CLI tool (Claude Code's `/statusline`) has already hit the "margins left set after disable/failure" variant of this class of bug in production. No equivalent open compatibility issues were found for native Linux/macOS xterm-descended terminals in this research pass.
- `crossterm::event::Event::Resize(u16, u16)` exists, is generated via a real `SIGWINCH` handler on Unix, and is delivered through the same `event::poll`/`event::read` calls `bevy_repl` already uses every frame in `src/input.rs`; the current `if let Ok(Event::Key(..))` pattern there consumes and discards any `Resize` event that arrives during that same poll loop, rather than being structurally unable to see it. Separately, there are 8 direct `terminal::size()` call sites across `src/print.rs` (3) and `src/repl.rs` (5), not only in `src/print.rs`.
- None of the three surveyed external prior-art tools (ratatui, indicatif, reedline) use a DECSTBM scroll region for the "pinned line, scrolling output above it" pattern; they use either full alternate-screen redraw with diffing (ratatui) or cursor-move/clear-line plus natural newline-driven scrolling (indicatif, reedline). `bevy_repl`'s scroll-region approach is architecturally distinct from all three, for better or worse on any given terminal.

---

## Unresolved / not independently verified

- Whether `make_repl_layer()`'s current generic return type in `src/tracing.rs:48-53` actually satisfies the `BoxedFmtLayer` bound (`Layer<PreFmtSubscriber>`) required by `LogPlugin::fmt_layer`, as opposed to only the looser `Layer<Registry>` bound required by `custom_layer`, was not checked with an actual compile — this would need to be built against `bevy_log 0.19.1` directly to confirm.
- DECSTBM behavior on legacy `conhost` (pre-Windows-Terminal, non-ConPTY) was not independently found in this pass; the Claude Code issue notes CMD ("legacy console") "often resets these automatically," which is suggestive but not a primary-sourced specification of conhost's DECSTBM handling.
- indicatif's and reedline's source was read from each project's `main` branch on GitHub at research time (2026-08-30), not from a version pinned anywhere in `bevy_repl`'s own dependency tree (neither crate is a dependency of `bevy_repl`), so exact line numbers may drift from what a future reader sees at those URLs.
