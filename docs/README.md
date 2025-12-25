# Documentation Builds

## Overview

The Rust edition of `pyvcloud` ships documentation generated directly from Rustdoc. The preferred workflow is to use `cargo doc` for local builds and to publish the generated static site as part of your release automation.

Rustdoc renders inline documentation from the crate sources and produces a set of static HTML pages in `target/doc`. These pages can be published to GitHub Pages or any other static host.

## Local documentation build

```bash
# build docs for the library and binary crates
cargo doc --no-deps

# open the landing page locally
xdg-open target/doc/pyvcloud/index.html
```

## Publishing tips

- Ensure `cargo doc` runs as part of CI so documentation stays in sync with the code.
- When publishing to GitHub Pages, copy the contents of `target/doc` to the desired branch (commonly `gh-pages`).
- Because the documentation is static, Jekyll processing should be disabled by placing a `.nojekyll` file in the published root if GitHub Pages is used.
