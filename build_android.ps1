$ErrorActionPreference = "Stop"

# Runs a native command and stops the script if it fails.
function Invoke-Native([scriptblock]$Command) {
    & $Command
    if ($LASTEXITCODE -ne 0) { throw "Failed: $Command" }
}

$JniLibs = "$PSScriptRoot\android\app\src\main\jniLibs"
$KotlinSrc = "$PSScriptRoot\android\app\src\main\java"

Push-Location "$PSScriptRoot\core"
try {
    Write-Host "Building core for the host (bindings are generated from it)..." -ForegroundColor Cyan
    Invoke-Native { cargo build --lib }

    Write-Host "Generating Kotlin bindings..." -ForegroundColor Cyan
    Invoke-Native { cargo run --bin uniffi_bindgen generate --library "$PSScriptRoot\target\debug\compras_core.dll" --language kotlin --out-dir $KotlinSrc }

    Write-Host "Cross-compiling core for Android..." -ForegroundColor Cyan
    New-Item -ItemType Directory -Force $JniLibs | Out-Null
    Invoke-Native { cargo ndk -t aarch64-linux-android -t x86_64-linux-android -o $JniLibs build --release }

}
finally {
    Pop-Location
}

Write-Host "Done" -ForegroundColor Green