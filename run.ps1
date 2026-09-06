# Aethyron — Single Entry Point Launcher
# Run from the Aethyron/ root directory:
#   .\run.ps1

$Root = $PSScriptRoot

Write-Host ""
Write-Host "╔══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         AETHYRON  LAUNCHER               ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Step 1: Build the UI
Write-Host "[ 1/2 ] Building UI..." -ForegroundColor Yellow
Set-Location "$Root\aethyron-ui"
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "X UI build failed. Aborting." -ForegroundColor Red
    exit 1
}
Write-Host "OK UI built." -ForegroundColor Green
Write-Host ""

# Step 2: Start the Rust server
Write-Host "[ 2/2 ] Starting Aethyron server..." -ForegroundColor Yellow
Set-Location "$Root\aethyron-core"
cargo run
