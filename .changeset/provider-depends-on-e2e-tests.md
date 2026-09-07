---
"rust:monosecret": none
"rust:monosecret_derive": none
"rust:monosecret_ffi": none
"@monosecret/client": none
---

# End-to-end regression tests for provider `depends_on` delivery

Adds `crates/monosecret/tests/provider_dependency_token.rs`, two integration
tests that run the real CLI binary against a temporary manifest mirroring the
dotfiles setup that broke in 0.3.2: an `op+token` provider alias bootstrapped
from a `depends_on` secret stored in another provider, with the `op` CLI
replaced by a stub that records the `OP_SERVICE_ACCOUNT_TOKEN` it was
exported.

- **`depends_on_token_reaches_op_child_through_full_resolution`** resolves a
  secret through the full pipeline (manifest parsing, fallback planning,
  `PreflightGuard`, the `Arc`-wrapped concrete provider, child-process
  environment) and asserts every `op` child ran with the delivered token and
  the value resolved. This test caught the `Arc` layer of the 0.3.2
  regression after the isolated unit tests all passed — a refactor that
  builds providers through a path that skips dependency delivery fails here
  even when wrapper-level tests stay green.
- **`missing_dependency_secret_fails_resolution_loudly`** asserts a missing
  bootstrap secret fails resolution hard with the
  `requires secret '<name>'` error, rather than silently continuing
  tokenless.

Together with the wrapper-level unit tests on PR #50 (guard forwarding,
`Arc` forwarding, child-env export, precedence), every layer of the delivery
path is pinned so the regression cannot return unnoticed on a future release.
