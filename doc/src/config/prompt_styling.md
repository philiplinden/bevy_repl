# Prompt styling

The REPL uses `bevy_ratatui` for rendering the prompt and input buffer. The
prompt renderer is configured via `ReplPromptConfig`. The default renderer is a
simple 1-line bottom prompt with a symbol and input buffer.

For now, we only support a "partial-TUI" approach where the REPL and terminal
outputs are rendered to the main terminal screen. Ratatui alternate screens are 
available if you add `bevy_ratatui::RatatuiPlugins` to your app before
`ReplPlugins`. Support for Ratatui alternate screens is experimental.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_ratatui::RatatuiPlugins::default(),
            bevy_repl::ReplPlugins,
        ))
        .run();
}
```

**Example**:
[alt_screen.rs](https://github.com/philiplinden/bevy_repl/blob/main/examples/alt_screen.rs)

The REPL prompt supports basic configuration via the `ReplPromptConfig` resource.

You can configure the prompt symbol:

```rust
app.insert_resource(ReplPromptConfig { symbol: Some("> ".to_string()) });
```

More advanced prompt styling is not yet implemented for the default prompt
renderer. It is possible to do advanced TUI styling with a custom renderer,
though. See [examples/custom_renderer.rs](examples/custom_renderer.rs).
