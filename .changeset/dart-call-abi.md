---
"dart": feat
---

# Dart SDK: inline specs and caller context via the versioned call ABI

The Dart SDK now binds the versioned `monosecret_call` native entry point,
matching the other language SDKs:

- `MonosecretBuilder.withInlineSpec(spec, baseDir)` resolves strict
  inline-spec v1 declarations through the versioned call envelope; inline
  resolution never falls back to a filesystem manifest, and `withPath`
  clears the inline spec.
- `CallerContext` and `MonosecretBuilder.withCaller` record the invoking
  integration in audit records (they never satisfy a `require_reason`
  policy); `MonosecretClient.resolve`/`report` accept an optional caller.
- The bundled native library is probed for the call entry point and the
  result is cached; older libraries raise a `capability`
  `MonosecretException` on inline requests instead of an opaque ffi error.
