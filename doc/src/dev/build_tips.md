# Building BevyREPL

This page lists the bash commands you can run locally to reproduce each of the
CI workflows defined in `.github/workflows`.

## Running tests

The `test.yml` workflow has two jobs that run on every push and pull request:
the unit tests, and a check that all examples still compile.

Reproduce the unit tests locally with:

```bash
cargo test --lib
```

Reproduce the example check locally with:

```bash
cargo check --examples --all-features --features bevy/bevy_window
```

`--all-features` covers examples gated behind this crate's own features (like
the `derive` example's `required-features = ["derive"]`), but the `window`
example requires `bevy/bevy_window`, which is a feature of the `bevy`
dependency rather than of this crate, so it must be passed explicitly.

> [!TIP]
> In CI, both jobs cache `~/.cargo` and `target/` with
> [`Swatinem/rust-cache`](https://github.com/Swatinem/rust-cache) to speed up
> repeated runs. Locally, cargo gets the same benefit for free as long as you
> don't delete the `target/` directory between runs.

## Building the docs

BevyREPL's user documentation is compiled using [mdbook](https://rust-lang.github.io/mdBook/index.html).
The `mdbook.yml` workflow builds this book and deploys it to GitHub Pages.

First, install `mdbook` and the plugins configured in `book.toml` (versions
pinned to match the workflow):

```bash
cargo install mdbook --version 0.4.52
cargo install mdbook-toc --version 0.14.2
cargo install mdbook-alerts
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

## Updating the changelog

The `changelog.yml` workflow uses [git-cliff](https://git-cliff.org/) to
generate `CHANGELOG.md` from [Conventional Commits](https://www.conventionalcommits.org/)
and posts a summary comment on pull requests.

Install `git-cliff` if you don't already have it:

```bash
cargo install git-cliff
```

Then, from the repository root, generate the short "unreleased changes"
summary (this is the version posted as a PR comment):

```bash
git cliff --config cliff.toml --latest --no-exec --unreleased --github-repo philiplinden/bevy_repl
```

Or regenerate the full `CHANGELOG.md` (this is the version committed back to
the repo when a tag is pushed):

```bash
git cliff --config cliff.toml --latest --no-exec --github-repo philiplinden/bevy_repl -o CHANGELOG.md
```

> [!NOTE]
> The `--github-repo` flag fetches contributor and PR metadata from the
> GitHub API. Set a `GITHUB_TOKEN` environment variable first to avoid
> rate limiting:
>
> ```bash
> export GITHUB_TOKEN=<your-personal-access-token>
> ```

After regenerating `CHANGELOG.md`, review the diff and commit it yourself —
in CI this step is done automatically by the `EndBug/add-and-commit` action.
