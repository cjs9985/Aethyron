use uuid::Uuid;

use crate::agents::{
    Agent,
    coder::CoderAgent,
    Task,
    planner::PlannerAgent,
};
use crate::core::{
    event_bus::EventBus,
    events::{Event, EventType},
    context_builder::ContextBuilder,
};
use crate::tools::filesystem::FileSystem;
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
        let task = Task {
            description: mission.goal.clone(),
};
       let tool_request =
    planner.execute(&task).await;

    coder.execute_with_context(
    &task, &context,).await;

if let Some(request) = tool_request {

    println!(
        "🛠 Tool requested: {:?}",
        request
    );

    let result =
        FileSystem::inspect_project_result();

    println!("🔧 Tool Result:");
    println!("{}", result.output);
}
  }
}