# Aethyron

Aethyron is an experimental Rust-based autonomous coding agent designed to inspect a Rust workspace, plan engineering tasks, generate code through a local language model, apply controlled changes, validate the result, review generated output, attempt repairs, and store mission results for later context.

Aethyron is currently an early working prototype. The core autonomous mission loop, CLI runtime, project inspection, local Ollama integration, API endpoints, path validation, editing tools, authentication support, repository hygiene, and fast structural verification are implemented.

---

## Current Status

Aethyron currently provides:

- Mission-based autonomous coding workflow.
- Planner agent for converting goals into implementation tasks.
- Coder agent for generating source changes.
- Reviewer agent for reviewing generated changes.
- Repair engine for attempting corrections after failures.
- Local Ollama model integration.
- Project indexing and project-context construction.
- Controlled file editing through `EditorTool`.
- Filesystem inspection and project discovery.
- Task queue management.
- Event publishing.
- Persistent mission-result memory.
- Rust source inspection.
- CLI commands for:
  - `run`
  - `inspect`
  - `doctor`
- HTTP API endpoints:
  - `/health`
  - `/agents`
- CORS support for the API.
- Password hashing and verification using bcrypt.
- Fast PowerShell structural verification.
- Repository build-artifact protection through `.gitignore`.

The current fast verifier has reached:

```text
Passed : 28
Failed : 0
```

                         ┌─────────────────────┐
                         │       User          │
                         │  Mission / Command  │
                         └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │      CLI / API      │
                         │  run / inspect /    │
                         │       doctor        │
                         └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │    Orchestrator     │
                         │                     │
                         │ Mission lifecycle   │
                         └──────────┬──────────┘
                                    │
                  ┌─────────────────┼─────────────────┐
                  │                 │                 │
                  ▼                 ▼                 ▼
          ┌──────────────┐ ┌───────────────┐ ┌──────────────┐
          │   Context    │ │    Planner    │ │    Memory    │
          │   Builder    │ │     Agent     │ │    Store     │
          └──────┬───────┘ └───────┬───────┘ └──────────────┘
                 │                 │
                 ▼                 ▼
          ┌──────────────┐   ┌───────────────┐
          │    Project   │   │    Task Queue  │
          │    Indexer   │   └───────┬───────┘
          └──────────────┘           │
                                     ▼
                              ┌───────────────┐
                              │  Coder Agent  │
                              └───────┬───────┘
                                      │
                                      ▼
                              ┌───────────────┐
                              │   Tool Layer  │
                              │               │
                              │ Editor        │
                              │ Filesystem    │
                              │ Terminal      │
                              │ Dispatcher    │
                              └───────┬───────┘
                                      │
                                      ▼
                              ┌───────────────┐
                              │   Validation  │
                              │ cargo check   │
                              └───────┬───────┘
                                      │
                                      ▼
                              ┌───────────────┐
                              │ Reviewer Agent│
                              └───────┬───────┘
                                      │
                              failure │ success
                                      │
                         ┌────────────┴────────────┐
                         │                         │
                         ▼                         ▼
                 ┌───────────────┐        ┌───────────────┐
                 │ Repair Engine │        │ Mission Result│
                 └───────┬───────┘        │    Memory     │
                         │                └───────────────┘
                         └──────► retry

Mission Goal
│
▼
Orchestrator
│
▼
Build Project Context
│
├── Cargo.toml
├── Project files
├── Project index
└── Previous memory
│
▼
Planner Agent
│
▼
Validated Task Queue
│
▼
Coder Agent
│
▼
Generated Code
│
▼
Path Validation
│
▼
EditorTool
│
▼
Compiler / Validation
│
▼
Reviewer Agent
│
├── Structural review
├── Security review
├── Compilation review
└── AI-assisted review
│
▼
Repair Engine
│
▼
Mission Result
│
▼
Memory Store

Aethyron/
│
├── README.md
│
└── aethyron-core/
│
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── verifier.ps1
├── aethyron_memory.json
│
├── src/
│ │
│ ├── main.rs
│ │
│ ├── agents/
│ │ ├── mod.rs
│ │ ├── planner.rs
│ │ ├── coder.rs
│ │ ├── reviewer.rs
│ │ └── tool_agent.rs
│ │
│ ├── core/
│ │ ├── mod.rs
│ │ ├── orchestrator.rs
│ │ ├── context_builder.rs
│ │ ├── event_bus.rs
│ │ ├── events.rs
│ │ ├── project_index.rs
│ │ ├── project_indexer.rs
│ │ ├── repair_engine.rs
│ │ ├── rust_parser.rs
│ │ └── task_queue.rs
│ │
│ ├── memory/
│ │ ├── mod.rs
│ │ └── store.rs
│ │
│ ├── models/
│ │ ├── mod.rs
│ │ ├── code_change.rs
│ │ ├── code_generator.rs
│ │ ├── coder_result.rs
│ │ ├── compiler.rs
│ │ ├── file_operation.rs
│ │ ├── mission_result.rs
│ │ ├── ollama.rs
│ │ ├── plan.rs
│ │ ├── project_context.rs
│ │ ├── review_report.rs
│ │ ├── tool_request.rs
│ │ └── tool_result.rs
│ │
│ ├── tools/
│ │ ├── mod.rs
│ │ ├── dispatcher.rs
│ │ ├── editor.rs
│ │ ├── filesystem.rs
│ │ └── terminal.rs
│ │
│ └── domain/
│
├── tests/
│ └── test_passwords.rs
│
└── workspace/
└── missions/
