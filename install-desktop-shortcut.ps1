# Recreate the Aethyron desktop shortcut with a real .ico and a reliable launch target.
$ErrorActionPreference = "Stop"

$ProjectRoot = $PSScriptRoot
$IcoPath     = Join-Path $ProjectRoot "aethyron.ico"
$LaunchPs1   = Join-Path $ProjectRoot "launch-aethyron.ps1"
$Desktop     = [Environment]::GetFolderPath("Desktop")
$Shortcut    = Join-Path $Desktop "Aethyron.lnk"

if (-not (Test-Path $IcoPath)) {
    throw "Icon not found: $IcoPath"
}
if (-not (Test-Path $LaunchPs1)) {
    throw "Launcher not found: $LaunchPs1"
}

$powershell = Join-Path $env:SystemRoot "System32\WindowsPowerShell\v1.0\powershell.exe"
if (-not (Test-Path $powershell)) {
    throw "powershell.exe not found: $powershell"
}

# Refresh the shortcut file so Explorer picks up the new icon.
if (Test-Path $Shortcut) {
    Remove-Item $Shortcut -Force
}

$shell = New-Object -ComObject WScript.Shell
$lnk = $shell.CreateShortcut($Shortcut)
$lnk.TargetPath       = $powershell
$lnk.Arguments        = "-NoProfile -ExecutionPolicy Bypass -File `"$LaunchPs1`""
$lnk.WorkingDirectory = $ProjectRoot
$lnk.IconLocation     = "$IcoPath,0"
$lnk.WindowStyle      = 1
$lnk.Description      = "Launch Aethyron autonomous coding agent"
$lnk.Save()

Write-Host "Desktop shortcut written:" $Shortcut
Write-Host "Target:" $lnk.TargetPath
Write-Host "Icon:" $lnk.IconLocation
