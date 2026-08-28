# Tracing Subscriber Integration

Type: grilling
Status: resolved

## Question

How should Bevy's default logging infrastructure (`bevy_log` / `tracing`) be routed so that log events format cleanly and print into the DECSTBM scroll region without clobbering the active prompt line?

Specifically:
1. How should a custom `tracing_subscriber::Layer` be implemented or attached to Bevy's `LogPlugin`?
2. How does the logger coordinate cursor positioning (e.g. moving to the last scrollable line before writing) to prevent cursor race conditions with the prompt rendering system?

## Answer

### 1. Direct Tracing Layer via `LogPlugin.custom_layer`
Provide `repl_tracing_layer(app: &mut App) -> Option<BoxedLayer>` configured with `ReplMakeWriter` and `ReplWriter`:

```rust
pub fn repl_tracing_layer(_app: &mut App) -> Option<BoxedLayer> {
    let layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_writer(ReplMakeWriter);

    Some(Box::new(layer))
}
```

User setup in Bevy:
```rust
App::new()
    .add_plugins((
        DefaultPlugins.set(bevy::log::LogPlugin {
            custom_layer: bevy_repl::log_ecs::repl_tracing_layer,
            ..default()
        }),
        ReplPlugins,
    ))
```

### 3. Automatic Tracing Fallback for Headless / Minimal Apps
If `bevy::log::LogPlugin` is **not** loaded in the app (e.g. when using `MinimalPlugins` or `App::new()`), `ReplPlugin::build` automatically attempts to register the REPL tracing layer on the global subscriber:

```rust
if !app.is_plugin_added::<bevy::log::LogPlugin>() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_ansi(true).with_writer(ReplMakeWriter))
        .try_init();
}
```

For applications using `DefaultPlugins`, a convenience extension `DefaultPlugins.with_repl_log()` configures `LogPlugin.custom_layer` with zero closure boilerplate:

```rust
App::new()
    .add_plugins(DefaultPlugins.with_repl_log())
    .add_plugins(ReplPlugins)
```
