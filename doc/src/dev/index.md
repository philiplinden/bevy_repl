# Development

This chapter contains developer notes, to-dos, known issues, and other
information for those who want to contribute to the crate.

## TUI Mode

TUI (terminal user interface) mode is the original approach to the REPL,
where the prompt and logs are captured and redirected away from the builtin
log handler to the TUI renderer.

The TUI adds large dependencies and can break the normal logging behavior.
On the other hand, it is togglable at runtime and supports custom keybinds.

### Feature Wishlist

- [x] **Derive pattern** (_Added in v0.3.0_) - Describe commands with clap's
  derive pattern.
- [x] **Support for games with rendering and windowing** (_Added in v0.3.0_) -
  The REPL is designed to work from the terminal, but the terminal normally
  prints logs when there is a window too. The REPL still works from the terminal
  while using the window for rendering if the console is enabled.
- [x] **Printing to stdout** (_Added in v0.4.0_) - The REPL should print to
  stdout instead of the TUI screen unless the user explicitly enables a TUI
  context that uses the alternate screen.
- [x] **Toggleable** (_Added in v0.4.1_) - The REPL is disabled by default and
  can be toggled. When disabled, the app runs normally in the terminal, no REPL
  systems run, and the prompt is hidden.
- [ ] **Scrollable TUI output** - The terminal output on the TUI screen should
  scroll to show past messages like a normal terminal screen printing to stdout.
- [ ] **Support for games with TUIs** - The REPL is designed to work as a sort
  of sidecar to the normal terminal output, so _in theory_ it should be
  compatible with games that use an alternate TUI screen. I don't know if it
  actually works, probably only with the minimal renderer or perhaps a custom
  renderer.
- [x] **Customizable keybinds** (_Added in v0.4.1_) - Allow the user to
  configure the REPL keybinds for all REPL controls, not just the toggle key.
- [ ] **Command history** - Use keybindings to navigate past commands and insert
  them in the prompt buffer.
- [ ] **Help text and command completion** - Use `clap`'s help text and
  completion features to provide a better REPL experience and allow for command
  discovery.
- [ ] **Customizable prompt** - Allow the user to configure the REPL prompt for
  all REPL controls, not just the toggle key.

## Non-TUI Mode

Starting in v0.5.0, the REPL can be used in non-TUI mode where the prompt
and logs are printed to stdout together with the builtin log handler. This
mode is lightweight and intended to be a minimal layer on top of the Bevy app.

Here, _lightweight_ means that the REPL adds minimal dependencies and is purely
additive with no disruptions to the normal logging behavior.

## Building BevyREPL

This page lists the bash commands you can run locally to reproduce each of the
CI workflows defined in `.github/workflows`.

### Running tests

The `test.yml` workflow (named "CI") runs on every pull request and every push
to `main`. The release workflow also runs it before publishing. Each job maps
to one command you can run locally:

```bash
# Format
cargo fmt --all --check

# Clippy (also checks that every example compiles)
cargo clippy --workspace --all-targets --all-features --features bevy/bevy_window -- -D warnings

# Test (unit tests and doctests)
cargo test --workspace --all-features --features bevy/bevy_window

# Docs
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features

# Package (builds both crates from the files crates.io would receive)
cargo package --workspace
```

`--all-features` covers examples gated behind this crate's own features (like
the `derive` example's `required-features = ["derive"]`), but the `window`
example requires `bevy/bevy_window`, which is a feature of the `bevy`
dependency rather than of this crate, so it must be passed explicitly.

> [!TIP]
> In CI, the compiling jobs cache `~/.cargo` and `target/` with
> [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache) to speed up
> repeated runs. Locally, cargo gets the same benefit for free as long as you
> don't delete the `target/` directory between runs.

### Building the docs

BevyREPL's user documentation is compiled using [mdbook](https://rust-lang.github.io/mdBook/index.html).
The `mdbook.yml` workflow (named "Book") builds this book on every pull request
and push to `main`. It only deploys to GitHub Pages when a release is
published, so the live book always matches the latest release.

First, install `mdbook` and the plugins configured in `book.toml` (versions
pinned to match the workflow):

```bash
cargo install mdbook --version 0.5.4
cargo install mdbook-toc --version 0.15.4
```

Then, from the repository root, build the book:

```bash
mdbook build
```

The rendered book is written to `doc/book/` (per `build-dir` in `book.toml`).
While writing docs, it's often more convenient to serve the book with live
reload instead:

```bash
mdbook serve --open
```

To build the crate documentation for docs.rs, run the following command:

```bash
cargo doc --no-deps --open
```

### Updating the changelog

[git-cliff](https://git-cliff.org/) generates `CHANGELOG.md` from
[Conventional Commits](https://www.conventionalcommits.org/). On pull requests,
the `changelog.yml` workflow posts the unreleased changes as a comment. The
release workflow regenerates `CHANGELOG.md` and commits it to `main`, so you
don't need to commit it by hand.

To preview it locally, install `git-cliff`:

```bash
cargo install git-cliff
```

Then, from the repository root, print the unreleased changes (what the PR
comment shows):

```bash
git cliff --unreleased --strip header --github-repo philiplinden/bevy_repl
```

Or regenerate the full `CHANGELOG.md`:

```bash
git cliff --github-repo philiplinden/bevy_repl -o CHANGELOG.md
```

> [!NOTE]
> The `--github-repo` flag fetches contributor and PR metadata from the
> GitHub API. Set a `GITHUB_TOKEN` environment variable first to avoid
> rate limiting:
>
> ```bash
> export GITHUB_TOKEN=<your-personal-access-token>
> ```

### Releasing

Pushing a tag like `v1.0.0` starts the `release.yml` workflow. It:

1. runs the full CI workflow;
2. checks that the tag matches the `bevy_repl` version in `Cargo.toml`;
3. publishes `bevy_repl_derive`, then `bevy_repl`, to crates.io, skipping any
   crate whose version is already published;
4. creates a GitHub Release with that version's changelog;
5. regenerates `CHANGELOG.md` and commits it to `main`;
6. deploys this book to GitHub Pages.

To cut a release:

1. Bump `version` in `Cargo.toml`. If `bevy_repl_derive` changed, bump its
   version too, along with the `bevy_repl_derive` dependency version in the
   root `Cargo.toml`.
2. Update the version table in `README.md`. While 1.0 is unreleased, the
   README's install line points at the git repository; switch it to the
   crates.io version once that version is published.
3. Merge to `main`, then tag that commit and push the tag:

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

Tags must start with `v`. git-cliff and the release workflow both ignore tags
without it.

#### One-time setup

The workflow needs these settings before the first release:

- **crates.io trusted publishing.** On crates.io, open the settings for both
  `bevy_repl` and `bevy_repl_derive` and add a trusted publisher for GitHub
  repository `philiplinden/bevy_repl` with workflow `release.yml`. No API
  token is stored in GitHub.
- **GitHub Pages deployments from tags.** In the repository settings, under
  Environments → `github-pages`, add a deployment rule allowing tags matching
  `v*`. Otherwise the book deploy is rejected.
- **Pushing to `main`.** The changelog commit pushes straight to `main`. If a
  ruleset that requires pull requests is enabled on `main`, add GitHub Actions
  to its bypass list.
