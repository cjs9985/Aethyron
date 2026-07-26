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
       println!("Cargo.toml size: {}", context.cargo_toml.len());println!("Files discovered: {}", context.files.len());

let planner = PlannerAgent;
let coder = CoderAgent;
let reviewer = ReviewerAgent;
let task = Task {
    description: mission.goal.clone(),
};


println!("🧭 Creating mission plan...");

if let Some(plan) = planner.create_plan_with_context(&task, 
Some(&context),
).await {

   let mut queue = crate::core::task_queue::TaskQueue::new();
   let mut files_changed: Vec<String> = Vec::new();

for description in plan.tasks {
    
    queue.add(Task {
        description,
    });
}

let mut review_notes = Vec::new();

while let Some(task) = queue.next() {

    if let Some(request) = coder.execute(&task).await {

        if let Err(error) = ToolDispatcher::execute(request) {
            println!(
                "❌ Tool execution failed: {}",
                error
            );
        }
    }

    let changed_files = coder.execute_with_context(
        &task,
        &context,
    ).await;

    files_changed.extend(changed_files);
    let review = reviewer.review(&task).await;
    review_notes.push(review.notes);
}
let notes = review_notes.join("\n");
let result = MissionResult {
    mission_id: mission.id.to_string(),
    goal: mission.goal.clone(),
    success: true,
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
    }
} 
}
  
