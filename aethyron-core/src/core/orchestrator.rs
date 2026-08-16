use uuid::Uuid;

use crate::agents::{
    Agent, Task, coder::CoderAgent, planner::PlannerAgent, reviewer::ReviewerAgent,
};

use crate::core::{
    context_builder::ContextBuilder,
    event_bus::EventBus,
    events::{Event, EventType},
};

use crate::memory::store::MemoryStore;

use crate::models::mission_result::MissionResult;

pub struct Mission {
    pub id: Uuid,
    pub goal: String,
}

impl Mission {
    pub fn new(goal: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal: goal.to_string(),
        }
    }
}

pub struct Orchestrator;

impl Orchestrator {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, mission: Mission) {
        let bus = EventBus::new();

        bus.publish(Event::new(
            EventType::MissionStarted,
            "Orchestrator",
            format!("Mission {} started", mission.id),
        ));

        println!("🌌 Aethyron Mission Started");
        println!("ID: {}", mission.id);
        println!("Goal: {}", mission.goal);

        let context = match ContextBuilder::build() {
            Ok(context) => context,
            Err(error) => {
                println!("❌ Context build failed: {}", error);
                return;
            }
        };

        println!("📦 Context Built");

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

        println!("🧭 Creating mission plan...");

        bus.publish(Event::new(
            EventType::PlanningStarted,
            "Planner",
            mission.goal.clone(),
        ));

        let plan = match planner
            .create_plan_with_context(&task, Some(&context))
            .await
        {
            Some(plan) => plan,
            None => {
                println!("❌ Planner failed to create a mission plan.");
                return;
            }
        };

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

        while let Some(task) = queue.next() {
            bus.publish(Event::new(
                EventType::TaskStarted,
                "TaskQueue",
                task.description.clone(),
            ));

            println!();
            println!("🔨 Task: {}", task.description);

            let coder_result = coder.execute_with_context(&task, &context).await;

            bus.publish(Event::new(
                EventType::CodeGenerated,
                "Coder",
                format!("{} files modified", coder_result.files_changed.len()),
            ));

            files_changed.extend(coder_result.files_changed.clone());

            let mut review = reviewer.review(&task, &coder_result.generated_code).await;

            if !review.passed {
                println!("⚠️ Review failed: {}", review.feedback);

                let repair_result = crate::core::repair_engine::RepairEngine::repair(
                    review.feedback.clone(),
                    coder_result.generated_code.clone(),
                )
                .await;

                match repair_result {
                    Ok(_) => {
                        repairs += 1;

                        println!("🔄 Repair completed.");

                        review = reviewer.review(&task, &coder_result.generated_code).await;

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

            review_notes.push(review.feedback.clone());

            if review.passed && !coder_result.files_changed.is_empty() {
                completed_tasks += 1;

                println!("✅ Task completed: {}", task.description);
            } else {
                if review.passed && coder_result.files_changed.is_empty() {
                    println!("❌ Task not completed: no files were modified.");
                } else {
                    println!("❌ Task not completed: {}", task.description);
                }
            }
        }

        let notes = review_notes.join("\n");

        let success = completed_tasks == review_notes.len()
            && !review_notes.is_empty()
            && review_notes
                .iter()
                .all(|note| !note.to_lowercase().contains("failed"));

        let result = MissionResult {
            mission_id: mission.id.to_string(),
            goal: mission.goal.clone(),
            success,
            files_changed,
            notes,
        };

        match MemoryStore::save_result(&result) {
            Ok(_) => {
                println!("🧠 Structured mission result stored.");
            }

            Err(error) => {
                println!("❌ Memory save failed: {}", error);
            }
        }

        println!();
        println!("========== Mission Summarpy ==========");
        println!("Tasks Completed : {}", completed_tasks);
        println!("Files Changed   : {}", result.files_changed.len());
        println!("Repairs         : {}", repairs);
        println!("Success         : {}", success);
        println!("=====================================");
    }
}
