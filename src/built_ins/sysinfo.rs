use bevy::prelude::*;
use crate::{ReplCommand, ReplResult, ReplCommandRegistry};
use clap::{Command, ArgMatches};

/// System info command - show system information
#[derive(Default, Clone)]
pub struct SysInfoCommand;

impl ReplCommand for SysInfoCommand {
    fn command(&self) -> Command {
        Command::new("sysinfo")
            .about("Show system information")
    }

    fn execute_with_world(&self, world: &World, _commands: &mut Commands, _matches: &ArgMatches) -> ReplResult<String> {
        let mut output = String::new();

        output.push_str(&format!("Bevy Version: {}\n", env!("CARGO_PKG_VERSION")));
        output.push_str(&format!("Rust Version: {}\n", env!("RUST_VERSION")));
        output.push_str(&format!("Entity Count: {}\n", world.entities().len()));
        output.push_str(&format!("Component Count: {}\n", world.components().len()));
        output.push_str(&format!("Resource Count: {}\n", world.resources().len()));
        // Get diagnostics store
        if let Some(diagnostics) = world.get_resource::<bevy::diagnostic::DiagnosticsStore>() {
            // Frame time
            if let Some(frame_time) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frame_time.smoothed() {
                    output.push_str(&format!("Frame Time: {:.2}ms\n", value));
                }
            }

            // FPS
            if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    output.push_str(&format!("FPS: {:.0}\n", value));
                }
            }

            // System info
            if let Some(system_info) = diagnostics.get(bevy::diagnostic::SystemInformationDiagnosticsPlugin::CPU_USAGE) {
                if let Some(value) = system_info.smoothed() {
                    output.push_str(&format!("CPU Usage: {:.1}%\n", value));
                }
            }

            // Memory stats
            if let Some(memory) = diagnostics.get(bevy::diagnostic::SystemInformationDiagnosticsPlugin::MEM_SYSTEM_USED) {
                if let Some(value) = memory.smoothed() {
                    output.push_str(&format!("Memory Used: {:.1} MB\n", value / (1024.0 * 1024.0)));
                }
            }
        } else {
            output.push_str("\nNote: Diagnostics not available. Enable `diagnostics` feature to see performance metrics.\n");
        }

        Ok(output)
    }

    fn needs_world_access(&self) -> bool {
        true
    }
}
