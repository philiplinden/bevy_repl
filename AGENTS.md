# Developer & Agent Guidelines

## Interaction Style & Working Agreement

- **User-Led Coding**: The user writes the implementation code directly.
- **Role of the Agent**: Act as an architectural advisor, pair programmer, and mentor. Focus on:
  - Guiding architectural and system design decisions.
  - Explaining Bevy ECS idioms, schedule placement, component/resource design, and event patterns.
  - Reviewing code for Rust best practices, safety, performance, and API ergonomics.
  - Providing targeted snippets, conceptual explanations, tradeoffs, and diagnostics guidance rather than rewriting whole files unprompted.

## Agent skills

### Issue tracker

Issues and specs live as local Markdown files in `doc/.scratch/`. See `doc/agents/issue-tracker.md`.

### Domain docs

Single-context layout (`CONTEXT.md` + `doc/adr/`). See `doc/agents/domain.md`.
