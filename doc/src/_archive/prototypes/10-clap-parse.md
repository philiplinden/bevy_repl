# Prototype 10: clap can parse a typed line without exiting

_Archived 2026-11-02 from `prototypes/10-clap-parse.md`. Part of the first prototype plan, superseded when the design was restarted. See the [2026-11-02 devlog](../../devlog/03-crossterm-rewrite/2026-11-02.md)._

- **Objective:** Find out whether clap can be used as a parser for lines,
  rather than for a program's arguments.
- **Claim:** A line split with `shell-words` and parsed with
  `try_get_matches_from` returns matches or an error. Parse errors and `--help`
  come back as text to print, and the process never exits.
- **Baseline:** clap's `get_matches` prints and exits the process on a parse
  error or `--help`.
- **Vision:** 1.
- **Proof:** A plain loop over hard-coded strings (no Bevy, no terminal): a
  valid command, an unknown argument, `--help`, a quoted argument.
- **File:** `proto_10_clap_parse.rs`
- **Result:**
