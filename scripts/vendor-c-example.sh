#!/usr/bin/env bash
#
# Builds the sm-encoders Rust crate and vendors the resulting C library
# and headers into example/c_lang/vendor/sm_encoders, so the CMake example
# can find them.
#
# Usage: scripts/vendor-c-example.sh [--release]

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
profile="debug"
cargo_args=()

if [[ "${1:-}" == "--release" ]]; then
    profile="release"
    cargo_args+=("--release")
fi

vendor_dir="$repo_root/example/c_lang/vendor/sm_encoders"
lib_dir="$vendor_dir/lib"
include_dir="$vendor_dir/include"

cd "$repo_root"
cargo build "${cargo_args[@]}"

mkdir -p "$lib_dir" "$include_dir"

# Only the static/shared libs are needed by the C example, not .rlib/.d.
mapfile -t libs < <(find "target/$profile" -maxdepth 1 -type f \
    -name 'lib_sm_encoders.*' ! -name '*.d' ! -name '*.rlib')

if [[ ${#libs[@]} -eq 0 ]]; then
    echo "error: no lib_sm_encoders build artifacts found in target/$profile" >&2
    exit 1
fi

cp -f "${libs[@]}" "$lib_dir/"
cp -f "$repo_root"/include/*.h "$include_dir/"

echo "Vendored sm-encoders ($profile) into $vendor_dir"
