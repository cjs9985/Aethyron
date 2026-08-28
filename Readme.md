# Aethyron

Aethyron is a Rust-based autonomous coding agent designed to help software engineers inspect, plan, modify, validate, review, and repair Rust projects through an orchestrated multi-agent workflow.

Aethyron combines local language-model inference with controlled project tooling, persistent mission memory, automated validation, A2A interoperability, and MCP tool integration.

The system is designed around a simple objective:

> Give Aethyron an engineering goal and allow it to inspect the project, plan the work, generate changes, validate those changes, review the result, repair failures when possible, and return a structured mission result.

---

## Project Status

Aethyron is an **operational working system**, not merely a static prototype.

The current implementation includes:

* Autonomous mission execution.
* Planner agent.
* Coder agent.
* Reviewer agent.
* Tool agent.
* Orchestrator-driven mission lifecycle.
* Project indexing and project-context construction.
* Local Ollama language-model integration.
* Controlled source-file editing.
* Filesystem inspection.
* Terminal and Cargo operations.
* Compilation validation.
* Automated repair workflow.
* Persistent mission-result memory.
* Rust source parsing and inspection.
* Password hashing and verification with bcrypt.
* CLI operation.
* HTTP API.
* A2A agent interoperability.
* A2A Agent Card discovery.
* JSON-RPC mission execution.
* MCP server integration.
* MCP tool routing.
* Path-aware code changes.
* Mission timing and structured mission results.
* Automated fast verification.

The current Aethyron Fast Verifier reports:

```text
==============================
AETHYRON FAST VERIFIER
==============================
Passed : 54
Failed : 0
==============================
```

A2A mission execution has also been exercised successfully through the HTTP JSON-RPC interface, returning completed task states and structured mission results.

---

# Architecture

```text
                         ┌─────────────────────┐
                         │        User         │
                         │  Engineering Goal   │
                         └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │      CLI / API      │
                         │ run / inspect /     │
                         │ doctor / HTTP API   │
                         └──────────┬──────────┘
                                    │
                    ┌───────────────┴────────────────┐
                    │                                │
                    ▼                                ▼
          ┌──────────────────┐             ┌──────────────────┐
          │   A2A Interface  │             │   MCP Interface  │
          │   Agent Card     │             │   MCP Server     │
          │   JSON-RPC       │             │   Tool Routing   │
          └────────┬─────────┘             └────────┬─────────┘
                   │                                │
                   └────────────────┬───────────────┘
                                    ▼
                         ┌─────────────────────┐
                         │    Orchestrator     │
                         │                     │
                         │ Mission lifecycle   │
                         └──────────┬──────────┘
                                    │
          ┌─────────────────────────┼─────────────────────────┐
          │                         │                         │
          ▼                         ▼                         ▼
 ┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
 │ Context Builder │      │  Planner Agent  │      │  Memory Store   │
 └────────┬────────┘      └────────┬────────┘      └─────────────────┘
          │                        │
          ▼                        ▼
 ┌─────────────────┐      ┌─────────────────┐
 │ Project Indexer │      │   Task Queue    │
 └─────────────────┘      └────────┬────────┘
                                   │
                                   ▼
                          ┌─────────────────┐
                          │   Coder Agent   │
                          └────────┬────────┘
                                   │
                                   ▼
                          ┌─────────────────┐
                          │    Tool Layer   │
                          │                 │
                          │ Editor          │
                          │ Filesystem      │
                          │ Terminal        │
                          │ Dispatcher      │
                          └────────┬────────┘
                                   │
                                   ▼
                          ┌─────────────────┐
                          │    Compiler     │
                          │   Validation    │
                          └────────┬────────┘
                                   │
                                   ▼
                          ┌─────────────────┐
                          │ Reviewer Agent  │
                          └────────┬────────┘
                                   │
                         ┌─────────┴─────────┐
                         │                   │
                         ▼                   ▼
                ┌─────────────────┐ ┌─────────────────┐
                │  Repair Engine  │ │ Mission Result │
                └────────┬────────┘ └────────┬────────┘
                         │                   │
                         └─────── retry      ▼
                                  Memory Store
```

---

# Autonomous Mission Workflow

Aethyron's core workflow is:

```text
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
     └── Previous mission memory
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
Generated Code Changes
     │
     ▼
Path Validation
     │
     ▼
Editor / Tool Layer
     │
     ▼
Cargo / Compiler Validation
     │
     ▼
Reviewer Agent
     │
     ├── Structural review
     ├── Security review
     ├── Compilation review
     └── AI-assisted review
     │
     ├───────────────┐
     │ failure       │ success
     ▼               ▼
Repair Engine    Mission Result
     │               │
     └── retry       ▼
                Memory Store
```

This separates planning, implementation, validation, review, and repair instead of treating code generation as a single uncontrolled operation.

---

# Agents

## Planner Agent

Converts an engineering objective into structured implementation tasks.

Responsibilities include:

* Understanding the mission goal.
* Examining available project context.
* Breaking the goal into tasks.
* Producing a structured implementation plan.

## Coder Agent

Generates implementation changes based on planned tasks and available project context.

Responsibilities include:

* Producing source changes.
* Identifying target files.
* Working with the project's existing architecture.
* Producing changes suitable for controlled application.

## Reviewer Agent

Reviews generated work before the mission is considered successful.

Review includes:

* Structural correctness.
* Compilation status.
* Security considerations.
* Implementation quality.
* AI-assisted review where available.

## Tool Agent

Coordinates project operations through the controlled tool layer.

---

# Tool Layer

Aethyron provides controlled project operations through dedicated tools.

### Editor

Provides controlled file modification.

### Filesystem

Provides project inspection and filesystem operations.

### Terminal

Provides command execution required for project operations.

### Dispatcher

Maps structured tool requests to their corresponding implementations.

Supported operations include:

* Project inspection.
* File reading.
* File writing.
* File appending.
* Directory creation.
* Cargo checking.
* Cargo formatting.
* Git status.
* Git staging.
* Git commits.

---

# Validation and Repair

Aethyron does not stop after generating code.

The generated changes can pass through a validation and review cycle:

```text
Generate
   │
   ▼
Apply
   │
   ▼
Compile
   │
   ├── failure ──► Repair
   │                 │
   │                 └──► Validate again
   │
   ▼
Review
   │
   ├── failure ──► Repair
   │
   ▼
Mission Result
```

This provides a foundation for autonomous coding tasks where generated changes must be tested rather than simply emitted.

---

# Local AI

Aethyron supports local language-model execution through Ollama.

The model layer is responsible for communicating with the local model service and supplying the agents with project context and task information.

The architecture is intentionally compatible with local inference so that coding missions can be executed without requiring every operation to depend on a hosted model provider.

---

# A2A Integration

Aethyron exposes an Agent-to-Agent interface for interoperability with external agent systems.

The implementation provides:

* A2A server integration.
* Agent Card discovery.
* JSON-RPC communication.
* Remote mission execution.
* Structured task results.
* Mission execution through the existing Aethyron orchestration layer.

The Agent Card is available through:

```text
/.well-known/agent-card.json
```

The A2A interface is exposed through:

```text
/a2a
```

A successful A2A request returns a structured task result containing the task state.

Example successful task state:

```json
{
  "state": "TASK_STATE_COMPLETED"
}
```

The A2A layer therefore acts as an interoperability boundary around the existing autonomous coding engine rather than replacing the core orchestration system.

---

# MCP Integration

Aethyron also includes a Model Context Protocol server implementation.

MCP provides a standardized interface through which compatible clients can discover and invoke Aethyron tools.

The MCP implementation is built with the Rust `rmcp` library and provides tool routing into the Aethyron environment.

The MCP layer is designed to complement the A2A interface:

```text
External Agent
      │
      ▼
     A2A
      │
      ▼
 Aethyron Mission
      │
      ▼
 Orchestrator
      │
      ▼
 Tool Layer
      ▲
      │
     MCP
      ▲
      │
MCP Client
```

A2A provides agent-to-agent communication while MCP provides standardized model/tool interaction.

---

# HTTP API

The HTTP service currently exposes:

### Health

```text
GET /health
```

Returns:

```text
Aethyron API is running
```

### Agents

```text
GET /agents
```

Returns the available Aethyron agent roles.

### Agent Card

```text
GET /.well-known/agent-card.json
```

Returns the Aethyron Agent Card used for A2A discovery.

### A2A

```text
POST /a2a
```

Accepts A2A JSON-RPC requests.

---

# CLI

The core application supports several command-line operations.

## Run a Mission

```powershell
cargo run -- run "your engineering mission"
```

Example:

```powershell
cargo run -- run "Add authentication validation to the existing application"
```

## Inspect the Project

```powershell
cargo run -- inspect
```

This indexes the current project and reports project information.

## Run the Doctor

```powershell
cargo run -- doctor
```

The doctor performs health checks including:

* Cargo project availability.
* Source directory availability.
* Project indexing.
* Local Ollama availability.
* Required model availability.

## Start the API

```powershell
cargo run
```

The HTTP service runs on:

```text
http://127.0.0.1:3000
```

---

# Example Mission Result

A successful mission produces structured information such as:

```text
========== Mission Summary ==========

Tasks Completed : 3
Files Changed   : 3
Repairs         : 0
Success         : true

=====================================
```

Mission results are stored so that subsequent missions can make use of previous context.

---

# Security and Control

Aethyron is designed around controlled modification rather than unrestricted source generation.

The implementation includes:

* Validated file paths.
* Controlled editor operations.
* Project-aware file handling.
* Compilation validation.
* Review before mission completion.
* Password hashing with bcrypt.
* Repository build-artifact protection.
* Explicit tool dispatching.

The system is intended to make autonomous code modification auditable and constrained by the project's tool layer.

---

# Repository Structure

```text
Aethyron/
│
├── Readme.md
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
    │   │
    │   ├── main.rs
    │   ├── a2a_executor.rs
    │   ├── editor_tool.rs
    │   ├── password.rs
    │   │
    │   ├── agents/
    │   │   ├── mod.rs
    │   │   ├── planner.rs
    │   │   ├── coder.rs
    │   │   ├── reviewer.rs
    │   │   └── tool_agent.rs
    │   │
    │   ├── core/
    │   │   ├── mod.rs
    │   │   ├── orchestrator.rs
    │   │   ├── context_builder.rs
    │   │   ├── event_bus.rs
    │   │   ├── events.rs
    │   │   ├── project_index.rs
    │   │   ├── project_indexer.rs
    │   │   ├── repair_engine.rs
    │   │   ├── rust_parser.rs
    │   │   └── task_queue.rs
    │   │
    │   ├── domain/
    │   │   ├── mod.rs
    │   │   ├── agent_result.rs
    │   │   ├── mission.rs
    │   │   └── task.rs
    │   │
    │   ├── memory/
    │   │   ├── mod.rs
    │   │   └── store.rs
    │   │
    │   ├── models/
    │   │   ├── mod.rs
    │   │   ├── code_change.rs
    │   │   ├── code_generator.rs
    │   │   ├── coder_result.rs
    │   │   ├── compiler.rs
    │   │   ├── file_operation.rs
    │   │   ├── fix_request.rs
    │   │   ├── mission_result.rs
    │   │   ├── ollama.rs
    │   │   ├── plan.rs
    │   │   ├── project_context.rs
    │   │   ├── review_report.rs
    │   │   ├── tool_request.rs
    │   │   └── tool_result.rs
    │   │
    │   ├── mcp/
    │   │   └── mod.rs
    │   │
    │   └── tools/
    │       ├── mod.rs
    │       ├── dispatcher.rs
    │       ├── editor.rs
    │       ├── filesystem.rs
    │       └── terminal.rs
    │
    ├── tests/
    │   └── test_passwords.rs
    │
    └── workspace/
        └── missions/
```

---

# Verification

Aethyron includes a fast PowerShell verification suite:

```powershell
.\verifier.ps1
```

Current verification result:

```text
==============================
AETHYRON FAST VERIFIER
==============================
Passed : 54
Failed : 0
==============================
```

The verifier provides a repeatable structural validation layer for the repository.

The project also undergoes normal Rust compilation through Cargo.

---

# Technology Stack

| Component           | Technology            |
| ------------------- | --------------------- |
| Core language       | Rust                  |
| Async runtime       | Tokio                 |
| HTTP framework      | Axum                  |
| Local AI            | Ollama                |
| Agent communication | A2A                   |
| Tool protocol       | MCP                   |
| MCP implementation  | rmcp                  |
| Authentication      | bcrypt                |
| Serialization       | Serde / JSON          |
| Validation          | Cargo / Rust compiler |
| Verification        | PowerShell            |
| Source control      | Git                   |

---

# Design Goals

Aethyron is built around five primary design goals:

### 1. Autonomous

A mission should progress through planning, implementation, validation, review, and repair with minimal manual intervention.

### 2. Controlled

Generated changes must pass through explicit project tools and validation rather than being written without safeguards.

### 3. Local

Local model execution through Ollama allows the core coding workflow to operate with locally hosted AI infrastructure.

### 4. Interoperable

A2A and MCP allow Aethyron to participate in broader agent and tool ecosystems.

### 5. Persistent

Mission results are stored so that knowledge from previous work can contribute to future missions.

---

# Development Philosophy

Aethyron is intended to demonstrate how autonomous software engineering can be implemented as an orchestrated system rather than as a single prompt-to-code operation.

The architecture separates:

```text
Planning
   ↓
Context
   ↓
Generation
   ↓
Tool Execution
   ↓
Validation
   ↓
Review
   ↓
Repair
   ↓
Persistence
```

This separation makes the system easier to test, extend, observe, and integrate with other agent systems.

---

# Current Milestone

The current repository milestone represents an integrated autonomous coding agent with:

* Multi-agent orchestration.
* Local model execution.
* Project-aware coding.
* Controlled editing.
* Automated validation.
* Review and repair.
* Persistent mission memory.
* HTTP API.
* A2A interoperability.
* MCP integration.
* Agent Card discovery.
* Automated verification.

The repository's current fast verification baseline is:

```text
54 passed
0 failed
```

Aethyron is therefore positioned as a functional autonomous coding-agent platform suitable for continued development, integration testing, demonstration, and portfolio presentation.
