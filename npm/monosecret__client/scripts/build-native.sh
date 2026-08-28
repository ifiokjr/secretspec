#!/usr/bin/env bash
#
# Build the napi-rs addon via `napi build` and place it as monosecret-client.node
# next to index.js. Set MONOSECRET_NODE_PROFILE=debug for a faster unoptimized
# build (default: release). Extra arguments are forwarded to `napi build`.
set -euo pipefail

pkg_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
napi_bin="$pkg_dir/node_modules/.bin/napi"
profile="${MONOSECRET_NODE_PROFILE:-release}"

case "$profile" in
release) profile_args=(--release) ;;
debug) profile_args=() ;;
*)
	echo "error: MONOSECRET_NODE_PROFILE must be 'release' or 'debug'" >&2
	exit 2
	;;
esac

# --output-dir keeps napi build's generated declarations out of the TypeScript
# build output.
tmp_out="$(mktemp -d)"
trap 'rm -rf "$tmp_out"' EXIT
(cd "$pkg_dir" && "$napi_bin" build "${profile_args[@]}" --output-dir "$tmp_out" "$@")

# Install atomically: node --test runs test files in parallel processes that
# may build concurrently, and overwriting in place SIGBUSes a process that has
# already mapped the addon. A rename keeps the old inode valid for them.
mv -f "$tmp_out/monosecret-client.node" "$pkg_dir/monosecret-client.node.tmp.$$"
mv -f "$pkg_dir/monosecret-client.node.tmp.$$" "$pkg_dir/monosecret-client.node"
echo "built monosecret-client.node"
