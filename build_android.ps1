$ErrorActionPreference = "Stop"

# Path Configurations
$RUST_CORE_DIR = "$PSScriptRoot\core"
$ANDROID_JNI_DIR = "$PSScriptRoot\android\app\src\main\jniLibs"
$TARGET_KOTLIN_PATH = "$PSScriptRoot\android\app\src\main\java\com\filipevl\compras"

$env:ANDROID_NDK_HOME = "C:\Users\filip\AppData\Local\Android\Sdk\ndk\29.0.14206865"

Write-Host "Building Core for windows host to extract bindings..." -ForegroundColor Cyan
Push-Location $RUST_CORE_DIR
cargo build --lib

if ($LASTEXITCODE -ne 0) {
    Pop-Location
    throw "Core build failed! Stopping script."
}

Write-Host "Generating kotlin bindings via UniFFI..." -ForegroundColor Cyan
cargo run --bin uniffi_bindgen generate --library "$PSScriptRoot\target\debug\compras_core.dll" --language kotlin --out-dir "$RUST_CORE_DIR\generated-bindings"

if ($LASTEXITCODE -ne 0) {
    Pop-Location
    throw "UniFFI binding generation failed! Stopping script."
}

Write-Host "Cross-compiling binaries for android architectures..." -ForegroundColor Cyan
if (-not (Test-Path $ANDROID_JNI_DIR)) {
    New-Item -ItemType Directory -Path $ANDROID_JNI_DIR | Out-Null
}

cargo ndk -t aarch64-linux-android -t x86_64-linux-android -o $ANDROID_JNI_DIR build --release

if ($LASTEXITCODE -ne 0) {
    Pop-Location
    throw "Android cross-compilation failed! Stopping script."
}
Pop-Location

Write-Host "Moving generated files into android project..." -ForegroundColor Cyan
# Ensure target package directory exists
if (-not (Test-Path $TARGET_KOTLIN_PATH)) {
    New-Item -ItemType Directory -Path $TARGET_KOTLIN_PATH | Out-Null
}

# Recursively hunt down any Kotlin files UniFFI created inside the package structure
$GeneratedFiles = Get-ChildItem -Path "$RUST_CORE_DIR\generated-bindings" -Filter "*.kt" -Recurse

if ($GeneratedFiles) {
    foreach ($File in $GeneratedFiles) {
        Copy-Item $File.FullName "$TARGET_KOTLIN_PATH\"
        Write-Host "Copied $($File.Name) to Android project." -ForegroundColor Green
    }
} else {
    throw "Could not find any generated Kotlin binding files inside the temporary directory!"
}

Remove-Item -Recurse -Force "$RUST_CORE_DIR\generated-bindings"

Write-Host "Done" -ForegroundColor Green