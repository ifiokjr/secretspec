---
"rust:monosecret": fix
"rust:monosecret_derive": fix
"rust:monosecret_ffi": fix
"@monosecret/client": fix
---

# Fix keyring lookups and 1Password `depends_on` tokens broken in 0.3.2

Two regressions broke every keyring-backed secret for specs whose providers
declare `depends_on` (e.g. an `op+token` provider bootstrapped from a
keyring-stored service account token):

- **whoami compiled without its `std` feature**: the workspace dependency
  `whoami = { default-features = false }` selected whoami's stub platform
  backend, which reports `"anonymous"` as the current username on every
  native platform. The keyring provider addresses convention entries by
  `(service = monosecret/{project}/{profile}/{key}, account = username)`, so
  every lookup silently missed and every `set` would have written to a
  non-existent account. Default features are restored (the `std` feature is
  what compiles the real macOS/Windows/Linux backend), and a regression test
  asserts the resolved username is not the stub value.

- **`PreflightGuard` dropped `depends_on` bootstrap secrets**: the guard
  wrapping providers with auth preflights forwarded `set_reason`,
  `set_profile`, and `with_base_dir`, but not
  `Provider::configure_dependency_secrets` — so the trait's no-op default
  swallowed every resolved dependency. A provider declared with
  `depends_on = [{ secret = "OP_SERVICE_ACCOUNT_TOKEN" }]` resolved the token
  correctly and then discarded it, running every `op` child tokenless (which
  fails with `"<vault>" isn't a vault in this account` or `account is not
  signed in`). The guard now forwards the call, and the `onepassword` provider
  (`op+token://`) implements it: a delivered `OP_SERVICE_ACCOUNT_TOKEN` is
  exported to every `op` child process, ranked after an explicitly supplied
  provider credential and ahead of the ambient environment variable, matching
  `onepassword+env`'s existing behavior. Forwarding and token-precedence
  regression tests included.
- **`Arc` wrapping dropped the same hook one layer deeper** (caught by the
  new end-to-end regression tests): providers registered with a preflight are
  built as `Box<Arc<P>>`, and the blanket `impl Provider for Arc<T>` cannot
  forward a `&mut self` hook — an `Arc` gives no `&mut` access — so the
  delivery died at that layer even with the guard fixed.
  `configure_dependency_secrets` is now a `&self` hook with interior
  mutability (matching `set_reason`/`set_profile`), forwarded explicitly by
  the `Arc` blanket impl and `PreflightGuard`. This also fixes the
  `onepassword+env` provider, whose pre-existing implementation was silently
  swallowed by the same wrapper stack.
