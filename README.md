# bevy_repl
Add an interactive REPL to a Bevy app.

Heavily inspired by [makspll/bevy-console](https://github.com/makspll/bevy-console).

## Usage

Add `ReplPlugin` and optionally the resource `ReplConfig` to customize the REPL.

```rust
use bevy::prelude::*;
use bevy_repl::{ReplConfig, ReplPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ReplPlugin))
        .insert_resource(ReplConfig {
            // override config here
            ..Default::default()
        });
}
```

## License

Except where noted (below and/or in individual files), all code in this
repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This
dual-licensing approach is the de-facto standard in the Rust ecosystem and there
are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to
include both.
