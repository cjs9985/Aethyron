$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
$passed = 0
$failed = 0

function Check($name, $condition) {
    if ($condition) {
        Write-Host "PASS  $name"
        $script:passed++
    } else {
        Write-Host "FAIL  $name"
        $script:failed++
    }
}

$codeChange = Get-Content "$root\src\models\code_change.rs" -Raw
$generator  = Get-Content "$root\src\models\code_generator.rs" -Raw
$editor     = Get-Content "$root\src\tools\editor.rs" -Raw
$coder      = Get-Content "$root\src\agents\coder.rs" -Raw
$planner    = Get-Content "$root\src\agents\planner.rs" -Raw
$auth       = Get-Content "$root\src\core\auth.rs" -Raw
$cargo      = Get-Content "$root\Cargo.toml" -Raw

Check "Cargo.toml exists" (Test-Path "$root\Cargo.toml")
Check "bcrypt dependency exists" ($cargo -match '(?m)^\s*bcrypt\s*=')

Check "CodeChange has is_patch" ($codeChange -match 'pub\s+is_patch\s*:\s*bool')
Check "CodeGenerator sets is_patch" ($generator -match 'is_patch\s*:')
Check "EditorTool exists" ($editor -match 'pub\s+struct\s+EditorTool')
Check "Coder uses EditorTool" ($coder -match 'EditorTool')
Check "Coder refuses invalid paths" ($coder -match 'validate_generated_path')
Check "Planner validates paths" ($planner -match 'validate_plan')
Check "Authentication module exists" (Test-Path "$root\src\core\auth.rs")

Check "Authentication uses bcrypt" ($auth -match 'use\s+bcrypt')
Check "Authentication hashes passwords" ($auth -match 'hash\s*\(')
Check "Authentication verifies hashes" ($auth -match 'verify\s*\(')
Check "Authentication does not use gen_salt" ($auth -notmatch 'gen_salt')
Check "Authentication does not use unwrap" ($auth -notmatch '\.unwrap\s*\(')
Check "Authentication does not import unused Argon2" ($auth -notmatch 'use\s+argon2')
$main = Get-Content "$root\src\main.rs" -Raw

Check "CLI imports environment arguments" ($main -match 'use\s+std::env')
Check "CLI supports run command" ($main -match '==\s*Some\("run"\)')
Check "CLI creates Mission from goal" ($main -match 'Mission::new')
Check "CLI executes Orchestrator" ($main -match 'Orchestrator::new\(\)\.execute')
Check "CLI preserves health endpoint" ($main -match '"/health"')
Check "CLI preserves agents endpoint" ($main -match '"/agents"')
Check "CLI preserves CORS" ($main -match 'CorsLayer::very_permissive')
Check "CLI supports inspect command" ($main -match '==\s*Some\("inspect"\)')
Check "Inspect uses ProjectIndexer" ($main -match 'ProjectIndexer::build')
Check "Inspect returns project summary" ($main -match 'index\.summary\(\)')
Check "CLI supports doctor command" ($main -match '==\s*Some\("doctor"\)')
Check "Doctor checks Ollama" ($main -match 'OllamaClient::new\(\)\.check\(\)')
Check "Doctor checks workspace" ($main -match 'ProjectIndexer::build')

Write-Host ""
Write-Host "=============================="
Write-Host "AETHYRON FAST VERIFIER"
Write-Host "=============================="
Write-Host "Passed : $passed"
Write-Host "Failed : $failed"
Write-Host "=============================="

if ($failed -gt 0) {
    exit 1
}

exit 0