#!/bin/sh
# Runs a wasm32-wasip1 test binary under wasmtime with the checkout mapped
# at its own path, so the tests read protocol/v2 as they do natively:
#   cargo test --target wasm32-wasip1
root=$(cd "$(dirname "$0")/../../.." && pwd)
binary=$1
shift
exec wasmtime run --dir "$root::$root" ${CY_BLESS:+--env CY_BLESS=1} ${CY_TRANSCRIPT:+--dir "$CY_TRANSCRIPT::$CY_TRANSCRIPT" --env CY_TRANSCRIPT="$CY_TRANSCRIPT"} -- "$binary" "$@"
