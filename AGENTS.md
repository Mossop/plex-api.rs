# Agent Guidelines for `plex-api.rs`

Welcome! This file captures repo-specific expectations for future agents. Its
rules apply to the entire repository unless a more specific `AGENTS.md` is
added in a subdirectory.

The workspace consists of 4 different crates, stored in the `crates` directory:

* `plex-api`, the main crate, representing the API wrapper for Plex
* `plex-api-test-helper`, a few macros to simplify tests development
* `plex-cli`, basically an example crate for the API. Allows the users to
  interact with the Plex Media Servers using CLI.
* `xtask`, another internal crate, used mostly for CI.

## Coding and Style

* This is a Rust workspace; keep code idiomatic and strongly typed.
* Run `cargo fmt --all` on any Rust changes to ensure consistent formatting.
* Prefer adding documentation comments (`///`) when you introduce new public
  items.
* Keep modules and functions small and focused; split large changes into helper
  functions when it improves readability.
* Do not update dependencies, if it causes a lot of not related to the task
  rewrites.

## Testing

* Default test command: `cargo test`.
* When touching code that interacts with Plex data or xtasks, also consider
  running `cargo xtask test --online` if practical. Working Docker setup is
  required for this to work.
* Add or update tests alongside code changes whenever feasible.
* Use the helpers from `crates/plex-api-test-helper` where possible.

## Tooling

* Clippy warnings should be addressed or explicitly justified. Run
  `cargo clippy --all-targets -- -D warnings` before finishing significant
  changes.
* This repo uses [`cargo-husky`](https://crates.io/crates/cargo-husky) for git
  hooks; running `cargo test` at least once will install the configured hooks.
  If you're running the tests and clippy manually, it might be worth committing
  to git with the `--no-verify` flag.
* The `xtask` crate contains automation used by CI. Mirror any CI-only command
  locally when feasible.

## Documentation

* Update relevant Markdown files (e.g., `README.md`, `CONTRIBUTING.md`) when
  behavior or workflows change.
* Use tables or lists for clarity in documentation updates.

## Commit and PR Expectations

* Follow the Conventional Commits specification for commit messages.
* Keep commits focused and descriptive.
* Provide clear PR descriptions summarizing the changes and tests performed.
  The commit message must be based on the overall changes too.
* Group related changes together; avoid mixing formatting-only diffs with
  logic updates unless formatting is integral to the change.

Thank you for maintaining these standards!
