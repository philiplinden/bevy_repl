# Development

This chapter contains developer notes, to-dos, known issues, and other
information for those who want to contribute to the crate.

## Status

`main` is groundwork for 1.0: a rewrite on `crossterm` that drops `ratatui`.
The target behavior is described in [Vision for BevyREPL](../design/index.md),
and the day-to-day reasoning is in the [Devlog](../devlog/index.md). The code
doesn't match the docs everywhere yet; [Known Issues](known_issues.md) lists
the gaps.

Not planned for now: toggling the REPL at runtime, command history, tab
completion, a custom prompt, and configurable keybinds.

### Test on both Windows and Unix

Terminals behave differently on Windows and on Unix (Linux and macOS), so a
fix on one can break the other. When you report or fix a terminal bug, say
which OS and terminal you used, and check the fix on both before calling it
done. See [Known Issues](known_issues.md#windows-and-unix-behave-differently)
for the differences we know about.

## Building BevyREPL

This section lists the bash commands you can run locally to reproduce each of the
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

Between releases, `main` carries the next version with a `-dev` suffix (for
example `1.0.0-dev`), the same convention Bevy uses. Nothing is ever published
under a `-dev` version.

To cut a release:

1. Drop the `-dev` suffix from `version` in `Cargo.toml` (for example
   `1.0.0-dev` → `1.0.0`). If `bevy_repl_derive` changed, bump its version
   too, along with the `bevy_repl_derive` dependency version in the root
   `Cargo.toml`.
2. Update the version table in `README.md`. While 1.0 is unreleased, the
   README's install line points at the git repository; switch it to the
   crates.io version once that version is published.
3. Merge to `main`, then tag that commit and push the tag:

   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

4. Set `main` to the next development version, for example `1.1.0-dev`.

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
