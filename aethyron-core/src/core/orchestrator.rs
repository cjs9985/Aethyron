use std::time::Instant;

use uuid::Uuid;

use crate::agents::{Agent, Task, coder::CoderAgent, planner::PlannerAgent, reviewer::ReviewerAgent};

use crate::core::{
    context_builder::ContextBuilder,
    event_bus::EventBus,
    events::{Event, EventType},
};

use crate::memory::store::MemoryStore;

use crate::models::mission_result::MissionResult;
use crate::models::project_context::ProjectContext;

pub struct Mission {
    pub id: Uuid,
    pub goal: String,
    pub context: Option<ProjectContext>,
}

impl Mission {
    pub fn new(goal: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal: goal.to_string(),
            context: None,
        }
    }

    pub fn new_with_context(goal: &str, context: ProjectContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal: goal.to_string(),
            context: Some(context),
        }
    }
}

pub struct Orchestrator;

impl Orchestrator {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, mut mission: Mission) -> MissionResult {
        let mission_started_at = Instant::now();
        let bus = EventBus::new();

        bus.publish(Event::new(
            EventType::MissionCreated,
            "Orchestrator",
            format!("Mission {} created", mission.id),
        ));

        bus.publish(Event::new(
            EventType::MissionStarted,
            "Orchestrator",
            format!("Mission {} started", mission.id),
        ));

        println!("🌌 Aethyron Mission Started");
        println!("ID: {}", mission.id);
        println!("Goal: {}", mission.goal);

        let phase_started_at = Instant::now();

        let context = match mission.context.take().map(Ok).unwrap_or_else(|| ContextBuilder::build(".")) {
            Ok(context) => context,
            Err(error) => {
                println!("❌ Context build failed: {}", error);

                bus.publish(Event::new(
                    EventType::Error,
                    "ContextBuilder",
                    error.to_string(),
                ));

                return MissionResult {
                    mission_id: mission.id.to_string(),
                    goal: mission.goal.clone(),
                    success: false,
                    files_changed: Vec::new(),
                    tasks_completed: 0,
                    repairs: 0,
                    duration_ms: mission_started_at.elapsed().as_millis(),
                    notes: format!("Context build failed: {}", error),
                };
            }
        };

        println!(
            "📦 Context Built in {} ms",
            phase_started_at.elapsed().as_millis()
        );

        bus.publish(Event::new(
            EventType::ContextBuilt,
            "ContextBuilder",
            "Project context indexed",
        ));

        println!("Cargo.toml size: {}", context.cargo_toml.len());
        println!("Files discovered: {}", context.files.len());

        let planner = PlannerAgent;
        let coder = CoderAgent;
        let reviewer = ReviewerAgent;

        let task = Task {
            description: mission.goal.clone(),
        };

        // Drive Agent::execute for the planner — publishes AgentStarted/tool request.
        planner.execute(&task).await;

        println!("🧭 Creating mission plan...");

        bus.publish(Event::new(
            EventType::PlanningStarted,
            "Planner",
            mission.goal.clone(),
        ));

        bus.publish(Event::new(
            EventType::AgentStarted,
            "PlannerAgent",
            "Planning started",
        ));

        let planning_started_at = Instant::now();

        let plan = match planner
            .create_plan_with_context(&task, Some(&context))
            .await
        {
            Some(plan) => plan,
            None => {
                println!("❌ Planner failed to create a mission plan.");

                bus.publish(Event::new(
                    EventType::Error,
                    "PlannerAgent",
                    "Planner failed to create a mission plan.",
                ));

                return MissionResult {
                    mission_id: mission.id.to_string(),
                    goal: mission.goal.clone(),
                    success: false,
                    files_changed: Vec::new(),
                    tasks_completed: 0,
                    repairs: 0,
                    duration_ms: mission_started_at.elapsed().as_millis(),
                    notes: "Planner failed to create a mission plan.".to_string(),
                };
            }
        };

        println!(
            "📋 Planning completed in {} ms",
            planning_started_at.elapsed().as_millis()
        );

        bus.publish(Event::new(
            EventType::AgentCompleted,
            "PlannerAgent",
            format!("{} tasks generated", plan.tasks.len()),
        ));

        bus.publish(Event::new(
            EventType::PlanningCompleted,
            "Planner",
            format!("{} tasks generated", plan.tasks.len()),
        ));

        let mut queue = crate::core::task_queue::TaskQueue::new();
        let mut files_changed: Vec<String> = Vec::new();
        let mut review_notes: Vec<String> = Vec::new();

        let mut repairs = 0usize;
        let mut completed_tasks = 0usize;

        for description in plan.tasks {
            queue.add(Task { description });
        }

        if queue.is_empty() {
            println!("⚠️ Planner produced no tasks. Mission cannot proceed.");

            return MissionResult {
                mission_id: mission.id.to_string(),
                goal: mission.goal.clone(),
                success: false,
                files_changed: Vec::new(),
                tasks_completed: 0,
                repairs: 0,
                duration_ms: mission_started_at.elapsed().as_millis(),
                notes: "Planner produced an empty task list.".to_string(),
            };
        }

        while let Some(task) = queue.next() {
            let task_started_at = Instant::now();

            bus.publish(Event::new(
                EventType::TaskStarted,
                "TaskQueue",
                task.description.clone(),
            ));

            println!();
            println!("🔨 Task: {}", task.description);

            bus.publish(Event::new(
                EventType::AgentStarted,
                "CoderAgent",
                task.description.clone(),
            ));

            // Agent::execute drives tool inspection before code generation.
            coder.execute(&task).await;

            let coder_started_at = Instant::now();

            bus.publish(Event::new(
                EventType::ModelRequested,
                "CoderAgent",
                "Requesting code generation from model",
            ));

            let coder_result = coder.execute_with_context(&task, &context).await;

            bus.publish(Event::new(
                EventType::ModelCompleted,
                "CoderAgent",
                "Model response received",
            ));

            println!(
                "🧠 Code generation completed in {} ms",
                coder_started_at.elapsed().as_millis()
            );

            bus.publish(Event::new(
                EventType::CodeGenerated,
                "Coder",
                format!("{} files modified", coder_result.files_changed.len()),
            ));

            bus.publish(Event::new(
                EventType::AgentCompleted,
                "CoderAgent",
                format!("{} files modified", coder_result.files_changed.len()),
            ));

            files_changed.extend(coder_result.files_changed.clone());

            let review_started_at = Instant::now();

            bus.publish(Event::new(
                EventType::AgentStarted,
                "ReviewerAgent",
                "Review started",
            ));

            // Agent::execute performs the structured review via the trait interface.
            reviewer.execute(&task).await;

            bus.publish(Event::new(
                EventType::ModelRequested,
                "ReviewerAgent",
                "Requesting AI review from model",
            ));

            let mut review = reviewer.review(&task, &coder_result.generated_code).await;

            bus.publish(Event::new(
                EventType::ModelCompleted,
                "ReviewerAgent",
                "AI review received",
            ));

            println!(
                "🔍 Review completed in {} ms",
                review_started_at.elapsed().as_millis()
            );

            println!(
                "   Structural : {}",
                if review.structural { "✅" } else { "❌" }
            );
            println!(
                "   Security   : {}",
                if review.security { "✅" } else { "❌" }
            );
            println!(
                "   Compilation: {}",
                if review.compilation { "✅" } else { "❌" }
            );
            println!(
                "   AI Review  : {}",
                if review.ai_review { "✅" } else { "❌" }
            );

            if !review.passed {
                println!("⚠️ Review failed: {}", review.feedback);

                bus.publish(Event::new(
                    EventType::Error,
                    "ReviewerAgent",
                    review.feedback.clone(),
                ));

                let repair_started_at = Instant::now();

                let repair_result = crate::core::repair_engine::RepairEngine::repair(
                    review.feedback.clone(),
                    coder_result.generated_code.clone(),
                )
                .await;

                println!(
                    "🔧 Repair phase completed in {} ms",
                    repair_started_at.elapsed().as_millis()
                );

                match repair_result {
                    Ok(repaired_code) => {
                        repairs += 1;

                        println!("🔄 Repair completed.");

                        let rereview_started_at = Instant::now();

                        review = reviewer.review(&task, &repaired_code.content).await;

                        println!(
                            "🔍 Re-review completed in {} ms",
                            rereview_started_at.elapsed().as_millis()
                        );

                        if review.passed {
                            println!("✅ Re-review passed.");
                        } else {
                            println!("❌ Re-review failed: {}", review.feedback);
                        }
                    }

                    Err(error) => {
                        println!("❌ Repair failed: {}", error);
                    }
                }
            }

            bus.publish(Event::new(
                EventType::AgentCompleted,
                "ReviewerAgent",
                review.feedback.clone(),
            ));

            review_notes.push(review.feedback.clone());

            if review.passed && !coder_result.files_changed.is_empty() {
                completed_tasks += 1;

                println!("✅ Task completed: {}", task.description);
            } else if review.passed && coder_result.files_changed.is_empty() {
                println!("❌ Task not completed: no files were modified.");
            } else {
                println!("❌ Task not completed: {}", task.description);
            }

            println!(
                "⏱️ Task duration: {} ms",
                task_started_at.elapsed().as_millis()
            );
        }

        let notes = review_notes.join("\n");

        let success = completed_tasks == review_notes.len()
            && !review_notes.is_empty()
            && review_notes
                .iter()
                .all(|note| !note.to_lowercase().contains("failed"));

        let total_duration_ms = mission_started_at.elapsed().as_millis();

        let result = MissionResult {
            mission_id: mission.id.to_string(),
            goal: mission.goal.clone(),
            success,
            files_changed,
            tasks_completed: completed_tasks,
            repairs,
            duration_ms: total_duration_ms,
            notes,
        };

        let memory_started_at = Instant::now();

        match MemoryStore::save_result(&result) {
            Ok(_) => {
                println!(
                    "🧠 Structured mission result stored in {} ms.",
                    memory_started_at.elapsed().as_millis()
                );
            }

            Err(error) => {
                println!("❌ Memory save failed: {}", error);
            }
        }

        bus.publish(Event::new(
            EventType::MissionCompleted,
            "Orchestrator",
            format!(
                "Mission {} completed. Success: {}",
                result.mission_id, result.success
            ),
        ));

        println!();
        println!("========== Mission Summary ==========");
        println!("Tasks Completed : {}", result.tasks_completed);
        println!("Files Changed   : {}", result.files_changed.len());
        println!("Repairs         : {}", result.repairs);
        println!("Duration        : {} ms", result.duration_ms);
        println!("Success         : {}", result.success);
        println!("=====================================");

        result
    }
}
