# ============================================================
#  Aethyron Launcher
#  Starts Ollama (if not running), builds & starts the
#  Aethyron server, then opens the UI in the default browser.
# ============================================================

$AethyronRoot = "C:\Users\jerem\Documents\Aethyron\aethyron-core"
$ServerUrl    = "http://127.0.0.1:3000"
$HealthUrl    = "$ServerUrl/health"

# ── Helper: write coloured status lines ──────────────────────
function Write-Status($msg, $color = "Cyan") {
    Write-Host "  $msg" -ForegroundColor $color
}

Clear-Host
Write-Host ""
Write-Host "  ╔══════════════════════════════════════════╗" -ForegroundColor DarkBlue
Write-Host "  ║           AETHYRON  LAUNCHER             ║" -ForegroundColor Blue
Write-Host "  ╚══════════════════════════════════════════╝" -ForegroundColor DarkBlue
Write-Host ""

# ── 1. Ensure Ollama is running ───────────────────────────────
$ollamaRunning = Get-Process -Name "ollama" -ErrorAction SilentlyContinue
if (-not $ollamaRunning) {
    Write-Status "Starting Ollama..." "Yellow"
    $ollamaExe = Get-Command "ollama" -ErrorAction SilentlyContinue
    if ($ollamaExe) {
        Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
        Start-Sleep -Seconds 3
        Write-Status "Ollama started." "Green"
    } else {
        Write-Status "WARNING: ollama not found in PATH. Continuing anyway." "Yellow"
    }
} else {
    Write-Status "Ollama already running." "Green"
}

# ── 2. Check if Aethyron is already running ───────────────────
$alreadyUp = $false
try {
    $resp = Invoke-WebRequest -Uri $HealthUrl -TimeoutSec 2 -ErrorAction Stop
    if ($resp.StatusCode -eq 200) { $alreadyUp = $true }
} catch {}

if ($alreadyUp) {
    Write-Status "Aethyron already running at $ServerUrl" "Green"
} else {
    # ── 3. Build a release binary if it doesn't exist ────────
    $releaseBin = Join-Path $AethyronRoot "target\release\aethyron-core.exe"
    if (-not (Test-Path $releaseBin)) {
        Write-Status "Building Aethyron release binary (first run — this takes a minute)..." "Yellow"
        $buildResult = & cargo build --release 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Status "Build failed. Falling back to debug binary." "Red"
            $releaseBin = Join-Path $AethyronRoot "target\debug\aethyron-core.exe"
        } else {
            Write-Status "Build complete." "Green"
        }
    }

    # Use release if available, otherwise debug
    $binaryPath = if (Test-Path $releaseBin) { $releaseBin } else {
        Join-Path $AethyronRoot "target\debug\aethyron-core.exe"
    }

    Write-Status "Starting Aethyron server..." "Yellow"

    # Start the server in a hidden window; it stays running after this script exits
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName               = $binaryPath
    $psi.WorkingDirectory       = $AethyronRoot
    $psi.WindowStyle            = [System.Diagnostics.ProcessWindowStyle]::Hidden
    $psi.CreateNoWindow         = $true
    $psi.UseShellExecute        = $false
    [System.Diagnostics.Process]::Start($psi) | Out-Null

    # ── 4. Wait for the server to become ready (up to 30 s) ──
    Write-Status "Waiting for server to come online..." "Yellow"
    $ready    = $false
    $deadline = (Get-Date).AddSeconds(30)

    while ((Get-Date) -lt $deadline) {
        try {
            $r = Invoke-WebRequest -Uri $HealthUrl -TimeoutSec 2 -ErrorAction Stop
            if ($r.StatusCode -eq 200) { $ready = $true; break }
        } catch {}
        Start-Sleep -Milliseconds 500
    }

    if (-not $ready) {
        Write-Status "Server did not respond in time. Opening browser anyway..." "Yellow"
    } else {
        Write-Status "Aethyron online." "Green"
    }
}

# ── 5. Open the UI in the default browser ────────────────────
Write-Status "Opening $ServerUrl ..." "Cyan"
Start-Process $ServerUrl

Write-Host ""
Write-Host "  Aethyron is running. You can close this window." -ForegroundColor DarkGray
Write-Host ""
Start-Sleep -Seconds 2
