# Derive Macro Ergonomics and Complexity

Type: grilling
Status: resolved

## Question

How should `bevy_repl_derive` be structured so that `#[derive(ReplCommand)]` provides an intuitive, zero-boilerplate experience without excessive macro complexity or fragile proc-macro dependencies?

Specifically:
1. How does `#[derive(ReplCommand)]` integrate with `clap::Parser`, `clap::CommandFactory`, and `clap::FromArgMatches`?
2. What attribute helpers (e.g. `#[command(...)]`, aliases, help text) should be supported out of the box?
3. How should compile-time error reporting and missing trait bounds be communicated cleanly to the developer?
4. How do we keep `bevy_repl_derive` optional (feature-gated behind `features = ["derive"]`) so users wanting a lean compile time can implement `ReplCommand` manually?

## Answer

### 1. Dual Macro Ergonomics: Attribute & Derive
To provide maximum ergonomic flexibility without adding proc-macro bloat:

- **All-in-One Attribute Macro (`#[repl_command]`)**:
  Automatically injects `#[derive(clap::Parser, bevy::prelude::Event, Clone)]` and implements `ReplCommand`, eliminating boilerplate entirely:

  ```rust
  #[repl_command(name = "spawn", about = "Spawn an entity")]
  pub struct SpawnCommand {
      pub name: String,
      pub count: u32,
  }
  ```

- **Additive Derive Macro (`#[derive(ReplCommand)]`)**:
  Available for developers who prefer explicit trait lists or customized derive combinations.

### 2. Lightweight Delegation to Clap
The macro delegates parsing and AST inspection to Clap's standard traits:
- `clap::CommandFactory::command()` generates the `clap::Command` specification.
- `clap::FromArgMatches::from_arg_matches(matches)` hydrates the typed struct.
- All standard `clap` attributes (`#[arg(...)]`, `#[command(...)]`, aliases, defaults) work out of the box with zero custom proc-macro attribute parsing.

### 3. No Artificial `Default` Trait Bound
Because `ReplCommand` requires `to_event` (settled in Ticket 05), commands with required, non-default fields do not need `#[derive(Default)]` or dummy fallbacks.

### 4. Feature Gating
`bevy_repl_derive` remains an optional sub-crate under the `derive` feature flag in `Cargo.toml`, with `clap/derive` enabled alongside it.
