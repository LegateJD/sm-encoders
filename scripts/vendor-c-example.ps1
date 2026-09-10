<#
.SYNOPSIS
Builds the ranis Rust crate and vendors the resulting C library
and headers into example/c_lang/vendor/ranis, so the CMake example
can find them.

.PARAMETER Release
Build in release mode instead of debug.

.EXAMPLE
scripts/vendor-c-example.ps1
scripts/vendor-c-example.ps1 -Release
#>
param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$profile = if ($Release) { "release" } else { "debug" }

$vendorDir = Join-Path $repoRoot "example/c_lang/vendor/ranis"
$libDir = Join-Path $vendorDir "lib"
$includeDir = Join-Path $vendorDir "include"

Push-Location $repoRoot
try {
    if ($Release) {
        cargo build --release
    } else {
        cargo build
    }
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed with exit code $LASTEXITCODE"
    }
} finally {
    Pop-Location
}

New-Item -ItemType Directory -Force -Path $libDir, $includeDir | Out-Null

$targetDir = Join-Path $repoRoot "target/$profile"

# MSVC builds produce "_ranis.*" (no "lib" prefix), MinGW builds
# produce "lib_ranis.*". Match both, but skip the DLL import library
# ("*.dll.lib") and dep-info files, neither of which the example needs.
$libs = Get-ChildItem -Path $targetDir -File |
    Where-Object {
        $_.Name -match '^(lib)?_ranis\.(a|lib|dll)$'
    }

if (-not $libs) {
    Write-Error "No ranis build artifacts found in $targetDir"
    exit 1
}

Copy-Item -Path $libs.FullName -Destination $libDir -Force
if (Test-Path $includeDir) {
    Remove-Item -Path $includeDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $includeDir | Out-Null
Copy-Item -Path (Join-Path $repoRoot "include/*") -Destination $includeDir -Recurse -Force

Write-Host "Vendored ranis ($profile) into $vendorDir"
