# Design

BevyREPL is read-execute-print-loop designed to be an interactive console for
the Bevy app at runtime. It provides a command line interface (CLI) to trigger
commands and inspect the app state without a window or renderer. The REPL leans
on common CLI patterns and conventions, including all features supported by
[`clap`](https://docs.rs/clap), for a familiar and intuitive user experience.

This chapter documents the design of the REPL and its components. It serves as
here as a reference for myself and for anyone who wants to understand how the
REPL works.

<!-- toc -->
