# Codex Agent Instructions

## Scope
These instructions apply to the entire repository.

## Reasoning
`pyvcloud`'s Python implementation is deprecated. We are migrating the codebase
to a Rust library for improved maintainability, performance and safety.

## Principles
- Follow SOLID and DRY principles when adding or modifying code.
- Keep commits focused on a single change.

## Commit Guidelines
- Use concise commit messages in the present tense, e.g. `Add docs`.
- When editing documentation or code, ensure README and relevant docs remain up to date.

## Testing
- Before committing, run `cargo fmt -- --check` and `cargo clippy -- -D warnings`.
- Run `cargo test` for the unit tests.
- Include the results of these commands in the PR **Testing** section.
- It's ok if tests fail, but note any failures in the PR summary.

