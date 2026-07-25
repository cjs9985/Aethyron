use uuid::Uuid;
use crate::memory::store::MemoryStore;
use crate::agents::{
    Agent,
    coder::CoderAgent,
    Task,
    planner::PlannerAgent,
    reviewer::ReviewerAgent,
};
use crate::core::{
    event_bus::EventBus,
    events::{Event, EventType},
    context_builder::ContextBuilder,
};

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

for description in plan.tasks {

    queue.add(Task {
        description,
    });
}


while let Some(task) = queue.next() {

    coder.execute_with_context(
        &task,
        &context,
    ).await;
    reviewer.execute(&task).await;
}
let summary = format!(
    "Mission completed: {}",
    mission.goal
);

match MemoryStore::save(&summary) {

    Ok(_) => {
        println!("🧠 Mission stored in memory.");
    }

    Err(error) => {
        println!("❌ Memory save failed: {}", error);
    }
}
    }
} 
}
  
