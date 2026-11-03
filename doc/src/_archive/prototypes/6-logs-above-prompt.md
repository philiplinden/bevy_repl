# Prototype 6: Bevy's unmodified logs land above the prompt

_Archived 2026-11-02 from `prototypes/6-logs-above-prompt.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether the pinned prompt works with Bevy's own
  logger, untouched.
- **Claim:** With `LogPlugin::default()` left alone, Bevy's log lines (stderr)
  scroll above the prompt exactly like `println!` does in 5.
- **Baseline:** `LogPlugin` writes to stderr at the cursor and knows nothing
  about a prompt (`bevy_log-0.20.1/src/lib.rs:367`).
- **Vision:** 3, 7.
- **Proof:** Same as 5 with `info!` instead of `println!`. `RUST_LOG` still
  filters the output.
- **File:** `proto_6_logs_above_prompt.rs`
- **Result:**
