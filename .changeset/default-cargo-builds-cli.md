---
monosecret: feat
---

# Default cargo builds to the CLI crate

Plain `cargo build`, `cargo check`, and `cargo test` now operate on the CLI crate only via workspace `default-members`. The language SDK members (FFI, npm, PHP, Python, and examples) require the `php`, `python`, and `node` interpreters at build time and are now selected explicitly with `--workspace` / `-p` in CI, devenv tasks, and publish workflows. Sandboxed CLI-only builds — such as Nix packaging, which installs monosecret without those interpreters — work again with a bare `cargo build`.
