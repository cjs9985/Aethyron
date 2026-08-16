use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum EventType {
    MissionCreated,
    MissionStarted,

    ContextBuilt,

    PlanningStarted,
    PlanningCompleted,

    TaskStarted,

    AgentStarted,
    ModelRequested,
    ModelCompleted,
    AgentCompleted,

    CodeGenerated,

    MissionCompleted,

    Error,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub source: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl Event {
    pub fn new(
        event_type: EventType,
        source: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            event_type,
            source: source.into(),
            message: message.into(),
            timestamp: Utc::now(),
        }
    }
}
