mod agents;
mod core;
mod mcp;
mod memory;
mod models;
mod tools;
mod a2a_executor;

use crate::memory::store::MemoryStore;
use crate::models::conversation::{ConversationHistory, ConversationTurn};

use a2a::*;
use a2a_server::{
    agent_card::{agent_card_router, StaticAgentCard},
    handler::DefaultRequestHandler,
    jsonrpc::jsonrpc_router,
    task_store::InMemoryTaskStore,
};

use crate::core::context_builder::ContextBuilder;
use crate::core::orchestrator::{Mission, Orchestrator};
use crate::core::project_indexer::ProjectIndexer;
use crate::mcp::AethyronMcp;
use crate::models::ollama::OllamaClient;

use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::{self, Stream};
use rmcp::ServiceExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

// ---------------------------------------------------------------------------
// Shared chat state — broadcast channel for SSE progress events
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
struct ChatEvent {
    /// One of: "progress" | "result" | "error"
    kind: String,
    message: String,
}

#[derive(Clone)]
struct ChatState {
    /// Sender half of the broadcast channel.  Every SSE subscriber
    /// receives a clone of the receiver end.
    tx: Arc<broadcast::Sender<ChatEvent>>,
}

impl ChatState {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self { tx: Arc::new(tx) }
    }

    fn send(&self, kind: &str, message: impl Into<String>) {
        // Ignore errors — no subscribers is fine.
        let _ = self.tx.send(ChatEvent {
            kind: kind.to_string(),
            message: message.into(),
        });
    }
}

async fn health() -> &'static str {
    "Aethyron API is running"
}

async fn agents() -> axum::Json<Vec<serde_json::Value>> {
    axum::Json(vec![
        serde_json::json!({
            "name": "PLANNER",
            "role": "Plans and coordinates missions"
        }),
        serde_json::json!({
            "name": "TOOLS",
            "role": "Executes tools and project operations"
        }),
        serde_json::json!({
            "name": "MEMORY",
            "role": "Stores and retrieves mission context"
        }),
    ])
}

#[derive(Deserialize)]
struct MissionRequest {
    goal: String,
}

// ---------------------------------------------------------------------------
// Chat request / response types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

// ---------------------------------------------------------------------------
// POST /chat
// Runs a mission with conversation context and returns the structured result.
// Also broadcasts SSE progress events so connected stream subscribers see
// live updates while the mission executes.
// ---------------------------------------------------------------------------

async fn post_chat(
    State(chat_state): State<ChatState>,
    Json(payload): Json<ChatRequest>,
) -> axum::Json<serde_json::Value> {
    let message = payload.message.trim().to_string();

    if message.is_empty() {
        return axum::Json(serde_json::json!({
            "error": "Message cannot be empty."
        }));
    }

    // Load prior conversation turns (up to 20) for context.
    let prior_turns = MemoryStore::load_conversation(20).unwrap_or_default();
    let mut history = ConversationHistory::new();
    for turn in prior_turns {
        history.add(turn.role, turn.content);
    }
    history.add("user", &message);

    // Persist the user turn.
    let user_turn = ConversationTurn {
        role: "user".to_string(),
        content: message.clone(),
    };
    if let Err(e) = MemoryStore::save_turn(&user_turn) {
        eprintln!("⚠️  Could not persist user turn: {}", e);
    }

    // ------------------------------------------------------------------
    // Intent detection: is this a conversational message or a mission?
    //
    // We classify as "conversational" if the message:
    //   • starts with a question word or greeting, OR
    //   • ends with '?', OR
    //   • is short (≤ 8 words) and contains no coding action verb
    // Everything else is treated as an engineering mission.
    // ------------------------------------------------------------------
    let lower = message.to_lowercase();
    let word_count = message.split_whitespace().count();

    let question_starters = [
        "what", "who", "how", "why", "when", "where", "can you", "could you",
        "do you", "are you", "is there", "tell me", "explain", "describe",
        "hello", "hi", "hey", "thanks", "thank you",
    ];
    let coding_verbs = [
        "implement", "create", "add", "fix", "refactor", "write", "build",
        "generate", "update", "modify", "remove", "delete", "rename",
        "migrate", "integrate", "optimize", "deploy", "test",
    ];

    let is_question = message.trim_end().ends_with('?')
        || question_starters
            .iter()
            .any(|w| lower.starts_with(w));

    let has_coding_verb = coding_verbs.iter().any(|v| lower.contains(v));

    let is_conversational = is_question && !has_coding_verb
        || (word_count <= 8 && !has_coding_verb);

    if is_conversational {
        chat_state.send("progress", "💬 Thinking…");

        let ollama = crate::models::ollama::OllamaClient::new();
        let conv_history = history.format_for_prompt();

        match ollama.chat(&message, &conv_history).await {
            Ok(reply) => {
                // Persist the assistant turn.
                let assistant_turn = ConversationTurn {
                    role: "aethyron".to_string(),
                    content: reply.clone(),
                };
                if let Err(e) = MemoryStore::save_turn(&assistant_turn) {
                    eprintln!("⚠️  Could not persist assistant turn: {}", e);
                }

                chat_state.send("result", &reply);

                return axum::Json(serde_json::json!({
                    "reply": reply,
                    "tasks_completed": 0,
                    "files_changed": 0,
                    "repairs": 0,
                    "success": true,
                }));
            }
            Err(e) => {
                let msg = format!("❌ Could not reach Aethyron model: {}", e);
                chat_state.send("error", &msg);
                return axum::Json(serde_json::json!({ "error": msg }));
            }
        }
    }

    // ------------------------------------------------------------------
    // Engineering mission — run through the full Orchestrator pipeline.
    // ------------------------------------------------------------------

    // Notify SSE subscribers that the mission is starting.
    chat_state.send("progress", "🌌 Building project context…");

    // Build context with the current conversation history.
    let context = match ContextBuilder::build_with_conversation(
        ".",
        history.format_for_prompt(),
    ) {
        Ok(ctx) => ctx,
        Err(error) => {
            let msg = format!("❌ Could not build project context: {}", error);
            chat_state.send("error", &msg);
            return axum::Json(serde_json::json!({ "error": msg }));
        }
    };

    chat_state.send("progress", "🧭 Planning mission tasks…");

    let mission = Mission::new_with_context(&message, context);
    let result = Orchestrator::new().execute(mission).await;

    let summary = format!(
        "Mission complete. Tasks: {} | Files changed: {} | Repairs: {} | Success: {}",
        result.tasks_completed,
        result.files_changed.len(),
        result.repairs,
        result.success,
    );

    // Persist the assistant turn.
    let assistant_turn = ConversationTurn {
        role: "aethyron".to_string(),
        content: summary.clone(),
    };
    if let Err(e) = MemoryStore::save_turn(&assistant_turn) {
        eprintln!("⚠️  Could not persist assistant turn: {}", e);
    }

    // Broadcast the final result to SSE subscribers.
    chat_state.send("result", &summary);

    axum::Json(serde_json::json!({
        "reply": summary,
        "tasks_completed": result.tasks_completed,
        "files_changed": result.files_changed.len(),
        "repairs": result.repairs,
        "success": result.success,
    }))
}

// ---------------------------------------------------------------------------
// GET /chat/stream
// Server-Sent Events stream.  Clients connect here first, then POST /chat.
// They receive live progress events while the mission runs.
// ---------------------------------------------------------------------------

async fn chat_stream(
    State(chat_state): State<ChatState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = chat_state.tx.subscribe();

    let stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(chat_event) => {
                let data = serde_json::to_string(&chat_event)
                    .unwrap_or_else(|_| r#"{"kind":"error","message":"serialize error"}"#.to_string());
                let sse_event = Event::default().data(data);
                Some((Ok(sse_event), rx))
            }
            Err(_) => None,
        }
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn run_mission(
    Json(payload): Json<MissionRequest>,
) -> axum::Json<serde_json::Value> {
    let goal = payload.goal.trim().to_string();

    if goal.is_empty() {
        return axum::Json(serde_json::json!({
            "status": "error",
            "message": "Mission goal cannot be empty."
        }));
    }

    let goal_clone = goal.clone();

    tokio::spawn(async move {
        let mission = crate::core::orchestrator::Mission::new(&goal_clone);
        crate::core::orchestrator::Orchestrator::new()
            .execute(mission)
            .await;
    });

    axum::Json(serde_json::json!({
        "status": "accepted",
        "message": format!("Mission started: {}", goal)
    }))
}

async fn run_doctor() -> bool {
    println!("==============================");
    println!("AETHYRON DOCTOR");
    println!("==============================");

    let mut healthy = true;

    let cargo_ok = std::path::Path::new("Cargo.toml").is_file();
    println!(
        "{} Cargo.toml exists",
        if cargo_ok { "PASS" } else { "FAIL" }
    );

    if !cargo_ok {
        healthy = false;
    }

    let src_ok = std::path::Path::new("src").is_dir();
    println!(
        "{} src directory exists",
        if src_ok { "PASS" } else { "FAIL" }
    );

    if !src_ok {
        healthy = false;
    }

    match ProjectIndexer::build(std::path::Path::new(".")) {
        Ok(index) => {
            println!("PASS Project workspace can be indexed");
            println!("     Files indexed: {}", index.files.len());
        }

        Err(error) => {
            println!("FAIL Project workspace indexing: {}", error);
            healthy = false;
        }
    }

    match OllamaClient::new().check().await {
        Ok(true) => println!("PASS Ollama and qwen2.5-coder:7b available"),

        Ok(false) => {
            println!("FAIL Ollama reachable, but qwen2.5-coder:7b not found");
            healthy = false;
        }

        Err(error) => {
            println!("FAIL Ollama check: {}", error);
            healthy = false;
        }
    }

    println!("==============================");

    healthy
}

async fn run_chat() {
    println!("==============================");
    println!("AETHYRON CHAT");
    println!("==============================");
    println!("Type your engineering goal and press Enter.");
    println!("Type 'exit' or 'quit' to leave.");
    println!("Type 'history' to review the conversation.");
    println!("==============================");
    println!();

    // Load persisted conversation turns from previous sessions.
    let prior_turns = MemoryStore::load_conversation(20).unwrap_or_default();

    let mut history = ConversationHistory::new();

    for turn in prior_turns {
        history.add(turn.role, turn.content);
    }

    if !history.is_empty() {
        println!("📖 Resuming previous conversation ({} turns loaded).", history.turns.len());
        println!();
    }

    loop {
        print!("You > ");

        // Flush so the prompt appears before the user types.
        use std::io::Write;
        std::io::stdout().flush().ok();

        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            println!("Aethyron > Goodbye.");
            break;
        }

        if input == "history" {
            if history.is_empty() {
                println!("Aethyron > No conversation history yet.");
            } else {
                println!("---------- Conversation History ----------");
                println!("{}", history.format_for_prompt());
                println!("-----------------------------------------");
            }
            println!();
            continue;
        }

        // Persist the user turn.
        let user_turn = ConversationTurn {
            role: "user".to_string(),
            content: input.clone(),
        };

        if let Err(e) = MemoryStore::save_turn(&user_turn) {
            eprintln!("⚠️  Could not persist user turn: {}", e);
        }

        history.add("user", &input);

        // Build context with the current conversation history.
        let context = match ContextBuilder::build_with_conversation(
            ".",
            history.format_for_prompt(),
        ) {
            Ok(ctx) => ctx,
            Err(error) => {
                println!("Aethyron > ❌ Could not build project context: {}", error);
                println!();
                continue;
            }
        };

        println!();
        println!("Aethyron > 🌌 Running mission...");
        println!();

        let mission = Mission::new_with_context(&input, context);
        let result = Orchestrator::new().execute(mission).await;

        let summary = format!(
            "Mission complete. Tasks: {} | Files changed: {} | Repairs: {} | Success: {}",
            result.tasks_completed,
            result.files_changed.len(),
            result.repairs,
            result.success,
        );

        println!();
        println!("Aethyron > {}", summary);
        println!();

        // Persist the assistant turn.
        let assistant_turn = ConversationTurn {
            role: "aethyron".to_string(),
            content: summary.clone(),
        };

        if let Err(e) = MemoryStore::save_turn(&assistant_turn) {
            eprintln!("⚠️  Could not persist assistant turn: {}", e);
        }

        history.add("aethyron", summary);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("mcp") {
        let server = AethyronMcp::new();

        server
            .serve(rmcp::transport::stdio())
            .await
            .expect("Aethyron MCP server failed");

        return;
    }

    if args.first().map(String::as_str) == Some("chat") {
        run_chat().await;
        return;
    }

    if args.first().map(String::as_str) == Some("run") {
        let goal = args.get(1..).unwrap_or(&[]).join(" ");

        if goal.trim().is_empty() {
            eprintln!("Usage: cargo run -- run \"<mission>\"");
            std::process::exit(1);
        }

        let mission = Mission::new(&goal);
        Orchestrator::new().execute(mission).await;

        return;
    }

    if args.first().map(String::as_str) == Some("inspect") {
        match ProjectIndexer::build(std::path::Path::new(".")) {
            Ok(index) => println!("{}", index.summary()),

            Err(error) => {
                eprintln!("Inspect failed: {}", error);
                std::process::exit(1);
            }
        }

        return;
    }

    if args.first().map(String::as_str) == Some("doctor") {
        let healthy = run_doctor().await;

        if !healthy {
            std::process::exit(1);
        }

        return;
    }

    // ---------------------------------------------------------------
    // A2A
    // ---------------------------------------------------------------

    let a2a_executor =
        crate::a2a_executor::AethyronA2AExecutor::new();

    let a2a_task_store =
        InMemoryTaskStore::new();

    let a2a_handler = Arc::new(
        DefaultRequestHandler::new(
            a2a_executor,
            a2a_task_store,
        )
    );

    let agent_card = AgentCard {
        name: "Aethyron".to_string(),
        description: "Aethyron autonomous coding agent".to_string(),
        version: "0.1.0".to_string(),

        supported_interfaces: vec![
            AgentInterface::new(
                "http://127.0.0.1:3000/a2a",
                "JSONRPC",
            )
        ],

        capabilities: AgentCapabilities {
            streaming: Some(true),
            push_notifications: Some(false),
            extensions: None,
            extended_agent_card: None,
        },

        default_input_modes: vec![
            "text/plain".to_string()
        ],

        default_output_modes: vec![
            "text/plain".to_string()
        ],

        skills: vec![
            AgentSkill {
                id: "aethyron-mission".to_string(),
                name: "Aethyron Mission Execution".to_string(),
                description:
                    "Plans, generates, reviews, and repairs code for autonomous missions."
                        .to_string(),
                tags: vec![
                    "coding".to_string(),
                    "planning".to_string(),
                    "repair".to_string(),
                ],
                examples: Some(vec![
                    "Inspect and improve this Rust project".to_string(),
                    "Implement the requested feature".to_string(),
                ]),
                input_modes: None,
                output_modes: None,
                security_requirements: None,
            }
        ],

        provider: None,
        documentation_url: None,
        icon_url: None,
        security_schemes: None,
        security_requirements: None,
        signatures: None,
    };

    let a2a_card = Arc::new(
        StaticAgentCard::new(agent_card)
    );

    // ---------------------------------------------------------------
    // HTTP API + UI Static Files
    // ---------------------------------------------------------------

    let chat_state = ChatState::new();

    let ui_dir = std::path::Path::new("ui-dist");
    let ui_index = ui_dir.join("index.html");

    let app = Router::new()
        .route("/health", get(health))
        .route("/agents", get(agents))
        .route("/mission", post(run_mission))
        .route("/chat", post(post_chat))
        .route("/chat/stream", get(chat_stream))
        .nest("/a2a", jsonrpc_router(a2a_handler).with_state(()))
        .merge(agent_card_router(a2a_card).with_state(()))
        .fallback_service(
            ServeDir::new(ui_dir)
                .fallback(ServeFile::new(ui_index))
        )
        .layer(CorsLayer::very_permissive())
        .with_state(chat_state);

    let listener = tokio::net::TcpListener::bind(
        "127.0.0.1:3000"
    )
    .await
    .expect("failed to bind Aethyron API");

    println!("╔══════════════════════════════════════════╗");
    println!("║           AETHYRON  ONLINE               ║");
    println!("╠══════════════════════════════════════════╣");
    println!("║  UI      →  http://127.0.0.1:3000        ║");
    println!("║  API     →  http://127.0.0.1:3000/agents ║");
    println!("║  Health  →  http://127.0.0.1:3000/health ║");
    println!("║  Chat    →  http://127.0.0.1:3000/chat   ║");
    println!("║  Stream  →  http://127.0.0.1:3000/chat/stream ║");
    println!("║  A2A     →  http://127.0.0.1:3000/a2a    ║");
    println!("╚══════════════════════════════════════════╝");

    axum::serve(listener, app)
        .await
        .expect("Aethyron server failed");
}