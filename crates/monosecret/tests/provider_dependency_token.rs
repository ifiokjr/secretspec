//! End-to-end guard for provider `depends_on` delivery (regression: 0.3.2).
//!
//! Reproduces the failure shape that broke `msload`/`ms` on 0.3.2: a provider
//! alias whose bootstrap secret (`depends_on`) is stored in *another* provider,
//! with every routed secret resolved through the full pipeline — manifest
//! parsing, per-secret fallback planning, `PreflightGuard` wrapping, and the
//! concrete provider's child-process environment.
//!
//! The two isolated layers are pinned in unit tests (`preflight.rs` forwarding,
//! `onepassword.rs` token export and precedence). This test pins the glue:
//! `Secrets::build_provider_for_use` must resolve the dependency *and* deliver
//! it through the wrapper to the `op` child process. A refactor that builds
//! providers through a path that skips `configure_dependency_secrets` fails
//! here, even if the isolated tests still pass.
#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use tempfile::TempDir;

fn bin() -> &'static str {
	env!("CARGO_BIN_EXE_monosecret")
}

fn forward_slashes(p: &std::path::Path) -> String {
	p.display().to_string().replace('\\', "/")
}

/// A stand-in `op` CLI that records the `OP_SERVICE_ACCOUNT_TOKEN` it was
/// exported on every invocation, and answers the two calls the onepassword
/// provider makes during a read: `vault list` (auth preflight) and
/// `read --no-newline <ref>` (the secret itself).
fn write_op_stub(script: &std::path::Path, log: &std::path::Path) {
	let script_body = format!(
		r#"#!/bin/sh
printf '%s\n' "$OP_SERVICE_ACCOUNT_TOKEN" >> '{}'
case "$1" in
	vault) printf '[]\n' ;;
	read) printf 'e2e-secret' ;;
	*) printf 'unexpected op call: %s\n' "$*" >&2; exit 1 ;;
esac
"#,
		log.display()
	);
	fs::write(script, script_body).unwrap();
	let mut permissions = fs::metadata(script).unwrap().permissions();
	permissions.set_mode(0o755);
	fs::set_permissions(script, permissions).unwrap();
}

#[test]
fn depends_on_token_reaches_op_child_through_full_resolution() {
	let dir = TempDir::new().unwrap();
	let auth_dotenv = dir.path().join("auth.env");
	fs::write(
		&auth_dotenv,
		"OP_SERVICE_ACCOUNT_TOKEN=ops_dependency-token\n",
	)
	.unwrap();
	let op_stub = dir.path().join("op-stub");
	let log = dir.path().join("op-calls.log");
	write_op_stub(&op_stub, &log);

	let config_path = dir.path().join("monosecret.toml");
	fs::write(
		&config_path,
		format!(
			r#"
[project]
name = "provider-dep-e2e"
revision = "1.0"

[providers]
bootstrap = "dotenv://{auth}"

[providers.op-token]
uri = "op+token://Development/Dotfiles"

[[providers.op-token.depends_on]]
secret = "OP_SERVICE_ACCOUNT_TOKEN"

[profiles.default]
SECRET_VIA_OP = {{ description = "via op", providers = [{{ provider = "op-token", path = ["registries"] }}] }}
OP_SERVICE_ACCOUNT_TOKEN = {{ description = "1password auth", providers = ["bootstrap"] }}
"#,
			auth = forward_slashes(&auth_dotenv),
		),
	)
	.unwrap();

	let output = Command::new(bin())
		.current_dir(dir.path())
		.arg("-f")
		.arg(&config_path)
		.arg("--reason")
		.arg("provider dependency regression test")
		.args(["get", "SECRET_VIA_OP"])
		.env("MONOSECRET_OPCLI_PATH", &op_stub)
		.output()
		.unwrap();

	assert!(
		output.status.success(),
		"stderr: {}",
		String::from_utf8_lossy(&output.stderr)
	);
	assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "e2e-secret");

	// The core assertion: the `depends_on`-delivered token reached the `op`
	// child process. Regression in 0.3.2: the token was resolved but dropped
	// by the wrapping PreflightGuard, so every op child ran tokenless.
	let log = fs::read_to_string(&log).unwrap();
	assert!(
		!log.is_empty(),
		"the op stub was never invoked; resolution did not reach the provider"
	);
	assert!(
		log.lines().all(|line| line == "ops_dependency-token"),
		"every op child must run with the depends_on-delivered token; log: {log:?}"
	);
}

/// The negative form: with the token deliberately absent from the bootstrap
/// store, resolution must fail hard (0.3.2's original symptom) rather than
/// silently continuing tokenless. Guards the error contract of
/// `resolve_legacy_provider_dependencies`.
#[test]
fn missing_dependency_secret_fails_resolution_loudly() {
	let dir = TempDir::new().unwrap();
	let auth_dotenv = dir.path().join("auth.env");
	// The bootstrap store exists but lacks the declared token.
	fs::write(&auth_dotenv, "OTHER_KEY=unrelated\n").unwrap();

	let config_path = dir.path().join("monosecret.toml");
	fs::write(
		&config_path,
		format!(
			r#"
[project]
name = "provider-dep-missing"
revision = "1.0"

[providers]
bootstrap = "dotenv://{auth}"

[providers.op-token]
uri = "op+token://Development/Dotfiles"

[[providers.op-token.depends_on]]
secret = "OP_SERVICE_ACCOUNT_TOKEN"

[profiles.default]
SECRET_VIA_OP = {{ description = "via op", providers = ["op-token"] }}
OP_SERVICE_ACCOUNT_TOKEN = {{ description = "1password auth", providers = ["bootstrap"] }}
"#,
			auth = forward_slashes(&auth_dotenv),
		),
	)
	.unwrap();

	let output = Command::new(bin())
		.current_dir(dir.path())
		.arg("-f")
		.arg(&config_path)
		.arg("--reason")
		.arg("provider dependency regression test")
		.args(["get", "SECRET_VIA_OP"])
		.output()
		.unwrap();

	assert!(
		!output.status.success(),
		"a missing depends_on secret must fail resolution, not silently continue tokenless"
	);
	let stderr = String::from_utf8_lossy(&output.stderr);
	assert!(
		stderr.contains("requires secret 'OP_SERVICE_ACCOUNT_TOKEN'"),
		"the failure must name the missing bootstrap secret: {stderr}"
	);
}
