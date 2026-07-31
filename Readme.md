Aethyron is an experimental Rust-based autonomous coding agent. It is designed to inspect a Rust workspace, plan implementation tasks, generate code with a local language model, apply changes, run validation, review the generated output, attempt repairs, and store mission results for later context.

The project is currently an early prototype. The core loop exists, but the runtime interface, tests, documentation, path handling, and repository hygiene still need hardening before this should be treated as a production-ready developer tool.

## Current Status

Aethyron currently includes:

- A mission orchestrator that coordinates planning, coding, review, repair, and memory storage.
- A planner agent that asks a local model to convert a mission into executable Rust engineering tasks.
- A coder agent that asks a local model to generate Rust source changes.
- A reviewer agent that performs structural, security, compilation, and AI-assisted review checks.
- A repair engine that attempts to fix generated code after compiler or review failures.
- File, editor, terminal, and dispatcher tools for local project operations.
- A memory store for mission results.
- A sample mission workspace under `aethyron-core/workspace/missions`.

Important limitations:

- The main executable currently uses a hardcoded demo mission.
- Aethyron depends on a local Ollama server.
- The model name is currently hardcoded as `qwen2.5-coder:7b`.
- Generated code parsing is text-marker based and should be replaced or strengthened with stricter structured output validation.
- Compiler validation depends on process working directory behavior and should be made explicit.
- There is not yet a stable CLI, test suite, CI workflow, or release package.

## Repository Layout

```text
.
├── aethyron-core/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── aethyron_memory.json
│   ├── src/
│   │   ├── agents/
│   │   │   ├── coder.rs
│   │   │   ├── planner.rs
│   │   │   ├── reviewer.rs
│   │   │   └── tool_agent.rs
│   │   ├── core/
│   │   │   ├── context_builder.rs
│   │   │   ├── event_bus.rs
│   │   │   ├── events.rs
│   │   │   ├── orchestrator.rs
│   │   │   ├── project_index.rs
│   │   │   ├── project_indexer.rs
│   │   │   ├── repair_engine.rs
│   │   │   ├── rust_parser.rs
│   │   │   └── task_queue.rs
│   │   ├── domain/
│   │   ├── memory/
│   │   ├── models/
│   │   ├── tools/
│   │   └── main.rs
│   └── workspace/
│       └── missions/
└── README.md
```

## Requirements

- Rust toolchain with Cargo installed.
- Ollama running locally at `http://127.0.0.1:11434`.
- The `qwen2.5-coder:7b` model available in Ollama.

Install the model with:

```powershell
ollama pull qwen2.5-coder:7b
```

Start Ollama before running Aethyron.

## Running Locally

From the repository root:

```powershell
cd aethyron-core
cargo run
```

The current binary starts Aethyron, inspects the workspace, creates a hardcoded mission, and runs the agent pipeline.

At the moment, the mission is defined in `aethyron-core/src/main.rs`:

```rust
let mission = Mission::new(
    "Build a Rust authentication service"
);
```

Until a CLI is added, change this string manually when testing different missions.

## How Aethyron Works

1. `main.rs` starts the core runtime and creates a mission.
2. `Orchestrator` builds project context and publishes a mission-started event.
3. `PlannerAgent` asks the local model to create a JSON task plan.
4. Planner validation removes tasks that reference invalid files or attempt unsafe project initialization.
5. `CoderAgent` asks the local model to generate code changes for each task.
6. Generated output is parsed into a target path and file content.
7. Path validation attempts to prevent placeholder paths and unrelated writes.
8. `EditorTool` writes generated code into the mission workspace.
9. `Compiler` runs `cargo check`.
10. `ReviewerAgent` performs structural, security, compilation, and AI review checks.
11. `RepairEngine` attempts to repair failures.
12. `MemoryStore` saves a structured mission result.

## Development Priorities

The next work should focus on making Aethyron reliable and easy to run before adding more agent behavior.

### 1. Repository Hygiene

- Remove committed build artifacts under `aethyron-core/target`.
- Ensure `target/` is ignored by `.gitignore`.
- Keep source, fixtures, examples, and documentation in version control; keep generated build output out.

### 2. Command-Line Interface

Replace the hardcoded mission with a real CLI, for example:

```powershell
cargo run -- run "Add validation to the user model"
cargo run -- inspect
cargo run -- doctor
```

Useful options:

- `--workspace <path>`
- `--model <name>`
- `--ollama-url <url>`
- `--max-retries <n>`
- `--dry-run`

### 3. Explicit Workspace Handling

Make all project paths explicit:

- The project being modified.
- The mission workspace.
- The directory where `cargo check` runs.
- The files that the model is allowed to edit.

This is important because writing files to one directory and validating another can produce false confidence.

### 4. Safer Model Output Contracts

The current generator expects output shaped like:

```text
PATH: src/example.rs
-----BEGIN CODE-----
...
-----END CODE-----
```

That is acceptable for a prototype, but long term Aethyron should prefer stricter structured output, schema validation, and clear recovery behavior when the model returns malformed content.

### 5. Tests

Recommended first tests:

- Planner JSON parsing and validation.
- Generated file parsing.
- Forbidden placeholder path rejection.
- Editor path resolution.
- Compiler working-directory behavior.
- Reviewer security checks.
- Repair loop behavior with mocked model responses.

### 6. Continuous Integration

Add a GitHub Actions workflow that runs:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo check
```

### 7. Configuration

Move hardcoded runtime settings into configuration:

- Ollama endpoint.
- Model name.
- Workspace path.
- Maximum repair attempts.
- Allowed file roots.
- Memory file path.

## Known Risks

- Local model responses may be malformed, incomplete, or unsafe.
- Text-based code extraction can fail if the model ignores formatting rules.
- Security checks are pattern-based and incomplete.
- Compilation checks need explicit working-directory control.
- The current prototype can overwrite files in the mission workspace.
- No automated tests currently protect behavior from regressions.

## Suggested Definition of Done

Aethyron should be considered ready for an initial public prototype when:

- A new user can follow the README and run a mission successfully.
- `target/` and other generated artifacts are removed from version control.
- The mission is supplied through a CLI.
- The model, endpoint, workspace, and retry settings are configurable.
- Generated edits are validated against an explicit allowlist.
- `cargo test`, `cargo check`, `cargo fmt`, and `cargo clippy` pass in CI.
- Core parsing, validation, editor, compiler, and reviewer behavior has automated test coverage.
