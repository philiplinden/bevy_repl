//! Example showing how to add the REPL to the app but disable it.
//! All logs and messages should appear as if the REPL were not present.

use bevy::prelude::*;
use bevy_repl::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ReplPlugins.set(ReplPlugin::disabled()),
        ))
        .run();
}