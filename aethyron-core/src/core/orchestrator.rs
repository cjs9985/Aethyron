use uuid::Uuid;

use crate::agents::{
    Agent,
    Task,
    planner::PlannerAgent,
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

        println!("🌌 Aethyron Mission Started");
        println!("ID: {}", mission.id);
        println!("Goal: {}", mission.goal);

        let planner = PlannerAgent;

        let task = Task {
    description: mission.goal,
};
        planner.execute(task).await;

    }
}