#!/bin/sh
# Builds the core's wasm32 module, sdk/core/dist/clunkyard-core.wasm, which
# the DSL loader (src/dsl/core.js) runs. The module is committed, so a
# checkout, npm test and local practice need no Rust; `make core-check`
# rebuilds it here and fails when the bytes differ.
#
#   sdk/core/build-wasm.sh [OUT]     OUT defaults to sdk/core/dist/clunkyard-core.wasm
#
# The bytes depend on the toolchain, so both tools are pinned and checked:
#   - Rust 1.96.0 (RUST_VERSION in the Dockerfile) with the
#     wasm32-unknown-unknown target; rustup's `1.96.0` toolchain is used
#     when the default rustc is another version;
#   - binaryen's wasm-opt, version 120.
# Paths are remapped and debug, producer and feature sections stripped, so
# the build is the same from any checkout path. The crates are vendored
# (sdk/core/vendor, through sdk/core/.cargo/config.toml, which cargo reads
# from the directory it runs in), so it builds offline.
set -eu

RUST_VERSION=1.96.0
WASM_OPT_VERSION=120

core=$(cd "$(dirname "$0")" && pwd)
out=${1:-$core/dist/clunkyard-core.wasm}

fail() { echo "build-wasm: $*" >&2; exit 1; }

# The pinned rustc, from rustup's toolchain of that name when the default
# is another version.
case "$(rustc --version 2>/dev/null)" in
  "rustc $RUST_VERSION "*) toolchain= ;;
  *) toolchain=$RUST_VERSION ;;
esac
if [ -n "$toolchain" ]; then
  command -v rustup >/dev/null || fail "needs rustc $RUST_VERSION (rustc is $(rustc --version 2>/dev/null || echo missing))"
  export RUSTUP_TOOLCHAIN="$toolchain"
  rustc --version 2>/dev/null | grep -q "^rustc $RUST_VERSION " || fail "needs rustc $RUST_VERSION: rustup toolchain install $RUST_VERSION --target wasm32-unknown-unknown"
fi
rustc --print target-list | grep -qx wasm32-unknown-unknown || fail "rustc has no wasm32-unknown-unknown"
[ -d "$(rustc --print sysroot)/lib/rustlib/wasm32-unknown-unknown" ] || fail "add the target: rustup target add wasm32-unknown-unknown --toolchain $RUST_VERSION"
command -v wasm-opt >/dev/null || fail "needs binaryen's wasm-opt, version $WASM_OPT_VERSION"
wasm-opt --version | grep -qE "version $WASM_OPT_VERSION( |$)" || fail "needs wasm-opt version $WASM_OPT_VERSION, not: $(wasm-opt --version)"

sysroot=$(rustc --print sysroot)
commit=$(rustc -vV | sed -n 's/^commit-hash: //p')
cargo_home=${CARGO_HOME:-$HOME/.cargo}
build=$(mktemp -d "${TMPDIR:-/tmp}/clunkyard-core-wasm.XXXXXX")
trap 'rm -rf "$build"' EXIT

# RUSTFLAGS replaces any build.rustflags a user's own cargo config sets, so
# the module does not depend on it. The standard library's paths read as
# /rustc/<commit> whether or not rust-src is installed.
RUSTFLAGS="--remap-path-prefix=$core=/clunkyard/sdk/core --remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$sysroot/lib/rustlib/src/rust=/rustc/$commit" \
CARGO_TARGET_DIR="$build/target" \
CARGO_BUILD_RUSTC_WRAPPER='' RUSTC_WRAPPER='' \
  sh -c 'cd "$1" && exec cargo build --offline --locked --lib --target wasm32-unknown-unknown --profile wasm --quiet' sh "$core"

mkdir -p "$(dirname "$out")"
wasm-opt -Oz --strip-debug --strip-producers --strip-target-features \
  "$build/target/wasm32-unknown-unknown/wasm/clunkyard.wasm" -o "$out"
