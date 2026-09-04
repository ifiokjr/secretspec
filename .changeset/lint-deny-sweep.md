---
"rust:monosecret": fix
"rust:monosecret_derive": fix
"rust:monosecret_ffi": fix
"@monosecret/client": fix
---

# Eliminate every clippy warning and promote lint groups to deny

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
