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

# ---------------------------------------------------------------
# Load source files
# ---------------------------------------------------------------

$codeChange = Get-Content "$root\src\models\code_change.rs" -Raw
$generator  = Get-Content "$root\src\models\code_generator.rs" -Raw
$editor     = Get-Content "$root\src\tools\editor.rs" -Raw
$coder      = Get-Content "$root\src\agents\coder.rs" -Raw
$planner    = Get-Content "$root\src\agents\planner.rs" -Raw
$auth       = Get-Content "$root\src\core\auth.rs" -Raw
$main       = Get-Content "$root\src\main.rs" -Raw
$cargo      = Get-Content "$root\Cargo.toml" -Raw

# New integration files
$a2aExecutor = Get-Content "$root\src\a2a_executor.rs" -Raw
$mcp         = Get-Content "$root\src\mcp\mod.rs" -Raw

# ---------------------------------------------------------------
# Core project checks
# ---------------------------------------------------------------

Check "Cargo.toml exists" `
    (Test-Path "$root\Cargo.toml")

Check "bcrypt dependency exists" `
    ($cargo -match '(?m)^\s*bcrypt\s*=')

Check "CodeChange has is_patch" `
    ($codeChange -match 'pub\s+is_patch\s*:\s*bool')

Check "CodeGenerator sets is_patch" `
    ($generator -match 'is_patch\s*:')

Check "EditorTool exists" `
    ($editor -match 'pub\s+struct\s+EditorTool')

Check "Coder uses EditorTool" `
    ($coder -match 'EditorTool')

Check "Coder refuses invalid paths" `
    ($coder -match 'validate_generated_path')

Check "Planner validates paths" `
    ($planner -match 'validate_plan')

# ---------------------------------------------------------------
# Authentication checks
# ---------------------------------------------------------------

Check "Authentication module exists" `
    (Test-Path "$root\src\core\auth.rs")

Check "Authentication uses bcrypt" `
    ($auth -match 'use\s+bcrypt')

Check "Authentication hashes passwords" `
    ($auth -match 'hash\s*\(')

Check "Authentication verifies hashes" `
    ($auth -match 'verify\s*\(')

Check "Authentication does not use gen_salt" `
    ($auth -notmatch 'gen_salt')

Check "Authentication does not use unwrap" `
    ($auth -notmatch '\.unwrap\s*\(')

Check "Authentication does not import unused Argon2" `
    ($auth -notmatch 'use\s+argon2')

# ---------------------------------------------------------------
# CLI checks
# ---------------------------------------------------------------

Check "CLI imports environment arguments" `
    ($main -match 'use\s+std::env')

Check "CLI supports run command" `
    ($main -match '==\s*Some\("run"\)')

Check "CLI creates Mission from goal" `
    ($main -match 'Mission::new')

Check "CLI executes Orchestrator" `
    ($main -match 'Orchestrator::new\(\)\.execute')

Check "CLI preserves health endpoint" `
    ($main -match '"/health"')

Check "CLI preserves agents endpoint" `
    ($main -match '"/agents"')

Check "CLI preserves CORS" `
    ($main -match 'CorsLayer::very_permissive')

Check "CLI supports inspect command" `
    ($main -match '==\s*Some\("inspect"\)')

Check "Inspect uses ProjectIndexer" `
    ($main -match 'ProjectIndexer::build')

Check "Inspect returns project summary" `
    ($main -match 'index\.summary\(\)')

Check "CLI supports doctor command" `
    ($main -match '==\s*Some\("doctor"\)')

Check "Doctor checks Ollama" `
    ($main -match 'OllamaClient::new\(\)\.check\(\)')

Check "Doctor checks workspace" `
    ($main -match 'ProjectIndexer::build')

# ---------------------------------------------------------------
# MCP integration checks
# ---------------------------------------------------------------

Check "MCP module exists" `
    (Test-Path "$root\src\mcp\mod.rs")

Check "MCP uses RMCP" `
    ($mcp -match 'use\s+rmcp')

Check "MCP tool router exists" `
    ($mcp -match '#\[tool_router\]')

Check "MCP tool exists" `
    ($mcp -match '#\[tool\(')

Check "MCP server type exists" `
    ($mcp -match 'pub\s+struct\s+AethyronMcp')

Check "MCP server has constructor" `
    ($mcp -match 'pub\s+fn\s+new')

Check "Main exposes MCP command" `
    ($main -match 'Some\("mcp"\)')

Check "Main starts MCP server" `
    ($main -match 'AethyronMcp::new')

Check "MCP uses stdio transport" `
    ($main -match 'rmcp::transport::stdio')

Check "MCP ServiceExt imported" `
    ($main -match 'use\s+rmcp::ServiceExt')

# ---------------------------------------------------------------
# A2A integration checks
# ---------------------------------------------------------------

Check "A2A executor module exists" `
    (Test-Path "$root\src\a2a_executor.rs")

Check "A2A executor type exists" `
    ($a2aExecutor -match 'pub\s+struct\s+AethyronA2AExecutor')

Check "A2A executor implements AgentExecutor" `
    ($a2aExecutor -match 'impl\s+AgentExecutor\s+for\s+AethyronA2AExecutor')

Check "A2A executor creates Mission" `
    ($a2aExecutor -match 'Mission::new')

Check "A2A executor invokes Orchestrator" `
    ($a2aExecutor -match 'Orchestrator::new\(\)\.execute')

Check "A2A executor returns completed state" `
    ($a2aExecutor -match 'TaskState::Completed')

Check "A2A executor returns failed state" `
    ($a2aExecutor -match 'TaskState::Failed')

Check "Main imports A2A server" `
    ($main -match 'a2a_server')

Check "Main creates A2A executor" `
    ($main -match 'AethyronA2AExecutor::new')

Check "Main creates A2A request handler" `
    ($main -match 'DefaultRequestHandler::new')

Check "Main exposes A2A JSON-RPC router" `
    ($main -match 'jsonrpc_router')

Check "Main mounts A2A endpoint" `
    ($main -match '\.nest\("/a2a"')

Check "Main defines Agent Card" `
    ($main -match 'AgentCard')

Check "Agent Card advertises A2A endpoint" `
    ($main -match 'http://127\.0\.0\.1:3000/a2a')

Check "Agent Card defines Aethyron mission skill" `
    ($main -match 'aethyron-mission')

Check "Agent Card router is mounted" `
    ($main -match 'agent_card_router')

# ---------------------------------------------------------------
# Final report
# ---------------------------------------------------------------

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
```

