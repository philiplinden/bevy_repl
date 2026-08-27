# In-Schedule Terminal Event Loop

We run terminal event polling and rendering directly inside Bevy's normal `Update` schedule sets (`ReplSet`), rather than running a separate background runner thread or sidecar event loop.

## Context

Terminal integrations often spawn a background thread or custom runner (like `bevy_crossterm`) to continuously read blocking stdin/terminal events and pass them across channels to the game engine. However, coordinating state and schedules across multiple event loops introduces synchronization overhead, potential race conditions with Bevy's ECS schedules, and complicates plugin integration.

## Decision

Execute non-blocking terminal polling and prompt updates synchronously within Bevy's standard system sets (`ReplSet::Capture`, `ReplSet::Buffer`, `ReplSet::Render`, `ReplSet::Post`).

## Consequences

- Terminal input handling and command dispatch are tightly coupled with Bevy's frame rate and schedule execution.
- No cross-thread locking, channel management, or external runner lifecycle coordination is required.
- Terminal polling must remain non-blocking (zero timeout) so it never stalls the Bevy main loop when no user input is pending.
