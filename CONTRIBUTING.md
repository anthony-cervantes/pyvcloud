# Contributing to pyvcloud

Welcome! We gladly accept contributions from the community. If you wish to contribute code and you have not signed our contributor license agreement (CLA), our bot will update the issue when you open a pull request. For any questions about the CLA process, please refer to our [FAQ](https://cla.vmware.com/faq).

## Logging Bugs

Anyone can log a bug using the GitHub 'New Issue' button. Please use a short title and give as much information as you can about what the problem is, relevant software versions, and how to reproduce it. If you know the fix or a workaround include that too.

## Code Contribution Flow

We use GitHub pull requests to incorporate code changes from external contributors. Typical contribution flow steps are:

- Fork the pyvcloud repo into a new repo on GitHub
- Clone the forked repo locally and set the original pyvcloud repo as the upstream repo
- Make changes in a topic branch and commit
- Fetch changes from upstream and resolve any merge conflicts so that your topic branch is up-to-date
- Push all commits to the topic branch in your forked repo
- Submit a pull request to merge topic branch commits to upstream main

If this process sounds unfamiliar, have a look at the excellent [overview of collaboration via pull requests on GitHub](https://help.github.com/categories/collaborating-with-issues-and-pull-requests) for more information.

## Project Coding Conventions

You'll raise the chance of having your fix accepted if it matches our coding conventions.

### Code Style

- Format code with [`cargo fmt`](https://rust-lang.github.io/rustfmt/) before submitting.
- Run [`cargo clippy`](https://rust-lang.github.io/rust-clippy/) and address reported warnings where practical.
- Prefer expressive, strongly typed interfaces and avoid `unwrap` in favor of structured error handling.
- Keep public APIs documented with Rustdoc comments and examples.

### Testing

- Add unit tests that cover new behaviors or regressions. `cargo test` should pass before opening a pull request.
- Prefer integration tests in `tests/` for end-to-end flows where possible.

### Documentation

- Update `README.md`, `docs/`, and inline Rustdoc when you change functionality or behavior.
- Include examples in Rustdoc where it helps users understand how to consume the API.

Thanks for contributing to pyvcloud!
