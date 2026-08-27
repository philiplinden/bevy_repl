# Tracing Subscriber Integration

Type: grilling
Status: open
Blocked by: 03

## Question

How should Bevy's default logging infrastructure (`bevy_log` / `tracing`) be routed so that log events format cleanly and print into the DECSTBM scroll region without clobbering the active prompt line?

Specifically:
1. How should a custom `tracing_subscriber::Layer` be implemented or attached to Bevy's `LogPlugin`?
2. How does the logger coordinate cursor positioning (e.g. moving to the last scrollable line before writing) to prevent cursor race conditions with the prompt rendering system?
