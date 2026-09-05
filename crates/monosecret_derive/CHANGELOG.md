# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2](https://github.com/ifiokjr/monosecret/releases/tag/v0.3.2) (2026-09-05)

### Fixes

#### Refresh cargo, pnpm, dart, and devenv dependencies

`cargo update`, `pnpm update --latest` (vitest 4 → 5, tsdown 0.22 → 0.23,
oxfmt 0.63 → 0.66, oxlint 1.78 → 1.81), `dart pub upgrade`, and
`devenv update` (devenv CLI, git-hooks.nix, custom nixpkgs inputs).

`keepass` is pinned to `=0.13.17`: the 0.13.25 release depends on a
cipher/cbc combination that `aes` 0.8 does not implement, which broke the
kdbx provider's build.

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #49](https://github.com/ifiokjr/monosecret/pull/49)

#### Eliminate every clippy warning and promote lint groups to deny

Fixed ~1330 clippy warnings across the workspace (format-arg inlining,
doc-comment backticks, digit-separated literals, redundant
qualifications/closures, needless borrows, `let … else`, internal
pass-by-value → references, `#[must_use]` additions, unnecessary `Result`
wraps, dead code) and converted `indexing_slicing` in production parsers to
bounds-checked access with error propagation, so malformed provider responses
can no longer panic. Fixed a latent `cached_route` panic (inline-URI alias
caching into its own store), a pre-existing flaky Infisical TCP test, and
reverted a clippy `--fix` regression that flipped the vault missing-`tls`
default.

All clippy groups (`complexity`, `pedantic`, `perf`, `style`, `suspicious`)
are now `deny`, the ffi/node/php/python crates inherit the workspace lints,
and CI clippy runs with `-D warnings`.

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #45](https://github.com/ifiokjr/monosecret/pull/45)

## [0.3.1](https://github.com/ifiokjr/monosecret/releases/tag/v0.3.1) (2026-08-28)

### Changed

- No package-specific changes were recorded; `monosecret_derive` was updated to 0.3.1 as part of group `monosecret`.

## [0.3.0](https://github.com/ifiokjr/monosecret/releases/tag/v0.3.0) (2026-08-15)

### Changed

- No package-specific changes were recorded; `monosecret_derive` was updated to 0.3.0 as part of group `monosecret`.

## [0.2.1](https://github.com/ifiokjr/monosecret/releases/tag/v0.2.1) (2026-08-14)

### Changed

- No package-specific changes were recorded; `monosecret_derive` was updated to 0.2.1 as part of group `monosecret`.

## [0.2.0](https://github.com/ifiokjr/monosecret/releases/tag/v0.2.0) (2026-08-13)

### Changed

- No package-specific changes were recorded; `monosecret_derive` was updated to 0.2.0 as part of group `monosecret`.

## [0.1.0](https://github.com/ifiokjr/monosecret/releases/tag/v0.1.0) (2026-07-05)

### Other

- Add a secret-value-free manifest command for SDK code generation, introduce a build_runner-based Dart typed SDK generator, reorganize source by ecosystem into `crates/`, `npm/`, and `dart/`, and wire Rust, Dart, and npm coverage reports with package-level Codecov flags.
