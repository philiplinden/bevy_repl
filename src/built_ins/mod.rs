use bevy::prelude::*;

#[cfg(feature = "quit")]
mod quit;

#[cfg(feature="clear")]
mod clear;

#[cfg(feature="help")]
mod help;

pub struct ReplDefaultCommandsPlugin;

impl Plugin for ReplDefaultCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(feature = "quit")]
            quit::plugin,
            #[cfg(feature = "clear")]
            clear::plugin,
            #[cfg(feature = "help")]
            help::plugin,
        ));
    }
}