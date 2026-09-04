\
# List available recipes.
default:
    @just --list

# Build the sm-encoders crate (debug).
build:
    cargo build

# Build the sm-encoders crate (release).
build-release:
    cargo build --release

# Build sm-encoders and vendor the library/headers into example/c_lang/vendor/sm_encoders (debug).
[unix]
vendor-c-example:
    ./scripts/vendor-c-example.sh

[windows]
vendor-c-example:
    powershell -ExecutionPolicy Bypass -File scripts/vendor-c-example.ps1

# Build sm-encoders and vendor the library/headers into example/c_lang/vendor/sm_encoders (release).
[unix]
vendor-c-example-release:
    ./scripts/vendor-c-example.sh --release

[windows]
vendor-c-example-release:
    powershell -ExecutionPolicy Bypass -File scripts/vendor-c-example.ps1 -Release
