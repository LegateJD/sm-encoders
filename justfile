\
# List available recipes.
default:
    @just --list

# Build the ranis crate (debug).
build:
    cargo build

# Build the ranis crate (release).
build-release:
    cargo build --release

# Build ranis and vendor the library/headers into example/c_lang/vendor/ranis (debug).
[unix]
vendor-c-example:
    ./scripts/vendor-c-example.sh

[windows]
vendor-c-example:
    powershell -ExecutionPolicy Bypass -File scripts/vendor-c-example.ps1

# Build ranis and vendor the library/headers into example/c_lang/vendor/ranis (release).
[unix]
vendor-c-example-release:
    ./scripts/vendor-c-example.sh --release

[windows]
vendor-c-example-release:
    powershell -ExecutionPolicy Bypass -File scripts/vendor-c-example.ps1 -Release
