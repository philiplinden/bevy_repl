use expectrl::{spawn, Regex};

// End-to-end PTY test: spawns the pretty example in a pseudo-terminal, drives a
// few keystrokes, and verifies expected visible tokens. This validates
// alternate-screen + general output flow at a high level.
//
// Marked ignored by default because it's slower and more environment-sensitive.
#[test]
#[ignore]
fn pretty_prompt_basic_interaction() -> anyhow::Result<()> {
    // Spawn the example under a PTY. Use `--quiet` for faster compiles locally if desired.
    let mut p = spawn("cargo run --example pretty")?;

    // Wait for welcome banner to ensure app initialized and alternate screen is set.
    p.expect(Regex("Welcome to the Bevy REPL"))?;

    // Trigger a parse error log by sending an empty line (press Enter).
    p.send_line("")?;

    // Read until we see the prompt marker again. This should ensure the prompt remains visible.
    let out = p.read_until(Regex("> "), Some(std::time::Duration::from_millis(800)))?;

    // Basic assertions on visible tokens.
    assert!(out.contains("> "), "prompt marker should be visible");
    // Pretty renderer should display border characters.
    assert!(out.contains("┌") || out.contains("+"), "top border should be present in pretty mode");

    // Clean exit
    let _ = p.send_control('c');
    Ok(())
}
