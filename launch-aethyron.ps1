# ============================================================
#  Aethyron Launcher
#  Builds the UI and server, starts Ollama if needed,
#  starts Aethyron, waits for health, then opens the UI.
# ============================================================

$ProjectRoot = $PSScriptRoot
$AethyronRoot = Join-Path $ProjectRoot "aethyron-core"
$UiRoot       = Join-Path $ProjectRoot "aethyron-ui"

$ServerUrl = "http://127.0.0.1:3000"
$HealthUrl = "$ServerUrl/health"

function Write-Status($msg, $color = "Cyan") {
    Write-Host "  $msg" -ForegroundColor $color
}

Clear-Host

Write-Host ""
Write-Host "  ╔══════════════════════════════════════════╗" -ForegroundColor DarkBlue
Write-Host "  ║           AETHYRON  LAUNCHER             ║" -ForegroundColor Blue
Write-Host "  ╚══════════════════════════════════════════╝" -ForegroundColor DarkBlue
Write-Host ""

# ------------------------------------------------------------
# 1. Validate project structure
# ------------------------------------------------------------

if (-not (Test-Path $AethyronRoot)) {
    Write-Status "Aethyron core directory not found: $AethyronRoot" "Red"
    Read-Host "Press Enter to exit"
    exit 1
}

if (-not (Test-Path $UiRoot)) {
    Write-Status "Aethyron UI directory not found: $UiRoot" "Red"
    Read-Host "Press Enter to exit"
    exit 1
}

# ------------------------------------------------------------
# 2. Ensure Ollama is running
# ------------------------------------------------------------

$ollamaRunning = Get-Process -Name "ollama" -ErrorAction SilentlyContinue

if (-not $ollamaRunning) {
    Write-Status "Starting Ollama..." "Yellow"

    $ollamaExe = Get-Command "ollama" -ErrorAction SilentlyContinue

    if ($ollamaExe) {
        Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden

        $ollamaReady = $false
        $ollamaDeadline = (Get-Date).AddSeconds(15)

        while ((Get-Date) -lt $ollamaDeadline) {
            try {
                $response = Invoke-WebRequest `
                    -Uri "http://127.0.0.1:11434/api/tags" `
                    -TimeoutSec 2 `
                    -ErrorAction Stop

                if ($response.StatusCode -eq 200) {
                    $ollamaReady = $true
                    break
                }
            } catch {}

            Start-Sleep -Milliseconds 500
        }

        if ($ollamaReady) {
            Write-Status "Ollama is ready." "Green"
        } else {
            Write-Status "WARNING: Ollama did not become ready in time." "Yellow"
        }
    } else {
        Write-Status "WARNING: ollama not found in PATH. Continuing anyway." "Yellow"
    }
} else {
    Write-Status "Ollama already running." "Green"
}

# ------------------------------------------------------------
# 3. Check whether Aethyron is already running
# ------------------------------------------------------------

$alreadyUp = $false

try {
    $resp = Invoke-WebRequest `
        -Uri $HealthUrl `
        -TimeoutSec 2 `
        -ErrorAction Stop

    if ($resp.StatusCode -eq 200) {
        $alreadyUp = $true
    }
} catch {}

if ($alreadyUp) {

    Write-Status "Aethyron already running at $ServerUrl" "Green"

} else {

    # --------------------------------------------------------
    # 4. Build the React UI
    # --------------------------------------------------------

    Write-Status "Building Aethyron UI..." "Yellow"

    Push-Location $UiRoot

    try {
        & npm run build

        if ($LASTEXITCODE -ne 0) {
            Write-Status "UI build failed." "Red"
            Pop-Location
            Read-Host "Press Enter to exit"
            exit 1
        }
    }
    finally {
        Pop-Location
    }

    Write-Status "UI build complete." "Green"

    # --------------------------------------------------------
    # 5. Build the Rust server
    # --------------------------------------------------------

    Write-Status "Building Aethyron release binary..." "Yellow"

    Push-Location $AethyronRoot

    try {
        & cargo build --release

        if ($LASTEXITCODE -ne 0) {
            Write-Status "Rust build failed." "Red"
            Pop-Location
            Read-Host "Press Enter to exit"
            exit 1
        }
    }
    finally {
        Pop-Location
    }

    Write-Status "Rust build complete." "Green"

    $binaryPath = Join-Path `
        $AethyronRoot `
        "target\release\aethyron-core.exe"

    if (-not (Test-Path $binaryPath)) {
        Write-Status "Release binary was not produced." "Red"
        Read-Host "Press Enter to exit"
        exit 1
    }

    # --------------------------------------------------------
    # 6. Start Aethyron server
    # --------------------------------------------------------

    Write-Status "Starting Aethyron server..." "Yellow"

    $psi = New-Object System.Diagnostics.ProcessStartInfo

    $psi.FileName         = $binaryPath
    $psi.WorkingDirectory = $AethyronRoot
    $psi.WindowStyle      = [System.Diagnostics.ProcessWindowStyle]::Hidden
    $psi.CreateNoWindow   = $true
    $psi.UseShellExecute  = $false

    [System.Diagnostics.Process]::Start($psi) | Out-Null

    # --------------------------------------------------------
    # 7. Wait for health endpoint
    # --------------------------------------------------------

    Write-Status "Waiting for Aethyron to come online..." "Yellow"

    $ready = $false
    $deadline = (Get-Date).AddSeconds(30)

    while ((Get-Date) -lt $deadline) {

        try {
            $r = Invoke-WebRequest `
                -Uri $HealthUrl `
                -TimeoutSec 2 `
                -ErrorAction Stop

            if ($r.StatusCode -eq 200) {
                $ready = $true
                break
            }
        } catch {}

        Start-Sleep -Milliseconds 500
    }

    if (-not $ready) {
        Write-Status "Aethyron did not respond within 30 seconds." "Red"
        Read-Host "Press Enter to exit"
        exit 1
    }

    Write-Status "Aethyron online." "Green"
}

# ------------------------------------------------------------
# 8. Open the UI
# ------------------------------------------------------------

Write-Status "Opening $ServerUrl ..." "Cyan"

Start-Process $ServerUrl

Write-Host ""
Write-Host "  Aethyron is running. You can close this window." `
    -ForegroundColor DarkGray
Write-Host ""

Start-Sleep -Seconds 2
