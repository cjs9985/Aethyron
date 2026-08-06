use uuid::Uuid;
use crate::memory::store::MemoryStore;
use crate::agents::{
    coder::CoderAgent,
    Task,
    Agent,
    planner::PlannerAgent,
    reviewer::ReviewerAgent,
};
use crate::core::{
    event_bus::EventBus,
    events::{Event, EventType},
    context_builder::ContextBuilder,
};
use crate::models::{ mission_result::MissionResult,
};
use crate::tools::dispatcher::ToolDispatcher;

#[derive(Debug)]
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

        let context = ContextBuilder::build().unwrap();

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

if let Some(plan) = planner.create_plan_with_context(&task, 
Some(&context),
).await {
   bus.publish(Event::new(
    EventType::PlanningCompleted,
    "Planner",
    format!("{} tasks generated", plan.tasks.len()),
));
   let mut queue = crate::core::task_queue::TaskQueue::new();
   let mut files_changed: Vec<String> = Vec::new();

for description in plan.tasks {
    
    queue.add(Task {
        description,
    });
}

let mut review_notes = Vec::new();

let repairs = 0usize;
let mut completed_tasks = 0usize;

while let Some(task) = queue.next() {
    bus.publish(Event::new(
    EventType::TaskStarted,
    "TaskQueue",
    task.description.clone(),
));
    if let Some(request) = coder.execute(&task).await {

        if let Err(error) = ToolDispatcher::execute(request) {
            println!(
                "❌ Tool execution failed: {}",
                error
            );
        }
        bus.publish(Event::new(
    EventType::MissionCompleted,
    "Orchestrator",
    mission.goal.clone(),
));
    }

    let coder_result = coder.execute_with_context(
        &task,
        &context,
    ).await;

    bus.publish(Event::new(
    EventType::CodeGenerated,
    "Coder",
    format!(
        "{} files modified",
        coder_result.files_changed.len()
    ),
));
let review = reviewer.review(
    &task,
    &coder_result.generated_code,
).await;

if !review.passed {
    println!(
        "⚠️ Review failed: {}",
        review.feedback
    );
}

completed_tasks += 1;

files_changed.extend(coder_result.files_changed.clone());

review_notes.push(review.feedback);

}
let notes = review_notes.join("\n");

let success = review_notes.iter().all(|n| {!n.to_lowercase().contains("failed")});

let result = MissionResult {
    mission_id: mission.id.to_string(),
    goal: mission.goal.clone(),
    success,
    files_changed: files_changed.clone(),
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
println!("========== Mission Summary ==========");
println!("Tasks Completed : {}", completed_tasks);
println!("Files Changed   : {}", files_changed.len());
println!("Repairs         : {}", repairs);
println!("Success         : {}", success);
println!("=====================================");
    }
} 
}
  
